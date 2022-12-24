use js::*;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::{get_property_f64, get_property_i64};

pub type TimerHandle = f64;

static ANIMATION_FRAME_EVENT_HANDLERS: Mutex<
    Option<HashMap<i64, Box<dyn FnMut() + Send + 'static>>>,
> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn web_one_time_empty_handler(id: i64) {
    let mut c = None;
    {
        let mut handlers = ANIMATION_FRAME_EVENT_HANDLERS.lock().unwrap();
        if let Some(h) = handlers.as_mut() {
            // remove
            if let Some(handler) = h.remove(&id) {
                c = Some(handler);
            }
        }
    }
    if let Some(mut c) = c {
        c();
    }
}

pub fn request_animation_frame(handler: impl FnMut() + Send + Sync + 'static) {
    let function_handle = js!(r#"
        function(){
            const handler = () => {
                this.module.instance.exports.web_one_time_empty_handler(id);
                this.releaseObject(id);
            };
            const id = this.storeObject(handler);
            requestAnimationFrame(handler);
            return id;
        }"#)
    .invoke_and_return_bigint(&[]);
    let mut h = ANIMATION_FRAME_EVENT_HANDLERS.lock().unwrap();
    if h.is_none() {
        *h = Some(HashMap::new());
    }
    h.as_mut()
        .unwrap()
        .insert(function_handle, Box::new(handler));
}

pub fn set_timeout(
    handler: impl FnMut() + 'static + Send + Sync,
    ms: impl Into<f64>,
) -> TimerHandle {
    let obj_handle = js!(r#"
        function(ms){
            const handler = () => {
                this.module.instance.exports.web_one_time_empty_handler(id);
                this.releaseObject(id);
            };
            const id = this.storeObject(handler);
            const handle = window.setTimeout(handler, ms);
            return {id,handle};
        }"#)
    .invoke_and_return_object(&[ms.into().into()]);
    let function_handle = get_property_i64(&obj_handle, "id");
    let timer_handle = get_property_f64(&obj_handle, "handle");
    let mut h = ANIMATION_FRAME_EVENT_HANDLERS.lock().unwrap();
    if h.is_none() {
        *h = Some(HashMap::new());
    }
    h.as_mut()
        .unwrap()
        .insert(function_handle, Box::new(handler));
    timer_handle
}

pub fn clear_timeout(interval_id: impl Into<f64>) {
    let clear_interval = js!(r#"
        function(interval_id){
            window.clearTimeout(interval_id);
        }"#);
    clear_interval.invoke(&[interval_id.into().into()]);
}
