#!/bin/zsh
local script_dir="${0:a:h}"
ls

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

print -r -- "$script_src"
rm -rf zcc.app
if ! osacompile -o zcc.app <(print -rn -- "$script_src"); then
	print -rn -- "$script_src" > zcc.applescript
	osacompile -o zcc.app zcc.applescript
	rm zcc.applescript
fi

cp -f ../assets/zcc.icns zcc.app/Contents/Resources/applet.icns
mv -f zcc.app ../
