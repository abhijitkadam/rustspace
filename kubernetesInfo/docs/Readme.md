kubectl run pod --image=busybox --command -oyaml --dry-run=client > pod.yaml -- sh -c 'sleep 1d'

kubectl -f pod.yaml create

kubectl exec -it pod --sh
