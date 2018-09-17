#! /bin/bash
set -e

# Release options
profile=debug

# Directories
self_dir="$(dirname "$(readlink -f "${BASH_SOURCE}")")"
repository_dir="$(dirname "${self_dir}")"
target_dir="${repository_dir}/target"
target_profile_dir="${target_dir}/${profile}"

coverage_dir="${target_dir}/coverage"
test -d "${coverage_dir}" || mkdir -p "${coverage_dir}"

kcov_exclude_line="kcov-ignore"
kcov_exclude_region="kcov-ignore-start:kcov-ignore-end"

test_bins="$(cargo test --no-run --message-format=json | jq -r "select(.profile.test == true) | .filenames[]")"
crate_coverage_dirs=()
for test_bin in $test_bins; do
  test_bin_name=${test_bin##*/target/debug/}
  crate_name=${test_bin_name%-*}
  crate_dir="${repository_dir}/crate/${crate_name}"

  test -d "${crate_dir}" || continue;

  crate_coverage_dir="${coverage_dir}/${test_bin_name}"
  crate_coverage_dirs+=("${crate_coverage_dir}")

  (
    export CARGO_MANIFEST_DIR="$crate_dir"
    kcov --include-pattern="${crate_dir}/src/" \
      "--exclude-line=${kcov_exclude_line}" \
      "--exclude-region=${kcov_exclude_region}" \
      "${crate_coverage_dir}" "${test_bin}"
  )
done

rm -rf "${coverage_dir}/merged"
kcov --merge "${coverage_dir}/merged" "${crate_coverage_dirs[@]}" \
  "--exclude-line=${kcov_exclude_line}" \
  "--exclude-region=${kcov_exclude_region}" \
