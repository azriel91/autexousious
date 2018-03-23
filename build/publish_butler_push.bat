@echo off

:: This script assumes that the binaries to publish are already in "target\publish\app\will".

setlocal enableDelayedExpansion

set "app=will"
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
  echo Failed to push to butler
  exit /b 1
)

endlocal

exit /b 0
