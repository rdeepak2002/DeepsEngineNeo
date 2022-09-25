extern crate gl;
extern crate sdl2;

use sdl2::libc::EXIT_SUCCESS;
use std::process;

mod log;
mod renderer;
mod window;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {
    unsafe {
        let mut renderer = renderer::OpenGLRenderer::new();
        renderer.compile_shaders();

        #[cfg(target_os = "emscripten")]
        {
            use emscripten::emscripten;
            emscripten_main_loop::run(renderer);
        }

        #[cfg(not(target_os = "emscripten"))]
        loop {
            renderer.update();
            renderer.swap_buffers();
            renderer.poll_events();
            if renderer.should_close() {
                renderer.destroy();
                process::exit(EXIT_SUCCESS);
            }
        }
    }
}
