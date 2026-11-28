#!/bin/bash
FULL_PATH=$(dirname $(realpath "$0"))
cat << EOF > ../zcc.desktop
[Desktop Entry]
Type=Application
Name=Zero-cross compressor
Comment=Ultimate no distortion dynamic compressor
Exec=$FULL_PATH/zccompressor %F
Icon=$FULL_PATH/zc.png
Terminal=true
Categories=AudioVideo
EOF
chmod +x ../zcc.desktop
ln -s "$FULL_PATH/config.ini" ../config.ini
