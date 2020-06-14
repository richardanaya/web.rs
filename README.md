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

Instantly get access to functions for invoking JavaScript

```rust
let fn_log = register_function("
function(context,strPtr,strLen){
    let str = context.getUtf8FromMemory(strPtr,strLen);
    console.log(str); 
}");
let msg = "Hello World";
fn_log.invoke_2(msg.as_ptr(),msg.len());
```

Works with any WebAssembly programming language out of the box:
* Rust
* C
* AssemblyScript
