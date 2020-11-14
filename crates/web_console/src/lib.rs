#![no_std]
use js::*;

pub fn clear() {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(){
                console.clear(); 
            }",
        )
    };};
    FN.invoke_0();
}

pub fn log(msg: &str) {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(strPtr,strLen){
                console.log(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    };};
    FN.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn warning(msg: &str) {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(strPtr,strLen){
                console.warn(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    };};
    FN.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn error(msg: &str) {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(strPtr,strLen){
                console.error(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    };};
    FN.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn time(msg: &str) {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(strPtr,strLen){
                console.time(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    };};
    FN.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn time_end(msg: &str) {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(strPtr,strLen){
                console.timeEnd(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
        )
    };};
    FN.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}
