$script_dir = $PSScriptRoot
$repository_dir = Split-Path -Path "$script_dir"
$app_name = "will"
$app_crate_dir = "$repository_dir\app\$app_name"
$app_assets_dir = "$app_crate_dir\assets"

# Download default assets
$assets_zip = "$Env:TMP\will_assets_test-master.zip";

New-Item -ItemType Directory -Force -Path "$app_assets_dir"
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;
Invoke-WebRequest "https://gitlab.com/azriel91/will_assets_test/-/archive/master/will_assets_test-master.zip" -OutFile $assets_zip;

# Unzip
Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::ExtractToDirectory($assets_zip, $app_assets_dir);
Rename-Item "$app_assets_dir\will_assets_test-master" "$app_assets_dir\default"
