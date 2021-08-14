set -e

wasm-pack build --release --target web --out-dir publish/pkg
find ./publish/pkg -name ".gitignore" -delete

function download_if_not_exist() {
    file_name="$1"
    file_path="./publish/$file_name"
    if test -f "$file_path"; then
        echo "$file_path exists"
    else
        curl "https://cdn.jsdelivr.net/npm/qr-scanner/$file" -o "$file_path"
        echo "Downloaded $file_path"
    fi
}

download_if_not_exist "qr-scanner.min.js"
download_if_not_exist "qr-scanner.min.js.map"
download_if_not_exist "qr-scanner-worker.min.js"
download_if_not_exist "qr-scanner-worker.min.js.map"
