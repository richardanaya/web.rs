#![no_std]
use js::*;

struct Console {
    fn_log: JSFunction,
    fn_clear: JSFunction,
    fn_error: JSFunction,
    fn_warning: JSFunction,
    fn_time: JSFunction,
    fn_time_end: JSFunction,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            fn_log: register_function(
                "function(strPtr,strLen){
                console.log(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
            ),
            fn_clear: register_function(
                "function(strPtr,strLen){
                console.clear(); 
            }",
            ),
            fn_error: register_function(
                "function(strPtr,strLen){
                console.error(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
            ),
            fn_warning: register_function(
                "function(strPtr,strLen){
                console.warn(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
            ),
            fn_time: register_function(
                "function(strPtr,strLen){
                console.time(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
            ),
            fn_time_end: register_function(
                "function(strPtr,strLen){
                console.timeEnd(this.readUtf8FromMemory(strPtr,strLen)); 
            }",
            ),
        }
    }
}

impl Console {
    fn clear(&self) {
        self.fn_clear.invoke_0();
    }

    fn log(&self, msg: &str) {
        self.fn_log.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
    }

    fn warning(&self, msg: &str) {
        self.fn_warning
            .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
    }

    fn error(&self, msg: &str) {
        self.fn_error
            .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
    }

    fn time(&self, label: Option<&str>) {
        if label.is_none() {
            self.fn_time.invoke_0();
        } else {
            let msg = label.unwrap();
            self.fn_time.invoke_2(msg.as_ptr() as u32, msg.len() as u32);
        }
    }

    fn time_end(&self, label: Option<&str>) {
        if label.is_none() {
            self.fn_time_end.invoke_0();
        } else {
            let msg = label.unwrap();
            self.fn_time_end
                .invoke_2(msg.as_ptr() as u32, msg.len() as u32);
        }
    }
}

pub fn clear() {
    globals::get::<Console>().clear();
}

pub fn log(msg: &str) {
    globals::get::<Console>().log(msg);
}

pub fn warning(msg: &str) {
    globals::get::<Console>().warning(msg);
}

pub fn error(msg: &str) {
    globals::get::<Console>().error(msg);
}

pub fn time(label: Option<&str>) {
    globals::get::<Console>().time(label);
}

pub fn time_end(label: Option<&str>) {
    globals::get::<Console>().time_end(label);
}
