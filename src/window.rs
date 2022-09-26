pub(crate) trait Window {
    fn swap_buffers(&self);
    fn poll_events(&mut self);
    fn should_close(&self) -> bool;
}

pub(crate) struct SDL2Window {
    pub gl_context: sdl2::video::GLContext,
    pub window: sdl2::video::Window,
    pub events_loop: sdl2::EventPump,
    pub should_close: bool,
}

impl Window for SDL2Window {
    fn swap_buffers(&self) {
        self.window.gl_swap_window();
    }

    fn poll_events(&mut self) {
        for event in self.events_loop.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => self.should_close = true,
                _ => {}
            }
        }
    }

    fn should_close(&self) -> bool {
        return self.should_close;
    }
}

pub(crate) fn create_sdl2_window() -> Box<dyn crate::window::Window> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    if cfg!(target_os = "emscripten") {
        gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    } else {
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    }
    gl_attr.set_context_version(3, 0);
    let window = video
        .window("DeepsEngine", 1024, 769)
        .opengl()
        .allow_highdpi()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let events_loop = sdl.event_pump().unwrap();
    let sdl_window = crate::window::SDL2Window {
        gl_context,
        window,
        events_loop,
        should_close: false,
    };
    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);
    return Box::new(sdl_window);
}
