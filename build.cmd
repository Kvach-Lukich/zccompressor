@echo off
set "PROJECT_DIR=%~dp0"
if "%PROJECT_DIR:~-1%"=="\" set "PROJECT_DIR=%PROJECT_DIR:~0,-1%"

if not defined FFMPEG_DIR (
    set "FFMPEG_DIR=%PROJECT_DIR%\libs\ffmpeg"
    set "Path=%Path%;%PROJECT_DIR%\libs\ffmpeg\bin"
    :: path to your clang
    set "LIBCLANG_PATH=C:\rust\C\bin"
    set "VCPKGRS_DYNAMIC=1"
    set "FFMPEG_NO_PKG_CONFIG=1"
)

cargo build --release
:: || exit /b %errorlevel%

if exist "target\release" if not exist "target\release\config.ini" (
    if exist "config.ini" (
        copy "config.ini" "target\release\config.ini"
    )
    copy libs\ffmpeg\bin\*.dll target\release
)
