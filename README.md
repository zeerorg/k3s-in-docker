# k3d - k3s in docker

A lightweight alternative to KinD for local development.

## Install and run

1. You need docker installed.
2. Just run:

   `curl -s https://api.github.com/repos/zeerorg/k3s-in-docker/releases/latest | grep "browser_download_url.*k3d\"" | cut -d : -f 2,3 | tr -d \" | wget -qi - -O k3d && ./k3d create`

3. If you have rust toolchain installed, you can install it using: `cargo install k3d && k3d create`

## Advantages over [KinD](https://github.com/kubernetes-sigs/kind)

1. Supports arm64 and armhf
2. Fast boot time
3. Supports starting and stopping without losing previous state
4. Lightweight compared to KinD

## Usage

Normal flow:

1. `k3d create`
2. `export KUBECONFIG=$(k3d get-kubeconfig)`
3. `kubectl get pods --all-namespaces`
4. If you want to delete the cluster do: `kubectl delete`

If port 6443 is occupied you can specify a different port in first step: `k3d create -p 10001`. This will create a docker container named k3s_default with port 10001 exposed.

You can specify a different name for cluster with `k3s create -n <name>`, but then keep in mind to do `k3d get-kubeconfig -n <name>` when getting kubeconfig.

```text
k3d 0.1.0
Rishabh Gupta <r.g.gupta@outlook.com>
Run k3s in Docker

USAGE:
    k3d [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check-tools       Check docker running
    create            Create a single node k3s server
    delete            Delete cluster
    get-kubeconfig    get kubeconfig.yaml location
    help              Prints this message or the help of the given subcommand(s)
    list              List all clusters
    start             Start a stopped cluster
    stop              Stop a cluster
```
