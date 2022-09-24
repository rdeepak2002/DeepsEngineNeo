mod renderer;

use crate::renderer::*;

fn main() {
    unsafe {
        let mut renderer = OpenGLRenderer::new();
        renderer.init();

        while !renderer.should_close() {
            renderer.update();
            renderer.swap_buffers();

            // TODO: call update from request animation frame
            #[cfg(target_arch = "wasm32")]
            {
                break;
            }
        }
    }
}
