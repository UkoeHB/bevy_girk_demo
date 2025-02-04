# Building for WASM
# TODO: find better way to input host server address
cp -r assets wasm/client
GIRK_HOST_ADDR=girk-demo-backend.online:48888 GIRK_HOST_IS_WSS=true cargo build -p client --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --out-name girk_client --out-dir wasm/client --target web target/wasm32-unknown-unknown/release/client.wasm
wasm-opt --all-features -Os wasm/client/girk_client_bg.wasm -o wasm/client/girk_client_bg.wasm
zip -r xbuilds/girk_client.zip wasm/client
