@echo off

:: This script assumes that the binaries to publish are already in "target\publish\app\will".

setlocal enableDelayedExpansion

set app=will
set app_server=session_server
for /f "skip=2 delims== tokens=2" %%i in (
  'c:\windows\system32\find.exe "version" "app/%app%/Cargo.toml"') do (
    set version=%%i
    set version=!version:~1!
)

if errorlevel 1 (
  echo Failed to parse version from app/%app%/Cargo.toml
  exit /b %errorlevel%
)

echo Parsed version from "app/%app%/Cargo.toml": !version!

butler push ^
  "target\publish\app\%app%" ^
  "%ITCH_IO_USER%/%app%:%CHANNEL%" ^
  --userversion !version! ^
  --if-changed

if errorlevel 1 (
  echo Failed to push %app% to itch.io
  exit /b 1
)

butler push ^
  "target\publish\app\%app_server%" ^
  "%ITCH_IO_USER%/%app%:%CHANNEL_SERVER%" ^
  --userversion !version! ^
  --if-changed

if errorlevel 1 (
  echo Failed to push %app_server% to itch.io
  exit /b 1
)

endlocal

exit /b 0
