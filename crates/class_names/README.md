# class_names

<a href="https://docs.rs/class_names"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

```toml
[dependencies]
class_names="0"
```

# How to use

This library includes a macro for easily expressing a list of CSS class names (some which may be optional).  The macro takes in a mixed list of `&str`,`String`,`Option<&str>`, or `Option<String>` and calculates a final list of class names for the HTML `class` attribute.  Here's some examples

1. use strings
```rust
class_names!("big-button", "red".to_string()) // "big-button red"
```
2. accepts optionals
```rust
class_names!("big-button", if btn_red { Some("red") } else { None } )
```
3. a concise way of writing an optional
```rust
class_names!("big-button", btn_inactive.then(|| "inactive"))
```
4. one day from future features in [Rust nightly](https://doc.rust-lang.org/std/primitive.bool.html#method.then_some) you'll be able to write like this
```rust
class_names!("big-button", btn_inactive.then_some("inactive"))
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `class_names` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
