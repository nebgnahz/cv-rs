param([Parameter(mandatory=$true)][string] $FileName)
$VERSION = "9.1"
$argumentList = "-s nvcc_$VERSION cublas_$VERSION cublas_dev_$VERSION cufft_$VERSION cufft_dev_$VERSION npp_$VERSION npp_dev_$VERSION"
$envPath = "$env:ProgramFiles\NVIDIA GPU Computing Toolkit\CUDA\v$VERSION\bin;$env:ProgramFiles\NVIDIA GPU Computing Toolkit\CUDA\v$VERSION\libnvvp";
Write-Host "Install CUDA from $FileName with argumentList $argumentList"
Start-Process -FilePath $FileName -ArgumentList $argumentList -Wait
[Environment]::SetEnvironmentVariable("Path", $env:Path + $envPath, [EnvironmentVariableTarget]::Machine)
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine")
nvcc.exe -V
if($LastExitCode -ne 0) { $host.SetShouldExit($LastExitCode )  }