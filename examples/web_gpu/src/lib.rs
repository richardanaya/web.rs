use web::*;

#[web::main]
async fn main() {
    if !WebGPU::is_available() {
        console_error("WebGPU is not available");
        return;
    }
    let adapter = WebGPU::request_adapter().await;
    let device = adapter.request_device().await;
    let canvas = query_selector("canvas");
    let preferred_canvas_texture_format = WebGPU::get_preferred_canvas_format();
    let context = GpuCanvasContext::from_element(&canvas);
    let canvas_config = GpuCanvasConfiguration::new();
    canvas_config.set_device(&device);
    canvas_config.set_format(&preferred_canvas_texture_format);
    canvas_config.set_alpha_mode(GPUCanvasAlphaMode::Opaque);
    context.configure(&canvas_config);

    let positions: Vec<f32> = vec![
        1.0, -1.0, 0.0, // bottom right
        -1.0, -1.0, 0.0, // bottom left
        0.0, 1.0, 0.0, // upper top
    ];
    let position_buffer_descriptor = GPUBufferDescriptor::new();
    position_buffer_descriptor.set_size(positions.len() * core::mem::size_of::<f32>());
    position_buffer_descriptor.set_usage(GPU_BUFFER_USAGE_VERTEX);
    position_buffer_descriptor.set_mapped_at_creation(true);
    let position_buffer = device.create_buffer(&position_buffer_descriptor);
    position_buffer.set_from_f32_array(&positions);

    let colors: Vec<f32> = vec![
        1.0, 0.0, 0.0, // 🔴
        0.0, 1.0, 0.0, // 🟢
        0.0, 0.0, 1.0, // 🔵
    ];
    let color_buffer_descriptor = GPUBufferDescriptor::new();
    color_buffer_descriptor.set_size(colors.len() * core::mem::size_of::<f32>());
    color_buffer_descriptor.set_usage(GPU_BUFFER_USAGE_VERTEX);
    color_buffer_descriptor.set_mapped_at_creation(true);
    let color_buffer = device.create_buffer(&color_buffer_descriptor);
    color_buffer.set_from_f32_array(&colors);

    let indices:Vec<u32> = vec![0, 1, 2];
    let index_buffer_descriptor = GPUBufferDescriptor::new();
    index_buffer_descriptor.set_size(indices.len() * core::mem::size_of::<u32>());
    index_buffer_descriptor.set_usage(GPU_BUFFER_USAGE_INDEX);
    index_buffer_descriptor.set_mapped_at_creation(true);
    let index_buffer = device.create_buffer(&index_buffer_descriptor);
    index_buffer.set_from_u32_array(&indices);
}
