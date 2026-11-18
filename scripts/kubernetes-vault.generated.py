"""Local Vault bootstrap. Private recovery state stays outside generated source."""
import json
import hashlib
import hmac
import os
import base64
import re
import secrets
import time
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
os.chdir(ROOT)
os.umask(0o077)
KUBE = ['docker', 'compose', '-f', 'docker-compose.kubernetes.yml', 'exec', '-T',
        'kubernetes', 'kubectl', '-n', 'vault-system', 'exec', '-i', 'vault-0', '--']
SERVICE_TARGETS = {
    'analyticsservice-env': [{'kind': 'Deployment', 'name': 'analyticsservice'}],
    'automationservice-env': [{'kind': 'Deployment', 'name': 'automationservice'}],
    'inventoryservice-env': [{'kind': 'Deployment', 'name': 'inventoryservice'}],
    'orderservice-env': [{'kind': 'Deployment', 'name': 'orderservice'}],
}
STATE = ROOT / '.artifacts/kubernetes/vault-bootstrap.json'

def command(arguments, data=None, allowed=(0,)):
    result = subprocess.run(arguments, input=data, capture_output=True)
    if result.returncode not in allowed:
        raise RuntimeError('Vault bootstrap command failed; inspect pod state (credentials suppressed)')
    return result.stdout

def vault(*args, data=None):
    state = json.loads(STATE.read_text())
    token = state['root_token'].encode() + b'\n'
    return command(KUBE + ['sh', '-c', 'IFS= read -r VAULT_TOKEN; export VAULT_TOKEN; exec vault "$@"', 'vault'] + list(args), token + (data or b''))

def bootstrap():
    status = json.loads(command(KUBE + ['vault', 'status', '-format=json'], allowed=(0,2)))
    if not status['initialized']:
        if STATE.exists():
            raise RuntimeError('Vault storage is empty but recovery state exists; inspect storage before initialization')
        STATE.parent.mkdir(parents=True, exist_ok=True)
        # Reserve a private recovery file before issuing the irreversible init.
        with STATE.open('x') as recovery:
            payload = command(KUBE + ['vault', 'operator', 'init', '-key-shares=1', '-key-threshold=1', '-format=json'])
            recovery.write(payload.decode())
            recovery.flush()
            os.fsync(recovery.fileno())
    if not STATE.is_file() or STATE.stat().st_mode & 0o077:
        raise RuntimeError('Private Vault recovery state is missing or permissions are not 0600; restore it, never reinitialize storage')
    state = json.loads(STATE.read_text())
    if status['sealed']:
        command(KUBE + ['vault', 'write', '-format=json', 'sys/unseal', 'key=-'], state['unseal_keys_b64'][0].encode())
    mounts = json.loads(vault('secrets', 'list', '-format=json'))
    if 'secret/' not in mounts:
        vault('secrets', 'enable', '-path=secret', 'kv-v2')
    elif mounts['secret/'].get('options', {}).get('version') != '2':
        raise RuntimeError('Existing secret mount is not KV v2')
    auth = json.loads(vault('auth', 'list', '-format=json'))
    if 'kubernetes/' not in auth:
        vault('auth', 'enable', 'kubernetes')
    vault('write', 'auth/kubernetes/config', 'kubernetes_host=https://kubernetes.default.svc:443')
    print('Vault initialized/unsealed; private recovery data retained locally, Kubernetes auth enabled.')

def existing_secret(namespace, name):
    kubectl = KUBE[:KUBE.index('-n')]
    payload = command(kubectl + ['-n', namespace, 'get', 'secret', name, '--ignore-not-found', '-o', 'json'])
    if not payload.strip():
        return None
    document = json.loads(payload)
    return {key: base64.b64decode(value).decode() for key, value in document.get('data', {}).items() if key != '_raw'}

def synchronize(namespace, name, payload, replace=False, targets=()):
    # Names also delimit Vault policy paths; never accept policy/path syntax.
    for value in (namespace, name):
        if not re.fullmatch(r'[a-z0-9]([-a-z0-9]*[a-z0-9])?', value) or len(value) > 63:
            raise ValueError('Namespace and Secret must be DNS labels')
    values = json.loads(payload)
    if not isinstance(values, dict) or any(not isinstance(v, str) for v in values.values()):
        raise ValueError('Secret input must be a JSON object of string values')
    if any(not re.fullmatch(r'[-._a-zA-Z0-9]+', key) for key in values):
        raise ValueError('Invalid Secret data key')
    location = f'local/{namespace}/{name}'
    role = f'{namespace}.{name}'
    # CAS=0 seeds once. Never overwrite a value changed through Vault on startup.
    if replace:
        vault('kv', 'put', '-mount=secret', location, '-', data=json.dumps(values).encode())
    else:
        try:
            vault('kv', 'get', '-mount=secret', location)
        except RuntimeError:
            previous = existing_secret(namespace, name)
            if previous is not None:
                values = previous
            vault('kv', 'put', '-cas=0', '-mount=secret', location, '-', data=json.dumps(values).encode())
    expected = json.loads(vault('kv', 'get', '-format=json', '-mount=secret', location))['data']['data']
    policy = 'path "secret/data/' + location + '" { capabilities = ["read"] }'
    vault('policy', 'write', role, '-', data=policy.encode())
    vault('write', f'auth/kubernetes/role/{role}',
          f'bound_service_account_names={name}', f'bound_service_account_namespaces={namespace}',
          f'policies={role}', 'ttl=1h')
    documents = [
        {'apiVersion': 'v1', 'kind': 'ServiceAccount', 'metadata': {'name': name, 'namespace': namespace}},
        {'apiVersion': 'secrets.hashicorp.com/v1beta1', 'kind': 'VaultAuth',
         'metadata': {'name': name, 'namespace': namespace},
         'spec': {'vaultConnectionRef': 'vault-system/default', 'method': 'kubernetes',
                  'mount': 'kubernetes', 'kubernetes': {'role': role, 'serviceAccount': name}}},
        {'apiVersion': 'secrets.hashicorp.com/v1beta1', 'kind': 'VaultStaticSecret',
         'metadata': {'name': name, 'namespace': namespace},
         'spec': {'vaultAuthRef': name, 'mount': 'secret', 'type': 'kv-v2', 'path': location,
                  'refreshAfter': '30s', 'destination': {'create': True, 'overwrite': True, 'name': name, 'transformation': {'excludeRaw': True}},
                  'rolloutRestartTargets': list(targets) or SERVICE_TARGETS.get(name, [])}}]
    kubectl = KUBE[:KUBE.index('-n')]
    command(kubectl + ['apply', '-f', '-'], json.dumps({'apiVersion': 'v1', 'kind': 'List', 'items': documents}).encode())
    deadline = time.monotonic() + 180
    while time.monotonic() < deadline:
        try:
            state = json.loads(command(kubectl + ['-n', namespace, 'get', 'secret', name, '-o', 'json']))
            if all(base64.b64decode(state.get('data', {}).get(k, '')).decode() == v for k, v in expected.items()):
                return
        except RuntimeError:
            pass
        time.sleep(2)
    raise RuntimeError(f'Vault Secret synchronization timed out: {namespace}/{name}')

def rabbitmq_namespace(namespace):
    if not re.fullmatch(r'[a-z0-9]([-a-z0-9]*[a-z0-9])?', namespace) or len(namespace) > 63:
        raise ValueError('RabbitMQ namespace must be a DNS label')
    return namespace

def render_rabbitmq_manifest(namespace):
    namespace = rabbitmq_namespace(namespace)
    manifest = (ROOT / 'kubernetes/rabbitmq-cluster.generated.yaml').read_text()
    if '__RABBITMQ_NAMESPACE__' not in manifest:
        raise ValueError('RabbitMQ manifest lacks namespace placeholder')
    return manifest.replace('__RABBITMQ_NAMESPACE__', namespace)

def seed_rabbitmq(namespace):
    namespace = rabbitmq_namespace(namespace)
    location = f'local/{namespace}/rabbitmq-admin'
    role = f'{namespace}.rabbitmq-server'
    # Preserve existing Vault values, including a rotated password. CAS=0 also
    # prevents a concurrent initializer from overwriting credentials.
    try:
        vault('kv', 'get', '-mount=secret', location)
    except RuntimeError:
        previous = existing_secret(namespace, 'rabbitmq-admin')
        values = ({key: previous[key] for key in ('username', 'password')} if previous is not None
                  else {'username': 'servicegen', 'password': secrets.token_urlsafe(32)})
        vault('kv', 'put', '-cas=0', '-mount=secret', location, '-', data=json.dumps(values).encode())
    policy = 'path "secret/data/' + location + '" { capabilities = ["read"] }'
    vault('policy', 'write', role, '-', data=policy.encode())
    # Cluster Operator owns/creates rabbitmq-server. No wildcard SA, namespace,
    # metadata listing, write privilege, or VSO copy of the admin secret needed.
    vault('write', f'auth/kubernetes/role/{role}',
          'bound_service_account_names=rabbitmq-server',
          f'bound_service_account_namespaces={namespace}',
          f'policies={role}', 'ttl=1h')

def seed_pulsar(namespace):
    # ASCII signing material is also valid binary HMAC key material. Keeping it
    # textual lets the existing Vault/VSO contract carry the exact mounted bytes.
    targets = [{'kind': 'Deployment', 'name': 'pulsar-standalone'}]
    synchronize(namespace, 'pulsar-token-symmetric-key',
                json.dumps({'SECRETKEY': secrets.token_urlsafe(48)}), targets=targets)
    key = existing_secret(namespace, 'pulsar-token-symmetric-key')['SECRETKEY']
    def b64(payload):
        return base64.urlsafe_b64encode(payload).decode().rstrip('=')
    signing_input = '.'.join(b64(json.dumps(value, separators=(',', ':')).encode())
                             for value in ({'alg': 'HS256', 'typ': 'JWT'}, {'sub': 'admin'}))
    token = signing_input + '.' + b64(hmac.new(key.encode(), signing_input.encode(), hashlib.sha256).digest())
    synchronize(namespace, 'pulsar-token-admin', json.dumps({'TOKEN': token}), targets=targets)

if __name__ == '__main__':
    try:
        if sys.argv[1:] == ['bootstrap']:
            bootstrap()
        elif len(sys.argv) == 3 and sys.argv[1] == 'seed-rabbitmq':
            seed_rabbitmq(sys.argv[2])
        elif len(sys.argv) == 3 and sys.argv[1] == 'rabbitmq-manifest':
            print(render_rabbitmq_manifest(sys.argv[2]), end='')
        elif len(sys.argv) == 3 and sys.argv[1] == 'seed-pulsar':
            seed_pulsar(sys.argv[2])
        elif len(sys.argv) == 4 and sys.argv[1] == 'seed-infrastructure':
            namespace, name = sys.argv[2:]
            if name == 'grafana-admin':
                values = existing_secret(namespace, 'monitoring-grafana') or {'admin-user': 'admin', 'admin-password': os.environ.get('KUBERNETES_GRAFANA_ADMIN_PASSWORD') or secrets.token_urlsafe(32)}
            elif name == 'rabbitmq-admin':
                seed_rabbitmq(namespace)
                raise SystemExit(0)
            elif name == 'redis-streams-auth':
                values = {'password': os.environ.get('KUBERNETES_REDIS_STREAMS_PASSWORD') or secrets.token_urlsafe(32)}
            elif name == 'argocd-redis':
                values = {'auth': secrets.token_urlsafe(32)}
            elif name == 'temporal-postgresql':
                values = {'password': os.environ.get('KUBERNETES_TEMPORAL_POSTGRES_PASSWORD') or secrets.token_urlsafe(32)}
            else:
                raise ValueError('Unknown infrastructure secret')
            targets = []
            if name == 'redis-streams-auth':
                targets = [{'kind': 'StatefulSet', 'name': 'redis-streams'}]
            if name == 'argocd-redis':
                targets = [{'kind': 'Deployment', 'name': n} for n in ('argocd-redis', 'argocd-server', 'argocd-repo-server')]
                targets.append({'kind': 'StatefulSet', 'name': 'argocd-application-controller'})
            synchronize(namespace, name, json.dumps(values), targets=targets)
        elif len(sys.argv) == 4 and sys.argv[1] == 'seed-literals':
            values = {}
            for item in sys.stdin.buffer.read().split(b'\0'):
                if item:
                    key, value = item.decode().removeprefix('--from-literal=').split('=', 1)
                    values[key] = value
            name = sys.argv[3]
            targets = []
            if name == os.environ.get('KUBERNETES_KAFKA_SERVICE_SECRET', 'servicegen-kafka'):
                targets = [target for group in SERVICE_TARGETS.values() for target in group]
            synchronize(sys.argv[2], name, json.dumps(values), targets=targets)
        elif len(sys.argv) == 4 and sys.argv[1] in ('seed', 'put'):
            synchronize(sys.argv[2], sys.argv[3], sys.stdin.buffer.read(), replace=sys.argv[1] == 'put')
        else:
            raise ValueError('Usage: kubernetes-vault.generated.py bootstrap | seed/put NAMESPACE SECRET < private.json')
    except (RuntimeError, ValueError, OSError, KeyError):
        print('Vault operation failed; inspect the operator/pod status. Secret contents suppressed.', file=sys.stderr)
        raise SystemExit(1)