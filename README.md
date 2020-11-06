# js-wasm
<a href="https://docs.rs/js"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

This project wants to be a simple, easy to learn, technology-agnostic way to call JavaScript from WebAssembly.

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
        <script src="https://cdn.jsdelivr.net/gh/richardanaya/js-wasm/js-wasm.js"></script>
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
        console.log(thi.getCStringFromMemory(cstrPtr));\
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
