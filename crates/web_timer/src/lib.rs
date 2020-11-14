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
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    let handle = FN
        .get_or_init(|| {
            register_function(
                "function(handler,time){
                window.setTimeout(this.createCallback(handler),time);
            }",
            )
        })
        .invoke_2(cb, milliseconds);
    (handle, cb.into())
}

pub fn sleep(milliseconds: impl Into<f64>) -> impl Future {
    let (future, cb) = create_callback_future_0();
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(handler,time){
                window.setTimeout(this.createCallback(handler),time);
            }",
        )
    })
    .invoke_2(cb, milliseconds);
    future
}

pub fn set_interval(
    callback: impl FnMut() -> () + Send + 'static,
    milliseconds: impl Into<f64>,
) -> (Handle, JSFunction) {
    let cb = create_callback_0(callback);
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    let handle = FN
        .get_or_init(|| {
            register_function(
                "function(handler,time){
                window.setInterval(this.createCallback(handler),time);
            }",
            )
        })
        .invoke_2(cb, milliseconds);
    (handle, cb.into())
}

pub fn request_animation_frame(callback: impl FnMut() -> () + Send + 'static) -> JSFunction {
    let cb = create_callback_0(callback);
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(handler){
                window.requestAnimationFrame(this.createCallback(handler));
            }",
        )
    })
    .invoke_1(cb);
    cb.into()
}

pub fn request_animation_loop(callback: impl FnMut(f64) -> () + Send + 'static) -> JSFunction {
    let cb = create_callback_1(callback);
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
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
    })
    .invoke_1(cb);
    cb.into()
}

pub fn clear_timeout(handle: Handle) {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(handle){
                window.clearTimeout(handle);
            }",
        )
    })
    .invoke_1(handle);
}

pub fn clear_interval(handle: Handle) {
    static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
    FN.get_or_init(|| {
        register_function(
            "function(handle){
                window.clearInterval(handle);
            }",
        )
    })
    .invoke_1(handle);
}
