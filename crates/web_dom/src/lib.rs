#![no_std]
use callback::*;
use js::*;
extern crate alloc;

pub fn get_element_by_id(id: &str) -> JSObject {
    js!("function(strPtr,strLen){
        let el = document.getElementById(this.readUtf8FromMemory(strPtr,strLen)); 
        return this.storeObject(el);
    }")
    .invoke_2(id.as_ptr() as u32, id.len() as u32)
    .into()
}

pub fn add_event_listener(
    dom: impl Into<f64>,
    event: &str,
    handler: impl FnMut(f64) -> () + Send + 'static,
) {
    let cb = create_callback_1(handler);
    js!("function(el,strPtr,strLen,callback){
        el = this.getObject(el);
        const event = this.readUtf8FromMemory(strPtr,strLen);
        el.addEventListener(event,this.createCallback(callback));
    }")
    .invoke_4(dom.into(), event.as_ptr() as u32, event.len() as u32, cb);
}

pub struct KeyDownEvent {
    pub handle: JSObject,
}

impl KeyDownEvent {
    pub fn from_event(ev: impl Into<JSObject>) -> KeyDownEvent {
        KeyDownEvent { handle: ev.into() }
    }

    pub fn key_code(&self) -> u32 {
        js!("function(ev){
            ev = this.getObject(ev);
            return ev.keyCode;
        }")
        .invoke_1(&self.handle) as u32
    }
}

pub fn attach_shadow(el: impl Into<f64>, open: bool) -> JSObject {
    js!(r#"function(el,open){
        el = this.getObject(el);
        el.attachShadow({mode:open?"open":"closed"});
        return this.storeObject(el.shadowRoot);
    }"#)
    .invoke_2(el.into(), if open { 1.0 } else { 0.0 })
    .into()
}

pub fn set_inner_html(el: impl Into<f64>, html: &str) {
    js!(r#"function(el,strPtr,strLen){
        el = this.getObject(el);
        el.innerHTML = this.readUtf8FromMemory(strPtr,strLen);
    }"#)
    .invoke_3(el.into(), html.as_ptr() as u32, html.len() as u32);
}

pub fn get_attribute(el: impl Into<f64>, name: &str) -> Option<alloc::string::String> {
    let attr = js!(r#"function(el,strPtr,strLen){
        el = this.getObject(el);
        const a = el.getAttribute(this.readUtf8FromMemory(strPtr,strLen));
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
