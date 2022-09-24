mod log;
mod renderer;
mod window;

fn main() {
    unsafe {
        let mut renderer = renderer::OpenGLRenderer::new();
        renderer.compile_shaders();
        loop {
            renderer.update();
            renderer.swap_buffers();
            renderer.poll_events();
            if renderer.should_close() {
                renderer.destroy();
                break;
            }
        }
    }
}
