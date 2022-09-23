mod renderer;
use crate::renderer::OpenGLRenderer;
use crate::renderer::Renderer;
use glow::*;

fn main() {
    let mut renderer = OpenGLRenderer::new();

    unsafe {
        // We handle events differently between targets
        #[cfg(feature = "sdl2")]
        {
            while !renderer.should_close() {
                renderer.update();
                renderer.swap_buffers();
            }

            renderer.destroy();
        }

        #[cfg(target_arch = "wasm32")]
        {
            // This could be called from `requestAnimationFrame`, a winit event
            // loop, etc.
            renderer.update();
            renderer.destroy();
        }
    }
}
