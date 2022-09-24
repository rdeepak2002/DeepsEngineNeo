use glow::HasContext;

#[cfg(feature = "sdl2")]
pub(crate) fn create_sdl2_context() -> (glow::Context, crate::window::SDL2Window) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
    let window = video
        .window("DeepsEngine", 1024, 769)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    unsafe {
        let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        let event_loop = sdl.event_pump().unwrap();
        let sdl_window = crate::window::SDL2Window::new(gl_context, window, event_loop);
        return (gl, sdl_window);
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn create_webgl_context() -> glow::Context {
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
    return gl;
}

// TODO: create general Renderer interface
pub struct OpenGLRenderer {
    gl: glow::Context,
    #[cfg(feature = "sdl2")]
    sdl2_window: crate::window::SDL2Window,
    program: Option<glow::Program>,
    vertex_array: Option<glow::VertexArray>,
}

impl OpenGLRenderer {
    pub fn new() -> Self {
        #[cfg(feature = "sdl2")]
        {
            let (gl, sdl2_window) = create_sdl2_context();

            Self {
                gl,
                sdl2_window,
                program: None,
                vertex_array: None,
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let gl = create_webgl_context();

            Self {
                gl,
                program: None,
                vertex_array: None,
            }
        }
    }

    pub unsafe fn init(&mut self) {
        // TODO: create logging system
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
            self.gl.shader_source(
                shader,
                &format!("{}\n{}", self.get_glsl_version(), shader_source),
            );
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

    pub fn swap_buffers(&self) {
        #[cfg(feature = "sdl2")]
        {
            self.sdl2_window.swap_buffers();
        }
    }

    pub fn should_close(&mut self) -> bool {
        #[cfg(feature = "sdl2")]
        {
            // TODO: move poll events call elsewhere
            self.sdl2_window.poll_events();
            return self.sdl2_window.should_close();
        }

        #[cfg(target_arch = "wasm32")]
        {
            return true;
        }
    }

    fn get_glsl_version(&self) -> &str {
        if cfg!(target_arch = "wasm32") {
            return "#version 300 es";
        } else if cfg!(feature = "sdl2") {
            return "#version 330";
        } else {
            panic!("Unable to determine shader version");
        }
    }
}
