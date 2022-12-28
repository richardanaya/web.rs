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
    context.configure(&GpuCanvasConfiguration {
        device: &device,
        format: preferred_canvas_texture_format,
        alpha_mode: GPUCanvasAlphaMode::Opaque,
    });

    // define geometry of a triangle
    let positions: Vec<f32> = vec![
        1.0, -1.0, 0.0, // bottom right
        -1.0, -1.0, 0.0, // bottom left
        0.0, 1.0, 0.0, // upper top
    ];
    let position_buffer = device.create_buffer(&GPUBufferDescriptor {
        size: positions.len() * core::mem::size_of::<f32>(),
        usage: GPU_BUFFER_USAGE_VERTEX,
        mapped_at_creation: true,
    });
    position_buffer.set_from_f32_array(&positions);
    position_buffer.unmap();

    let colors: Vec<f32> = vec![
        1.0, 0.0, 0.0, // 🔴
        0.0, 1.0, 0.0, // 🟢
        0.0, 0.0, 1.0, // 🔵
    ];
    let color_buffer = device.create_buffer(&GPUBufferDescriptor {
        size: colors.len() * core::mem::size_of::<f32>(),
        usage: GPU_BUFFER_USAGE_VERTEX,
        mapped_at_creation: true,
    });
    color_buffer.set_from_f32_array(&colors);
    color_buffer.unmap();

    let indices: Vec<u32> = vec![0, 1, 2];
    let index_buffer = device.create_buffer(&GPUBufferDescriptor {
        size: indices.len() * core::mem::size_of::<u32>(),
        usage: GPU_BUFFER_USAGE_INDEX,
        mapped_at_creation: true,
    });
    index_buffer.set_from_u32_array(&indices);
    index_buffer.unmap();

    let vertex_module = device.create_shader_module_from_source(include_str!("vertex.wgsl"));
    let fragment_module = device.create_shader_module_from_source(include_str!("fragment.wgsl"));

    let pipeline_layout = device.create_pipeline_layout(&GPUPipelineLayoutDescriptor {
        bind_group_layouts: vec![],
    });

    // create render pipeline
    let pipeline = device.create_render_pipeline(&GPURenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex: GPUVertexState {
            module: &vertex_module,
            entry_point: "main",
            buffers: vec![
                GPUVertexBufferLayout {
                    array_stride: 3 * core::mem::size_of::<f32>(),
                    step_mode: GPUInputStepMode::Vertex,
                    attributes: vec![GPUVertexAttribute {
                        format: GPUVertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    }],
                },
                GPUVertexBufferLayout {
                    array_stride: 3 * core::mem::size_of::<f32>(),
                    step_mode: GPUInputStepMode::Vertex,
                    attributes: vec![GPUVertexAttribute {
                        format: GPUVertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 1,
                    }],
                },
            ],
        },
        fragment: GPUFragmentState {
            module: &fragment_module,
            entry_point: "main",
            targets: vec![GPUColorTargetState {
                format: preferred_canvas_texture_format,
            }],
        },
        primitive: GPUPrimitiveState {
            topology: GPUPrimitiveTopology::TriangleList,
            front_face: GPUFrontFace::CW,
            cull_mode: GPUCullMode::None,
        },
    });

    let queue = device.get_queue();

    loop {
        let command_encoder = device.create_command_encoder();

        wait_til_animation_frame().await;
    }
}
