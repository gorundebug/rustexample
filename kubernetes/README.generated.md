# Local Kubernetes

The project owns one local k3s cluster and OCI registry. Each service owns its
independently installable Helm chart under `<service>/helm`. The local cluster
also installs pinned Redpanda, Prometheus, Grafana, OpenTelemetry Collector,
Jaeger and Loki releases so metrics, traces, logs and generated dashboards can
be verified together.

```bash
make kubernetes-up       # build runtime images, start k3s, deploy and verify
make kubernetes-services-up # rebuild and replace only services in the running cluster
make kubernetes-status   # show nodes, pods, services and Helm releases
make kubernetes-test     # verify probes, metrics and the order -> Kafka -> analytics flow
make kubernetes-down     # stop the local cluster and preserve its volumes
make kubernetes-clean    # remove the cluster, registry and all local volumes
```

Set `KUBERNETES_API_PORT=0 KUBERNETES_REGISTRY_PORT=0` for Docker-assigned
host ports. Image publication automatically discovers the assigned registry
port; fixed ports and an explicit `KUBERNETES_REGISTRY_HOST` remain supported.

When `DEPENDENCY_PROXY_DIR` is set, Helm repositories and all external OCI
registries used by the generated infrastructure are routed through the shared
Nexus instance. k3s disables direct registry fallback in this mode: a missing
or unavailable proxy is an explicit deployment failure, not an unnoticed
Internet download. Without the variable, Kubernetes uses the normal public
registries. The persistent k3s containerd volume reuses pulled image layers
between ordinary `kubernetes-down` / `kubernetes-up` cycles.
`kubernetes-services-up` keeps the installed infrastructure intact and is the
fast path for testing another service implementation against the same local
Redpanda, Temporal and observability stack.

Use `<service>/helm/values.yaml` for project-specific overrides. Generated
defaults stay in `values.generated.yaml` and are replaced on regeneration.
Production clusters use the same service charts with their own registry,
resources, security policy, ingress, secrets and Kafka configuration.

The service charts remain observability-vendor-neutral. Local OTLP endpoints,
backends and dashboard provisioning are applied only by the project-level
`kubernetes.generated.sh` orchestration.

## Inspecting the local observability stack

The stack is reachable through explicit port forwards:

```bash
# Grafana (admin; password stored in Vault at grafana-admin)
docker compose -f docker-compose.kubernetes.yml exec kubernetes \
  kubectl -n rustexample port-forward service/monitoring-grafana 3000:80

# Prometheus
docker compose -f docker-compose.kubernetes.yml exec kubernetes \
  kubectl -n rustexample port-forward \
  service/monitoring-kube-prometheus-prometheus 9090:9090

# Jaeger
docker compose -f docker-compose.kubernetes.yml exec kubernetes \
  kubectl -n rustexample port-forward service/jaeger 16686:16686
```

Set `KUBERNETES_NAMESPACE` when using a namespace other than the generated
default. Loki uses a local persistent volume and Jaeger uses ephemeral memory;
production retention and storage remain cluster-operator responsibilities.

## Secrets and Kafka authentication

Non-secret runtime settings are rendered into a `ConfigMap`. Secret values are
never generated into chart values or committed files. Each service chart exposes
the vendor-neutral `secretEnvFrom` list, so production may use a Secret created
by Vault, External Secrets Operator, Sealed Secrets, a cloud provider or ordinary
`kubectl` without changing the chart.

The local cluster can exercise real Redpanda SASL/SCRAM authentication. The
script seeds Vault from the process environment once; Vault Secrets Operator
creates the broker and service Secrets and references the service Secret through `secretEnvFrom`:

```bash
KUBERNETES_KAFKA_SASL_ENABLED=true \
KUBERNETES_KAFKA_USERNAME=servicegen-local \
KUBERNETES_KAFKA_PASSWORD='replace-this-local-password' \
make kubernetes-up
```

`KUBERNETES_KAFKA_SASL_MECHANISM` defaults to `SCRAM-SHA-512`.
`KUBERNETES_KAFKA_SUPERUSERS_SECRET` and `KUBERNETES_KAFKA_SERVICE_SECRET`
select the local Vault-managed Secret names.

The Compose cluster is local development infrastructure, not a production
Kubernetes distribution.

## Temporal

The graph contains a Temporal connector, so the local environment also starts
the pinned Temporal Server, matching Admin Tools, PostgreSQL persistence and Web
UI. Compose exposes the frontend on `localhost:${TEMPORAL_PORT:-7233}` and the UI
on `http://localhost:${TEMPORAL_UI_PORT:-8080}`.

The Kubernetes path installs the official pinned Temporal chart and a local
PostgreSQL StatefulSet. Vault stores a randomly generated database password,
or the initial `KUBERNETES_TEMPORAL_POSTGRES_PASSWORD` override; it is
never rendered into a ConfigMap or generated values file. Generated service
configuration uses the cluster-internal `temporal-frontend:7233` address.

Observability keeps three metric owners separate. Temporal Server exports its
official server metrics; each supported SDK/Worker process exports official SDK
metrics on port `9464`; and ServiceLib exports application/graph metrics on the
service HTTP metrics handler. Prometheus scrapes all three sources in both
Compose and Kubernetes. ServiceLib does not remeasure Temporal queue, Workflow,
Activity, or retry latency; dashboards present the official Temporal endpoint
series and add only graph-level context.

Production deployments should replace the local PostgreSQL manifest and Secret
with operator-owned persistence and secret management while retaining the same
service chart and connector environment contract.

## Argo CD and Vault Community

Local startup installs pinned Argo CD, persistent Vault Community and Vault
Secrets Operator before application infrastructure. Vault is not in dev mode.
The bootstrap recovery file `.artifacts/kubernetes/vault-bootstrap.json` contains
the local root token and unseal key, has owner-only permissions and must never be
committed. Keep this file together with the cluster volumes: ordinary `down`
preserves both. If Vault storage is lost, restore it or explicitly remove the
obsolete recovery file before initializing a new empty instance.

Argo CD runs in namespace `argocd`. It is installed for GitOps use; local Helm
commands still own deployments. Configure an Application and stop using local
Helm deployment commands for that application when adopting GitOps. No repository
credentials or invented Git repository are generated.

Workload secrets use Vault KV v2 at `secret/local/<namespace>/<secret-name>`.
Each service receives `<service>-env` through the existing `secretEnvFrom` chart
setting. Its environment names are the same names used by generated config.
Kubernetes authentication has a separate read-only Vault path policy per Secret.
Applications need neither a Vault SDK nor a root token.

Before the first service deployment, start the platform and import a private JSON
object of environment names and string values. Repeat `put` later to rotate it:

```bash
bash scripts/kubernetes.generated.sh infra-up
python3 scripts/kubernetes-vault.generated.py put <namespace> <service>-env < /private/path/service-secrets.json
make kubernetes-up
```

The helper reads secret contents from stdin. The operator refreshes every 30s and
restarts the service Deployment or StatefulSet after a change, because environment
variables are read at process startup. Normal startup seeds only missing paths;
it never replaces values already stored in Vault. Use `put` for deliberate updates.

Grafana and Temporal credentials are seeded once, using random passwords unless
`KUBERNETES_GRAFANA_ADMIN_PASSWORD` or `KUBERNETES_TEMPORAL_POSTGRES_PASSWORD` is
provided. Kafka authentication also goes through Vault. Database and broker
password rotation must include updating their internal accounts; changing a KV
value alone is not a database-account rotation procedure.

Access Argo CD and Vault through loopback port forwarding using the generated
kubeconfig. They are ClusterIP services, not public endpoints. Vault uses HTTP
inside this local cluster; production transport and unseal policies belong to the
production deployment.

## Local NATS and JetStream

NATS declarations select one shared local broker. Only a JetStream declaration
turns on JetStream and its persistent volume. The official image and chart are
pinned in the dependency manifest. Service charts remain independent of the
broker release. This script uses only the generated local Kubernetes environment.

No stream or consumer is created by the deployment templates. Existing resources
remain required unless an endpoint explicitly enables `createStream` or
`createConsumer`; those endpoint settings are handled by the service runtime.

Server URLs are not inferred or rewritten. For local use, explicitly set the
connector's `servers` to `["nats://nats:4222"]`. To override it through the declared
`envServers` binding, use the encoding required by the generated service language.
In these shell examples, replace `EVENTS_SERVERS` with that binding's actual name:

| Generated service | Environment value example |
| --- | --- |
| Go | `export EVENTS_SERVERS='nats://first:4222,nats://second:4222'` |
| C++ Coro | `export EVENTS_SERVERS='nats://first:4222,nats://second:4222'` |
| C++ userver | `export EVENTS_SERVERS='nats://first:4222,nats://second:4222'` |
| Python | `export EVENTS_SERVERS='nats://first:4222,nats://second:4222'` |
| TypeScript | `export EVENTS_SERVERS='nats://first:4222,nats://second:4222'` |
| Rust | `export EVENTS_SERVERS='["nats://first:4222","nats://second:4222"]'` |

Go and both C++ backends split on commas without trimming; omit whitespace and
empty entries. Python and TypeScript trim comma-separated entries and discard
empty entries. Rust requires a JSON array of strings, even for a single URL;
a comma-separated list or a bare URL is not valid for Rust. Keep the shell quotes
shown above so the JSON double quotes reach the process unchanged.

Compose passes the binding's exact string through; the local Kubernetes script
applies a supplied binding as a Helm env override.
An unset binding preserves the authored runtime configuration. User values.yaml
can also override `env`. Use `KUBERNETES_NATS_ENABLED=false` for an external-only
Kubernetes deployment. Compose can start individual application services without
the local broker: NATS endpoints have no dependency on a local broker or peers.

Credentials stay in the existing Vault/environment secret system; server URLs
must not embed credentials. No local NATS password is generated. The local client
port is bound to loopback on the host; monitoring is internal. Do not use these
unauthenticated local-development settings as a production deployment profile.

`DEPENDENCY_HELM_NATS_URL` selects the chart repository override, including the
`helm-nats` Nexus proxy. The dependency-cache env output and Make targets propagate
it in the same way as the Redpanda repository override.