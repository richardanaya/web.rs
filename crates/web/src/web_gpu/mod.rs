use crate::EventHandlerFuture;
use js::*;

pub struct WebGPU;

#[no_mangle]
pub extern "C" fn web_extern_ref_callback(id: i64, value: i64) {
    EventHandlerFuture::<ExternRef>::wake_future_with_state_id(id, ExternRef { value });
}

impl WebGPU {
    pub fn is_available() -> bool {
        js!(r#"
        function(element){
            return (typeof navigator !== "undefined" && navigator.gpu);
        }"#)
        .invoke_and_return_bool(&[])
    }

    pub async fn request_adapter() -> WebGPUAdapter {
        let (future, state_id) = EventHandlerFuture::<ExternRef>::create_future_with_state_id();
        js!(r#"
            async function(state_id){
                const a = await navigator.gpu.requestAdapter();
                const ref = this.storeObject(a);
                this.module.instance.exports.web_extern_ref_callback(state_id, ref);
            }"#)
        .invoke(&[state_id.into()]);
        let adapter_ref = future.await;
        WebGPUAdapter(adapter_ref)
    }

    pub fn get_preferred_canvas_format() -> String {
        js!(r#"
            function(){
                return navigator.gpu.getPreferredCanvasFormat();
            }"#)
        .invoke_and_return_string(&[])
    }
}

pub struct WebGPUAdapter(ExternRef);

impl WebGPUAdapter {
    pub async fn request_device(&self) -> WebGPUDevice {
        let (future, state_id) = EventHandlerFuture::<ExternRef>::create_future_with_state_id();
        js!(r#"
            async function(adapter, state_id){
                const d = await adapter.requestDevice();
                const ref = this.storeObject(d);
                this.module.instance.exports.web_extern_ref_callback(state_id, ref);
            }"#)
        .invoke(&[(&(self.0)).into(), state_id.into()]);
        let device_ref = future.await;
        WebGPUDevice(device_ref)
    }
}

pub struct WebGPUDevice(ExternRef);

pub struct GpuCanvasContext(ExternRef);

impl GpuCanvasContext {
    pub fn from_element(element: &ExternRef) -> Self {
        let get_context = js!(r#"
            function(element){
                return element.getContext("webgpu");
            }"#);
        let ctx_ref = get_context.invoke_and_return_object(&[element.into()]);
        GpuCanvasContext(ctx_ref)
    }
}
