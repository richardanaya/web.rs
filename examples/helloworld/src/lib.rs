#[no_mangle]
pub fn main() {
    let fn_log = js::register_function(
        "function(strPtr,strLen){
            console.log(this.getUtf8FromMemory(strPtr,strLen)); 
        }",
    );
    let msg = "Hello World!";
    fn_log.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}
