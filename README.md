<p align="center">
<img src="https://user-images.githubusercontent.com/294042/208995865-88502572-76f7-4ce7-8157-9bca9f1c9444.png"/>
</p>


<a href="https://docs.rs/web"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

*Make writing web applications using Rust WebAssembly easy*

I wanted a library that someone could learn in an afternoon how to use and start making interactive browser experiences with.  This project doesn't support every browser function under the sun.  Though you can easily add your own using the runtime [Javascript invoking mechanism](https://github.com/richardanaya/web.rs/tree/master/crates/js) used by this library.  Feel free to submit a PR for more functionality.

* async & coroutine support
* element operations
* mouse, keyboard, and change event listeners
* canvas2d
* localstorage
* fetch & XMLHttpRequest
* style & classes
* history & location info
* WebGPU
* other utilities

Check out the documentation [here](https://docs.rs/web/latest/web/#functions)

```terminal
cargo add web
```

# Hello World

Let's just look at a basic example of how to put things in the console:

```rust
use web::*;

fn main() {
    console_log("Hello, world!");
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
Remember to configure your library `Cargo.toml` for WebAssembly

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

# Something more advanced?

Let's look at our [snake example](https://github.com/richardanaya/web.rs/tree/master/examples/web_snake) and some of it's key feature usages:

<img width="521" alt="Screen Shot 2022-12-21 at 12 35 48 PM" src="https://user-images.githubusercontent.com/294042/208998255-3b21cd21-e96e-4671-94e1-0ef1f52b59fa.png">

Play demo [here](https://wasm.js.org/examples/web_snake/)

## canvas

This example uses canvas

```rust
//get an element and get the 2D context for canvas
let screen = query_selector("#screen");
let width: f64 = get_property_f64(&screen, "width");
let height: f64 = get_property_f64(&screen, "height");
let ctx = CanvasContext::from_element(&screen);

...

//clear screen
self.ctx.clear_rect(
    0.0,
    0.0,
    self.canvas_width as f64,
    self.canvas_height as f64,
);

// iterate through all the cells of the screen and draw a rectangle
for (_id, (pos, color)) in &mut self.world.query::<(&Position, &Color)>() {
    self.ctx.set_fill_style(&color.0);
    self.ctx.fill_rect(
        (pos.0 * (self.canvas_width / MAP_WIDTH)) as f64,
        (pos.1 * (self.canvas_height / MAP_HEIGHT)) as f64,
        (self.canvas_width / MAP_WIDTH) as f64,
        (self.canvas_height / MAP_HEIGHT) as f64,
    );
}
```


## request animation frame

Let's see how to run the game loop

```rust
fn game_loop() {
    // run game loop assuming 15 ms has passed
    match Game::instance().run(15.0) {
        Err(e) => console_error(&e.to_string()),
        _ => (),
    };
    // request next animation frame
    request_animation_frame(game_loop);
}

... 

// start the loop
request_animation_frame(game_loop);
```

## events

```
let body = query_selector("body");
element_add_key_down_listener(&body, |e| {
    Game::instance().key_down(e.key_code as u32);
});
```

# Async & Coroutines

This library has support for async and spawning coroutines. Consider this program that starts a looping console log and also draws random squares on a screen.

```rust
use web::*;

// easily make your first function async
#[web::main]
async fn main() {
    let canvas = query_selector("#canvas");
    let ctx = CanvasContext::from_element(&canvas);

    // we can spawn concurrent operations
    coroutine(async {
        loop {
            console_log("⏰ tik");
            // hand async set_timeout
            sleep(1000).await;
            console_log("⏰ tok");
            sleep(1000).await;
        }
    });

    loop {
        // draw a random color rect
        ctx.set_fill_style(&format!(
            "rgb({}, {}, {})",
            random() * 255.0,
            random() * 255.0,
            random() * 255.0
        ));
        ctx.fill_rect(
            random() * 500.0,
            random() * 500.0,
            random() * 500.0,
            random() * 500.0,
        );
        // a more async way to write graphics code
        wait_til_animation_frame().await;
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
for inclusion in `web` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
