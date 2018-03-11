#! /bin/bash
# Installs required packages to compile Autexousious for Linux.
pkgs_desired=(
  # amethyst
  libasound2-dev

  # enigo
  libxtst-dev
)

# Open a pipe to a file descriptor that ignores apt warning
exec 3> >(grep -v "^$" | grep -vF "apt does not have a stable CLI interface" 1>&2)

pkgs_installed="$(
  apt list --installed 2>&3
)"
pkgs_existing=()
pkgs_required=()
for pkg in "${pkgs_desired[@]}"; do
  echo "${pkgs_installed}" | grep -qF $pkg && pkgs_existing+=($pkg) || pkgs_required+=($pkg)
done
echo "Existing packages: ${pkgs_existing[@]}"
echo "Required packages: ${pkgs_required[@]}"

apt_retry() {
  cmd="${1}"
  attempts="${2}"
  delay="${3}"

  (
    for i in {1..5}
    do
        # Allow stdout to remain piped to stdout, but capture stderr
        exec 4>&1

        update_stderr=$($cmd 2>&1 1>&4)
        update_result=$?

        test "${update_result}" = 0 && break

        if echo "${update_stderr}" | grep -qF "11: Resource temporarily unavailable"; then
          echo "Unable to acquire apt lock, sleeping 5 seconds before retry"
          sleep 5
        else
          exit 1
        fi
    done
  )
}

if test ${#pkgs_required[@]} -gt 0; then
  apt_retry "sudo apt update -qq" 5 5
  apt_retry "sudo apt install -y -qq ${pkgs_required[@]}" 5 5
else
  echo "All desired packages already installed."
fi
