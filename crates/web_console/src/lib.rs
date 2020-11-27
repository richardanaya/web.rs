#![no_std]
use js::*;

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

pub fn error(msg: &str) {
    js!("function(strPtr,strLen){
        console.error(this.readUtf8FromMemory(strPtr,strLen)); 
    }")
    .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn time(msg: &str) {
    js!("function(strPtr,strLen){
        console.time(this.readUtf8FromMemory(strPtr,strLen)); 
    }")
    .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}

pub fn time_end(msg: &str) {
    js!("function(strPtr,strLen){
        console.timeEnd(this.readUtf8FromMemory(strPtr,strLen)); 
    }")
    .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}
