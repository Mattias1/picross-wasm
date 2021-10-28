echo "Compile picross from Rust to WASM and run a simple python webserver with the html/js/wasm files."

# Compile rust to wasm
cargo build --target wasm32-unknown-unknown

# Generate a wasm-bindgen js file that allows importing as an ES6 module
cd target/wasm32-unknown-unknown/debug
wasm-bindgen --target web --no-typescript --out-dir . picross_wasm.wasm

# Minimize the code size
wasm-gc picross_wasm_bg.wasm
cd ../../../

# Copy the compiled wasm file
cp target/wasm32-unknown-unknown/debug/picross_wasm_bg.wasm ./www
cp target/wasm32-unknown-unknown/debug/picross_wasm.js ./www

# Run the server
cd www
python3 server.py
cd ../
