@echo off
delete /r zccompressor
delete win_x64_zccompressor.zip
mkdir zccompressor\data
copy "target\release\zccompressor.exe" "zccompressor\data\"
copy libs\ffmpeg\bin\*.dll "zccompressor\data"
copy config.ini "zccompressor\data"
copy installers\win\* "zccompressor\data"

if "%~1"=="-z" (
  7z a -tzip -mx=9 win_x64_zccompressor.zip zccompressor
)