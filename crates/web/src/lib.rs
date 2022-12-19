#![no_std]
pub use js::*;

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
