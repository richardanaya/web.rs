use crate::{add_to_array, console_error, create_array, EventHandlerFuture};
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

    pub fn get_preferred_canvas_format() -> GPUTextureFormat {
        let f = js!(r#"
            function(){
                return navigator.gpu.getPreferredCanvasFormat();
            }"#)
        .invoke_and_return_string(&[]);
        GPUTextureFormat::from_str(&f).unwrap()
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
pub struct GPUShaderModule(ExternRef);
pub struct GPUPipelineLayout(ExternRef);
pub struct GPUQueue(ExternRef);
pub struct GPUCommandEncoder(ExternRef);
pub struct GPURenderPipeline(ExternRef);

impl GPUQueue {
    pub fn submit(&self, command_buffers: &[GPUCommandBuffer]) {
        let command_buffers_ref = create_array();
        command_buffers.iter().for_each(|command_buffer| {
            add_to_array(&command_buffers_ref, &command_buffer.0);
        });
        let submit = js!(r#"
            function(queue, commandBuffers){
                queue.submit(commandBuffers);
            }"#);
        submit.invoke(&[(&(self.0)).into(), (&command_buffers_ref).into()]);
    }
}

impl GPUDevice {
    pub fn create_buffer(&self, descriptor: &GPUBufferDescriptor) -> GPUBuffer {
        let create_buffer = js!(r#"
            function(device, size, usage, mappedAtCreation){
                return device.createBuffer({
                    size,
                    usage,
                    mappedAtCreation
                });
            }"#);
        let buffer_ref = create_buffer.invoke_and_return_object(&[
            (&(self.0)).into(),
            (descriptor.size).into(),
            (descriptor.usage).into(),
            (descriptor.mapped_at_creation).into(),
        ]);
        GPUBuffer(buffer_ref)
    }

    pub fn create_shader_module_from_source(&self, source: &str) -> GPUShaderModule {
        let create_shader_module = js!(r#"
            function(device, source){
                return device.createShaderModule({code: source});
            }"#);
        let shader_module_ref =
            create_shader_module.invoke_and_return_object(&[(&(self.0)).into(), source.into()]);
        GPUShaderModule(shader_module_ref)
    }

    pub fn create_pipeline_layout(
        &self,
        descriptor: &GPUPipelineLayoutDescriptor,
    ) -> GPUPipelineLayout {
        let create_pipeline_layout = js!(r#"
            function(device, bindGroupLayouts){
                return device.createPipelineLayout({
                    bindGroupLayouts:[]
                });
            }"#);
        if descriptor.bind_group_layouts.len() > 0 {
            console_error("bind_group_layouts not supported yet");
            panic!("bind_group_layouts not supported yet");
        }
        let pipeline_layout_ref =
            create_pipeline_layout.invoke_and_return_object(&[(&(self.0)).into()]);
        GPUPipelineLayout(pipeline_layout_ref)
    }

    pub fn get_queue(&self) -> GPUQueue {
        let get_queue = js!(r#"
            function(device){
                return device.queue;
            }"#);
        let queue_ref = get_queue.invoke_and_return_object(&[(&(self.0)).into()]);
        GPUQueue(queue_ref)
    }

    pub fn create_command_encoder(&self) -> GPUCommandEncoder {
        let create_command_encoder = js!(r#"
            function(device){
                return device.createCommandEncoder();
            }"#);
        let command_encoder_ref =
            create_command_encoder.invoke_and_return_object(&[(&(self.0)).into()]);
        GPUCommandEncoder(command_encoder_ref)
    }

    pub fn create_render_pipeline(
        &self,
        descriptor: &GPURenderPipelineDescriptor,
    ) -> GPURenderPipeline {
        let fragment_state_ref = {
            let fragment_state = &descriptor.fragment;
            let targets_ref = create_array();
            descriptor.fragment.targets.iter().for_each(|target| {
                let format = target.format;
                let target_ref = js!(r#"
                function(format){
                    return {
                        format
                    };
                }"#)
                .invoke_and_return_object(&[format.as_str().into()]);
                add_to_array(&targets_ref, &target_ref);
            });

            js!(r#"
                function(module, entryPoint, targets){
                    return {
                        module,
                        entryPoint,
                        targets
                    };
                }"#)
            .invoke_and_return_object(&[
                (&(fragment_state.module.0)).into(),
                fragment_state.entry_point.into(),
                (&targets_ref).into(),
            ])
        };
        let vertex_state_ref = {
            let vertex_state = &descriptor.vertex;
            let vertex_buffers_ref = create_array();
            vertex_state.buffers.iter().for_each(|buffer| {
                let attributes_ref = create_array();
                buffer.attributes.iter().for_each(|attribute| {
                    let attribute_ref = js!(r#"
                    function(shaderLocation, offset, format){
                        return {
                            shaderLocation,
                            offset,
                            format
                        };
                    }"#)
                    .invoke_and_return_object(&[
                        attribute.shader_location.into(),
                        attribute.offset.into(),
                        attribute.format.as_str().into(),
                    ]);
                    add_to_array(&attributes_ref, &attribute_ref);
                });
                let buffer_ref = js!(r#"
                    function(arrayStride, stepMode, attributes){
                        return {
                            arrayStride,
                            stepMode,
                            attributes
                        };
                    }"#)
                .invoke_and_return_object(&[
                    buffer.array_stride.into(),
                    buffer.step_mode.as_str().into(),
                    (&attributes_ref).into(),
                ]);
                add_to_array(&vertex_buffers_ref, &buffer_ref);
            });
            js!(r#"
                function(module, entryPoint, buffers){
                    return {
                        module,
                        entryPoint,
                        buffers
                    };
                }"#)
            .invoke_and_return_object(&[
                (&(vertex_state.module.0)).into(),
                vertex_state.entry_point.into(),
                (&vertex_buffers_ref).into(),
            ])
        };
        let primitive_state_ref = js!(r#"
            function(topology, cullMode, frontFace){
                return {
                    topology,
                    cullMode,
                    frontFace
                };
            }"#)
        .invoke_and_return_object(&[
            descriptor.primitive.topology.as_str().into(),
            descriptor.primitive.cull_mode.as_str().into(),
            descriptor.primitive.front_face.as_str().into(),
        ]);

        let pipeline_ref = js!(r#"
            function(device, layout, fragment, vertex, primitive){
                let config = {
                    layout,
                    fragment,
                    vertex,
                    primitive
                };
                return device.createRenderPipeline(config);
            }"#)
        .invoke_and_return_object(&[
            (&(self.0)).into(),
            (&(descriptor.layout.0)).into(),
            (&fragment_state_ref).into(),
            (&vertex_state_ref).into(),
            (&primitive_state_ref).into(),
        ]);
        GPURenderPipeline(pipeline_ref)
    }
}

pub struct GPUCanvasContext(ExternRef);
pub struct GPUTexture(ExternRef);

impl GPUTexture {
    pub fn create_view(&self) -> GPUTextureView {
        let create_view = js!(r#"
            function(texture){
                return texture.createView();
            }"#);
        let view_ref = create_view.invoke_and_return_object(&[(&(self.0)).into()]);
        GPUTextureView(view_ref)
    }
}

pub struct GPUTextureView(ExternRef);

impl GPUCanvasContext {
    pub fn from_element(element: &ExternRef) -> Self {
        let get_context = js!(r#"
            function(element){
                return element.getContext("webgpu");
            }"#);
        let ctx_ref = get_context.invoke_and_return_object(&[element.into()]);
        GPUCanvasContext(ctx_ref)
    }

    pub fn configure(&self, config: &GpuCanvasConfiguration) {
        js!(r#"
            function(ctx, device, format, alphaMode){
                ctx.configure({
                    device: device,
                    format: format,
                    alphaMode: alphaMode,
                });
            }"#)
        .invoke(&[
            (&(self.0)).into(),
            (&(config.device.0)).into(),
            config.format.as_str().into(),
            config.alpha_mode.as_str().into(),
        ]);
    }

    pub fn get_current_texture(&self) -> GPUTexture {
        let texture_ref = js!(r#"
            function(ctx){
                return ctx.getCurrentTexture();
            }"#)
        .invoke_and_return_object(&[(&(self.0)).into()]);
        GPUTexture(texture_ref)
    }
}

pub struct GpuCanvasConfiguration<'a> {
    pub device: &'a GPUDevice,
    pub format: GPUTextureFormat,
    pub alpha_mode: GPUCanvasAlphaMode,
}

#[derive(Clone, Copy)]
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

pub struct GPUBufferDescriptor {
    pub size: usize,
    pub usage: usize,
    pub mapped_at_creation: bool,
}

pub const GPU_BUFFER_USAGE_MAP_READ: usize = 0x0001;
pub const GPU_BUFFER_USAGE_MAP_WRITE: usize = 0x0002;
pub const GPU_BUFFER_USAGE_COPY_SRC: usize = 0x0004;
pub const GPU_BUFFER_USAGE_COPY_DST: usize = 0x0008;
pub const GPU_BUFFER_USAGE_INDEX: usize = 0x0010;
pub const GPU_BUFFER_USAGE_VERTEX: usize = 0x0020;
pub const GPU_BUFFER_USAGE_UNIFORM: usize = 0x0040;
pub const GPU_BUFFER_USAGE_STORAGE: usize = 0x0080;
pub const GPU_BUFFER_USAGE_INDIRECT: usize = 0x0100;
pub const GPU_BUFFER_USAGE_QUERY_RESOLVE: usize = 0x0200;

pub struct GPUBuffer(ExternRef);

impl GPUBuffer {
    pub fn set_from_f32_array(&self, data: &[f32]) {
        js!(r#"
            function(buffer, data){
                new Float32Array(buffer.getMappedRange()).set(data);
            }"#)
        .invoke(&[(&(self.0)).into(), data.into()]);
    }

    pub fn set_from_u32_array(&self, data: &[u32]) {
        js!(r#"
            function(buffer, data){
                new Uint32Array(buffer.getMappedRange()).set(data);
            }"#)
        .invoke(&[(&(self.0)).into(), data.into()]);
    }

    pub fn unmap(&self) {
        js!(r#"
            function(buffer){
                buffer.unmap();
            }"#)
        .invoke(&[(&(self.0)).into()]);
    }
}

pub struct BindGroupLayout;
pub struct GPUPipelineLayoutDescriptor {
    pub bind_group_layouts: Vec<BindGroupLayout>,
}

#[derive(Clone, Copy)]
pub enum GPUTextureFormat {
    BGRA8Unorm,
}

impl GPUTextureFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPUTextureFormat::BGRA8Unorm => "bgra8unorm",
        }
    }

    pub fn from_str(s: &str) -> Option<GPUTextureFormat> {
        match s {
            "bgra8unorm" => Some(GPUTextureFormat::BGRA8Unorm),
            _ => None,
        }
    }
}

pub struct GPUColorTargetState {
    pub format: GPUTextureFormat,
}

pub struct GPUFragmentState<'a> {
    pub module: &'a GPUShaderModule,
    pub entry_point: &'a str,
    pub targets: Vec<GPUColorTargetState>,
}

#[derive(Clone, Copy)]
pub enum GPUPrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

impl GPUPrimitiveTopology {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPUPrimitiveTopology::PointList => "point-list",
            GPUPrimitiveTopology::LineList => "line-list",
            GPUPrimitiveTopology::LineStrip => "line-strip",
            GPUPrimitiveTopology::TriangleList => "triangle-list",
            GPUPrimitiveTopology::TriangleStrip => "triangle-strip",
        }
    }
}

#[derive(Clone, Copy)]
pub enum GPUCullMode {
    None,
    Front,
    Back,
}

impl GPUCullMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPUCullMode::None => "none",
            GPUCullMode::Front => "front",
            GPUCullMode::Back => "back",
        }
    }
}

#[derive(Clone, Copy)]
pub enum GPUFrontFace {
    CCW,
    CW,
}

impl GPUFrontFace {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPUFrontFace::CCW => "ccw",
            GPUFrontFace::CW => "cw",
        }
    }
}

pub struct GPUPrimitiveState {
    pub topology: GPUPrimitiveTopology,
    pub cull_mode: GPUCullMode,
    pub front_face: GPUFrontFace,
}

pub struct GPURenderPipelineDescriptor<'a> {
    pub layout: &'a GPUPipelineLayout,
    pub primitive: GPUPrimitiveState,
    pub vertex: GPUVertexState<'a>,
    pub fragment: GPUFragmentState<'a>,
}

pub struct GPUVertexAttribute {
    pub format: GPUVertexFormat,
    pub offset: usize,
    pub shader_location: usize,
}

#[derive(Clone, Copy)]
pub enum GPUVertexFormat {
    Uint8x2,
    Uint8x4,
    Sint8x2,
    Sint8x4,
    Unorm8x2,
    Unorm8x4,
    Snorm8x2,
    Snorm8x4,
    Uint16x2,
    Uint16x4,
    Sint16x2,
    Sint16x4,
    Unorm16x2,
    Unorm16x4,
    Snorm16x2,
    Snorm16x4,
    Float16x2,
    Float16x4,
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    Uint32,
    Uint32x2,
    Uint32x3,
    Uint32x4,
    Sint32,
    Sint32x2,
    Sint32x3,
    Sint32x4,
}

impl GPUVertexFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPUVertexFormat::Uint8x2 => "uint8x2",
            GPUVertexFormat::Uint8x4 => "uint8x4",
            GPUVertexFormat::Sint8x2 => "sint8x2",
            GPUVertexFormat::Sint8x4 => "sint8x4",
            GPUVertexFormat::Unorm8x2 => "unorm8x2",
            GPUVertexFormat::Unorm8x4 => "unorm8x4",
            GPUVertexFormat::Snorm8x2 => "snorm8x2",
            GPUVertexFormat::Snorm8x4 => "snorm8x4",
            GPUVertexFormat::Uint16x2 => "uint16x2",
            GPUVertexFormat::Uint16x4 => "uint16x4",
            GPUVertexFormat::Sint16x2 => "sint16x2",
            GPUVertexFormat::Sint16x4 => "sint16x4",
            GPUVertexFormat::Unorm16x2 => "unorm16x2",
            GPUVertexFormat::Unorm16x4 => "unorm16x4",
            GPUVertexFormat::Snorm16x2 => "snorm16x2",
            GPUVertexFormat::Snorm16x4 => "snorm16x4",
            GPUVertexFormat::Float16x2 => "float16x2",
            GPUVertexFormat::Float16x4 => "float16x4",
            GPUVertexFormat::Float32 => "float32",
            GPUVertexFormat::Float32x2 => "float32x2",
            GPUVertexFormat::Float32x3 => "float32x3",
            GPUVertexFormat::Float32x4 => "float32x4",
            GPUVertexFormat::Uint32 => "uint32",
            GPUVertexFormat::Uint32x2 => "uint32x2",
            GPUVertexFormat::Uint32x3 => "uint32x3",
            GPUVertexFormat::Uint32x4 => "uint32x4",
            GPUVertexFormat::Sint32 => "sint32",
            GPUVertexFormat::Sint32x2 => "sint32x2",
            GPUVertexFormat::Sint32x3 => "sint32x3",
            GPUVertexFormat::Sint32x4 => "sint32x4",
        }
    }
}

#[derive(Clone, Copy)]
pub enum GPUVertexStepMode {
    Vertex,
    Instance,
}

impl GPUVertexStepMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPUVertexStepMode::Vertex => "vertex",
            GPUVertexStepMode::Instance => "instance",
        }
    }
}

pub struct GPUVertexBufferLayout {
    pub array_stride: usize,
    pub step_mode: GPUInputStepMode,
    pub attributes: Vec<GPUVertexAttribute>,
}

pub struct GPUVertexState<'a> {
    pub module: &'a GPUShaderModule,
    pub entry_point: &'a str,
    pub buffers: Vec<GPUVertexBufferLayout>,
}

#[derive(Clone, Copy)]
pub enum GPUInputStepMode {
    Vertex,
    Instance,
}

impl GPUInputStepMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPUInputStepMode::Vertex => "vertex",
            GPUInputStepMode::Instance => "instance",
        }
    }
}

pub struct GPURenderPassDescriptor<'a> {
    pub color_attachments: Vec<GPURenderPassColorAttachment<'a>>,
}

pub struct GPUColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Clone, Copy)]
pub enum GPULoadOp {
    Load,
    Clear,
}

impl GPULoadOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPULoadOp::Load => "load",
            GPULoadOp::Clear => "clear",
        }
    }
}

#[derive(Clone, Copy)]
pub enum GPUStoreOp {
    Store,
    Discard,
}

impl GPUStoreOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPUStoreOp::Store => "store",
            GPUStoreOp::Discard => "discard",
        }
    }
}

pub struct GPURenderPassColorAttachment<'a> {
    pub view: &'a GPUTextureView,
    pub load_op: GPULoadOp,
    pub store_op: GPUStoreOp,
    pub clear_value: GPUColor,
}

pub struct GPURenderPass(ExternRef);

impl GPUCommandEncoder {
    pub fn begin_render_pass(&self, descriptor: &GPURenderPassDescriptor) -> GPURenderPass {
        let color_attachments_ref = create_array();
        for attachment in &descriptor.color_attachments {
            let clear_value = js!(r#"
            function(r, g, b, a){
                return {r, g, b, a};
            }
            "#)
            .invoke_and_return_object(&[
                attachment.clear_value.r.into(),
                attachment.clear_value.g.into(),
                attachment.clear_value.b.into(),
                attachment.clear_value.a.into(),
            ]);
            let attachment_ref = js!(r#"
                function(view, loadOp, storeOp, clearValue){
                    return {
                        view,
                        loadOp,
                        storeOp,
                        clearValue
                    };
                }
                "#)
            .invoke_and_return_object(&[
                (&attachment.view.0).into(),
                attachment.load_op.as_str().into(),
                attachment.store_op.as_str().into(),
                (&clear_value).into(),
            ]);
            add_to_array(&color_attachments_ref, &attachment_ref);
        }

        let render_pass_ref = js!(r#"
        function(encoder, colorAttachments){
            return encoder.beginRenderPass({
                colorAttachments
            });
        }
        "#)
        .invoke_and_return_object(&[(&self.0).into(), (&color_attachments_ref).into()]);
        GPURenderPass(render_pass_ref)
    }

    pub fn finish(&self) -> GPUCommandBuffer {
        let command_buffer_ref = js!(r#"
        function(encoder){
            return encoder.finish();
        }
        "#)
        .invoke_and_return_object(&[(&self.0).into()])
        .into();
        GPUCommandBuffer(command_buffer_ref)
    }
}

pub struct GPUCommandBuffer(ExternRef);

impl GPURenderPass {
    pub fn set_pipeline(&self, pipeline: &GPURenderPipeline) {
        js!(r#"
        function(encoder, pipeline){
            encoder.setPipeline(pipeline);
        }
        "#)
        .invoke(&[(&self.0).into(), (&pipeline.0).into()]);
    }

    pub fn set_viewport(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        min_depth: f64,
        max_depth: f64,
    ) {
        js!(r#"
        function(encoder, x, y, width, height, minDepth, maxDepth){
            encoder.setViewport(x, y, width, height, minDepth, maxDepth);
        }
        "#)
        .invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
            min_depth.into(),
            max_depth.into(),
        ]);
    }

    pub fn set_scissor_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        js!(r#"
        function(encoder, x, y, width, height){
            encoder.setScissorRect(x, y, width, height);
        }
        "#)
        .invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn set_vertex_buffer(&self, slot: usize, buffer: &GPUBuffer) {
        js!(r#"
        function(encoder, slot, buffer){
            encoder.setVertexBuffer(slot, buffer);
        }
        "#)
        .invoke(&[(&self.0).into(), slot.into(), (&buffer.0).into()]);
    }

    pub fn set_index_buffer(&self, buffer: &GPUBuffer, index_format: &str) {
        js!(r#"
        function(encoder, buffer, indexFormat){
            encoder.setIndexBuffer(buffer, indexFormat);
        }
        "#)
        .invoke(&[(&self.0).into(), (&buffer.0).into(), index_format.into()]);
    }

    pub fn draw_indexed(&self, index_count: usize) {
        js!(r#"
        function(encoder, indexCount){
            encoder.drawIndexed(indexCount);
        }
        "#)
        .invoke(&[(&self.0).into(), index_count.into()]);
    }

    pub fn end(&self) {
        js!(r#"
        function(encoder){
            encoder.end();
        }
        "#)
        .invoke(&[(&self.0).into()]);
    }
}
