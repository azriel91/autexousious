#! /bin/bash

# License compatibility
licenses_allowed=(
  Apache-2.0
  BSD-2-Clause
  BSD-3-Clause
  BSL-1.0
  CC0-1.0
  FTL
  ISC
  MIT
  MPL-2.0
  N/A
  Unlicense
  Zlib
)

IFS=$'\n'
blacklist=()
for license_group in $(cargo license); do
  for allowed in "${licenses_allowed[@]}"; do

    # If we detect a compatible license, continue
    if printf "%s" "${license_group}" | grep -qF "${allowed}"; then
      continue 2 # Continue outer loop
    fi
  done

  blacklist=("${blacklist[@]}" "${license_group}")
done

if test -n "${blacklist}"; then
    echo -e "Incompatible license detected in dependent crate(s). \n"\
            "The following are not compatible with a closed-source commercial application: \n"\
            "${blacklist}"
    exit 1
fi
