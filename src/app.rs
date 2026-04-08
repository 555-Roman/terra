use std::rc::Rc;
use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::prelude::GlConfig;
use glutin_winit::DisplayBuilder;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalSize, Size};
use winit::error::EventLoopError;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::Key;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::renderer::Renderer;

pub trait Rendering {
    fn init(&mut self, renderer: Rc<Renderer>);
    fn render(&self, renderer: Rc<Renderer>);
    fn drop(&mut self, renderer: Rc<Renderer>);
}

pub struct App<'a, T: Rendering> {
    pub window: Option<Window>,
    pub renderer: Option<Rc<Renderer>>,

    pub title: String,
    pub width: i32,
    pub height: i32,

    user_app: &'a mut T,
}
impl<'a, T: Rendering> Drop for App<'a, T> {
    fn drop(&mut self) {
        self.user_app.drop(Rc::clone(self.renderer.as_ref().unwrap()));
    }
}

impl<'a, T: Rendering> App<'a, T> {
    pub fn new(title: &str, width: i32, height: i32, user_app: &'a mut T) -> Self {
        Self {
            window: None,
            renderer: None,

            title: title.to_string(),
            width,
            height,
            user_app,
        }
    }

    pub fn run_app(mut app: Self) -> Result<(), EventLoopError> {
        EventLoop::new()?.run_app(&mut app)?;
        Ok(())
    }
}

impl<'a, T: Rendering> ApplicationHandler for App<'a, T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(false);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes(self.title.clone(), self.width, self.height)));
        let (window, gl_config) = match display_builder.build(
            event_loop,
            template,
            gl_config_picker,
        ) {
            Ok((window, gl_config)) => (window.unwrap(), gl_config),
            Err(_err) => {
                event_loop.exit();
                return;
            },
        };

        self.renderer = unsafe { Some(Rc::new(Renderer::new(&gl_config, &window))) };
        self.window = Some(window);

        self.user_app.init(Rc::clone(self.renderer.as_ref().unwrap()));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput {event, .. } => {
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.logical_key {
                        Key::Named(winit::keyboard::NamedKey::Escape) => {
                            event_loop.exit();
                        }
                        _ => (),
                    }
                }
            },
            WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
                let renderer = self.renderer.as_ref().unwrap();
                renderer.resize(size);
            },
            WindowEvent::RedrawRequested => {
                let renderer = self.renderer.as_ref().unwrap();
                let window = self.window.as_ref().unwrap();

                self.user_app.render(Rc::clone(renderer));

                window.request_redraw();
            },
            _ => (),
        }
    }
}

fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

fn window_attributes(title: String, width: i32, height: i32) -> WindowAttributes {
    Window::default_attributes()
        .with_transparent(true)
        .with_title(title)
        .with_inner_size(Size::new(PhysicalSize::new(width, height)))
}