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

# Interacting with DOM objects

Here's a more complex example that invokes functions that return references to DOM objects

```rust
use js::*;

fn query_selector(selector: &str) -> ExternRef {
    let query_selector = js!(r#"
        function(selector){
            return document.querySelector(selector);
        }"#);
    query_selector.invoke_and_return_object(&[selector.into()])
}

fn canvas_get_context(canvas: &ExternRef) -> ExternRef {
    let get_context = js!(r#"
        function(canvas){
            return canvas.getContext("2d");
        }"#);
    get_context.invoke_and_return_object(&[canvas.into()])
}

fn canvas_set_fill_style(ctx: &ExternRef, color: &str) {
    let set_fill_style = js!(r#"
        function(ctx, color){
            ctx.fillStyle = color;
        }"#);
    set_fill_style.invoke(&[ctx.into(), color.into()]);
}

fn canvas_fill_rect(ctx: &ExternRef, x: f64, y: f64, width: f64, height: f64) {
    let fill_rect = js!(r#"
        function(ctx, x, y, width, height){
            ctx.fillRect(x, y, width, height);
        }"#);
    fill_rect.invoke(&[ctx.into(), x.into(), y.into(), width.into(), height.into()]);
}

#[no_mangle]
pub fn main() {
    let screen = query_selector("#screen");
    let ctx = &canvas_get_context(&screen);
    canvas_set_fill_style(&ctx, "red");
    canvas_fill_rect(&ctx, 10.0, 10.0, 100.0, 100.0);
    canvas_set_fill_style(&ctx, "green");
    canvas_fill_rect(&ctx, 20.0, 20.0, 100.0, 100.0);
    canvas_set_fill_style(&ctx, "blue");
    canvas_fill_rect(&ctx, 30.0, 30.0, 100.0, 100.0);
}
```

The invocation `invoke_and_return_object` returns a structure called an `ExternRef` that is an indirect reference to something received from JavaScript. You can pass around this reference to other JavaScript invocations that will receive the option. When the structure dropped according to Rust lifetimes, it's handle is released from the JavaScript side.

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
