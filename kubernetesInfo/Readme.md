# Kubernetes demo

## About
This workspace contains 2 apis services that will be used  
> * apia 
> * apib 

<br/>

* apib returns data
* "apia" calls "apib", gets data and returns enriched data
* client should "apia"

<br/>
So we have, 
<br/>

Client -> apia -> apib 

## Containerization
Both the apis can be packaged into docker images \
Relavant Dockerfile is present in the project directory

## Deployment:
We will deploy apia and apib in kubernetes. \
The deployments will have multiple replicas of apia as well as apib\
Both the deployments will be be exposed as service

## Service
* All the pods into the apia deployment will be exposed as service and the name of service will be apia-svc. This will be load-balancer service since external clients will call it
* All the pods into the apib deployment will be exposed as service and the name of the service will be apib-svc. This will be Kubernetes internal service type ClusterIP and accesible to the pods inside of kubernetes cluster 
<br/><br/>

![Alt text](docs/kunernetesinfo.png?raw=true "Deployment")
<br/>

## Commands

### Create images
<br/>
First build the project using:

> cargo build --release 

Then build container images for apis using following commands. The images will have binaries for api copied from target/release folder. They will be started on continer start. \
<br/>
Note that rust env is not in image. We are not building code in image and just copying the binary. So this needs to be build on ubuntu since Docker file also uses Ubuntu for binary compatibility.
<br/><br/>

> docker build -t apib:latest -f apib/Dockerfile .

> docker build -t apia:latest -f apia/Dockerfile .

After this "docker images" should list the created images

### Deployment
How to create deployment yaml & other help is in [docs/deployment.md](docs/deployment.md) file

The deployment files for apis are in deploy folder 
* [apiadeployment.yaml](deploy/apiadeployment.yaml)  
* [apibdeployment.yaml](deploy/apibdeployment.yaml)  

We are using `imagePullPolicy: Never` since we will not pull images from container repo and use it locally. Also we will use 3 replica.

Now apply the deployment:
> kubectl apply -f deploy/apibdeployment.yaml
> kubectl apply -f deploy/apiadeployment.yaml

You can run following to see the deployments
>kubectl get deployments

You can run following to see the pods 
> kubectl get pods

If you want to change the scaling of deployment:
>kubectl scale deployment apib --replicas 2

In case you have to delete 
> kubectl delete deployment {deployment_name}

### Service
Exposing deployment pods as service

How to create service yaml & other help is in [docs/service.md](docs/service.md) file


First we will expose deployment apib as service apib-svc over port 80
> kubectl apply -f deploy/apibservice.yaml 

Then we will expose deployment apia as service apia-svc over port. The type of service is Loadbalancer. In local environement it will be localhost and in cloud it will be cloud load balancer.

> kubectl apply -f deploy/apiaservice.yaml 

The pods of apia uses service apib-svc. The name of the service is taken from environment variable which is in yaml file of apia deployment
