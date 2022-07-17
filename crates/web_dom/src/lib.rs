#![no_std]
use js::*;
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use web_common::*;

pub fn get_element_by_id(id: &str) -> JSObject {
    js!("function(strPtr,strLen){
        const el = document.getElementById(this.readUtf8FromMemory(strPtr,strLen)); 
        return this.storeObject(el);
    }")
    .invoke_2(id.as_ptr() as u32, id.len() as u32)
    .into()
}

pub fn query_selector(id: &str) -> JSObject {
    js!("function(strPtr,strLen){
        const selector = document.querySelector(this.readUtf8FromMemory(strPtr,strLen)); 
        return this.storeObject(selector);
    }")
    .invoke_2(id.as_ptr() as u32, id.len() as u32)
    .into()
}

pub fn set_style(dom: impl Into<f64>, name: &str, value: &str) {
    js!("function(el,strPtr,strLen,valPtr,valLen){
        el = this.getObject(el);
        const name = this.readUtf8FromMemory(strPtr,strLen);
        const value = this.readUtf8FromMemory(valPtr,valLen);
        el.style[name] = value;
    }")
    .invoke_5(
        dom.into(),
        name.as_ptr() as u32,
        name.len() as u32,
        value.as_ptr() as u32,
        value.len() as u32,
    );
}

pub fn add_event_listener(
    dom: impl Into<f64>,
    event: &str,
    handler: impl FnMut(f64) + Send + 'static,
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
        return this.writeUtf8ToMemory(a);
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    if attr == -1.0 {
        None
    } else {
        let allocation_id = attr as usize;
        let s = extract_string_from_memory(allocation_id);
        clear_allocation(allocation_id);
        Some(s)
    }
}

pub struct KeyEventHandler {
    pub handler: Option<Box<dyn Sync + FnMut(KeyEvent) + 'static + Send>>,
}

impl KeyEventHandler {
    pub fn new(f: impl Sync + FnMut(KeyEvent) + 'static + Send) -> KeyEventHandler {
        KeyEventHandler {
            handler: Some(Box::new(f)),
        }
    }
}

pub struct KeyEvent {
    obj: JSObject,
}

impl KeyEvent {
    pub fn new(o: f64) -> KeyEvent {
        KeyEvent {
            obj: JSObject::from(o),
        }
    }

    pub fn key_code(&self) -> usize {
        let key_code: f64 = get_property(self.obj.handle, "keyCode").unwrap();
        key_code as usize
    }

    pub fn target(&self) -> JSObject {
        get_property(self.obj.handle, "target").unwrap()
    }
}

pub struct InputElement {
    obj: JSObject,
}

impl InputElement {
    pub fn new(o: f64) -> InputElement {
        InputElement {
            obj: JSObject::from(o),
        }
    }

    pub fn from(o: JSObject) -> InputElement {
        InputElement { obj: o }
    }

    pub fn value(&self) -> Option<String> {
        get_property(&self.obj, "value")
    }

    pub fn set_value(&mut self, s: &str) {
        set_property(&self.obj, "value", s)
    }
}

pub struct MouseEventHandler {
    pub handler: Option<Box<dyn Sync + FnMut(MouseEvent) + 'static + Send>>,
}

impl MouseEventHandler {
    pub fn new(f: impl Sync + FnMut(MouseEvent) + 'static + Send) -> MouseEventHandler {
        MouseEventHandler {
            handler: Some(Box::new(f)),
        }
    }
}

pub struct MouseEvent {
    obj: JSObject,
}

impl MouseEvent {
    pub fn new(o: f64) -> MouseEvent {
        MouseEvent {
            obj: JSObject::from(o),
        }
    }
    pub fn target(&self) -> JSObject {
        get_property(self.obj.handle, "target").unwrap()
    }
}

pub struct EventHandler {
    pub handler: Option<Box<dyn Sync + FnMut(Event) + 'static + Send>>,
}

impl EventHandler {
    pub fn new(f: impl Sync + FnMut(Event) + 'static + Send) -> EventHandler {
        EventHandler {
            handler: Some(Box::new(f)),
        }
    }
}

pub struct Event {
    obj: JSObject,
}

impl Event {
    pub fn new(o: f64) -> Event {
        Event {
            obj: JSObject::from(o),
        }
    }
    pub fn target(&self) -> JSObject {
        get_property(self.obj.handle, "target").unwrap()
    }
}
