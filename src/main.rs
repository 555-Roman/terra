mod app;

use glow::{Context, HasContext, COLOR_BUFFER_BIT};
use log::error;
use crate::app::App;

unsafe fn my_render(gl: &Context) { unsafe {
    gl.clear_color(0.0, 1.0, 1.0, 1.0);
    gl.clear(COLOR_BUFFER_BIT);
}}

fn main() {
    pretty_env_logger::init();

    let app = App::new("ee", my_render);

    App::run_app(app).unwrap_or_else(|err| {
        error!("Failed to run app!: {:?}", err);
    });
}