#! /bin/bash
# Installs required packages to compile Autexousious for Linux.
pkgs_desired=(
  # amethyst
  libasound2-dev

  # enigo
  libxtst-dev
)

# `apt_install` can be found at:
#
# <https://gitlab.com/azriel91/gitlab_runner_setup/blob/master/linux/bin/apt_install>
apt_install "${pkgs_desired[@]}"
