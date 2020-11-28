# js-bindgen

<p align="center">
  <img height="300" src="../../images/undraw_convert_2gjv.png">
</p>

Generate WebAssembly bindings to JavaSCript via [`js-wasm`](https://wasm.js.org) for various languages:

* Rust
* C

```
cargo install js-bindgen
```

# Getting Started

This project is able to take JavaScript API descriptions in yaml like the one below:

```yaml
- namespace: console
  functions:
    clear:
    log:
      parameters:
        - name: msg
          type: string
    warn:
      name: warning
      parameters:
        - name: msg
          type: string
    error:
      parameters:
        - name: msg
          type: string
    time:
      parameters:
        - name: msg
          type: string
    timeEnd:
      parameters:
        - name: msg
          type: string
```

And turn them into code.

# Rust

```
js-bindgen --lang rust console.yaml
```

```rust
#![no_std]
use js::*;

mod console {
    pub fn clear() {
        js!("function(){
            console.clear(); 
        }")
        .invoke_0();
    }

    pub fn log(msg: &str) {
        js!("function(strPtr,strLen){
            console.log(this.readUtf8FromMemory(strPtr,strLen)); 
        }")
        .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
    }

    pub fn warning(msg: &str) {
        js!("function(strPtr,strLen){
            console.warn(this.readUtf8FromMemory(strPtr,strLen)); 
        }")
        .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
    }
}

...
```

# C

```
js-bindgen --lang c console.yaml
```

```C
#include "js-wasm.h"

void console_clear() {
    static int fn;
    if(fn == 0){
        fn = js_register("function(){\
            console.clear();\
        }");
    }
    js_invoke0(fn);
}

void console_log(char *msg) { 
    static int fn;
    if(fn == 0){
        fn = js_register("function(strPtr,strLen){\
            console.log(this.readUtf8FromMemory(strPtr,strLen));\
        }");
    }
    js_invoke2(fn, msg,strlen(msg));
}

void console_warning(char *msg) { 
    static int fn;
    if(fn == 0){
        fn = js_register("function(strPtr,strLen){\
            console.warn(this.readUtf8FromMemory(strPtr,strLen));\
        }");
    }
    js_invoke2(fn, msg, strlen(msg));
}

...
```

# Custom Code

Sometimes you may want to create a binding to code that doesn't exist and still have the power to generate libraries for many targets

```
- namespace: unicorn
  functions:
    makeUnicorns:
      code: |
        function() {
          console.log("🦄🦄🦄")
        }
```
