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

impl GPUDevice {
    pub fn create_buffer(&self, descriptor: &GPUBufferDescriptor) -> GPUBuffer {
        let create_buffer = js!(r#"
            function(device, descriptor){
                return device.createBuffer(descriptor);
            }"#);
        let buffer_ref = create_buffer.invoke_and_return_object(&[(&(self.0)).into(), (&(descriptor.0)).into()]);
        GPUBuffer(buffer_ref)
    }
}

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

pub struct GPUBufferDescriptor(ExternRef);

impl GPUBufferDescriptor {
    pub fn new() -> Self {
        let descriptor_ref = create_object();
        GPUBufferDescriptor(descriptor_ref)
    }

    pub fn set_size(&self, size: usize) {
        js!(r#"
            function(descriptor, size){
                descriptor.size = size;
            }"#)
        .invoke(&[(&(self.0)).into(), size.into()]);
    }

    pub fn set_usage(&self, usage: u32) {
        js!(r#"
            function(descriptor, usage){
                descriptor.usage = usage;
            }"#)
        .invoke(&[(&(self.0)).into(), (usage as f64).into()]);
    }

    pub fn set_mapped_at_creation(&self, mapped_at_creation: bool) {
        js!(r#"
            function(descriptor, mapped_at_creation){
                descriptor.mappedAtCreation = mapped_at_creation;
            }"#)
        .invoke(&[(&(self.0)).into(), mapped_at_creation.into()]);
    }
}

pub const GPU_BUFFER_USAGE_MAP_READ: u32 = 0x0001;
pub const GPU_BUFFER_USAGE_MAP_WRITE: u32 = 0x0002;
pub const GPU_BUFFER_USAGE_COPY_SRC: u32 = 0x0004;
pub const GPU_BUFFER_USAGE_COPY_DST: u32 = 0x0008;
pub const GPU_BUFFER_USAGE_INDEX: u32 = 0x0010;
pub const GPU_BUFFER_USAGE_VERTEX: u32 = 0x0020;
pub const GPU_BUFFER_USAGE_UNIFORM: u32 = 0x0040;
pub const GPU_BUFFER_USAGE_STORAGE: u32 = 0x0080;
pub const GPU_BUFFER_USAGE_INDIRECT: u32 = 0x0100;
pub const GPU_BUFFER_USAGE_QUERY_RESOLVE: u32 = 0x0200;

pub struct GPUBuffer(ExternRef);

impl GPUBuffer {
    pub fn set_from_f32_array(&self, data: &[f32]) {
        js!(r#"
            function(buffer, data){
                new Float32Array(buffer.getMappedRange()).set(data);
                positionBuffer.unmap();
            }"#)
        .invoke(&[(&(self.0)).into(), data.into()]);
    }

    pub fn set_from_u32_array(&self, data: &[u32]) {
        js!(r#"
            function(buffer, data){
                new Uint32Array(buffer.getMappedRange()).set(data);
                positionBuffer.unmap();
            }"#)
        .invoke(&[(&(self.0)).into(), data.into()]);
    }
}