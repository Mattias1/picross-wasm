Picross wasm
=============
A little picross app I made to play with WebAssembly and Rust.

Dev setup
----------
Install rust and web assembly (wasm) compilation.
```
curl https://sh.rustup.rs -sSf | sh
rustup target add wasm32-unknown-unknown
cargo install wasm-gc
```

Also install python to run a local webserver.

Then compile and run with `./run.sh` (make sure it's executable).
It runs at [localhost:8000](http://localhost:8000).

Include on a page
------------------
To include picross on your page, add the `.js`, `.css` and `.wasm` scripts from
the [/www](/www) folder and add the following to your html file:
```html
<div id="picross-wasm-player"></div>
```
and:
```html
<link rel="stylesheet" type="text/css" href="./picross_wasm.css">
<script type="module">
  import { default as init, load_puzzle } from './picross_wasm.js';
  await init();
  await load_puzzle("eyes");
</script>
```
