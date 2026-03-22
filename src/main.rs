use winit;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Resumed!");

        self.window = Some(event_loop.create_window(WindowAttributes::default()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        println!("Event!");

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::KeyboardInput {device_id, event, is_synthetic} => {
                println!("Received keyboard input");

                if event.state == winit::event::ElementState::Pressed {
                    let char = event.logical_key.to_text().unwrap();
                    println!("  Received key: {:?}", char);
                }
            }
            _ => (),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let event_loop = EventLoop::new().unwrap();

    let mut app = App {
        window: None
    };

    event_loop.run_app(&mut app).unwrap();
}