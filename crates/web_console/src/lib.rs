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
    
    pub fn warning(msg: &str){
        let a0 = msg.as_ptr() as u32;
        let a1 = msg.len() as u32;
        let func = js!(r###"function(msgPtr,msgLen){
                console.warn(this.readUtf8FromMemory(msgPtr,msgLen));
            }"###);
        func.invoke_2(a0, a1);
    }
    
    pub fn error(msg: &str){
        let a0 = msg.as_ptr() as u32;
        let a1 = msg.len() as u32;
        let func = js!(r###"function(msgPtr,msgLen){
                console.error(this.readUtf8FromMemory(msgPtr,msgLen));
            }"###);
        func.invoke_2(a0, a1);
    }
    
    pub fn time(msg: &str){
        let a0 = msg.as_ptr() as u32;
        let a1 = msg.len() as u32;
        let func = js!(r###"function(msgPtr,msgLen){
                console.time(this.readUtf8FromMemory(msgPtr,msgLen));
            }"###);
        func.invoke_2(a0, a1);
    }
    
    pub fn time_end(msg: &str){
        let a0 = msg.as_ptr() as u32;
        let a1 = msg.len() as u32;
        let func = js!(r###"function(msgPtr,msgLen){
                console.timeEnd(this.readUtf8FromMemory(msgPtr,msgLen));
            }"###);
        func.invoke_2(a0, a1);
    }
}
