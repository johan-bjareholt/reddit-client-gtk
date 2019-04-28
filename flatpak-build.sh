#!/bin/bash
ARCH=$1
if [ -z "$1" ]; then
    ARCH_OPT=
else
    ARCH_OPT=--arch=$ARCH
fi
flatpak-builder --force-clean $ARCH_OPT target/flatpak org.johan.RedditClient.json
