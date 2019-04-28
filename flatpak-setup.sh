#!/bin/bash

flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo

flatpak install --user flathub \
    org.gnome.Platform//3.30 \
    org.gnome.Sdk//3.30 \
    org.freedesktop.Sdk.Extension.rust-stable//18.08

flatpak install --user flathub \
    org.gnome.Platform//3.30 \
    org.gnome.Sdk//3.30 \
    org.freedesktop.Sdk.Extension.rust-stable//18.08
