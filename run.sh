set -e
bash build.sh && python -m http.server --directory publish
