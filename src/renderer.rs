use gl::types::*;

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::{mem, process};

pub struct OpenGLRenderer {
    window: Box<dyn crate::window::Window>,
    VAO: u32,
    shaderProgram: u32,
}

const vertexShaderSource: &str = r##"
layout (location = 0) in vec3 aPos;
void main() {
    gl_Position = vec4(aPos, 1.0);
}"##;

const fragmentShaderSource: &str = r##"
precision highp float;
out vec4 FragColor;
void main() {
    FragColor = vec4(1.0, 0.5, 0.2, 1.0);
}"##;

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

        Self {
            window,
            VAO: 0,
            shaderProgram: 0,
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
        // build and compile our shader program
        // ------------------------------------
        // vertex shader
        let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(
            format!("{}\n{}", self.get_glsl_version(), vertexShaderSource)
                .as_str()
                .as_bytes(),
        )
        .unwrap();
        gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertexShader);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut infoLog = Vec::with_capacity(512);
        infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                vertexShader,
                512,
                ptr::null_mut(),
                infoLog.as_mut_ptr() as *mut GLchar,
            );
            let error_text = format!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&infoLog).unwrap()
            );
            crate::log::error(error_text.as_str());
            process::exit(crate::EXIT_FAILURE)
        }

        // fragment shader
        let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(
            format!("{}\n{}", self.get_glsl_version(), fragmentShaderSource)
                .as_str()
                .as_bytes(),
        )
        .unwrap();
        gl::ShaderSource(fragmentShader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragmentShader);
        // check for shader compile errors
        gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                fragmentShader,
                512,
                ptr::null_mut(),
                infoLog.as_mut_ptr() as *mut GLchar,
            );
            let error_text = format!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&infoLog).unwrap()
            );
            crate::log::error(error_text.as_str());
            process::exit(crate::EXIT_FAILURE)
        }

        // link shaders
        self.shaderProgram = gl::CreateProgram();
        gl::AttachShader(self.shaderProgram, vertexShader);
        gl::AttachShader(self.shaderProgram, fragmentShader);
        gl::LinkProgram(self.shaderProgram);
        // check for linking errors
        gl::GetProgramiv(self.shaderProgram, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                self.shaderProgram,
                512,
                ptr::null_mut(),
                infoLog.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                str::from_utf8(&infoLog).unwrap()
            );
        }
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(fragmentShader);

        gl::UseProgram(self.shaderProgram);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];
        let mut VBO = 0;
        gl::GenVertexArrays(1, &mut self.VAO);
        gl::GenBuffers(1, &mut VBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(self.VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
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

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);
    }

    pub unsafe fn render(&self) {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // crate::log::debug("Clear screen");

        // draw our first triangle
        gl::UseProgram(self.shaderProgram);
        // crate::log::debug("Using shader program");
        gl::BindVertexArray(self.VAO); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
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

    fn get_glsl_version(&self) -> &str {
        return if cfg!(target_os = "emscripten") {
            "#version 300 es"
        } else {
            "#version 330 core"
        };
    }
}
