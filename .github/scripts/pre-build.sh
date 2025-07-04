#!/usr/bin/env bash
set -e

# Detect package manager
if command -v apk >/dev/null 2>&1; then
  # Alpine Linux
  echo "Detected Alpine Linux - installing via apk"
  apk update
  apk add --no-cache wayland-dev build-base pkgconf clang-dev \
  libx11-dev libxcb-dev libxrandr-dev libxinerama-dev libxcursor-dev libxkbcommon-dev \
  mesa-dev vulkan-headers vulkan-loader \
  alsa-lib-dev pulseaudio-dev pipewire-dev \
  openssl-dev dbus-dev libudev-dev fontconfig-dev freetype-dev  

elif command -v apt-get >/dev/null 2>&1; then
  # Debian / Ubuntu
  echo "Detected Debian-based system - installing via apt"
  sudo apt-get update
  DEBIAN_FRONTEND=noninteractive sudo apt-get install -y --no-install-recommends \
    build-essential pkg-config clang libclang-dev libx11-dev libxcb1-dev libxrandr-dev libxinerama-dev libxcursor-dev libxkbcommon-dev libdrm-dev libgbm-dev libegl-dev libvulkan-dev   libasound2-dev libpulse-dev libpipewire-0.3-dev libssl-dev libdbus-1-dev libudev-dev libfontconfig1-dev libfreetype6-dev libwayland-dev
else
  echo >&2 "No supported package manager found (need apk or apt-get)."
fi

