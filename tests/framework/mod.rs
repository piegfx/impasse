use self::{window::Window, pie::GraphicsDevice};

pub mod window;
pub mod pie;

pub trait Demo {
    fn init(&mut self, framework: &mut Framework);
    fn update(&mut self, framework: &mut Framework);
    fn draw(&mut self, framework: &mut Framework);
}

pub struct Framework {
    pub window: Window,
    pub graphics: GraphicsDevice,
    pub clear_color: Color
}

impl Framework {
    pub fn new() -> Self {
        let window = Window::new(Size::new(1280, 720), "Test");
        let graphics = GraphicsDevice {};
        let clear_color = Color::from_rgba_f32(0.0, 0.0, 0.0, 1.0);

        Self {
            window,
            graphics,
            clear_color
        }
    }

    pub fn update(&mut self) {

    }

    pub fn draw(&mut self) {
        self.graphics.clear(self.clear_color);
    }
}

pub struct FrameworkRunner<'a> {
    demo: &'a mut dyn Demo,
    framework: Framework
}

impl<'a> FrameworkRunner<'a> {
    pub fn new(demo: &'a mut dyn Demo) -> Self {
        let framework = Framework::new();

        Self {
            demo,
            framework
        }
    }

    pub fn run(&mut self) {
        self.demo.init(&mut self.framework);

        while !self.framework.window.should_close {
            self.framework.window.process_events();

            self.demo.update(&mut self.framework);
            self.demo.draw(&mut self.framework);

            self.framework.window.swap_buffers(1);
        }
    }
}

pub struct Size {
    pub width:  i32,
    pub height: i32
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn from_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }

    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_rgba_f32(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
    }
}