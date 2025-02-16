@echo off

REM Build the project with cargo in release mode for the entire workspace
cargo build --release --workspace

REM Create the release/windows directory if it does not exist
if not exist release\windows (
    mkdir release\windows
)

REM Copy the executables to the release/windows directory
copy /Y target\release\progetto_g32.exe release\windows\progetto_g32.exe
copy /Y target\release\setup.exe release\windows\setup.exe
copy /Y target\release\uninstall.exe release\windows\uninstall.exe

echo Build and copy process completed successfully.