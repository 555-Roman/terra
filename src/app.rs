use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::prelude::GlConfig;
use glutin_winit::DisplayBuilder;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::renderer::Renderer;

pub struct App<'a, T> {
    pub window: Option<Window>,
    pub renderer: Option<Renderer>,

    pub title: String,

    custom_render: unsafe fn(&mut T, &Renderer),
    user_app: &'a mut T,
}

impl<'a, T> App<'a, T> {
    pub fn new(title: &str, custom_render: unsafe fn(&mut T, &Renderer), user_app: &'a mut T) -> Self {
        Self {
            window: None,
            renderer: None,

            title: title.to_string(),
            custom_render,
            user_app,
        }
    }

    pub fn run_app(mut app: Self) -> Result<(), EventLoopError> {
        EventLoop::new()?.run_app(&mut app)?;
        Ok(())
    }

    pub fn _renderer(&self) -> &Renderer {
        self.renderer.as_ref().unwrap()
    }
}

impl<'a, T> ApplicationHandler for App<'a, T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(false);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes(self.title.clone())));
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

        self.renderer = unsafe { Some(Renderer::new(&gl_config, &window)) };
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput {event, .. } => {
                println!("Received keyboard input");

                if event.state == winit::event::ElementState::Pressed {
                    match event.logical_key.to_text() {
                        None => {}
                        Some(char) => println!("  Received key: {:?}", char)
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
                let func = self.custom_render;

                unsafe {
                    func(self.user_app, renderer);
                }

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

fn window_attributes(title: String) -> WindowAttributes {
    Window::default_attributes()
        .with_transparent(true)
        .with_title(title)
}