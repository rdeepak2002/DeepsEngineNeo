use glow::*;
use sdl2::video::{GLContext, Window};
use sdl2::EventPump;

#[cfg(feature = "sdl2")]
pub(crate) fn create_sdl2_context() -> (Context, Window, EventPump, GLContext) {
    // Create a context from a sdl2 window
    unsafe {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 0);
        let window = video
            .window("Hello triangle!", 1024, 769)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        let event_loop = sdl.event_pump().unwrap();
        return (gl, window, event_loop, gl_context);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn create_webgl_context() -> (Context, web_sys::WebGl2RenderingContext) {
    use wasm_bindgen::JsCast;
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    let gl = glow::Context::from_webgl2_context(webgl2_context);
    return (gl, webgl2_context);
}

pub struct OpenGLRenderer {
    gl: Context,
    sdl2_gl_context: Option<GLContext>,
    window: Window,
    events_loop: EventPump,
    web_gl_context: Option<web_sys::WebGl2RenderingContext>,
    program: Option<NativeProgram>,
    vertex_array: Option<VertexArray>,
}

impl OpenGLRenderer {
    pub fn new() -> Self {
        #[cfg(feature = "sdl2")]
        {
            let (gl, mut window, mut events_loop, gl_context) = create_sdl2_context();

            Self {
                gl,
                sdl2_gl_context: Some(gl_context),
                window,
                events_loop,
                web_gl_context: None,
                program: None,
                vertex_array: None,
            }
        }

        #[cfg(feature = "wasm32")]
        {
            let (gl, gl_context) = create_webgl_context();

            Self {
                gl,
                sdl2_gl_context: None,
                window,
                events_loop: events_loop,
                web_gl_context: Some(gl_context),
                program: None,
                vertex_array: None,
            }
        }
    }

    pub unsafe fn init(&mut self) {
        println!("gl {}.{}", self.gl.version().major, self.gl.version().minor);

        let vertex_array = self
            .gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        self.gl.bind_vertex_array(Some(vertex_array));
        self.vertex_array = Some(vertex_array);
        let program = self.gl.create_program().expect("Cannot create program");
        self.program = Some(program);

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#,
            r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = self
                .gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            self.gl
                .shader_source(shader, &format!("{}\n{}", "#version 330", shader_source));
            self.gl.compile_shader(shader);
            if !self.gl.get_shader_compile_status(shader) {
                panic!("{}", self.gl.get_shader_info_log(shader));
            }
            self.gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        self.gl.link_program(program);
        if !self.gl.get_program_link_status(program) {
            panic!("{}", self.gl.get_program_info_log(program));
        }

        for shader in shaders {
            self.gl.detach_shader(program, shader);
            self.gl.delete_shader(shader);
        }

        self.gl.use_program(Some(program));
        self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }

    pub unsafe fn update(&self) {
        self.gl.clear(glow::COLOR_BUFFER_BIT);
        self.gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }

    pub unsafe fn destroy(&self) {
        match self.program {
            Some(x) => self.gl.delete_program(x),
            _ => {}
        }

        match self.vertex_array {
            Some(x) => self.gl.delete_vertex_array(x),
            _ => {}
        }
    }

    pub unsafe fn swap_buffers(&self) {
        #[cfg(feature = "sdl2")]
        {
            self.window.gl_swap_window();
        }
    }

    pub fn should_close(&mut self) -> bool {
        #[cfg(feature = "sdl2")]
        {
            for event in self.events_loop.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => return true,
                    _ => {}
                }
            }
        }

        #[cfg(feature = "wasm32")]
        {
            return true;
        }

        return false;
    }
}
