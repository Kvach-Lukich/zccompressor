$WshShell = New-Object -ComObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$PSScriptRoot\..\zcc.lnk")
$Shortcut.TargetPath = "$PSScriptRoot\zccompressor.exe"
$Shortcut.WorkingDirectory = "$PSScriptRoot"
$Shortcut.IconLocation = "$PSScriptRoot\zccompressor.exe,0"
$Shortcut.Description = "Zero-cross compressor CLI"
$Shortcut.Save()

$WshShell = New-Object -ComObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$PSScriptRoot\..\config.lnk")
$Shortcut.TargetPath = "$PSScriptRoot\config.ini"
$Shortcut.WorkingDirectory = "$PSScriptRoot"
$Shortcut.Description = "Zero-cross compressor Config"
$Shortcut.Save()