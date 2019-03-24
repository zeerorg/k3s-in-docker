#!/bin/bash

version=$(curl -sI https://github.com/zeerorg/k3s-in-docker/releases/latest | grep Location | awk -F"/" '{ printf "%s", $NF }' | tr -d '\r')
if [ ! $version ]; then
    echo "Failed while attempting to install k3d. Please manually install:"
    echo ""
    echo "1. Open your web browser and go to https://github.com/zeerorg/k3s-in-docker/releases"
    echo "2. Download the latest release for your platform. Call it 'k3d'."
    echo "3. chmod +x ./k3d"
    echo "4. mv ./k3d /usr/local/bin"
    exit 1
fi

hasCli() {

    has=$(which k3d)

    if [ "$?" = "0" ]; then
        echo
        echo "You already have the k3d!"
        export n=1
        echo "Overwriting in $n seconds.. Press Control+C to cancel."
        echo
        sleep $n
    fi

    hasCurl=$(which curl)
    if [ "$?" = "1" ]; then
        echo "You need curl to use this script."
        exit 1
    fi
}


checkHash(){

    sha_cmd="sha256sum"

    if [ ! -x "$(command -v $sha_cmd)" ]; then
    sha_cmd="shasum -a 256"
    fi

    if [ -x "$(command -v $sha_cmd)" ]; then

    filesum=$($sha_cmd $targetFile | awk '{print $1;}')
    matchsum=$(curl -sSL https://github.com/zeerorg/k3s-in-docker/releases/download/$version/k3d$suffix.sha256 | awk '{print $1;}')

        if [ "$filesum" != "$matchsum" ]; then
            echo "File checksum: $filesum"
            echo "Checksum to match: $matchsum"
            rm $targetFile
            echo "Binary checksum didn't match. Exiting"
            exit 1
        fi
    fi
}

getPackage() {
    uname=$(uname)
    userid=$(id -u)

    suffix=""
    case $uname in
    "Darwin")
    suffix="-darwin"
    ;;
    "Linux")
        arch=$(uname -m)
        echo $arch
        case $arch in
        "aarch64")
        suffix="-arm64"
        ;;
        esac
        case $arch in
        "armv7l")
        suffix="-armhf"
        ;;
        esac
    ;;
    esac

    targetFile="/tmp/k3d$suffix"

    if [ "$userid" != "0" ]; then
        targetFile="$(pwd)/k3d$suffix"
    fi

    if [ -e $targetFile ]; then
        rm $targetFile
    fi

    url=https://github.com/zeerorg/k3s-in-docker/releases/download/$version/k3d$suffix
    echo "Downloading package $url as $targetFile"

    curl -sSL $url --output $targetFile

    if [ "$?" = "0" ]; then

    checkHash

    chmod +x $targetFile

    echo "Download complete."

        if [ "$userid" != "0" ]; then

            echo
            echo "========================================================="
            echo "==    As the script was run as a non-root user the     =="
            echo "==    following commands may need to be run manually   =="
            echo "========================================================="
            echo
            echo "  sudo cp k3d$suffix /usr/local/bin/k3d"
            echo

        else

            echo
            echo "Running as root - Attempting to move k3d to /usr/local/bin"

            mv $targetFile /usr/local/bin/k3d

            if [ "$?" = "0" ]; then
                echo "New version of k3d installed to /usr/local/bin"
            fi

            if [ -e $targetFile ]; then
                rm $targetFile
            fi
        fi
    fi
}

hasCli
getPackage
