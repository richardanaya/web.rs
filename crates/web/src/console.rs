use js::*;

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
