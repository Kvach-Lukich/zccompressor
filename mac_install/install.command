#!/bin/zsh
local script_dir="${0:a:h}"
ls
local script_src='on open droppedItems
	set args to ""
	repeat with anItem in droppedItems
		set args to args & " " & quoted form of (POSIX path of anItem)
	end repeat
	do shell script "'"cd \\\"${script_dir}\\\" && \\\"${script_dir}/zccompressor\\\""'" & args
end open'
print -rn -- "$script_src"

# rm -rf zcc.app
command -v osacompile
osacompile -o zcc.app <(print -rn -- "$script_src")

cp -f ../assets/zcc.icns zcc.app/Contents/Resources/applet.icns
# mv -f zcc.app ../
