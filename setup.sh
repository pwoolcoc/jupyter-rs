#!/bin/bash

JUPYTER=$(which jupyter)
KERNEL=${1:-$PWD/target/debug/jupyter-rust}

if [[ -z "$JUPYTER" ]] ; then
    echo "jupyter not found"
    exit 255
fi

if [[ -z "$KERNEL" ]] ; then
    echo "jupyter-rust not found"
    exit 255
fi

$JUPYTER kernelspec install --user --debug --name=rust ./kernelspec

