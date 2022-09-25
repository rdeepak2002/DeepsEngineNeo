extern crate gl;
extern crate sdl2;

use sdl2::libc::EXIT_SUCCESS;
use std::process;

mod log;
mod renderer;
mod window;

fn main() {
    unsafe {
        log::debug("DeepsEngine Version 0.1");

        let mut renderer = renderer::OpenGLRenderer::new();
        renderer.compile_shaders();

        #[cfg(target_os = "emscripten")]
        {
            emscripten_main_loop::run(renderer);
        }

        #[cfg(not(target_os = "emscripten"))]
        loop {
            if renderer.update() {
                break;
            }
        }

        process::exit(EXIT_SUCCESS);
    }
}
