#[no_mangle]
pub fn main() {
    let fn_log = ramen::register_function(
        "function(context,strPtr,strLen){
            let str = context.getUtf8FromMemory(strPtr,strLen);
            console.log(str); 
        }",
    );
    let msg = "Hello World!";
    fn_log.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
}
