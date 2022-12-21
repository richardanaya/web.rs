# js-wasm

<a href="https://docs.rs/js"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

*JavaScript and WebAssembly should be a joy to use together.*

This project aims to provide a simple, easy to learn, technology-agnostic way bridge the Rust and Javascript using an extremely minimal setup with out-of-box cargo compilation tools. My hope is almost any Rust developer familiar with JavaScript could learn how to use it in a lazy afternoon.

# Hello World

Let's just look at a basic example of how to put things in the console:

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

Notice the basic syntax is building up a function, and then invoking it with an array of parameters.  Underneath the covers, this is an array of enums called `InvokeParameter`, i've made little converters for various types (see below) to help the data cross the barrier.  For the most part you can convert data using `.into()` for `InvokeParameter`.

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

This library has a fairly simple mechanism for executing your WebAssembly during page load.

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

Below are several examples that show common operations one might want to do.

# Interacting with DOM objects

Here's a more complex example that invokes functions that return references to DOM objects

<img width="152" alt="Screen Shot 2022-12-18 at 9 21 54 PM" src="https://user-images.githubusercontent.com/294042/208353503-00c21fcb-f45b-4612-be3c-e9624040d0e9.png">


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
    let ctx = canvas_get_context(&screen);
    canvas_set_fill_style(&ctx, "red");
    canvas_fill_rect(&ctx, 10.0, 10.0, 100.0, 100.0);
    canvas_set_fill_style(&ctx, "green");
    canvas_fill_rect(&ctx, 20.0, 20.0, 100.0, 100.0);
    canvas_set_fill_style(&ctx, "blue");
    canvas_fill_rect(&ctx, 30.0, 30.0, 100.0, 100.0);
}
```

The invocation `invoke_and_return_object` returns a structure called an `ExternRef` that is an indirect reference to something received from JavaScript. You can pass around this reference to other JavaScript invocations that will receive the option. When the structure dropped according to Rust lifetimes, it's handle is released from the JavaScript side.

# Callbacks and timers

This library is not opinionated about how to callback into Rust. There are several methods one can use. Here's a simple example.

```rust
use js::*;

fn console_log(s: &str) {
    let console_log = js!(r#"
        function(s){
            console.log(s);
        }"#);
    console_log.invoke(&[s.into()]);
}

fn random() -> f64 {
    let random = js!(r#"
        function(){
            return Math.random();
        }"#);
    random.invoke(&[])
}

#[no_mangle]
pub fn main() {
    let start_loop = js!(r#"
        function(){
            window.setInterval(()=>{
                this.module.instance.exports.run_loop();
            }, 1000)
        }"#);
    start_loop.invoke(&[]);
}

#[no_mangle]
pub fn run_loop(){
    console_log(&format!("⏰ {}", random()));
}
```

Notice how in the `start_loop` function, `this` actually references a context object that can be used to perform useful functions (see below) and for the importance of this demo, get ahold of the WebAssembly module so we can callback functions on it.

# Getting data back into WebAssembly

Let's focus on one last example.  A button that when you click it, fetches data from the public Pokemon API and put's it on the screen.

```rust
use js::*;

fn query_selector(selector: &str) -> ExternRef {
    let query_selector = js!(r#"
        function(selector){
            return document.querySelector(selector);
        }"#);
    query_selector.invoke_and_return_object(&[selector.into()])
}

fn add_click_listener(element: &ExternRef, callback: &str) {
    let add_click_listener = js!(r#"
        function(element, callback){
            element.addEventListener("click", ()=>{
                this.module.instance.exports[callback]();
            });
        }"#);
    add_click_listener.invoke(&[element.into(), callback.into()]);
}

fn element_set_inner_html(element: &ExternRef, html: &str) {
    let set_inner_html = js!(r#"
        function(element, html){
            element.innerHTML = html;
        }"#);
    set_inner_html.invoke(&[element.into(), html.into()]);
}

fn fetch(url: &str, callback: &str) {
    let fetch = js!(r#"
        function(url, callback){
            fetch(url).then((response)=>{
                return response.text();
            }).then((text)=>{
                const allocationId = this.writeUtf8ToMemory(text);
                this.module.instance.exports[callback](text);
            });
        }"#);
    fetch.invoke(&[url.into(), callback.into()]);
}

#[no_mangle]
pub fn main() {
    let button = query_selector("#fetch_button");
    add_click_listener(&button, "button_clicked");
}

#[no_mangle]
pub fn button_clicked() {
    // get pokemon data
    let url = "https://pokeapi.co/api/v2/pokemon/1/";
    fetch(url, "fetch_callback");
}

#[no_mangle]
pub fn fetch_callback(text_allocation_id: usize) {
    let text = extract_string_from_memory(text_allocation_id);
    let result = query_selector("#data_output");
    element_set_inner_html(&result, &text);
}
```

Notice in the fetch function handling, we have a function specifically for helping put strings inside of WebAssembly `writeUtf8ToMemory`.  This returns back an ID that can be used to rebuild the string on WebAssembly side `extract_string_from_memory`.

# The `web` crate

If you don't feel like recreating the wheel, there's an ongoing collection of commonly used functions accumulationg in `web`.

```rust
use web::*;

#[no_mangle]
fn main() {
    console_log("Hello world!");
    let body = query_selector("body");
    element_add_click_listener(&body, |e| {
        console_log(format!("Clicked at {}, {}", e.offset_x, e.offset_y).as_str());
    });
    element_add_mouse_move_listener(&body, |e| {
        console_log(format!("Mouse moved to {}, {}", e.offset_x, e.offset_y).as_str());
    });
    element_add_mouse_down_listener(&body, |e| {
        console_log(format!("Mouse down at {}, {}", e.offset_x, e.offset_y).as_str());
    });
    element_add_mouse_up_listener(&body, |e| {
        console_log(format!("Mouse up at {}, {}", e.offset_x, e.offset_y).as_str());
    });
    element_add_key_down_listener(&body, |e| {
        console_log(format!("Key down: {}", e.key_code).as_str());
    });
    element_add_key_up_listener(&body, |e| {
        console_log(format!("Key up: {}", e.key_code).as_str());
    });
}

```

Check out the documentation [here](https://docs.rs/web)

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
