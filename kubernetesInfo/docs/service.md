## Service help <hr/>

This section is help on how to create yaml file for service

Service creation( or exposing deployment as service) can be done using command line as well instead of yaml file. So we will use commands to generate yaml file. For any kubectl command if you give follwing option it will not actually execute command but do a dry run as well as output the equivalent yaml file 

> -o yaml --dry-run=client

### Service:
https://kubernetes.io/docs/reference/generated/kubectl/kubectl-commands#expose

kubectl expose (-f FILENAME | TYPE NAME) [--port=port] [--protocol=TCP|UDP|SCTP] [--target-port=number-or-name] [--name=name] [--external-ip=external-ip-of-service] [--type=type]

> kubectl expose deployment nginx --port=80 --target-port=8080 -o yaml --dry-run=client

service which serves on port 80 and connects to the containers on port 8000.

In above default service type is ClusterIP, otherwise specify as below

> --type={type}\
Possbile types are: 
> ClusterIP, NodePort, LoadBalancer, or ExternalName

Name of service as below:
> --name=frontend

>port: The port that the service should serve on\
>target-port: Name or number for the port on the container that the service should direct traffic to. Optional.


Another example:
>kubectl expose deployment apib --port=80 --target-port=8080 --name=apib -o yaml --dry-run=client

>kubectl expose deployment apia --port=80 --target-port=8081 --name=apia-svc --type=LoadBalancer -o yaml --dry-run=client > deploy/apiaservice.yaml