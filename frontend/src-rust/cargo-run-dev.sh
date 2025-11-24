!#/bin/bash

cargo watch -s "wasm-pack build ./ --target web --out-dir ../frontend/pkg"
