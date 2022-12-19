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
        let v = el.into();
        if !is_property_number(v, name) {
            return None;
        }
        Some(
            js!("function(el,strPtr,strLen){
            el = this.getObject(el);
            return el[this.readUtf8FromMemory(strPtr,strLen)];
        }")
            .invoke_3(v, name.as_ptr() as u32, name.len() as u32),
        )
    }
}

impl GetProperty for bool {
    fn get_property(el: impl Into<f64>, name: &str) -> Option<Self> {
        let v = js!(r#"function(el,strPtr,strLen){
            el = this.getObject(el);
            let i = el[this.readUtf8FromMemory(strPtr,strLen)];
            if(typeof i !== "boolean"){
                return -1;
            }
            return el[this.readUtf8FromMemory(strPtr,strLen)] ? 1 : 0;
        }"#)
        .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
        if v == -1.0 {
            return None;
        } else {
            Some(v == 1.0)
        }
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
                return this.writeUtf8ToMemory(a);
            }"#)
        .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
        if attr == -1.0 {
            return None;
        } else {
            let allocation_id = attr as usize;
            let s = extract_string_from_memory(allocation_id);
            clear_allocation(allocation_id);
            Some(s)
        }
    }
}

impl GetProperty for JSObject {
    fn get_property(el: impl Into<f64>, name: &str) -> Option<Self> {
        let o = js!(r#"function(o,strPtr,strLen){
                o = this.getObject(o);
                const a = o[this.readUtf8FromMemory(strPtr,strLen)];
                if(a === null){
                    return -1;
                } 
                return this.storeObject(a);
            }"#)
        .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
        if o == -1.0 {
            return None;
        } else {
            Some(JSObject::from(o))
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

impl SetProperty for bool {
    fn set_property(el: impl Into<f64>, name: &str, value: Self) {
        js!(r#"function(o,strPtr,strLen,val){
            o = this.getObject(o);
            o[this.readUtf8FromMemory(strPtr,strLen)] = val > 0;
        }"#)
        .invoke_4(
            el.into(),
            name.as_ptr() as u32,
            name.len() as u32,
            if value { 1.0 } else { 0.0 },
        );
    }
}

impl SetProperty for JSObject {
    fn set_property(el: impl Into<f64>, name: &str, value: Self) {
        js!(r#"function(o,strPtr,strLen,val){
            o = this.getObject(o);
            o[this.readUtf8FromMemory(strPtr,strLen)] = this.getObject(val);
        }"#)
        .invoke_4(
            el.into(),
            name.as_ptr() as u32,
            name.len() as u32,
            value.handle,
        );
    }
}

pub fn set_property<T>(el: impl Into<f64>, id: &str, v: T)
where
    T: SetProperty,
{
    T::set_property(el, id, v)
}

pub fn is_property_null(el: impl Into<f64>, name: &str) -> bool {
    let v = js!(r#"function(o,strPtr,strLen,val){
        o = this.getObject(o);
        return o[this.readUtf8FromMemory(strPtr,strLen)] === null ? 1.0 : 0.0;
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    v == 1.0
}

pub fn is_property_undefined(el: impl Into<f64>, name: &str) -> bool {
    let v = js!(r#"function(o,strPtr,strLen,val){
        o = this.getObject(o);
        return o[this.readUtf8FromMemory(strPtr,strLen)] === undefined ? 1.0 : 0.0;
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    v == 1.0
}

pub fn is_property_number(el: impl Into<f64>, name: &str) -> bool {
    let v = js!(r#"function(o,strPtr,strLen,val){
        o = this.getObject(o);
        return typeof o[this.readUtf8FromMemory(strPtr,strLen)] === "number" ? 1.0 : 0.0;
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    v == 1.0
}

pub fn is_property_bool(el: impl Into<f64>, name: &str) -> bool {
    let v = js!(r#"function(o,strPtr,strLen,val){
        o = this.getObject(o);
        return typeof o[this.readUtf8FromMemory(strPtr,strLen)] === "boolean" ? 1.0 : 0.0;
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    v == 1.0
}

pub fn is_property_string(el: impl Into<f64>, name: &str) -> bool {
    let v = js!(r#"function(o,strPtr,strLen,val){
        o = this.getObject(o);
        return typeof o[this.readUtf8FromMemory(strPtr,strLen)] === "string" ? 1.0 : 0.0;
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    v == 1.0
}

pub fn is_property_object(el: impl Into<f64>, name: &str) -> bool {
    let v = js!(r#"function(o,strPtr,strLen,val){
        o = this.getObject(o);
        return typeof o[this.readUtf8FromMemory(strPtr,strLen)] === "object" ? 1.0 : 0.0;
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    v == 1.0
}

pub fn is_property_array(el: impl Into<f64>, name: &str) -> bool {
    let v = js!(r#"function(o,strPtr,strLen,val){
        o = this.getObject(o);
        return Array.isArray(o[this.readUtf8FromMemory(strPtr,strLen)]) ? 1.0 : 0.0;
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    v == 1.0
}
