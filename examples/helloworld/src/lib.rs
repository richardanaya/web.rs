use js::*;

#[no_mangle]
pub fn main() {
    js!("function(str){
        console.log(str)
    }")
    .invoke(&["Hello, World!".into()]);
}
