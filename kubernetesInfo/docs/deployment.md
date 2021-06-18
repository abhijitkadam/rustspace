## Deployment help <hr/>

This section is help on how to create yaml file with deployment

Deployment can be done using command line as well instead of yaml file. So we will use commands to generate yaml file. For any kubectl command if you give follwing option it will not actually execute command but do a dry run as well as output the equivalent yaml file 

> -o yaml --dry-run=client

### Deployment:
https://kubernetes.io/docs/reference/generated/kubectl/kubectl-commands#-em-deployment-em-
<br/><br/>

> kubectl create deployment apib --image=apib:latest -o yaml --dry-run=client

> kubectl create deployment apia --image=apia:latest -o yaml --dry-run=client

<br/>
Example with file output 

> kubectl create deployment apib --image=apib:latest -o yaml --dry-run=client > deploy/apibdeployment.yaml

<br/>
Above commands would give you yaml files that you can modify for customizations

For image pull policy and environment variables modify the yaml file as below\
If you don't want to push images to container repo and pull from there then use imagePullPolicy Never or IfNotPresent
<br/><br/>

```
spec:
      containers:        
      - image: apia:latest
        name: apia
        resources: {}
        imagePullPolicy: Never
        env:
        - name: apibservice
          value: "apib-svc"
```

### other commands
Get deployments:
> kubectl get deployments

Delete deployment:
> kubectl delete deployment <nameofdeployment>