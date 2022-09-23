mod renderer;
use crate::renderer::*;
use glow::*;

fn main() {
    unsafe {
        // let (gl, window, events_loop) = renderer::create_sdl2_context();

        // We handle events differently between targets
        #[cfg(feature = "sdl2")]
        {
            let mut renderer = OpenGLRenderer::new();
            renderer.init();

            // while !renderer.should_close() {
            //     renderer.update();
            //     renderer.swap_buffers();
            // }
            //
            // renderer.destroy();
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
