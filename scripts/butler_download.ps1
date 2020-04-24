# Download Butler
$butler_zip = "$Env:TMP\butler.zip";
$butler_dir = ".";

$butler_path = "$butler_dir\butler.exe";
if (-not (Test-Path $butler_path)) {
  if (-not (Test-Path $butler_dir)) {
    New-Item -ItemType Directory -Force -Path "$butler_dir"
  }
  [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;
  Invoke-WebRequest https://broth.itch.ovh/butler/windows-386/LATEST/archive/default -OutFile $butler_zip;

  # Unzip
  Expand-Archive -Path $butler_zip -DestinationPath $butler_dir -Force
}

iex "$butler_path upgrade --assume-yes --force"

# echo version
iex "$butler_path --version"
