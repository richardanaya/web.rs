# js-wasm
<a href="https://docs.rs/js"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

*JavaScript and WebAssembly should be a joy to use together.*

This project aims to provide a simple, easy to learn, technology-agnostic way bridge the Rust and Javascript using an extremely minimal setup with out-of-box cargo compilation tools.

# Example
```bash
cargo new helloworld --lib
cd helloworld
cargo add js
vim src/lib.rs
```
```rust
use js::*;

#[no_mangle]
pub fn main() {
    js!("function(str){
        console.log(str)
    }")
    .invoke(&["Hello, World!".into()]);
}
```

```bash
vim index.html
```
```html
<html>
    <head>
        <meta charset="utf-8">
        <script src="https://unpkg.com/js-wasm/js-wasm.js"></script>
        <script type="application/wasm" src="helloworld.wasm"></script>
    </head>
    <body>
        Open my console.
    </body>
</html>
```
```bash
vim Cargo.toml
```
```toml
# add these lines for WebAssembly to end of Cargo.toml

[lib]
crate-type =["cdylib"]

[profile.release]
lto = true
```
```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/helloworld.wasm .
python3 -m http.server

# open http://localhost:8000 in browser
# right click, inspect, look at message in console
```

Full example is [here](https://github.com/richardanaya/js-wasm/tree/master/examples/helloworld).

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

# How it works?

The `js` crate makes it really easy to instantiate a javascript function and pass it parameters.  Right now this crate supports these types as parameters:

* Undefined,
* Float64
* BigInt
* String
* Javascript Object References
* Float32Array
* Float64Array
* Boolean

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `js-wasm` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
