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
js = "0"
```
```rust
let fn_log = js::register_function(
    "function(context,strPtr,strLen){
        let str = context.getUtf8FromMemory(strPtr,strLen);
        console.log(str); 
    }",
);

let msg = "Hello World!";

fn_log.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
```

**C/C++:**
```c
 #include "js.h"
 
JSFunction fnLog = js_register_function(
    "function(context,strPtr,strLen){\
        let str = context.getUtf8FromMemory(strPtr,strLen);\
        console.log(str);\
    }",
);

char *msg = "Hello World!";

js_invoke_function_2(fnLog, msg, 11);
```

In your JS function `context` is passed in to handle most issues you'll encounter

* `context.getUtf8FromMemory(start,length)` - Extract utf-8 text from your program's memory.
* `context.writeUtf8ToMemory(start,str)` - Write utf-8 to a memory location you are sure it should go.
* `context.storeObject(object)` - Store an object in your context for later reference, get a handle you can give to WebAssembly.
* `context.getObject(handle)` - Retreive and object from your context with a handle.
* `context.releaseObject(handle)` - Release a stored object so it's memory can be freed.
* `context.module` - Get access to your program so you can call methods on it.
