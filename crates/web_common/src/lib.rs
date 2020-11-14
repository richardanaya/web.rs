#![no_std]
use js::*;

pub trait GetProperty {
    fn get_property(el: impl Into<f64>, id: &str) -> Self;
}

impl GetProperty for f64 {
    fn get_property(el: impl Into<f64>, id: &str) -> Self {
        lazy_static::lazy_static! {
            static ref FN: JSFunction= {
            register_function(
                "function(el,strPtr,strLen){
                        el = this.getObject(el);
                        return el[this.readUtf8FromMemory(strPtr,strLen)];
                }",
            )
        };};
        FN.invoke_3(el.into(), id.as_ptr() as u32, id.len() as u32)
    }
}

pub fn get_property<T>(el: impl Into<f64>, id: &str) -> T
where
    T: GetProperty,
{
    T::get_property(el, id)
}
