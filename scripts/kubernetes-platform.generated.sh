# Sourced by kubernetes.generated.sh; uses its containerized Helm/kubectl helpers.
platform_up() {
  progress "installing Argo CD and Vault Community"
  ensure_helm_repo argo "${DEPENDENCY_HELM_ARGO_URL:-https://argoproj.github.io/argo-helm}"
  ensure_helm_repo hashicorp "${DEPENDENCY_HELM_HASHICORP_URL:-https://helm.releases.hashicorp.com}"
  # A newly installed Vault is intentionally sealed. Initialize/unseal before wait.
  helm upgrade --install vault hashicorp/vault --version 0.34.1 \
    --namespace vault-system --create-namespace \
    --values kubernetes/vault-values.generated.yaml
  kubectl --namespace vault-system wait --for=create pod/vault-0 --timeout="${TIMEOUT}"
  kubectl --namespace vault-system wait --for=jsonpath='{.status.phase}'=Running \
    pod/vault-0 --timeout="${TIMEOUT}"
  python3 scripts/kubernetes-vault.generated.py bootstrap
  kubectl --namespace vault-system wait --for=condition=Ready pod/vault-0 --timeout="${TIMEOUT}"
  helm upgrade --install vault-secrets-operator hashicorp/vault-secrets-operator --version 1.6.2 \
    --namespace vault-system --create-namespace \
    --values kubernetes/vault-operator-values.generated.yaml --wait --timeout "${TIMEOUT}"
  kubectl create namespace argocd --dry-run=client -o yaml | kubectl apply -f - >/dev/null
  python3 scripts/kubernetes-vault.generated.py seed-infrastructure argocd argocd-redis
  helm upgrade --install argocd argo/argo-cd --version 10.17.2 \
    --namespace argocd --create-namespace \
    --values kubernetes/argocd-values.generated.yaml --wait --timeout "${TIMEOUT}"

}