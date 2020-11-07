#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use core::future::Future;
use js::*;
use callback::*;

pub type Handle = f64;

pub struct Timer {
    fn_set_timeout: JSFunction,
    fn_set_interval: JSFunction,
    fn_request_animation_frame: JSFunction,
    fn_request_animation_loop: JSFunction,
    fn_clear_timeout: JSFunction,
    fn_clear_interval: JSFunction,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            fn_set_timeout: register_function("function(handler,time){
                window.setTimeout(this.createCallback(handler),time);
            }"),
            fn_set_interval: register_function("function(handler,time){
                window.setInterval(this.createCallback(handler),time);
            }"),
            fn_request_animation_frame: register_function("function(handler){
                window.requestAnimationFrame(this.createCallback(handler));
            }"),
            fn_request_animation_loop: register_function("function(cb){
                cb = this.createCallback(cb);
                let time = Date.now();
                function run(){
                    let new_time = Dateusize.now();
                    let delta = new_time-time;
                    time = new_time;
                    window.requestAnimationFrame(run);
                    cb(delta);
                }
                window.requestAnimationFrame(run);
            }"),
            fn_clear_timeout: register_function("function(handle){
                window.clearTimeout(handle);
            }"),
            fn_clear_interval: register_function("function(handle){
                window.clearInterval(handle);
            }"),
        }
    }
}

impl Timer {
    pub fn set_timeout(
        &self,
        callback: Box<dyn FnMut() -> () + Send>,
        milliseconds: u32,
    ) -> (Handle, JSFunction) {
        let cb = create_callback_0(callback);
        let handle = self.fn_set_timeout.invoke_2(cb, milliseconds);
        (handle, cb.into())
    }

    pub fn sleep(&self, milliseconds: u32) -> impl Future {
        let (future, cb) = create_callback_future_0();
        self.fn_set_timeout.invoke_2(cb, milliseconds);
        future
    }

    pub fn set_interval(
        &self,
        callback: Box<dyn FnMut() -> () + Send>,
        milliseconds: u32,
    ) -> (Handle, JSFunction) {
        let cb = create_callback_0(callback);
        let handle = self.fn_set_interval.invoke_2(cb, milliseconds);
        (handle, cb.into())
    }

    pub fn request_animation_frame(&self, callback: Box<dyn FnMut() -> () + Send>) -> JSFunction {
        let cb = create_callback_0(callback);
        self.fn_request_animation_frame.invoke_1(cb);
        cb.into()
    }

    pub fn request_animation_loop(
        &self,
        callback: Box<dyn FnMut(f64) -> () + Send>,
    ) -> JSFunction {
        let cb = create_callback_1(callback);
        self.fn_request_animation_loop.invoke_1(cb);
        cb.into()
    }

    pub fn clear_timeout(&self, handle: Handle) {
        self.fn_clear_timeout.invoke_1(handle);
    }

    pub fn clear_interval(&self, handle: Handle) {
        self.fn_clear_interval.invoke_1(handle);
    }
}
