mod renderer;
use crate::renderer::*;
use glow::*;

fn main() {
    unsafe {
        let mut renderer = OpenGLRenderer::new();
        renderer.init();

        // while !renderer.should_close() {
        //     renderer.update();
        //     renderer.swap_buffers();
        // }

        while {
            renderer.update();
            renderer.swap_buffers();
            !renderer.should_close()
        } {}

        renderer.destroy();
    }
}
