use core::hash::{Hash, Hasher};
pub use js::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub fn random() -> f64 {
    let random = js!(r#"
        function(){
            return Math.random();
        }"#);
    random.invoke(&[])
}

pub fn console_log(message: &str) {
    let console_log = js!(r#"
        function(message){
            console.log(message);
        }"#);
    console_log.invoke(&[message.into()]);
}

pub fn console_error(message: &str) {
    let console_error = js!(r#"
        function(message){
            console.error(message);
        }"#);
    console_error.invoke(&[message.into()]);
}

pub fn console_warn(message: &str) {
    let console_warn = js!(r#"
        function(message){
            console.warn(message);
        }"#);
    console_warn.invoke(&[message.into()]);
}

pub fn console_time(label: &str) {
    let console_time = js!(r#"
        function(label){
            console.time(label);
        }"#);
    console_time.invoke(&[label.into()]);
}

pub fn console_time_end(label: &str) {
    let console_time_end = js!(r#"
        function(label){
            console.timeEnd(label);
        }"#);
    console_time_end.invoke(&[label.into()]);
}

pub fn query_selector(selector: &str) -> ExternRef {
    let query_selector = js!(r#"
        function(selector){
            return document.querySelector(selector);
        }"#);
    query_selector.invoke_and_return_object(&[selector.into()])
}

pub fn element_set_inner_html(element: &ExternRef, html: &str) {
    let set_inner_html = js!(r#"
        function(element, html){
            element.innerHTML = html;
        }"#);
    set_inner_html.invoke(&[element.into(), html.into()]);
}

pub fn element_add_class(element: &ExternRef, class: &str) {
    let add_class = js!(r#"
        function(element, class){
            element.classList.add(class);
        }"#);
    add_class.invoke(&[element.into(), class.into()]);
}

pub fn element_remove_class(element: &ExternRef, class: &str) {
    let remove_class = js!(r#"
        function(element, class){
            element.classList.remove(class);
        }"#);
    remove_class.invoke(&[element.into(), class.into()]);
}

pub fn element_set_attribute(element: &ExternRef, attribute: &str, value: &str) {
    let set_attribute = js!(r#"
        function(element, attribute, value){
            element.setAttribute(attribute, value);
        }"#);
    set_attribute.invoke(&[element.into(), attribute.into(), value.into()]);
}

pub fn element_remove(element: &ExternRef) {
    let remove = js!(r#"
        function(element){
            element.remove();
        }"#);
    remove.invoke(&[element.into()]);
}

pub struct FunctionHandle(ExternRef);

impl PartialEq for FunctionHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0.value == other.0.value
    }
}

impl Eq for FunctionHandle {}

impl Hash for FunctionHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.value.hash(state);
    }
}

// Mouse Events
pub struct MouseEvent {
    pub offset_x: f64,
    pub offset_y: f64,
}

static MOUSE_EVENT_HANDLERS: Mutex<
    Option<HashMap<Arc<FunctionHandle>, Box<dyn FnMut(MouseEvent) + Send + 'static>>>,
> = Mutex::new(None);

fn add_mouse_event_handler(
    id: Arc<FunctionHandle>,
    handler: Box<dyn FnMut(MouseEvent) + Send + 'static>,
) {
    let mut handlers = MOUSE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        h.insert(id, handler);
    } else {
        let mut h = HashMap::new();
        h.insert(id, handler);
        *handlers = Some(h);
    }
}

fn remove_mouse_event_handler(id: &Arc<FunctionHandle>) {
    let mut handlers = MOUSE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        h.remove(id);
    }
}

#[no_mangle]
pub extern "C" fn web_handle_mouse_event_handler(id: i64, x: f64, y: f64) {
    let mut handlers = MOUSE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        for (key, handler) in h.iter_mut() {
            if key.0.value == id {
                handler(MouseEvent {
                    offset_x: x,
                    offset_y: y,
                });
            }
        }
    }
}

pub fn element_add_click_listener(
    element: &ExternRef,
    handler: impl FnMut(MouseEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_mouse_event_handler(id,e.offsetX, e.offsetY);
            };
            const id = this.storeObject(handler);
            element.addEventListener("click",handler);
            return id;
        }"#).invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_mouse_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_click_listener(element: &ExternRef, function_handle: &Arc<FunctionHandle>) {
    let remove_click_listener = js!(r#"
        function(element, f){
            element.removeEventListener("click", f);
        }"#);
    remove_click_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_mouse_event_handler(function_handle);
}

pub fn element_add_mouse_move_listener(
    element: &ExternRef,
    handler: impl FnMut(MouseEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_mouse_event_handler(id,e.offsetX, e.offsetY);
            };
            const id = this.storeObject(handler);
            element.addEventListener("mousemove",handler);
            return id;
        }"#).invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_mouse_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_mouse_move_listener(
    element: &ExternRef,
    function_handle: &Arc<FunctionHandle>,
) {
    let remove_mouse_move_listener = js!(r#"
        function(element, f){
            element.removeEventListener("mousemove", f);
        }"#);
    remove_mouse_move_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_mouse_event_handler(function_handle);
}

pub fn element_add_mouse_down_listener(
    element: &ExternRef,
    handler: impl FnMut(MouseEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_mouse_event_handler(id,e.offsetX, e.offsetY);
            };
            const id = this.storeObject(handler);
            element.addEventListener("mousedown",handler);
            return id;
        }"#).invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_mouse_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_mouse_down_listener(
    element: &ExternRef,
    function_handle: &Arc<FunctionHandle>,
) {
    let remove_mouse_down_listener = js!(r#"
        function(element, f){
            element.removeEventListener("mousedown", f);
        }"#);
    remove_mouse_down_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_mouse_event_handler(function_handle);
}

pub fn element_add_mouse_up_listener(
    element: &ExternRef,
    handler: impl FnMut(MouseEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_mouse_event_handler(id,e.offsetX, e.offsetY);
            };
            const id = this.storeObject(handler);
            element.addEventListener("mouseup",handler);
            return id;
        }"#).invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_mouse_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_mouse_up_listener(
    element: &ExternRef,
    function_handle: &Arc<FunctionHandle>,
) {
    let remove_mouse_up_listener = js!(r#"
        function(element, f){
            element.removeEventListener("mouseup", f);
        }"#);
    remove_mouse_up_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_mouse_event_handler(function_handle);
}

// Keyboard Events

pub struct KeyboardEvent {
    pub key_code: f64,
}

static KEYBOARD_EVENT_HANDLERS: Mutex<
    Option<HashMap<Arc<FunctionHandle>, Box<dyn FnMut(KeyboardEvent) + Send + 'static>>>,
> = Mutex::new(None);

fn add_keyboard_event_handler(
    function_handle: Arc<FunctionHandle>,
    handler: Box<dyn FnMut(KeyboardEvent) + Send + 'static>,
) {
    let mut h = KEYBOARD_EVENT_HANDLERS.lock().unwrap();
    if h.is_none() {
        *h = Some(HashMap::new());
    }
    h.as_mut().unwrap().insert(function_handle, handler);
}

fn remove_keyboard_event_handler(function_handle: &Arc<FunctionHandle>) {
    let mut h = KEYBOARD_EVENT_HANDLERS.lock().unwrap();
    if h.is_none() {
        return;
    }
    h.as_mut().unwrap().remove(function_handle);
}

#[no_mangle]
pub extern "C" fn web_handle_keyboard_event_handler(id: i64, key_code: f64) {
    let mut handlers = KEYBOARD_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        for (key, handler) in h.iter_mut() {
            if key.0.value == id {
                handler(KeyboardEvent { key_code });
            }
        }
    }
}

pub fn element_add_key_down_listener(
    element: &ExternRef,
    handler: impl FnMut(KeyboardEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_keyboard_event_handler(id,e.keyCode);
            };
            const id = this.storeObject(handler);
            element.addEventListener("keydown",handler);
            return id;
        }"#)
    .invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_keyboard_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_key_down_listener(
    element: &ExternRef,
    function_handle: &Arc<FunctionHandle>,
) {
    let remove_key_down_listener = js!(r#"
        function(element, f){
            element.removeEventListener("keydown", f);
        }"#);
    remove_key_down_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_keyboard_event_handler(function_handle);
}

pub fn element_add_key_up_listener(
    element: &ExternRef,
    handler: impl FnMut(KeyboardEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_keyboard_event_handler(id,e.keyCode);
            };
            const id = this.storeObject(handler);
            element.addEventListener("keyup",handler);
            return id;
        }"#)
    .invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_keyboard_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_key_up_listener(element: &ExternRef, function_handle: &Arc<FunctionHandle>) {
    let remove_key_up_listener = js!(r#"
        function(element, f){
            element.removeEventListener("keyup", f);
        }"#);
    remove_key_up_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_keyboard_event_handler(function_handle);
}
