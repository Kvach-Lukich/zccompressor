#!/bin/bash
export FFMPEG_DIR=/root/rust/zccompressor/libs/mac_ffmpeg
#export FFMPEG_INCLUDE_DIR="$FFMPEG_DIR/include"
#export FFMPEG_LIB_DIR="$FFMPEG_DIR/lib"

export SDKROOT="/toolchain/macos/SDK/MacOSX26.1.sdk"
export CC_aarch64_apple_darwin="/toolchain/macos/bin/oa64-clang"
export CXX_aarch64_apple_darwin="/toolchain/macos/bin/oa64-clang++"
export AR_aarch64_apple_darwin="/toolchain/macos/bin/aarch64-apple-darwin25.1-ar"
export CFLAGS_aarch64_apple_darwin="--target=arm64-apple-macosx -mmacosx-version-min=11.0 -isysroot /toolchain/macos/SDK/MacOSX26.1.sdk"
export CXXFLAGS_aarch64_apple_darwin="--target=arm64-apple-macosx -mmacosx-version-min=11.0 -isysroot /toolchain/macos/SDK/MacOSX26.1.sdk"
export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_darwin="--target=arm64-apple-macosx -isysroot /toolchain/macos/SDK/MacOSX26.1.sdk"
export OSXCROSS_PKG_CONFIG_USE_NATIVE_VARIABLES="1"
#export PKG_CONFIG_PATH="$FFMPEG_LIB_DIR/pkgconfig"
#export PKG_CONFIG_LIBDIR="$FFMPEG_LIB_DIR/pkgconfig"

cargo build --target aarch64-apple-darwin --release && \
#rcodesign sign ./target/aarch64-apple-darwin/release/zccompressor
