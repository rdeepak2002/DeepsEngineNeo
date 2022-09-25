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

        let mut main_loop = || {
            renderer.compile_shaders();
            renderer.update();
            renderer.swap_buffers();
            renderer.poll_events();
            if renderer.should_close() {
                renderer.destroy();
                process::exit(EXIT_SUCCESS);
            }
        };

        #[cfg(target_os = "emscripten")]
        {
            use emscripten::emscripten;
            emscripten::set_main_loop_callback(main_loop);
        }

        #[cfg(not(target_os = "emscripten"))]
        loop {
            main_loop();
        }
    }
}
