use js::*;
use std::collections::HashMap;
use std::sync::Mutex;

// Request Animation Frame

static ANIMATION_FRAME_EVENT_HANDLERS: Mutex<
    Option<HashMap<i64, Box<dyn FnMut() + Send + 'static>>>,
> = Mutex::new(None);

fn add_animation_frame_event_handler(
    function_handle: i64,
    handler: Box<dyn FnMut() + Send + 'static>,
) {
    let mut h = ANIMATION_FRAME_EVENT_HANDLERS.lock().unwrap();
    if h.is_none() {
        *h = Some(HashMap::new());
    }
    h.as_mut().unwrap().insert(function_handle, handler);
}

#[no_mangle]
pub extern "C" fn web_handle_animation_frame_event_handler(id: i64) {
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
    let function_ref = js!(r#"
        function(){
            const handler = () => {
                this.module.instance.exports.web_handle_animation_frame_event_handler(id);
                this.releaseObject(id);
            };
            const id = this.storeObject(handler);
            requestAnimationFrame(handler);
            return id;
        }"#)
    .invoke_and_return_bigint(&[]);
    add_animation_frame_event_handler(function_ref, Box::new(handler));
}
