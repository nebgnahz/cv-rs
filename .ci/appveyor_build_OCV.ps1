#SCRIPT CONSTANTS
$pwd = Get-Location
$OPENCV_DIR = "$pwd\install\opencv";
$REPO_SOURCE = "opencv"
$OPENCV_VERSION_TAG = "3.4.0"
$CMAKE_CONFIG_GENERATOR = "Visual Studio 15 2017 Win64"
$CMAKE_OPTIONS = @(
  "-DWITH_CUDA:BOOL=ON",
  "-DCUDA_ARCH_BIN=5.2",
  "-DCUDA_ARCH_PTX=",
  "-DBUILD_opencv_java:BOOL=OFF",
  "-DBUILD_opencv_python:BOOL=OFF",
  "-DBUILD_opencv_python2:BOOL=OFF",
  "-DBUILD_opencv_python3:BOOL=OFF",
  "-DBUILD_TESTS:BOOL=OFF",
  "-DBUILD_PERF_TESTS:BOOL=OFF",
  "-DBUILD_DOCS:BOOL=OFF",
  "-DBUILD_EXAMPLES:BOOL=OFF",
  "-DINSTALL_CREATE_DISTRIB:BOOL=ON"
)
$INSTALL_LOCATION = "$pwd\install\$REPO_SOURCE"
$REPO_LOCATION = "$pwd\$REPO_SOURCE"

#SCRIPT BODY
Write-Host "CONFIGURE OPENCV PATHS"
[Environment]::SetEnvironmentVariable("OPENCV_DIR", $OPENCV_DIR, [EnvironmentVariableTarget]::Machine)
[Environment]::SetEnvironmentVariable("OPENCV_LIB", "%OPENCV_DIR%\x64\vc15\lib", [EnvironmentVariableTarget]::Machine)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";%OPENCV_DIR%\x64\vc15\bin", [EnvironmentVariableTarget]::Machine)

if (Test-Path "$OPENCV_DIR\x64\vc15\bin") {
	Write-Host "Compiled OpenCV found. Skip installation"
	return;
}
Write-Host "INSTALL OPENCV"

mkdir build\opencv -ErrorAction SilentlyContinue
mkdir install\opencv -ErrorAction SilentlyContinue

git clone -b $OPENCV_VERSION_TAG --depth 1 https://github.com/opencv/opencv.git

Push-Location -Path "build\$REPO_SOURCE"
Write-Host "cmake -G $CMAKE_CONFIG_GENERATOR -DCMAKE_INSTALL_PREFIX=$INSTALL_LOCATION $REPO_LOCATION $CMAKE_OPTIONS"
cmake -G $CMAKE_CONFIG_GENERATOR "-DCMAKE_INSTALL_PREFIX=$INSTALL_LOCATION" $REPO_LOCATION @CMAKE_OPTIONS
if($LastExitCode -ne 0) { $host.SetShouldExit($LastExitCode )  }
cmake --build .  --target install --config release
if($LastExitCode -ne 0) { $host.SetShouldExit($LastExitCode )  }
Pop-Location