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
