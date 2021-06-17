## Service & Pod Debug
Services of type ClusterIP are accesible inside kubernetes network. In order to debug ot troubleshoot we need to launch another container into that network to probe service.

This can also be used to probe pods from deployment

### One way is to launch busybox image container that has nslookup
> kubectl run -it --rm --restart=Never busybox --image=gcr.io/google-containers/busybox sh

[Because of --rm option the container will be terminated on exit]

Then we can use nslookup to see if service is resolved

> nslookup servicename\
> wget -O- servicename


Sometimes busybox nslookup does not work and it does not have curl or other utilities so we launch ubuntu image or can prepare own unbuntu image and launch it for probing

> kubectl run -it --rm --restart=Never --image=ubuntu sh

Custom image that has utilities like curl
>kubectl run -it --rm --restart=Never --image=ubuntuprobe --image-pull-policy Never sh
<br/><br/>

### Creating local image ubuntuprobe
Launch unbuntu container
>docker run -t -i image 

Install following:
>apt-get update\
>apt-get install curl wget\
>apt install dnsutils

take image after installing above
>docker commit {containername}\
>docker tag imageid {ubuntuprobe}


