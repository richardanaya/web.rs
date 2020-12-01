#![no_std]

use js::*;

pub fn console_clear() {
    let func = js!(r###"function(){
                console.clear();
        }"###);
    func.invoke_0();
}

pub fn console_log(msg: &str) {
    let a0 = msg.as_ptr() as u32;
    let a1 = msg.len() as u32;
    let func = js!(r###"function(msgPtr,msgLen){
                console.log(this.readUtf8FromMemory(msgPtr,msgLen));
        }"###);
    func.invoke_2(a0, a1);
}

pub fn console_warning(msg: &str) {
    let a0 = msg.as_ptr() as u32;
    let a1 = msg.len() as u32;
    let func = js!(r###"function(msgPtr,msgLen){
                console.warn(this.readUtf8FromMemory(msgPtr,msgLen));
        }"###);
    func.invoke_2(a0, a1);
}

pub fn console_error(msg: &str) {
    let a0 = msg.as_ptr() as u32;
    let a1 = msg.len() as u32;
    let func = js!(r###"function(msgPtr,msgLen){
                console.error(this.readUtf8FromMemory(msgPtr,msgLen));
        }"###);
    func.invoke_2(a0, a1);
}

pub fn console_time(msg: &str) {
    let a0 = msg.as_ptr() as u32;
    let a1 = msg.len() as u32;
    let func = js!(r###"function(msgPtr,msgLen){
                console.time(this.readUtf8FromMemory(msgPtr,msgLen));
        }"###);
    func.invoke_2(a0, a1);
}

pub fn console_time_end(msg: &str) {
    let a0 = msg.as_ptr() as u32;
    let a1 = msg.len() as u32;
    let func = js!(r###"function(msgPtr,msgLen){
                console.timeEnd(this.readUtf8FromMemory(msgPtr,msgLen));
        }"###);
    func.invoke_2(a0, a1);
}
