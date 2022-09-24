mod log;
mod renderer;
mod window;

fn main() {
    unsafe {
        let mut renderer = renderer::OpenGLRenderer::new();
        renderer.init();

        loop {
            renderer.update();
            renderer.swap_buffers();
            if renderer.should_close() {
                renderer.destroy();
                break;
            }
        }
    }
}
