#![no_std]
use js::*;
extern crate alloc;

pub trait GetProperty {
    fn get_property(el: impl Into<f64>, name: &str) -> Option<Self>
    where
        Self: Sized;
}

impl GetProperty for f64 {
    fn get_property(el: impl Into<f64>, name: &str) -> Option<Self> {
        Some(
            js!("function(el,strPtr,strLen){
            el = this.getObject(el);
            return el[this.readUtf8FromMemory(strPtr,strLen)];
        }")
            .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32),
        )
    }
}

impl GetProperty for alloc::string::String {
    fn get_property(el: impl Into<f64>, name: &str) -> Option<Self> {
        let attr = js!(r#"function(o,strPtr,strLen){
                o = this.getObject(o);
                const a = o[this.readUtf8FromMemory(strPtr,strLen)];
                if(a === null){
                    return -1;
                } 
                return this.writeCStringToMemory(a);
            }"#)
        .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
        if attr == -1.0 {
            return None;
        } else {
            Some(cstr_to_string(attr as i32))
        }
    }
}

pub fn get_property<T>(el: impl Into<f64>, id: &str) -> Option<T>
where
    T: GetProperty,
{
    T::get_property(el, id)
}

pub trait SetProperty {
    fn set_property(el: impl Into<f64>, id: &str, s: Self);
}

impl SetProperty for f64 {
    fn set_property(el: impl Into<f64>, id: &str, v: Self) {
        js!("function(el,strPtr,strLen,value){
            el = this.getObject(el);
            return el[this.readUtf8FromMemory(strPtr,strLen)] = value;
        }")
        .invoke_4(el.into(), id.as_ptr() as u32, id.len() as u32, v);
    }
}

impl SetProperty for &str {
    fn set_property(el: impl Into<f64>, name: &str, txt: Self) {
        js!(r#"function(o,strPtr,strLen,valPtr,valLen){
            o = this.getObject(o);
            o[this.readUtf8FromMemory(strPtr,strLen)] = this.readUtf8FromMemory(valPtr,valLen);
        }"#)
        .invoke_5(
            el.into(),
            name.as_ptr() as u32,
            name.len() as u32,
            txt.as_ptr() as u32,
            txt.len() as u32,
        );
    }
}

pub fn set_property<T>(el: impl Into<f64>, id: &str, v: T)
where
    T: SetProperty,
{
    T::set_property(el, id, v)
}

pub fn get_object(handle: impl Into<f64>) -> JSObject {
    let r = js!("function(o){
        return this.storeObject(this.getObject(o).target);
    }")
    .invoke_1(handle.into());
    JSObject::from(r)
}
