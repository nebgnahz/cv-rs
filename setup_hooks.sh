#!/bin/sh
rustup component add rustfmt-preview

rustfmt_path=`find $HOME/.rustup/toolchains/ -wholename '*bin/rustfmt*'`
echo "#!/bin/sh
find . -name *.rs -type f -print | xargs $rustfmt_path"  > .git/hooks/pre-commit

chmod +x .git/hooks/pre-commit

echo "Hooks updated"
