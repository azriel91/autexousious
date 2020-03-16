#! /bin/bash
# Installs required packages to compile Autexousious for Linux.
pkgs_desired=(
  # amethyst
  libasound2-dev
  libfreetype6-dev
  libexpat1-dev
  libxcb-composite0-dev

  # enigo
  libxdo-dev

  # publishing
  jq
  unzip

  # cargo install-update
  libssl-dev
)

# `apt_install` can be found at:
#
# <https://gitlab.com/azriel91/gitlab_runner_setup/blob/master/linux/bin/apt_install>
if type apt_install 2>/dev/null; then
  apt_install "${pkgs_desired[@]}"
else
  sudo apt update -y && sudo apt install -y "${pkgs_desired[@]}"
fi
