# Electron + WebAssembly = :heart:

<p align="center">
  <img height="300" src="../images/undraw_web_developer_p3e5.png">
</p>


Creating desktop apps with WebAssembly is as simple as making website! `electron` is a technology that packages chrome into an desktop app-like minimalistic web view experience. It gives the user access to the local systems resources via a JavaScript api. Since `js-wasm` is a technology agnostic library for executing javascript from WebAssembly, we can easily bind to the parts of the `electron` API we need.

# Getting Started

This directory contains an example to get started using quickly.

```
git clone git@github.com:richardanaya/js-wasm.git
cd js-wasm/electron
make run
```

Let's take a look at the various files:

| File | Description |
|------|-------------|
| `index.js` | Electron wants a javascript file to tell it where to start, this is just basic setup you can tweak like starting width/height and does your app have a menubar. |
| `index.html` | This is the index file  that gets loaded by `index.js` to be the first thing you see. Consider this just like any old web page. |

All our `index.html` does is quickly tell WebAssembly to start like any old web application

```html
<html>
    <head>
        <title>My App!</title>
        <link rel="shortcut icon" href="#" />
        <script src="js-wasm.js"></script>
        <script type="application/wasm" src="app.wasm"></script>
    </head>
    <body>
        <canvas id="screen" width=500 height=500></canvas>
    </body>
</html>
```

## Getting what you need

Now you can create JavaScript binding functions to invoke using [`js-wasm`](https://github.com/richardanaya/js-wasm/) as normal.

```rust
pub fn read_file(msg: &str) {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(pathPtr,pathLen){
                const fs = require('fs');
                const data String(fs.readFileSync(this.readUtf8FromMemory(pathPtr,pathLen)));
                return this.writeCStringToMemory(data);
            }",
        )
    };};
    FN.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

#[no_mangle]
pub fn main() {
    let fileContent = read_file("foo.txt");
    ...
}
```

