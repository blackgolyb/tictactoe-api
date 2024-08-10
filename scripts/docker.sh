#!/bin/bash

function build_arm() {
    docker buildx build \
        --platform linux/arm64 \
        -f Dockerfile.arm \
        -t blackgolyb/tic-tac-toe-api:latest \
        --push \
        .
}

function build_x86() {
    docker buildx build \
        --platform linux/amd64 \
        -f Dockerfile.x86 \
        -t blackgolyb/tic-tac-toe-api:latest \
        --push \
        .
}

case $1 in
    "x86")
        build_x86
        ;;
    "arm")
        build_arm
        ;;
    "arm_test")
        build_arm_test
        ;;
    *)
        echo "Unknown architecture $1"
        ;;
esac
