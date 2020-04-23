#! /bin/bash
set -e

# Release options
profile=release

# Directories
self_dir="$(dirname "$(readlink -f "${BASH_SOURCE}")")"
repository_dir="$(dirname "${self_dir}")"
target_dir="${repository_dir}/target"
target_profile_dir="${target_dir}/${profile}"
target_publish_dir="${target_dir}/publish"

# Application to publish
app_name=will
app_name_server=session_server
app_crate_dir="${repository_dir}/app/${app_name}"
app_crate_dir_server="${repository_dir}/app/${app_name_server}"
app_assets_dir="$(readlink -f "${app_crate_dir}/assets")"
app_resources_dir="$(readlink -f "${app_crate_dir}/resources")"

app_publish_artifacts=(
  "${target_profile_dir}/${app_name}"
  "${app_assets_dir}"
  "${app_resources_dir}"
  "${app_crate_dir}/${app_name}.toml"
  "${app_crate_dir}/logger.yaml"
)
app_publish_artifacts_server=(
  "${target_profile_dir}/${app_name_server}"
  "${app_crate_dir_server}/logger.yaml"
)

# Download "default" assets.
#
# `VERSION` is set in `ci.yml` / `publish.yml`.
assets_ref="master"
if [[ "${VERSION}" =~ '^[0-9]+[.][0-9]+[.][0-9]+$' ]]
then assets_ref="${VERSION}"
fi

wget "https://gitlab.com/azriel91/will_assets_test/-/archive/${assets_ref}/will_assets_test-${assets_ref}.zip"
unzip -uoq "will_assets_test-${assets_ref}.zip" -d "${app_assets_dir}"
mv "${app_assets_dir}/will_assets_test-${assets_ref}" "${app_assets_dir}/default"

# Ensure the source files exist before transferring
for f in "${app_publish_artifacts[@]}"; do
  if ! test -e "${f}"; then
    echo "ERROR: Publish artifact does not exist: '${f}'"
    exit 1
  fi
done
for f in "${app_publish_artifacts_server[@]}"; do
  if ! test -e "${f}"; then
    echo "ERROR: Publish artifact does not exist: '${f}'"
    exit 1
  fi
done

# Publish settings
target_publish_app_dir="${target_publish_dir}/app/${app_name}"
target_publish_app_dir_server="${target_publish_dir}/app/${app_name_server}"

# Prepare the publish directory
test -d "${target_publish_app_dir}" || mkdir -p "${target_publish_app_dir}"

# To remove extraneous files from the destination directory, we need to use a temporary directory.
#
# * Clean destination directory: <https://stackoverflow.com/a/15383897/1576773>
# * Temporary directory: <https://unix.stackexchange.com/a/84980>
case "${OSTYPE}" in
  linux*          ) cmd_mktmp="mktemp -d";;
  darwin | Darwin ) cmd_mktmp="mktemp -d -t "${target_publish_app_dir}.rsync"";;
  msys            ) echo "Error: Publish app script only usable on *nix systems" 1>&2; exit 1;;
  *               ) echo "Error: Unknown OSTYPE: '${OSTYPE}'" 1>&2; exit 1;;
esac
case "${OSTYPE}" in
  linux*          ) cmd_mktmp_server="mktemp -d";;
  darwin | Darwin ) cmd_mktmp_server="mktemp -d -t "${target_publish_app_dir_server}.rsync"";;
  msys            ) echo "Error: Publish app script only usable on *nix systems" 1>&2; exit 1;;
  *               ) echo "Error: Unknown OSTYPE: '${OSTYPE}'" 1>&2; exit 1;;
esac

target_publish_app_temp_dir="$($cmd_mktmp)"
target_publish_app_temp_dir_server="$($cmd_mktmp_server)"

# Deletes the temp directory
function cleanup {
  rm -rf "${target_publish_app_temp_dir}"
  rm -rf "${target_publish_app_temp_dir_server}"
}

# Register the cleanup function to be called on the EXIT signal
trap cleanup EXIT

# First rsync from src to dest, as well as hard link the transferred files to a temporary directory.
# Then rsync delete from the temporary directory to the intended dest directory.
rsync -rL --link-dest="${target_publish_app_dir}" "${app_publish_artifacts[@]}" "${target_publish_app_temp_dir}"
rsync -raL --delete "${target_publish_app_temp_dir}/" "${target_publish_app_dir}"

rsync -rL --link-dest="${target_publish_app_dir_server}" "${app_publish_artifacts_server[@]}" "${target_publish_app_temp_dir_server}"
rsync -raL --delete "${target_publish_app_temp_dir_server}/" "${target_publish_app_dir_server}"

exit 0
