#! /bin/sh
#
# Installs cargo-update
#
# Note: The command to run is `cargo install-update`, since `cargo update` is one of `cargo`'s
#       subcommands.

# For the `cargo --list` command, we need to pipe stderr to `/dev/null` because of these issues:
#
# * https://github.com/sfackler/cargo-tree/issues/25
# * https://github.com/rust-lang/rust/issues/46016
is_cargo_update_installed() {
  existing_crates="$(cargo --list 2>&1)"
  return $(echo "${existing_crates}" | grep -q '\binstall-update\b'; echo $?)
}

# The `--force` simply avoids a failure in case two jobs run on the same agent and both install
# `cargo-update`.
is_cargo_update_installed || cargo install cargo-update --force
