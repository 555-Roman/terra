use std::num::NonZeroU32;
use std::process::abort;
use glow::{Context, HasContext, COLOR_BUFFER_BIT};
use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay};
use glutin::prelude::{GlConfig, GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext};
use glutin::surface::{GlSurface, Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use log::error;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::{HasWindowHandle, RawDisplayHandle}; // cant u just "use *" e it no work
use winit::window::{Window, WindowAttributes, WindowId};

struct App {
    window: Option<Window>,
    gl_surface: Option<Surface<WindowSurface>>,
    gl_context: Option<PossiblyCurrentContext>,
    gl: Option<Context>
}

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
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

fn window_attributes() -> WindowAttributes {
    Window::default_attributes()
        .with_transparent(true)
        .with_title("eeeeee")
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Resumed!");

        let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(false);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));
        let (window, gl_config) = match display_builder.build(
            event_loop,
            template,
            gl_config_picker,
        ) {
            Ok((window, gl_config)) => (window.unwrap(), gl_config),
            Err(err) => {
                event_loop.exit();
                return;
            },
        };

        let raw_window_handle = window.window_handle().unwrap_or_else(|err| {
            error!("why tf did that not work?? {:?}", err);
            abort()
        }).as_raw();

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
        let gl_display = gl_config.display();

        unsafe {
            let gl_context = gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                error!("ur device is too old noob, skill issue");
                abort()
            }).treat_as_possibly_current();

            let attrs = window
                .build_surface_attributes(Default::default())
                .unwrap_or_else(|_| {
                    error!("ur hardware dont work or smth prob too old skill issue");
                    abort()
                });
            let gl_surface = gl_config.display().create_window_surface(&gl_config, &attrs).unwrap();

            gl_context.make_current(&gl_surface).unwrap();

            let gl = Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));

            gl_surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();

            self.gl_surface = Some(gl_surface);
            self.gl_context = Some(gl_context);
            self.gl = Some(gl);
        }

        self.window = Some(window);


    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
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
                let width = unsafe { NonZeroU32::new_unchecked(size.width) };
                let height = unsafe { NonZeroU32::new_unchecked(size.height) };

                let context = self.gl_context.as_ref().unwrap();
                let surface = self.gl_surface.as_ref().unwrap();

                surface.resize(context, width, height);
            },
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let context = self.gl_context.as_ref().unwrap();
                let surface = self.gl_surface.as_ref().unwrap();
                let gl = self.gl.as_ref().unwrap();

                unsafe {
                    gl.clear_color(0.0, 1.0, 1.0, 1.0);
                    gl.clear(COLOR_BUFFER_BIT);
                }

                window.request_redraw();
                surface.swap_buffers(context).unwrap();
            },
            _ => (),
        }
    }
}

fn main() {
    println!("Hello, world!");

    pretty_env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App {
        window: None,
        gl_surface: None,
        gl_context: None,
        gl: None,
    };

    event_loop.run_app(&mut app).unwrap();
}