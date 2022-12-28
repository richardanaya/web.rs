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
}
