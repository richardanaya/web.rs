<p align="center">
<img src="https://user-images.githubusercontent.com/294042/208995865-88502572-76f7-4ce7-8157-9bca9f1c9444.png"/>
</p>


<a href="https://docs.rs/web"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

*Write web applications using Rust using WebAssembly*

I wanted a library that someone could learn in an afternoon how to use and start making interactive browser experiences with.  This project doesn't support every browser function under the sun.  Though you can easily add your own using the [Javascript invoking mechanism](https://github.com/richardanaya/web.rs/tree/master/crates/js) used by this library.

Check out the documentation [here](https://docs.rs/web)

```terminal
cargo add web
```

# Hello World

Let's just look at a basic example of how to put things in the console:

```rust
use web::*;

#[no_mangle]
pub fn main() {
    console_log("Hello World!");
}
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

Remember to configure your library for WebAssembly

```toml
# add these lines for WebAssembly to end of your Cargo.toml

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

Full example is [here](https://github.com/richardanaya/web.rs/tree/master/examples/helloworld).

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `web` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
