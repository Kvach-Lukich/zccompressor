#!/bin/zsh
local script_dir="${0:a:h}"
local script_src='on open droppedItems
	set args to ""
	repeat with anItem in droppedItems
		set args to args & " " & quoted form of (POSIX path of anItem)
	end repeat
	tell application "Terminal"
		activate
		do script "'"cd \\\"${script_dir}\\\" && \\\"${script_dir}/zccompressor\\\""'" & args
	end tell
end open'
rm -rf zcc.app
if ! osacompile -o zcc.app -e "${script_src}"; then
	print -rn -- "$script_src" > zcc.applescript
	osacompile -o zcc.app zcc.applescript
	rm zcc.applescript
fi

rm zcc.app/Contents/Resources/Assets.car
rm zcc.app/Contents/Resources/applet.icns
rm zcc.app/Contents/Resources/droplet.icns
cp -f zcc.icns zcc.app/Contents/Resources/droplet.icns
ln -sf droplet.icns zcc.app/Contents/Resources/applet.icns
mv -f zcc.app ../
ln -sfn ${script_dir}/config.ini ../config.ini

codesign --force --sign - ../zcc.app
codesign --force --sign - *.dylib
codesign --force --sign - zccompressor
xattr -dr com.apple.quarantine ../../zccompressor
