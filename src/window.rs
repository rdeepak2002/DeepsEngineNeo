pub(crate) trait Window {
    fn swap_buffers(&self);
    fn poll_events(&mut self);
    fn should_close(&self) -> bool;
}

// #[cfg(target_arch = "wasm32")]
// pub(crate) struct WebGL2Window {}
//
// #[cfg(target_arch = "wasm32")]
// impl Window for WebGL2Window {
//     fn swap_buffers(&self) {}
//
//     fn poll_events(&mut self) {}
//
//     fn should_close(&self) -> bool {
//         return true;
//     }
// }

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
