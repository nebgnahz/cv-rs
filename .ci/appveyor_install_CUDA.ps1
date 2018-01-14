appveyor DownloadFile https://developer.nvidia.com/compute/cuda/8.0/prod/local_installers/cuda_8.0.44_windows-exe -FileName cuda_8.0.44_windows.exe
Start-Process -FilePath "cuda_8.0.44_windows.exe" -ArgumentList "-s compiler_8.0" -Wait -NoNewWindow
set PATH=%ProgramFiles%\NVIDIA GPU Computing Toolkit\CUDA\v8.0\bin;%ProgramFiles%\NVIDIA GPU Computing Toolkit\CUDA\v8.0\libnvvp;%PATH%
nvcc -V