# Stop on error: <https://stackoverflow.com/a/44810914/1576773>
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$PSDefaultParameterValues["*:ErrorAction"]="Stop"

$script_dir = $PSScriptRoot
$repository_dir = Split-Path -Path "$script_dir"
$app_name = "will"
$app_crate_dir = "$repository_dir\app\$app_name"
$app_assets_dir = "$app_crate_dir\assets"

# Download "default" assets.
$assets_ref = "master"
$git_described = git describe HEAD --tags | Out-String

if ($git_described -match "^[0-9]+\.[0-9]+\.[0-9]+$") { $assets_ref = $git_described }
$assets_zip = "$Env:TMP\will_assets_test-$assets_ref.zip";

New-Item -ItemType Directory -Force -Path "$app_assets_dir"
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;
Invoke-WebRequest "https://gitlab.com/azriel91/will_assets_test/-/archive/$assets_ref/will_assets_test-$assets_ref.zip" -OutFile $assets_zip;

# Unzip
Add-Type -AssemblyName System.IO.Compression.FileSystem
Expand-Archive -Path $assets_zip -DestinationPath $app_assets_dir -Force
$assets_default_dir = "$app_assets_dir\default"
if (Test-Path $assets_default_dir) {
  Remove-Item -Recurse -Force $assets_default_dir
}
Start-Sleep -s 0.5 # Allow directory handle to be freed
Rename-Item "$app_assets_dir\will_assets_test-$assets_ref" $assets_default_dir -Force
