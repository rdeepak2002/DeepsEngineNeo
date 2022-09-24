#[cfg(feature = "sdl2")]
pub(crate) struct SDL2Window {
    gl_context: sdl2::video::GLContext,
    window: sdl2::video::Window,
    events_loop: sdl2::EventPump,
    should_close: bool,
}

#[cfg(feature = "sdl2")]
impl SDL2Window {
    pub fn new(
        gl_context: sdl2::video::GLContext,
        window: sdl2::video::Window,
        events_loop: sdl2::EventPump,
    ) -> Self {
        Self {
            gl_context,
            window,
            events_loop,
            should_close: false,
        }
    }

    pub fn swap_buffers(&self) {
        self.window.gl_swap_window();
    }

    pub fn poll_events(&mut self) {
        for event in self.events_loop.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => self.should_close = true,
                _ => {}
            }
        }
    }

    pub fn should_close(&self) -> bool {
        return self.should_close;
    }
}
