#!/bin/sh
rustup component add rustfmt-preview

rustfmt_path=`find $HOME/.rustup/toolchains/ -wholename '*bin/rustfmt*'`
echo "#!/bin/sh
declare -a rust_files=()
declare -a cpp_files=()

for file in \$(git ls-files -m -o --exclude-standard); do
    if [[ \"\${file}\" == *.rs ]]; then
        rust_files+=(\"\${file}\")
    fi
    if [[ \"\${file}\" =~ (\.h|\.cpp|\.cc) ]]; then
        cpp_files+=(\"\${file}\")
    fi
done
echo \${rust_files[@]} | xargs --no-run-if-empty $rustfmt_path &
echo \${cpp_files[@]} | xargs --no-run-if-empty clang-format -i &
wait"  > .git/hooks/pre-commit

chmod +x .git/hooks/pre-commit

echo "Hooks updated"
