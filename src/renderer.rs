use gl::types::*;

// use egui::Checkbox;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
// Alias the backend to something less mouthful
use crate::shader::Shader;

pub struct OpenGLRenderer {
    window: Box<dyn crate::window::Window>,
    vao: u32,
    shader_program: Shader,
}

impl emscripten_main_loop::MainLoop for OpenGLRenderer {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        return if self.update() {
            emscripten_main_loop::MainLoopEvent::Terminate
        } else {
            emscripten_main_loop::MainLoopEvent::Continue
        };
    }
}

impl OpenGLRenderer {
    pub fn new() -> Self {
        let window = crate::window::create_sdl2_window();

        // println!("{}", std::env::current_dir());
        // p.into_os_string().into_string()

        // let current_path_ostr = std::env::current_dir().unwrap().into_os_string();
        // let blank_project_path = Path::new(current_path_ostr.to_str().unwrap()).join("examples");
        println!(
            "{}",
            crate::project::blank_project_path()
                .to_path_buf()
                .to_str()
                .unwrap()
        );

        Self {
            window,
            vao: 0,
            shader_program: Shader::new(
                crate::project::blank_project_path()
                    .join("shader")
                    .join("shader.vert")
                    .to_path_buf()
                    .to_str()
                    .unwrap(),
                crate::project::blank_project_path()
                    .join("shader")
                    .join("shader.frag")
                    .to_path_buf()
                    .to_str()
                    .unwrap(),
            ),
        }
    }

    pub(crate) fn update(&mut self) -> bool {
        unsafe {
            self.render();
            self.swap_buffers();
            self.poll_events();
            if self.should_close() {
                self.destroy();
                return true;
            }
        }
        return false;
    }

    pub unsafe fn compile_shaders(&mut self) {
        self.shader_program.useProgram();

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];
        let mut vbo = 0;
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut vbo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(self.vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);
    }

    pub unsafe fn render(&mut self) {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        self.window.update_editor();

        // crate::log::debug("Clear screen");

        // draw our first triangle
        self.shader_program.useProgram();
        // crate::log::debug("Using shader program");
        gl::BindVertexArray(self.vao); // seeing as we only have a single vao there's no need to bind it every time, but we'll do so to keep things a bit more organized
                                       // crate::log::debug("Binded vertex array");
        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        // crate::log::debug("Drew arrays");
    }

    pub unsafe fn destroy(&self) {}

    pub fn swap_buffers(&self) {
        self.window.swap_buffers();
    }

    pub fn poll_events(&mut self) {
        self.window.poll_events();
    }

    pub fn should_close(&self) -> bool {
        return self.window.should_close();
    }
}
