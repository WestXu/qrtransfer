set -e

wasm-pack build --release --target web --out-dir publish/pkg
find ./publish/pkg -name ".gitignore" -delete
