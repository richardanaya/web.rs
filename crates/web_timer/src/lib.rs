#![no_std]
use callback::*;
use core::future::Future;
use js::*;

pub type Handle = f64;

pub fn set_timeout(
    callback: impl FnMut() -> () + Send + 'static,
    milliseconds: impl Into<f64>,
) -> (Handle, JSFunction) {
    let cb = create_callback_0(callback);
    let fn = {
        lazy_static::lazy_static! {
        static ref FN: JSFunction= {
            register_function(
                "function(handler,time){
                    window.setTimeout(this.createCallback(handler),time);
                }",
            )
        };};
        &FN
    };
    let handle = fn.invoke_2(cb, milliseconds);
    (handle, cb.into())
}

pub fn sleep(milliseconds: impl Into<f64>) -> impl Future {
    let (future, cb) = create_callback_future_0();
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(handler,time){
                window.setTimeout(this.createCallback(handler),time);
            }",
        )
    };};
    FN.invoke_2(cb, milliseconds);
    future
}

pub fn set_interval(
    callback: impl FnMut() -> () + Send + 'static,
    milliseconds: impl Into<f64>,
) -> (Handle, JSFunction) {
    let cb = create_callback_0(callback);
    lazy_static::lazy_static! {
    static ref FN: JSFunction= {
        register_function(
            "function(handler,time){
                window.setInterval(this.createCallback(handler),time);
            }",
        )
    };};
    let handle = FN.invoke_2(cb, milliseconds);
    (handle, cb.into())
}

pub fn request_animation_frame(callback: impl FnMut() -> () + Send + 'static) -> JSFunction {
    let cb = create_callback_0(callback);
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(handler){
                window.requestAnimationFrame(this.createCallback(handler));
            }",
        )
    };};
    FN.invoke_1(cb);
    cb.into()
}

pub fn request_animation_loop(callback: impl FnMut(f64) -> () + Send + 'static) -> JSFunction {
    let cb = create_callback_1(callback);
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(cb){
                cb = this.createCallback(cb);
                let time = Date.now();
                function run(){
                    let new_time = Date.now();
                    let delta = new_time-time;
                    time = new_time;
                    window.requestAnimationFrame(run);
                    cb(delta);
                }
                window.requestAnimationFrame(run);
            }",
        )
    };};
    FN.invoke_1(cb);
    cb.into()
}

pub fn clear_timeout(handle: Handle) {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(handle){
                window.clearTimeout(handle);
            }",
        )
    };};
    FN.invoke_1(handle);
}

pub fn clear_interval(handle: Handle) {
    lazy_static::lazy_static! {
        static ref FN: JSFunction= {
        register_function(
            "function(handle){
                window.clearInterval(handle);
            }",
        )
    };};
    FN.invoke_1(handle);
}
