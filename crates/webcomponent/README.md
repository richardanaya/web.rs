# webcomponent

<a href="https://docs.rs/webcomponent"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

[Web components](https://www.webcomponents.org/) are a W3C standard for writing your own HTML element. `webcomponent` is a Rust library for easily writing your own web components in Rust with [`js_ffi`](https://github.com/richardanaya/js_ffi).

Features:
- [x] Shadow DOM
- [x] Observable attributes
- [x] Helper functions
- [x] `#![no_std]` and `alloc`

# Hello World
```toml
[lib]
crate-type =["cdylib"] # configures rust project to build a web assembly module

[dependencies]
webcomponent="0.5" # for registering our web component
```
```rust
use webcomponent::*;

struct HelloWorld {
    element: HTMLElement
}

impl CustomElement for HelloWorld {
    fn new(element:HTMLElement) -> Self {
        HelloWorld(element)
    }
    fn connected(&mut self){
        set_html(&self.element,"Hello World!");
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}
```
```html
<!-- a polyfill for web components on some browsers -->
<script src="https://unpkg.com/@webcomponents/webcomponentsjs@latest/webcomponents-loader.js"></script>
<!-- for running your js_ffi library -->
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi@latest/js_ffi.js"></script>
<!-- get things started -->
<script>js_ffi.run("helloworld.wasm");</script>
<!-- now you can put your hello-world element anywhere! -->
<hello-world></hello-world>
```
```makefile
# cli commands for building web assembly I find useful
build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/helloworld.wasm .
lint:
	@cargo fmt
serve:
	python3 -m http.server 8080
```


See demo [here](https://richardanaya.github.io/webcomponent/examples/helloworld/)

# Shadow DOM

```rust
struct HelloWorld {
    element:HTMLElement
}

impl CustomElement for HelloWorld {
    fn new(element: HTMLElement) -> Self {
        HelloWorld(element)
    }
    fn connected(&mut self) {
        let shadow_dom = attach_shadow(&self.element, true);
        set_html(&shadow_dom, r#"<div>Hello <slot name="fname"></slot>!</div>"#);
        set_html(&self.element, r#"<span slot="fname">Richard</span>"#);
    }
}
```

See demo [here](https://richardanaya.github.io/webcomponent/examples/shadowdom/)

# Observable Attributes

```rust
struct HelloPerson {
    element: HTMLElement
}

impl CustomElement for HelloPerson {
    fn new(element: HTMLElement) -> Self {
        HelloPerson(element)
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["first_name"]
    }

    fn connected(&mut self) {
        self.render();
    }

    fn attribute_changed(&mut self, _name: String, _old_value: Option<String>, _new_value: Option<String>) {
        self.render();
    }
}

impl HelloPerson {
    fn render(&mut self){
        let first_name = get_attribute(&self.element, "first_name").unwrap_or("human".to_string());
        let msg = "Hello ".to_string() + &first_name;
        set_html(&self.element, &msg);
    }
}
```

See demo [here](https://richardanaya.github.io/webcomponent/examples/observable_attributes/)

# What about the rest of Javscript?

With `webcomponent` you have a handle to an html element, this is simply a reference to your component that exists in the DOM.  With it we can use [`js_ffi`](https://github.com/richardanaya/js_ffi) ( or any [`js_ffi` based library](https://github.com/richardanaya/js_ffi#standard-web-libraries) ) to do whatever we want to do.

```rust
use webcomponent::*;
use js_ffi::*;

struct LoudButton {
    element: HTMLElement
}

impl CustomElement for LoudButton {
    fn new(element:HTMLElement) -> Self {
        LoudButton(element)
    }
    fn connected(&mut self){
        set_html(&self.element,r#"<button>Shout!</button>"#);
	js!(Node.prototype.addEventListener).call_2(
	  &self.element,
	  "click",
	  create_callback_0(|| {
	    js!(window.alert).invoke_1("I was clicked!");
	  }),
    	);
    }
}
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
for inclusion in `webcomponent` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

