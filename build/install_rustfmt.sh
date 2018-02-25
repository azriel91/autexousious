#! /bin/sh
#
# Installs Rustfmt if on nightly.
if rustc --version | grep -qF 'nightly'; then
  # Install Rustfmt
  rustup component add rustfmt-preview
fi
