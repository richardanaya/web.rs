# js-wasm
<a href="https://docs.rs/js"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

*JavaScript and WebAssembly should be a joy to use together.*

This project aims to provide a simple, easy to learn, technology-agnostic way bridge the Rust and Javascript using an extremely minimal setup with out-of-box cargo compilation tools.

See a [demo](https://richardanaya.github.io/js-wasm/examples/snake/index.html) of it working!

# How It Works?
```bash
cargo new helloworld
cd helloworld
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
        ...
    </body>
</html>
```
```bash
cargo add js
```
```rust
use js::*;

#[no_mangle]
pub fn main() {
    let fn_log = js!("function(strPtr,strLen){
        console.log(this.readUtf8FromMemory(strPtr,strLen)); 
    }");
    let msg = "Hello World!";
    fn_log.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}
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
```

# details 
In your JS function context `this` contains several functions handle most issues you'll encounter

| Name          | Description   |
| ------------- | ------------- |
| `readUtf8FromMemory(start,length)` | Extract utf-8 text from your program's memory. |
| `readUtf16FromMemory(start,length)` | Extract utf-16 text from your program's memory. |
| `writeUtf8ToMemory(str)` | Write utf-8 to a memory location you are sure it should go. |
| `readUint8ArrayFromMemory(start)` | Read a list of uint8 from a pointer to a location of a number of elements, followed by a pointer to bytes in memory. |
| `storeObject(object)` | Store an object in your context for later reference, get a handle you can give to WebAssembly. |
| `getObject(handle)` | Retreive and object from your context with a handle. |
| `releaseObject(handle)` | Release a stored object so it's memory can be freed. |
| `createCallback(callbackHandle)` | Creates a callback function that will pass its arguments to the associated WebAssembly function represented by the handle. |
| `module` | Get access to your program so you can call methods on it. |

<p align="center">
  <img height="300" src="images/undraw_design_team_af2y.png">
</p>

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `js-wasm` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
