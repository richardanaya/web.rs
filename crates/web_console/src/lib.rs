#![no_std]
use js::*;

pub fn clear() {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(){
                console.clear(); 
            }",
        )
    })
    .invoke_0();
}

pub fn log(msg: &str) {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(strPtr,strLen){
                console.log(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    })
    .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn warning(msg: &str) {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(strPtr,strLen){
                console.warn(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    })
    .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn error(msg: &str) {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(strPtr,strLen){
                console.error(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    })
    .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn time(msg: &str) {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(strPtr,strLen){
                console.time(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    })
    .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn time_end(msg: &str) {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(strPtr,strLen){
                console.timeEnd(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    })
    .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}
