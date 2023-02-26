pub struct Window {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    ctx: sdl2::video::GLContext,
    event_pump: sdl2::EventPump,

    pub should_close: bool
}

impl Window {
    pub fn new(size: super::Size, title: &str) -> Self {
        let sdl = sdl2::init().unwrap();
        sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");
        let video = sdl.video().unwrap();

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 3);

        let window = video.window(title, size.width as u32, size.height as u32)
            .opengl()
            .position_centered()
            .build()
            .unwrap();
        
        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

        let event_pump = sdl.event_pump().unwrap();

        Self {
            sdl,
            video,
            window,
            ctx,
            event_pump,
            should_close: false,
        }
    }

    pub fn process_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => self.should_close = true,
                _ => {}
            }
        }
    }

    pub fn swap_buffers(&mut self, interval: i32) {
        self.video.gl_set_swap_interval(interval).unwrap();
        self.window.gl_swap_window();
    }
}