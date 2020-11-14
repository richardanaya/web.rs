#![no_std]
use js::*;

pub fn random() -> f64 {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(){
                    return Math.random();
                }",
        )
    };};
    FN.invoke_0()
}
