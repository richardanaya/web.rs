#![no_std]
use callback::*;
use core::future::Future;
use js::*;

pub type Handle = f64;

struct Timer {
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
            fn_set_timeout: register_function(
                "function(handler,time){
                window.setTimeout(this.createCallback(handler),time);
            }",
            ),
            fn_set_interval: register_function(
                "function(handler,time){
                window.setInterval(this.createCallback(handler),time);
            }",
            ),
            fn_request_animation_frame: register_function(
                "function(handler){
                window.requestAnimationFrame(this.createCallback(handler));
            }",
            ),
            fn_request_animation_loop: register_function(
                "function(cb){
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
            }",
            ),
            fn_clear_timeout: register_function(
                "function(handle){
                window.clearTimeout(handle);
            }",
            ),
            fn_clear_interval: register_function(
                "function(handle){
                window.clearInterval(handle);
            }",
            ),
        }
    }
}

pub fn set_timeout(
    callback: impl FnMut() -> () + Send + 'static,
    milliseconds: impl Into<f64>,
) -> (Handle, JSFunction) {
    let cb = create_callback_0(callback);
    let handle = globals::get::<Timer>()
        .fn_set_timeout
        .invoke_2(cb, milliseconds);
    (handle, cb.into())
}

pub fn sleep(milliseconds: impl Into<f64>) -> impl Future {
    let (future, cb) = create_callback_future_0();
    globals::get::<Timer>()
        .fn_set_timeout
        .invoke_2(cb, milliseconds);
    future
}

pub fn set_interval(
    callback: impl FnMut() -> () + Send + 'static,
    milliseconds: impl Into<f64>,
) -> (Handle, JSFunction) {
    let cb = create_callback_0(callback);
    let handle = globals::get::<Timer>()
        .fn_set_interval
        .invoke_2(cb, milliseconds);
    (handle, cb.into())
}

pub fn request_animation_frame(callback: impl FnMut() -> () + Send + 'static) -> JSFunction {
    let cb = create_callback_0(callback);
    globals::get::<Timer>()
        .fn_request_animation_frame
        .invoke_1(cb);
    cb.into()
}

pub fn request_animation_loop(callback: impl FnMut(f64) -> () + Send + 'static) -> JSFunction {
    let cb = create_callback_1(callback);
    globals::get::<Timer>()
        .fn_request_animation_loop
        .invoke_1(cb);
    cb.into()
}

pub fn clear_timeout(handle: Handle) {
    globals::get::<Timer>().fn_clear_timeout.invoke_1(handle);
}

pub fn clear_interval(handle: Handle) {
    globals::get::<Timer>().fn_clear_interval.invoke_1(handle);
}
