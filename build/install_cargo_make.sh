#! /bin/sh
#
# Installs cargo-make
is_cargo_make_installed() {
  existing_crates="$(cargo --list 2>&1)"
  return $(echo "${existing_crates}" | grep -q '\bmake\b'; echo $?)
}
is_cargo_make_minimum() {
  # Minimum version 0.10
  # return $(cargo make --version | grep -q '\b[0-9]\+[.][0-9]\{2,\}'; echo $?)
  return $(cargo make --version | grep -qF '0.10.5'; echo $?)
}

if ! is_cargo_make_installed; then
  cargo install cargo-make
elif ! is_cargo_make_minimum; then
  cargo install cargo-make --force
fi
