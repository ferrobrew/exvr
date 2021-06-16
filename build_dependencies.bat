@echo off
setlocal

cd external\OpenXR-SDK
if not exist build\win64\NUL mkdir build\win64
cd build\win64
cmake -G "Visual Studio 16 2019" -A x64 ..\..
cd src\loader
msbuild openxr_loader.vcxproj /p:configuration=debug /property:MultiProcessorCompilation=true
msbuild openxr_loader.vcxproj /p:configuration=release /property:MultiProcessorCompilation=true