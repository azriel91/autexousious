#! /bin/bash

# Release options
profile=release

self_dir="$(dirname "$(readlink -f "${BASH_SOURCE}")")"
repository_dir="$(dirname "${self_dir}")"
target_dir="${repository_dir}/target"
target_profile_dir="${target_dir}/${profile}"
pkg_dir="${repository_dir}/pkg"

app_name=will
app_crate_dir="${repository_dir}/app/${app_name}"

set -ex

# A few steps are necessary to get this build working which makes it slightly
# nonstandard compared to most other builds.
#
# * First, the Rust standard library needs to be recompiled with atomics
#   enabled. to do that we use Cargo's unstable `-Zbuild-std` feature.
#
# * Next we need to compile everything with the `atomics` and `bulk-memory`
#   features enabled, ensuring that LLVM will generate atomic instructions,
#   shared memory, passive segments, etc.
#
# * Finally, `-Zbuild-std` is still in development, and one of its downsides
#   right now is rust-lang/wg-cargo-std-aware#47 where using `rust-lld` doesn't
#   work by default, which the wasm target uses. To work around that we find it
#   and put it in PATH

(
  cd "${app_crate_dir}"
  RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' \
    cargo build --bin will -v --target wasm32-unknown-unknown -Z build-std=std,panic_abort \
    --features "wasm" --release

  # Note the usage of `--no-modules` here which is used to create an output which
  # is usable from Web Workers. We notably can't use `--target bundler` since
  # Webpack doesn't have support for atomics yet.
  wasm-bindgen "${target_dir}/wasm32-unknown-unknown/${profile}/will.wasm" \
    --out-dir "${pkg_dir}" --no-modules

  # worker.js crashes because it does not have `AudioContext` / `webkitAudioContext` in scope.
  # This prevents it from crashing.
  audio_context_workaround="const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : null)"

  sed -i "s/const lAudioContext.\+\$/${audio_context_workaround}/" "${pkg_dir}/will.js"
)
