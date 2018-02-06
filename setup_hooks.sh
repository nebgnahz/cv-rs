#!/bin/sh
rustup component add rustfmt-preview

rustfmt_path=`which rustfmt`
echo "#!/bin/sh
declare -a rust_files=()
declare -a cpp_files=()

files=\$(git diff-index --name-only HEAD)
echo 'Formatting source files'
for file in \$files; do
    if [ ! -f \"\${file}\" ]; then
        continue
    fi
    if [[ \"\${file}\" == *.rs ]]; then
        rust_files+=(\"\${file}\")
    fi
    if [[ \"\${file}\" =~ (\.h|\.cpp|\.cc) ]]; then
        cpp_files+=(\"\${file}\")
    fi
done
if [ \${#rust_files[@]} -ne 0  ]; then
     $rustfmt_path \${rust_files[@]} &
fi
if [ \${#cpp_files[@]} -ne 0  ]; then
     clang-format -i \${cpp_files[@]} &
fi
wait

changed_files=(\"\${rust_files[@]}\" \"\${cpp_files[@]}\")
if [ \${#changed_files[@]} -ne 0 ]; then
    git add \${changed_files[@]}
    echo \"Formatting done, changed files: \${changed_files[@]}\"
else
    echo \"No changes, formatting skipped\"
fi"  > .git/hooks/pre-commit

chmod +x .git/hooks/pre-commit

echo "Hooks updated"
