#!/bin/bash
rm ./zccompressor/*
rm ./zccompressor/data/*
mkdir -p zccompressor/data
cp ./target/release/zccompressor ./zccompressor/data/

libs=(
  libavcodec.so.63
  libavdevice.so.63
  libavfilter.so.12
  libavformat.so.63
  libavutil.so.61
  libswresample.so.7
  libswscale.so.10
)

for lib in "${libs[@]}"; do
  cp "./libs/linux_ffmpeg/lib/${lib}."* "zccompressor/data/${lib}"
done

cp ./installers/linux/install.sh ./zccompressor/data/
cp config.ini ./zccompressor/data/
cp ./assets/zc.png ./zccompressor/data/

if [[ "$1" == "-z" ]]; then
  rm -f linux_x64_zccompressor.zip
  zip -9 -ry linux_x64_zccompressor.zip zccompressor
fi