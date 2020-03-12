@echo off

:: `CMD` syntax: <https://ss64.com/nt/>
::
:: WARNING: This script's exit code doesn't work properly, so it may exit with 0 even if you run:
:: `exit /b 1`.

setlocal enableDelayedExpansion

:: Provides additional executables such as `find` and `xcopy`.
set "path=%path%;C:\WINDOWS\system32"

:: Release options
set profile=release

:: Directories
set "self_dir=%~dp0"
set "self_dir=%self_dir:~0,-1%"
call :dirname repository_dir "%self_dir%"
set "target_dir=%repository_dir%\target"
set "target_profile_dir=%target_dir%\%profile%"
set "target_publish_dir=%target_dir%\publish"

:: Application to publish
set app_name=will
set app_name_server=session_server
set "app_crate_dir=%repository_dir%\app\%app_name%"
set "app_crate_dir_server=%repository_dir%\app\%app_name_server%"
call :readlink app_assets_dir "%app_crate_dir%\assets"
call :readlink app_resources_dir "%app_crate_dir%\resources"

:: Download default assets
C:\Windows\System32\WindowsPowerShell\v1.0\PowerShell -NoProfile -ExecutionPolicy Bypass -Command "& '%~dp0assets_default_download.ps1'"

:: Fake array
::
:: Useful reference: <https://stackoverflow.com/a/10167990/1576773>
set "app_publish_artifacts[0]=%target_profile_dir%\%app_name%.exe"
set "app_publish_artifacts[1]=%app_assets_dir%"
set "app_publish_artifacts[2]=%app_resources_dir%"
set "app_publish_artifacts[3]=%app_crate_dir%\%app_name%.toml"
set "app_publish_artifacts[4]=%app_crate_dir%\logger.yaml"

set "app_publish_artifacts_server[0]=%target_profile_dir%\%app_name_server%.exe"
set "app_publish_artifacts_server[1]=%app_crate_dir_server%\logger.yaml"

:: Ensure the source files exist before transferring
set artifacts_first_index=0
set artifacts_last_index=4
for /L %%i in (%artifacts_first_index%,1,%artifacts_last_index%) do (
  setlocal
  set "f=!app_publish_artifacts[%%i]!"
  if not exist !f! (
    echo ERROR: Publish artifact does not exist: '!f!'
    exit /b 1
  )
  endlocal
)
set artifacts_first_index_server=0
set artifacts_last_index_server=1
for /L %%i in (%artifacts_first_index_server%,1,%artifacts_last_index_server%) do (
  setlocal
  set "f=!app_publish_artifacts_server[%%i]!"
  if not exist !f! (
    echo ERROR: Publish artifact does not exist: '!f!'
    exit /b 1
  )
  endlocal
)

:: Publish settings
set "target_publish_app_dir=%target_publish_dir%\app\%app_name%"
set "target_publish_app_dir_server=%target_publish_dir%\app\%app_name_server%"

:: Remove the publish directory, then copy desired artifacts across
2>nul rmdir /s /q "%target_publish_dir%"
2>nul rmdir /s /q "%target_publish_app_dir_server%"

:: will
for /L %%i in (%artifacts_first_index%,1,%artifacts_last_index%) do (
  setlocal
  set "f=!app_publish_artifacts[%%i]!"
  for /r %%f in (!f!) do (set "f_basename=%%~nxf")
  echo F | xcopy /S /I /Y /F "!f!" "%target_publish_app_dir%\!f_basename!"
  if errorlevel 1 exit /b 1
  endlocal
)
:: session_server
for /L %%i in (%artifacts_first_index_server%,1,%artifacts_last_index_server%) do (
  setlocal
  set "f=!app_publish_artifacts_server[%%i]!"
  for /r %%f in (!f!) do (set "f_basename=%%~nxf")
  echo F | xcopy /S /I /Y /F "!f!" "%target_publish_app_dir_server%\!f_basename!"
  if errorlevel 1 exit /b 1
  endlocal
)

endlocal
exit /b

:dirname
  setlocal
  set "file=%~2"
  for %%i in (%file%\..) do set "parent_dir=%%~fi"
  endlocal & set "%~1=%parent_dir%"
exit /b

:: Returns the absolute, dereferenced path of a potential symlink.
::
:: Taken and adjusted from <https://stackoverflow.com/a/27407405/1576773>
:readlink
  setlocal enableDelayedExpansion
  set "file=%~2"
  set "abs_path=%file%"
  for /f "tokens=2 delims=[]" %%i in ('dir %file%* ^| find "<SYMLINK"') do (
    call :dirname parent_dir "%file%"
    pushd "!parent_dir!"
    pushd "%%i"
    set "abs_path=!cd!"
    popd
    popd
  )
  endlocal & set "%~1=%abs_path%"
exit /b

:strlen <resultVar> <stringVar>
(
  setlocal EnableDelayedExpansion
  set "s=!%~2!#"
  set "len=0"
  for %%P in (4096 2048 1024 512 256 128 64 32 16 8 4 2 1) do (
    if "!s:~%%P,1!" NEQ "" (
      set /a "len+=%%P"
      set "s=!s:~%%P!"
    )
  )
)
(
  endlocal
  set "%~1=%len%"
  exit /b
)
