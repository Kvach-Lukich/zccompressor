#!/bin/zsh
setopt NULL_GLOB
chmod 777 ./zccompressor
rm -rf ./pkgconfig
rm -f ./*.a
rm -f *(N@)

ls -la
libs=(
libavcodec.63.dylib
libavdevice.63.dylib
libavfilter.12.dylib
libavformat.63.dylib
libavutil.61.dylib
libswresample.7.dylib
libswscale.10.dylib
libffmpeg.9.dylib
)

for target in $libs; do
  ver_base=${target%.dylib}
  ver_base=${ver_base%.*}
  name=${target%%.*}

  if [[ -f $target && ! -L $target ]]; then
    rm -f "${ver_base}"*.dylib "${name}".dylib
  else
    mv "${ver_base}"*.dylib "$target"
  fi
done

codesign --force --sign - ./*.dylib
codesign --force --sign - ./zccompressor

xattr -dr com.apple.quarantine ./