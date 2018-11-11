#!/bin/bash
rm -rf target/flatpak
flatpak-builder target/flatpak org.johan.RedditClient.json
