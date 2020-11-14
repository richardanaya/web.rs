# web-rs

<a href="https://docs.rs/web"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A Rust library full of useful functions from various microlibraries for interacting with the web browser using [`js-wasm`](../../).

<p align="center">
  <img height="300" src="../../images/undraw_web_developer_p3e5.png">
</p>

```toml
[dependencies]
web = "0.1"
```

```rust
use web::*;

#[no_mangle]
pub fn main() {
    set_interval(|| {
            log(&format!("⏰ {}", random()));
        }, 1000);
}
```
```html
<html>
    <head>
        <script src="https://unpkg.com/js-wasm/js-wasm.js"></script>
        <script type="application/wasm" src="helloworld.wasm"></script>
    </head>
    <body>
        ...
    </body>
</html>
```
```make
# cli commands for building web assembly
build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/helloworld.wasm .
lint:
	@cargo fmt
serve:
	python3 -m http.server 8080
```

- [x] console, errors, timing
- [x] timers, render loops, intervals
- [x] random numbers
- [ ] DOM
- [ ] canvas
- [ ] webgl

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
