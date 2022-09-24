use glow::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// pub struct WebGlContexts {
//     contexts: Vec<web_sys::WebGl2RenderingContext>,
// }
//
// unsafe impl Send for WebGlContexts {}
// unsafe impl Sync for WebGlContexts {}
//
// lazy_static! {
//     static ref TMP: WebGlContexts = WebGlContexts {
//         contexts: Vec::new(),
//     };
// }

#[cfg(feature = "sdl2")]
pub(crate) fn create_sdl2_context() -> (
    Context,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
) {
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

// #[cfg(target_arch = "wasm32")]
// pub(crate) fn create_webgl_context() -> (Context) {
//     use wasm_bindgen::JsCast;
//     let canvas = web_sys::window()
//         .unwrap()
//         .document()
//         .unwrap()
//         .get_element_by_id("canvas")
//         .unwrap()
//         .dyn_into::<web_sys::HtmlCanvasElement>()
//         .unwrap();
//     let webgl2_context = canvas
//         .get_context("webgl2")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<web_sys::WebGl2RenderingContext>()
//         .unwrap();
//     let gl = glow::Context::from_webgl2_context(webgl2_context);
//
//     return gl;
// }

pub struct OpenGLRenderer {
    gl: Context,
    #[cfg(feature = "sdl2")]
    sdl2_gl_context: Option<sdl2::video::GLContext>,
    #[cfg(feature = "sdl2")]
    window: sdl2::video::Window,
    #[cfg(feature = "sdl2")]
    events_loop: sdl2::EventPump,
    program: Option<glow::Program>,
    vertex_array: Option<glow::VertexArray>,
}

impl OpenGLRenderer {
    pub fn new(gl: Context) -> Self {
        #[cfg(feature = "sdl2")]
        {
            let (gl, window, events_loop, gl_context) = create_sdl2_context();
            Self {
                gl,
                sdl2_gl_context: Some(gl_context),
                window,
                events_loop,
                program: None,
                vertex_array: None,
            }
        }

        // #[cfg(target_arch = "wasm32")]
        {
            log("Flag C");

            // let gl = create_webgl_context();
            // let gl = glow::Context::from_webgl2_context(webgl2_context);
            // let gl = std::rc::Rc::new(gl);

            Self {
                gl,
                program: None,
                vertex_array: None,
            }
        }
    }

    pub unsafe fn init(&mut self) {
        log("Flag D");

        println!("gl {}.{}", self.gl.version().major, self.gl.version().minor);

        log("Flag E");

        let vertex_array = self
            .gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        self.gl.bind_vertex_array(Some(vertex_array));
        self.vertex_array = Some(vertex_array);
        let program = self.gl.create_program().expect("Cannot create program");
        self.program = Some(program);

        log("Flag F");

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

        log("Flag G");

        let mut shader_version = "#version 330";

        #[cfg(target_arch = "wasm32")]
        {
            shader_version = "#version 300 es";
        }

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = self
                .gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            self.gl
                .shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
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

        log("Flag H");
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
