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
    let preferred_format = WebGPU::get_preferred_canvas_format();
    console_log(&format!("Preferred format: {:?}", preferred_format));
    let context = GpuCanvasContext::from_element(&canvas);
}
