use tesseract_engine::gpu_canvas::commands::{GpuCommand, Layer};
use tesseract_engine::gpu_canvas::compositor::Compositor;
use tesseract_engine::gpu_canvas::renderer::GpuRenderer;
use tesseract_engine::gpu_canvas::GpuCanvas;

fn main() {
    let canvas = GpuCanvas::new();
    println!("GPU available: {}", canvas.is_gpu_available());

    // Render a simple rectangle
    let cmd = GpuCommand::render_rect(0, 0, 800, 600, 64, 128, 192, 255);
    match canvas.execute_command(&cmd) {
        Ok(output) => {
            println!(
                "Rendered {}x{} ({} bytes, GPU: {})",
                output.width,
                output.height,
                output.pixels.len(),
                output.gpu,
            );
        }
        Err(e) => {
            eprintln!("Render failed: {e}");
        }
    }

    // Compositor example
    let mut compositor = Compositor::new();
    compositor.add_layer(Layer {
        z_index: 0,
        opacity: 1.0,
        command: GpuCommand::Clear { r: 255, g: 255, b: 255, a: 255 },
    });
    compositor.add_layer(Layer {
        z_index: 1,
        opacity: 0.8,
        command: GpuCommand::render_rect(100, 100, 400, 300, 255, 0, 0, 200),
    });

    let renderer = GpuRenderer::new();
    match compositor.compose(&renderer, 1024, 768) {
        Ok(framebuffer) => {
            println!("Composed 1024x768 framebuffer: {} bytes", framebuffer.len());
        }
        Err(e) => {
            eprintln!("Composition failed: {e}");
        }
    }
}
