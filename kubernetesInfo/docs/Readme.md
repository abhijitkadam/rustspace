kubectl run pod --image=busybox --command -oyaml --dry-run=client > pod.yaml -- sh -c 'sleep 1d'

kubectl -f pod.yaml create

kubectl exec -it pod --sh


k run pod --image=bash --command -oyaml --dry-run=client > app.yaml -- sh -c 'ping google.com'

k -f app.yaml create

k get pod

k logs -f app


check logs of container proxy (if there are multiple containers in pod):
k logs -f app -c proxy


force delete pod:
k -f app.yaml delete --force --grace-period 0


=====================

docker run --priviledged
priviledged means container user 0 (root) => is directly mapped to host user 0 (root)


==================
RBAC:
role => role binding   (in one namespace)
clusterrole => clusterrolebinding
  api
  
-----

Network policy

Pod security policy

===================

