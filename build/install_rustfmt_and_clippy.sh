#! /bin/sh
#
# Installs Rustfmt and Clippy if on nightly.
#
# TODO: Use `rustup component add` for clippy when it is ready.
#
# * https://github.com/rust-lang-nursery/rust-clippy/issues/1358
# * https://github.com/nrc/dev-tools-team/issues/4
if rustc --version | grep -qF 'nightly'; then

  # Install Rustfmt
  rustup component add rustfmt-preview

  # Install Clippy
  is_clippy_nightly_compatible() {
    return $(
      cd /tmp

      # lightweight crate to lint to test compatibility
      empty_crate="empty-$(date +%N)"
      cargo init "${empty_crate}"

      clippy_output=$(cd "${empty_crate}" && cargo clippy 2>&1)
      clippy_result=$?

      rm -rf "${empty_crate}"

      if test $clippy_result = 0; then
        echo 0
        exit
      else
        if echo "${clippy_output}" | grep -qF 'error while loading shared libraries: librustc_driver-'; then
          echo 1
          exit
        else
          echo 2
          exit
        fi
      fi
    )
  }

  # Need to save `cargo --list` output to a variable, otherwise it panics sometimes
  #
  # * https://github.com/sfackler/cargo-tree/issues/25
  # * https://github.com/rust-lang/rust/issues/46016
  existing_crates="$(cargo --list 2>&1)"
  if ! echo "${existing_crates}" | grep -q '\bclippy\b'; then
    cargo install clippy
  else
    compatible=$(is_clippy_nightly_compatible; echo $?)
    case "$compatible" in
      0) echo "Clippy is nightly compatible. No update required.";;
      1) echo "Clippy needs recompilation with nightly."; cargo install clippy --force;;
      *) echo "Unknown error while detecting clippy compatibility with Rustc. Read previous logs for details"; exit 1;;
    esac
  fi
fi
