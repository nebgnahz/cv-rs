# How to contribute

Implement more OpenCV functions/modules and submit a PR request.

Before submitting a PR, make sure you have formatted both C/C++ code and Rust
code, otherwise Travis will complain.

You have to have installed `clang-format`. You can install it as part of full llvm toolchain ([download link](http://releases.llvm.org/download.html)) to make formatting happen. On mac you can use `brew install clang-format`.

When you're done run `setup_hooks.sh`. That will enable automatic source code formatting on commit.