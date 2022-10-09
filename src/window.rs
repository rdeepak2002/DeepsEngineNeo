// use egui::Checkbox;
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, gl, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use std::time::Instant;
// Alias the backend to something less mouthful
use crate::log;
use egui_sdl2_gl as egui_backend;
use egui_sdl2_gl::egui::CtxRef;
use egui_sdl2_gl::painter::Painter;
use egui_sdl2_gl::EguiStateHandler;
use sdl2::video::SwapInterval;

pub(crate) trait Window {
    fn swap_buffers(&self);
    fn poll_events(&mut self);
    fn should_close(&self) -> bool;
    fn update_editor(&mut self);
}

pub(crate) struct SDL2Window {
    pub gl_context: sdl2::video::GLContext,
    pub window: sdl2::video::Window,
    pub events_loop: sdl2::EventPump,
    pub should_close: bool,
    pub egui_painter: Painter,
    pub egui_state: EguiStateHandler,
    pub egui_ctx: CtxRef,
    pub start_time: Instant,
}

impl Window for SDL2Window {
    fn swap_buffers(&self) {
        self.window.gl_swap_window();
    }

    fn poll_events(&mut self) {
        for event in self.events_loop.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => self.should_close = true,
                _ => {
                    self.egui_state
                        .process_input(&self.window, event, &mut self.egui_painter);
                }
            }
        }
    }

    fn should_close(&self) -> bool {
        return self.should_close;
    }

    fn update_editor(&mut self) {
        self.egui_state.input.time = Some(self.start_time.elapsed().as_secs_f64());
        self.egui_ctx.begin_frame(self.egui_state.input.take());

        egui::CentralPanel::default().show(&self.egui_ctx, |ui| {
            ui.label(" ");
            let mut test_str: String =
                "A text box to write in. Cut, copy, paste commands are available.".to_owned();
            ui.text_edit_multiline(&mut test_str);
        });

        let (egui_output, paint_cmds) = self.egui_ctx.end_frame();

        // Process ouput
        self.egui_state.process_output(&self.window, &egui_output);

        // if self.egui_ctx.used_size() != self.egui_painter.screen_rect.size() {
        //     let _size = self.egui_ctx.used_size();
        //     let (w, h) = (_size.x as u32, _size.y as u32);
        //     self.window.set_size(w, h).unwrap();
        // }

        let paint_jobs = self.egui_ctx.tessellate(paint_cmds);

        self.egui_painter
            .paint_jobs(None, paint_jobs, &self.egui_ctx.font_image());
    }
}

pub(crate) fn create_sdl2_window() -> Box<dyn crate::window::Window> {
    // create sdl context
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
        // .allow_highdpi()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let events_loop = sdl.event_pump().unwrap();

    // load egui context
    let mut shader_ver = ShaderVersion::Default;
    if cfg!(target_os = "emscripten") {
        shader_ver = ShaderVersion::Adaptive;
    }
    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&window, shader_ver, DpiScaling::Custom(2.0));
    let mut egui_ctx = egui::CtxRef::default();

    let start_time = Instant::now();

    // create sdl window
    let sdl_window = crate::window::SDL2Window {
        egui_painter: painter,
        egui_state,
        egui_ctx,
        gl_context,
        window,
        events_loop,
        should_close: false,
        start_time,
    };
    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);
    return Box::new(sdl_window);
}
