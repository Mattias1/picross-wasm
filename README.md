Picross wasm
=============
A little picross app I made to play with WebAssembly and Rust.

Dev setup
----------
Install the tools as described in <https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c>.
Then compile and run with `./run.sh` (make sure it's executable).
It runs at [localhost:8000](http://localhost:8000).

Include on a page
------------------
To include picross on your page, add the `.js` and `.wasm` scripts from the [/www](/www) folder and
add the following to your html file:
```html
<div id="picross-wasm-player"></div>
```
and:
```html
<script type="module">
  import { default as init, load_puzzle } from './picross_wasm.js';
  await init();
  await load_puzzle("eyes");
</script>
```