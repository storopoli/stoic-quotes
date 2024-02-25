alias b := build

build:
    echo "Building..."
    rm -rf public
    cargo build --target wasm32-unknown-unknown --release
    mkdir -p public/
    wasm-bindgen target/wasm32-unknown-unknown/release/stoic_quotes.wasm --out-dir ./public --target web
    mkdir -p public/assets
    cp assets/favicon.ico public/assets/favicon.ico
    cp assets/htmx.min.js public/assets/htmx.min.js
    cp assets/main.css public/assets/main.css
    cp assets/index.html public/index.html

serve:
    live-server public
