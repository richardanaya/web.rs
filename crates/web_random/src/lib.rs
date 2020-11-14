#![no_std]
use js::*;

pub fn random() -> f64 {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(){
                    return Math.random();
                }",
        )
    })
    .invoke_0()
}
