# Publishes Will and Session Server to itch.io
$itch_io_game = "will";
$app = "will";
$app_server = "session_server";

$butler_creds_path = "target\butler_creds";
$env:BUTLER_API_KEY > $butler_creds_path

.\butler.exe `
  -i "$butler_creds_path" `
  push `
  "target\publish\app\${app}" `
  "${env:ITCH_IO_USER}/${game}:${env:CHANNEL}" `
  --userversion "${env:VERSION}" `
  --if-changed

.\butler.exe `
  -i "$butler_creds_path" `
  push `
  "target\publish\app\${app_server}" `
  "${env:ITCH_IO_USER}/${game}:${env:CHANNEL_SERVER}" `
  --userversion "${env:VERSION}" `
  --if-changed
