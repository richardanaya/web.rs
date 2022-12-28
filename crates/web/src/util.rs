use crate::EventHandlerFuture;
use core::future::Future;
use js::*;

pub fn random() -> f64 {
    let random = js!(r#"
        function(){
            return Math.random();
        }"#);
    random.invoke(&[])
}

pub fn random_i64() -> i64 {
    let r = random();
    let i64 = std::i64::MAX;
    (r * i64 as f64) as i64
}

pub fn get_property_i64(element: &ExternRef, property: &str) -> i64 {
    let get_property = js!(r#"
        function(element, property){
            return element[property];
        }"#);
    get_property.invoke_and_return_bigint(&[element.into(), property.into()])
}

pub fn set_property_i64(element: &ExternRef, property: &str, value: i64) {
    let set_property = js!(r#"
        function(element, property, value){
            element[property] = value;
        }"#);
    set_property.invoke(&[element.into(), property.into(), value.into()]);
}

pub fn get_property_f64(element: &ExternRef, property: &str) -> f64 {
    let get_property = js!(r#"
        function(element, property){
            return element[property];
        }"#);
    get_property.invoke(&[element.into(), property.into()])
}

pub fn set_property_f64(element: &ExternRef, property: &str, value: f64) {
    let set_property = js!(r#"
        function(element, property, value){
            element[property] = value;
        }"#);
    set_property.invoke(&[element.into(), property.into(), value.into()]);
}

pub fn get_property_bool(element: &ExternRef, property: &str) -> bool {
    let get_property = js!(r#"
        function(element, property){
            return element[property]?1:0;
        }"#);
    let v = get_property.invoke(&[element.into(), property.into()]);
    v == 1.0
}

pub fn set_property_bool(element: &ExternRef, property: &str, value: bool) {
    let set_property = js!(r#"
        function(element, property, value){
            element[property] = value !==0;
        }"#);
    set_property.invoke(&[element.into(), property.into(), value.into()]);
}

pub fn get_property_string(element: &ExternRef, property: &str) -> String {
    let get_property = js!(r#"
        function(element, property){
            const text = element[property];
            const allocationId = this.writeUtf8ToMemory(text);
            return allocationId;
        }"#);
    let text_allocation_id = get_property.invoke(&[element.into(), property.into()]);
    let text = extract_string_from_memory(text_allocation_id as usize);
    text
}

pub fn set_property_string(element: &ExternRef, property: &str, value: &str) {
    let set_property = js!(r#"
        function(element, property, value){
            element[property] = value;
        }"#);
    set_property.invoke(&[element.into(), property.into(), value.into()]);
}

#[no_mangle]
pub extern "C" fn web_handle_empty_callback(id: i64) {
    EventHandlerFuture::<()>::wake_future_with_state_id(id, ());
}

pub fn sleep(ms: impl Into<f64>) -> impl Future<Output = ()> {
    let sleep = js!(r#"
        function(ms, state_id){
            window.setTimeout(()=>{
                this.module.instance.exports.web_handle_empty_callback(state_id);
            }, ms);
        }"#);
    let ms = ms.into();
    let (future, state_id) = EventHandlerFuture::<()>::create_future_with_state_id();
    sleep.invoke(&[ms.into(), state_id.into()]);
    future
}

pub fn wait_til_animation_frame() -> impl Future<Output = ()> {
    let wait_til_animation_frame = js!(r#"
        function(state_id){
            window.requestAnimationFrame(()=>{
                this.module.instance.exports.web_handle_empty_callback(state_id);
            });
        }"#);
    let (future, state_id) = EventHandlerFuture::<()>::create_future_with_state_id();
    wait_til_animation_frame.invoke(&[state_id.into()]);
    future
}

pub fn create_object() -> ExternRef {
    let config_ref = js!(r#"
        function(){
            return {};
        }"#)
    .invoke_and_return_object(&[]);
    config_ref
}

pub fn create_array() -> ExternRef {
    let config_ref = js!(r#"
        function(){
            return [];
        }"#)
    .invoke_and_return_object(&[]);
    config_ref
}

pub fn add_to_array(array: &ExternRef, value: &ExternRef) {
    let add_to_array = js!(r#"
        function(array, value){
            array.push(value);
        }"#);
    add_to_array.invoke(&[(array).into(), (value).into()]);
}
