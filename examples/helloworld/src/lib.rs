use js::*;

#[no_mangle]
pub fn main() {
    js!("function(){
        debugger;
        // iterate through arguments
        for (var i = 0; i < arguments.length; i++) {
            console.log(arguments[i]);
        }
    }")
    .invoke(&vec![
        InvokeParam::Undefined,
        InvokeParam::Null,
        InvokeParam::Bool(true),
        InvokeParam::Bool(false),
        InvokeParam::Float64(123.0),
        InvokeParam::BigInt(53533),
        InvokeParam::String("hello world"),
        InvokeParam::Float32Array(&[1.0, 2.0, 3.0, 4.0, 5.0]),
        InvokeParam::Float64Array(&[10.0, 9.0, 8.0, 7.0, 6.0]),
    ]);
}
