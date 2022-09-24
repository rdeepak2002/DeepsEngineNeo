mod log;
mod renderer;
mod window;

extern crate strum;
#[macro_use]
extern crate strum_macros;

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
