@echo off
title NativeKit Windows Dev Shell
echo ===================================================
echo   NativeKit Windows Dev Shell
echo ===================================================
echo.
echo Running dotnet run in Debug mode...
echo.
dotnet run -c Debug
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Application exited with code %ERRORLEVEL%
    pause
)
