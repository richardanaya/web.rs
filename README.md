# ramen :ramen:

Load WebAssembly like JavaScript

```html
<html>
    <head>
        <script src="https://cdn.jsdelivr.net/gh/richardanaya/ramen/ramen.js"></script>
        <script type="application/wasm" src="helloworld.wasm"></script>
    </head>
    <body>
        ...
    </body>
</html>
```

Quickly get access to functions for invoking JavaScript

```rust
let fn_log = ramen::register_function(
    "function(context,strPtr,strLen){
        let str = context.getUtf8FromMemory(strPtr,strLen);
        console.log(str); 
    }",
);

let msg = "Hello World!";

fn_log.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
```

```c
JSFunction fnLog = js_register_function(
    "function(context,strPtr,strLen){\
        let str = context.getUtf8FromMemory(strPtr,strLen);\
        console.log(str);\
    }",
);

char *msg = "Hello World!";
js_invoke_function(fnLog, msg, 11);
```

Works with any WebAssembly programming language out of the box:
* Rust
* C
* AssemblyScript

Lot's of helpers!

In your JS function `context` is passed in to handle most chores for binding and give access to your program.

* `getUtf8FromMemory` - extract utf-8 text from your program's memory.
* `storeObject` - store an object in your context for later reference, get a handle you can give to WebASsembly.
* `getObject` - retreive and object from your context with a handle.
* `releaseObject` - release a stored object so it's memory can be freed.

