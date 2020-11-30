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
Bindings to web console
----
- name: console
  functions:
    - name: clear
    - name: log
      parameters:
        - name: msg
          parameter_type: string
    - name: warn
      friendly_name: warning
      parameters:
        - name: msg
          parameter_type: string
    - name: error
      parameters:
        - name: msg
          parameter_type: string
    - name: time
      parameters:
        - name: msg
          parameter_type: string
    - name: timeEnd
      parameters:
        - name: msg
          parameter_type: string

```

And turn them into code.

# Rust

```
js-bindgen --lang rust console.yaml
```

```rust
#![no_std]

pub mod console {
    use js::*;
    
    pub fn clear(){
        let func = js!(r###"function(){
                console.clear();
            }"###);
        func.invoke_0();
    }
    
    pub fn log(msg: &str){
        let a0 = msg.as_ptr() as u32;
        let a1 = msg.len() as u32;
        let func = js!(r###"function(msgPtr,msgLen){
                console.log(this.readUtf8FromMemory(msgPtr,msgLen));
            }"###);
        func.invoke_2(a0, a1);
    }

...
```

# C

```
js-bindgen --lang c console.yaml
```

```C
#include "js-wasm.h"

void console_clear(){
    static int fn;
    char *fn_code = "function(){ console.clear(); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_0(fn);
}

void console_log(char * msg){
    static int fn;
    unsigned int a0 = (unsigned int)msg;
    unsigned int a1 = js_strlen(msg);
    char *fn_code = "function(msgPtr,msgLen){ console.log(this.readUtf8FromMemory(msgPtr,msgLen)); }";
    if(fn == 0){
        fn = js_register_function(fn_code,js_strlen(fn_code));
    }
    js_invoke_function_2(fn, a0, a1);
}

...
```

# Custom Code

Sometimes you may want to create a binding to code that doesn't exist and still have the power to generate libraries for many targets

```yaml
- namespace: unicorn
  functions:
    makeUnicorns:
      code: |
        function() {
          console.log("🦄🦄🦄")
        }
```
