@echo off
mkdir zccompressor\data
copy "target\release\zccompressor.exe" "zccompressor\data"
copy libs\ffmpeg\bin\*.dll "zccompressor\data"
copy config.ini "zccompressor\data"
copy installers\win\* "zccompressor\data"