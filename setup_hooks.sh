#!/bin/sh
rustup component add rustfmt-preview

rustfmt_path=`find $HOME/.rustup/toolchains/ -wholename '*bin/rustfmt*'`
echo "#!/bin/sh
find . -name '*.rs' -type f | xargs $rustfmt_path &
find native/ -type f | xargs clang-format -i &
wait"  > .git/hooks/pre-commit

chmod +x .git/hooks/pre-commit

echo "Hooks updated"
