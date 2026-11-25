#!/bin/bash
export FFMPEG_DIR="$PWD/libs/linux_ffmpeg"
export PKG_CONFIG_PATH="$FFMPEG_DIR/lib/pkgconfig:$PKG_CONFIG_PATH"
export RUSTFLAGS="-C link-arg=-Wl,--disable-new-dtags -C link-arg=-Wl,-rpath,\$ORIGIN -C link-arg=-Wl,-rpath,\$ORIGIN/lib -C linker=clang -C link-arg=-fuse-ld=lld"

cargo build --release
