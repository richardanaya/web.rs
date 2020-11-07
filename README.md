# js-wasm
<a href="https://docs.rs/js"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

JavaScript from WebAssembly should be a joy to use together. This project aims to provide a simple, easy to learn, technology-agnostic way bridge the two worlds.

<p align="center">
  <img width="460" height="300" src="images/undraw_good_team_m7uu.png">
</p>

Use any WebAssembly programming language out of the box:
* [Rust](https://docs.rs/js/latest/js/)
* [C/C++](js-wasm.h)
* AssemblyScript

See a [demo](https://richardanaya.github.io/js-wasm/examples/canvas/index.html) of it working!

# How It Works

Load WebAssembly like JavaScript. 

```html
<html>
    <head>
        <script src="unpkg.com/js-wasm/js-wasm.js"></script>
        <script type="application/wasm" src="helloworld.wasm"></script>
    </head>
    <body>
        ...
    </body>
</html>
```

Create JavaScript functions and invoke them

**Rust:**
```toml
[dependencies]
js = "0.2"
```
```rust
let fn_log = js::register_function(
    "function(strPtr,strLen){
        console.log(this.readUtf8FromMemory(strPtr,strLen)); 
    }");

let msg = "Hello World!";

fn_log.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
```

**C/C++:**
```c
#include "js-wasm.h"
 
JSFunction fnLog = js_register_function(
    "function(context,cstrPtr){\
        console.log(this.getCStringFromMemory(cstrPtr));\
    }");

js_invoke_function_2(fnLog, "Hello World!");
```

In your JS function context `this` contains several functions handle most issues you'll encounter

* `readUtf8FromMemory(start,length)` - Extract utf-8 text from your program's memory.
* `writeUtf8ToMemory(start,str)` - Write utf-8 to a memory location you are sure it should go.
* `readCStringFromMemory(start,length)` - Extract C string text from your program's memory.
* `writeCStringToMemory(start,str)` - Write C string to a memory location you are sure it should go.
* `readUint8ArrayFromMemory(start)` - Read a list of uint8 from a pointer to a location of a number of elements, followed by a pointer to bytes in memory.
* `storeObject(object)` - Store an object in your context for later reference, get a handle you can give to WebAssembly.
* `getObject(handle)` - Retreive and object from your context with a handle.
* `releaseObject(handle)` - Release a stored object so it's memory can be freed.
* `module` - Get access to your program so you can call methods on it.

```html
<html>
    <head>
        <script src="unpkg.com/js-wasm/js-wasm.js"></script>
        <script type="application/wasm" src="helloworld.wasm"></script>
    </head>
    <body>
        ...
    </body>
</html>
```

<p align="center">
  <img width="460" height="300" src="images/undraw_elements_cipa.png">
</p>

# Don't Recreate the Wheel

A number of libraries exist to help save you time from writing your own bindings. 


**Browser Functionality Libraries:**

| Name          | Documentation | Description   |
| ------------- | ------------- | ------------- |
| [`web_console`](https://github.com/richardanaya/js-wasm/tree/master/rust/web_console) | <a href="https://docs.rs/web_console"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>  |  simple logging  |
| [`web_timer`](https://github.com/richardanaya/js-wasm/tree/master/rust/web_timer) | <a href="https://docs.rs/web_console"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>  |  timers, animation loops |
| [`web_random`](https://github.com/richardanaya/js-wasm/tree/master/rust/web_console) | <a href="https://docs.rs/web_console"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>  |  random numbers  |

**UI:** 

| Name          | Documentation | Description   |
| ------------- | ------------- | ------------- |
| [`web_console`](https://github.com/richardanaya/js-wasm/tree/master/rust/lit-html) | <a href="https://docs.rs/lit-html"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>  |  a prototype of using [lit-html](https://lit-html.polymer-project.org/) HTML rendering  |


<p align="center">
  <img width="460" height="300" src="images/undraw_true_love_cy8x.png">
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
