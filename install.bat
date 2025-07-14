@echo off

cargo build --release 

set INSTALL_DIR=%USERPROFILE%\pin

if not exist %INSTALL_DIR% mkdir %INSTALL_DIR%

copy /y target\release\gitlicense.exe "%INSTALL_DIR%\"

set PATH="%PATH%;%INSTALL_DIR%" REM Maybe restart terminalafter it

echo Gitlicense was succesfully installed!

pause
