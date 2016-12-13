#!/bin/sh

set -ex

cargo doc --no-deps --features gpu
cp -r target/doc/* docs

cat > docs/index.html <<EOF
<!DOCTYPE HTML>
<html lang="en-US">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="refresh" content="1;url=cv/index.html">
        <script type="text/javascript">
            window.location.href = "cv/index.html"
        </script>
        <title>Page Redirection</title>
    </head>
    <body>
        <!-- Note: don't tell people to `click` the link, just tell them that it is a link. -->
        If you are not redirected automatically, follow the <a href='cv/index.html'>link</a>
    </body>
</html>
EOF


git checkout -B gh-pages
git add -f docs
git commit -am "Rebuild Documentation"
git filter-branch -f --prune-empty --subdirectory-filter docs
git push -f origin gh-pages
git checkout -
