@echo off
title NativeKit Auto-Sync to Desktop
echo ===================================================
echo   NativeKit Auto-Sync to Desktop (Safe Mode)
echo ===================================================
echo.
powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$src = '%~dp0'; $dst = Join-Path $env:USERPROFILE 'Desktop\NativeKitWindows'; " ^
  "Write-Host '🧹 Cleaning stale XAML build cache (obj/bin) on Desktop...' -ForegroundColor Yellow; " ^
  "Remove-Item -Path (Join-Path $dst 'obj') -Recurse -Force -ErrorAction SilentlyContinue; " ^
  "Remove-Item -Path (Join-Path $dst 'bin') -Recurse -Force -ErrorAction SilentlyContinue; " ^
  "Write-Host '✨ Syncing ' $src ' -> ' $dst -ForegroundColor Green; " ^
  "robocopy $src $dst *.cs *.xaml *.csproj *.bat *.png *.ico *.dll /XD bin obj .vs x64 ARM64 /NJH /NJS /NDL /NC /NS > $null; " ^
  "Write-Host '✅ Sync complete! You can now open and Run the project in Visual Studio.' -ForegroundColor Cyan; " ^
  "while($true) { " ^
  "  Start-Sleep -Milliseconds 1000; " ^
  "  robocopy $src $dst *.cs *.xaml *.csproj *.bat *.png *.ico *.dll /IS /IT /XD bin obj .vs x64 ARM64 /NJH /NJS /NDL /NC /NS > $null; " ^
  "}"
pause
