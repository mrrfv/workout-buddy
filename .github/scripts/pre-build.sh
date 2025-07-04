#!/usr/bin/env bash
set -e

# Detect package manager
if command -v apk >/dev/null 2>&1; then
  # Alpine Linux
  echo "Detected Alpine Linux - installing via apk"
  apk update
  apk add --no-cache pkgconf wayland-dev pipewire-dev clang-dev alsa-lib-dev mesa-dev

elif command -v apt-get >/dev/null 2>&1; then
  # Debian / Ubuntu
  echo "Detected Debian-based system - installing via apt"
  sudo apt-get update
  DEBIAN_FRONTEND=noninteractive sudo apt-get install -y --no-install-recommends \
    pkg-config \
    libwayland-dev \
    libpipewire-0.3-dev \
    clang libclang-dev \
    libasound2-dev \
    libegl-dev
else
  echo >&2 "No supported package manager found (need apk or apt-get)."
fi

