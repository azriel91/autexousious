@echo off
setlocal

:: Installs cargo-update
::
:: Note: The command to run is `cargo install-update`, since `cargo update` is one of `cargo`'s
::       subcommands.

:: For the `cargo --list` command, we need to loop over its output before piping to `find.exe`
:: because of these issues:
::
:: * https://github.com/sfackler/cargo-tree/issues/25
:: * https://github.com/rust-lang/rust/issues/46016
set "is_installed=false"
for /f %%i in ('cargo --list') do (
  echo %%i | C:\Windows\System32\find.exe "install-update">nul
  if not errorlevel 1 (set is_installed=true)
)

if %is_installed% equ true (
  :: Update cargo-update
  cargo install-update cargo-update
  if errorlevel 1 (
    1>&2 echo Failed to update cargo-update
    exit /b 1
  )
) else (
  cargo install cargo-update
  if errorlevel 1 (
    1>&2 echo Failed to install cargo-update
    exit /b 1
  )
)

endlocal

exit /b 0
