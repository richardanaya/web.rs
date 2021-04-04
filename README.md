# js-wasm
<a href="https://docs.rs/js"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

*JavaScript and WebAssembly should be a joy to use together.*

This project aims to provide a simple, easy to learn, technology-agnostic way bridge the two worlds.

<p align="center">
  <img height="300" src="images/undraw_good_team_m7uu.png">
</p>

Use any WebAssembly programming language out of the box:
* [Rust](https://docs.rs/js/latest/js/)
* [C/C++](headers/js-wasm.h)
* [AssemblyScript](https://github.com/richardanaya/js-wasm/blob/master/assemblyscript/js-wasm.ts)
* [Your own programming language](https://github.com/richardanaya/wasm-script)

See a [demo](https://richardanaya.github.io/js-wasm/examples/snake/index.html) of it working!

# How It Works?

Load WebAssembly like JavaScript. 

```html
<html>
    <head>
        <meta charset="utf-8">
        <script src="https://unpkg.com/js-wasm/js-wasm.js"></script>
        <script type="application/wasm" src="helloworld.wasm"></script>
    </head>
    <body>
        ...
    </body>
</html>
```

Create JavaScript functions and invoke them

## Rust
```toml
[dependencies]
js = "0.2"
```
```rust
let fn_log = js!(
    "function(strPtr,strLen){
        console.log(this.readUtf8FromMemory(strPtr,strLen)); 
    }");

let msg = "Hello World!";

fn_log.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
```
## AssemblyScript:
```typescript
import {js_register_function,js_invoke_function} from "./js-wasm"

let console_log_fn:f64 = 0;
export function console_log(msg: string) : void {
  if(console_log_fn === 0){
    const code = "function(msgPtr,msgLen){ console.log(this.readUtf16FromMemory(msgPtr,msgLen)); }";
    console_log_fn = <f64>js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
  }
  js_invoke_2(<f64>console_log_fn,<f64>changetype<usize>(msg),<f64>msg.length*2);
}

export function main():void {
  console_log("Hello World!");
}
```

## C/C++
```c
#include "js-wasm.h"
 
char *code = "function(context,cstrPtr){\
        console.log(this.getCStringFromMemory(cstrPtr));\
    }";
JSFunction fnLog = js_register_function(code,strlen(code));

js_invoke_function_2(fnLog, "Hello World!");
```

In your JS function context `this` contains several functions handle most issues you'll encounter

| Name          | Description   |
| ------------- | ------------- |
| `readUtf8FromMemory(start,length)` | Extract utf-8 text from your program's memory. |
| `writeUtf8ToMemory(str)` | Write utf-8 to a memory location you are sure it should go. |
| `readCStringFromMemory(start,length)` | Extract C string text from your program's memory. |
| `writeCStringToMemory(str)` | Write C string to a memory location you are sure it should go. |
| `readUtf16FromMemory(start,length)` | Extract utf-16 text from your program's memory. |
| `readUint8ArrayFromMemory(start)` | Read a list of uint8 from a pointer to a location of a number of elements, followed by a pointer to bytes in memory. |
| `storeObject(object)` | Store an object in your context for later reference, get a handle you can give to WebAssembly. |
| `getObject(handle)` | Retreive and object from your context with a handle. |
| `releaseObject(handle)` | Release a stored object so it's memory can be freed. |
| `module` | Get access to your program so you can call methods on it. |

<p align="center">
  <img height="300" src="images/undraw_design_team_af2y.png">
</p>

# Who is making this?

My name is [RICHΛRD ΛNΛYΛ](https://github.com/richardanaya), i'm a technologist from Portland, OR. I love the potential WebAssembly has for bringing powerful experiences to the world! 

<p align="center">
  <img height="300" src="images/undraw_elements_cipa.png">
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
