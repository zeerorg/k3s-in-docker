# Goals

1. Create a k3s docker server
2. Start a k3s docker server
3. Stop a k3s docker server
4. delete a k3s docker server

docker run --rm -it -e K3S_KUBECONFIG_OUTPUT=/output/kubeconfig.yaml --publish 6443:6443 --privileged rancher/k3s:v0.1.0 server  --https-listen-port 6443

