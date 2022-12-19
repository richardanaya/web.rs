use js::*;

#[no_mangle]
pub fn main() {
    let fn_log = js!("function(str){
        debugger;
        console.log(str); 
    }");
    let msg = "Hello World!";
    fn_log.invoke(&vec![InvokeParams::String(&msg)]);
}
