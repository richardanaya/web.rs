#![no_std]
use js::*;

pub fn random() -> f64 {
    js!("function(){
        return Math.random();
    }")
    .invoke_0()
}
