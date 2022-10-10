extern crate gl;
extern crate sdl2;

use std::process;

mod log;
mod project;
mod renderer;
mod shader;
mod window;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

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
