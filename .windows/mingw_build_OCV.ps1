param([Parameter(mandatory=$true)][string] $MinGWPath)
#SCRIPT CONSTANTS
$pwd = Get-Location
$REPO_LOCATION = "$pwd\opencv"
$OPENCV_VERSION_TAG = "3.4.0"
$COMPILER = "mingw"
$CMAKE_CONFIG_GENERATOR = "MinGW Makefiles"
$OPENCV_BUILD_DIR = "$pwd\artifacts\$COMPILER\build\opencv";
$OPENCV_DIR = "$pwd\artifacts\$COMPILER\install\opencv";
$OPENCV_CONTRIB_DIR = "$pwd\opencv_contrib\modules";
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
  "-DINSTALL_CREATE_DISTRIB:BOOL=ON",
  "-DCPU_DISPATCH="
)

#SCRIPT BODY
Write-Host "CONFIGURE OPENCV PATHS"

$env:OPENCV_DIR = $OPENCV_DIR
$env:OPENCV_LIB = "$OPENCV_DIR\x64\$COMPILER\lib"
if ($env:Path.IndexOf("$OPENCV_DIR\x64\$COMPILER\bin") -eq (-1)) {
	$env:Path = "$env:Path;$OPENCV_DIR\x64\$COMPILER\bin"
}
if ($env:Path.IndexOf($MinGWPath) -eq (-1)) {
	$env:Path = "$env:Path;$MinGWPath"
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
Write-Host (Get-Command mingw32-make).Source
$ErrorActionPreference = $oldErrorAction


Write-Host "INSTALL OPENCV AT $OPENCV_DIR"

mkdir $OPENCV_BUILD_DIR -ErrorAction SilentlyContinue
mkdir $OPENCV_DIR -ErrorAction SilentlyContinue

git submodule update --init --recursive

Push-Location -Path $OPENCV_BUILD_DIR
$CMakeArgs = $CMAKE_OPTIONS + ("-DCMAKE_INSTALL_PREFIX=$OPENCV_DIR", "-DCMAKE_BUILD_TYPE=Release", "-DOPENCV_EXTRA_MODULES_PATH=$OPENCV_CONTRIB_DIR", $REPO_LOCATION)
Write-Host "cmake -G $CMAKE_CONFIG_GENERATOR $CMakeArgs"
cmake -G $CMAKE_CONFIG_GENERATOR @CMakeArgs
if($LastExitCode -ne 0) { $host.SetShouldExit($LastExitCode )  }
cmake --build .  --target install --config release -- -j $env:NUMBER_OF_PROCESSORS 
if($LastExitCode -ne 0) { $host.SetShouldExit($LastExitCode )  }
Pop-Location
