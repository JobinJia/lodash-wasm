rm -rf @jobinjia &&
#wasm-pack build --out-dir @jobinjia/lodash-wasm --scope jobinjia &&
wasm-pack build --out-dir @jobinjia/lodash-wasm --scope jobinjia --target bundler &&
#cp -r @jobinjia   playground/node_modules/