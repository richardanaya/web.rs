use js::*;

#[no_mangle]
pub fn main() {
    js!("function(str,n){
        console.log(str,n); 
    }")
    .invoke(&vec!["Hello World!".into(), 1.into()]);
}
