docker build -t apib:latest -f Dockerfiles/DockerfileApib ../
docker build -t apia:latest -f Dockerfiles/DockerfileApia ../

kubectl apply -f deploy/apibdeployment.yaml
kubectl apply -f deploy/apiadeployment.yaml

kubectl apply -f deploy/apiaservice.yaml
kubectl apply -f deploy/apibservice.yaml
