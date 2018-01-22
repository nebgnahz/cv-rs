param([Parameter(mandatory=$true)][bool] $EnableCuda)
$CudaSwitch = If ($EnableCuda) {"ON"} Else {"OFF"}
#SCRIPT CONSTANTS
$pwd = Get-Location
$REPO_LOCATION = "$pwd\opencv"
$OPENCV_VERSION_TAG = "3.4.0"
$COMPILER = "vc14"
$CMAKE_CONFIG_GENERATOR = "Visual Studio 14 2015 Win64"
$OPENCV_BUILD_DIR = "$pwd\$COMPILER\build\opencv";
$OPENCV_DIR = "$pwd\$COMPILER\install\opencv_$COMPILER";
$CMAKE_OPTIONS = @(
  "-DWITH_CUDA:BOOL=OFF",
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

#SCRIPT BODY
Write-Host "CONFIGURE OPENCV PATHS"

$env:OPENCV_DIR = $OPENCV_DIR
$env:OPENCV_LIB = "$OPENCV_DIR\x64\$COMPILER\lib"
if ($env:Path.IndexOf("$OPENCV_DIR\x64\$COMPILER\bin") -eq (-1)) {
	$env:Path = "$env:Path;$OPENCV_DIR\x64\$COMPILER\bin"
}

[Environment]::SetEnvironmentVariable("OPENCV_DIR", $env:OPENCV_DIR, [EnvironmentVariableTarget]::Machine)
[Environment]::SetEnvironmentVariable("OPENCV_LIB", $env:OPENCV_LIB, [EnvironmentVariableTarget]::Machine)
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::Machine)

if (Test-Path "$OPENCV_DIR\x64\$COMPILER\bin") {
	Write-Host "Compiled OpenCV found. Skip installation"
	return;
}

#CHECK EXISTENCE OF GIT AND CMAKE
$oldErrorAction = $ErrorActionPreference
$ErrorActionPreference = "Stop"
git --version
cmake --version
$ErrorActionPreference = $oldErrorAction


Write-Host "INSTALL OPENCV AT $OPENCV_DIR"

mkdir $OPENCV_BUILD_DIR -ErrorAction SilentlyContinue
mkdir $OPENCV_DIR -ErrorAction SilentlyContinue

$oldErrorAction = $ErrorActionPreference
$ErrorActionPreference = "SilentlyContinue"
git clone -b $OPENCV_VERSION_TAG --depth 1 https://github.com/opencv/opencv.git
$ErrorActionPreference = $oldErrorAction

Push-Location -Path $OPENCV_BUILD_DIR
Write-Host "cmake -G $CMAKE_CONFIG_GENERATOR -DCMAKE_INSTALL_PREFIX=$OPENCV_DIR -DCMAKE_BUILD_TYPE=Release $REPO_LOCATION $CMAKE_OPTIONS"
cmake -G $CMAKE_CONFIG_GENERATOR "-DCMAKE_INSTALL_PREFIX=$OPENCV_DIR" -DCMAKE_BUILD_TYPE=Release $REPO_LOCATION @CMAKE_OPTIONS
if($LastExitCode -ne 0) { $host.SetShouldExit($LastExitCode )  }
cmake --build .  --target install --config release
if($LastExitCode -ne 0) { $host.SetShouldExit($LastExitCode )  }
Pop-Location
