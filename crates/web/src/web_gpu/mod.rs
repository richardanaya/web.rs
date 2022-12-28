use crate::create_object;
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

    pub async fn request_adapter() -> GPUAdapter {
        let (future, state_id) = EventHandlerFuture::<ExternRef>::create_future_with_state_id();
        js!(r#"
            async function(state_id){
                const a = await navigator.gpu.requestAdapter();
                const ref = this.storeObject(a);
                this.module.instance.exports.web_extern_ref_callback(state_id, ref);
            }"#)
        .invoke(&[state_id.into()]);
        let adapter_ref = future.await;
        GPUAdapter(adapter_ref)
    }

    pub fn get_preferred_canvas_format() -> String {
        js!(r#"
            function(){
                return navigator.gpu.getPreferredCanvasFormat();
            }"#)
        .invoke_and_return_string(&[])
    }
}

pub struct GPUAdapter(ExternRef);

impl GPUAdapter {
    pub async fn request_device(&self) -> GPUDevice {
        let (future, state_id) = EventHandlerFuture::<ExternRef>::create_future_with_state_id();
        js!(r#"
            async function(adapter, state_id){
                const d = await adapter.requestDevice();
                const ref = this.storeObject(d);
                this.module.instance.exports.web_extern_ref_callback(state_id, ref);
            }"#)
        .invoke(&[(&(self.0)).into(), state_id.into()]);
        let device_ref = future.await;
        GPUDevice(device_ref)
    }
}

pub struct GPUDevice(ExternRef);

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

    pub fn configure(&self, config: &GpuCanvasConfiguration) {
        js!(r#"
            function(ctx, config){
                ctx.configure(config);
            }"#)
        .invoke(&[(&(self.0)).into(), (&(config.0)).into()]);
    }
}

pub struct GpuCanvasConfiguration(ExternRef);

impl GpuCanvasConfiguration {
    pub fn new() -> Self {
        let config_ref = create_object();
        GpuCanvasConfiguration(config_ref)
    }

    pub fn set_device(&self, device: &GPUDevice) {
        js!(r#"
            function(config, device){
                config.device = device;
            }"#)
        .invoke(&[(&(self.0)).into(), (&(device.0)).into()]);
    }

    pub fn set_format(&self, format: &str) {
        js!(r#"
            function(config, format){
                config.format = format;
            }"#)
        .invoke(&[(&(self.0)).into(), format.into()]);
    }

    pub fn set_alpha_mode(&self, alpha_mode: GPUCanvasAlphaMode) {
        js!(r#"
            function(config, alpha_mode){
                config.alphaMode = alpha_mode;
            }"#)
        .invoke(&[(&(self.0)).into(), alpha_mode.as_str().into()]);
    }
}

pub enum GPUCanvasAlphaMode {
    Premultiplied,
    Opaque,
}

impl GPUCanvasAlphaMode {
    fn as_str(&self) -> &'static str {
        match self {
            GPUCanvasAlphaMode::Premultiplied => "premultiplied",
            GPUCanvasAlphaMode::Opaque => "opaque",
        }
    }
}
