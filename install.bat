@echo off

cargo build --release 

set INSTALL_DIR=%USERPROFILE%\pin

if not exist %INSTALL_DIR% mkdir %INSTALL_DIR%

copy /y target\release\gitlicense.exe "%INSTALL_DIR%\"

set PATH="%PATH%;%INSTALL_DIR%" REM Consider restarting terminal after invoking this command

echo Gitlicense was succesfully installed!

pause
