mod app;

use glow::{Context, HasContext, COLOR_BUFFER_BIT};
use crate::app::new_app;

unsafe fn my_render(gl: &Context) {unsafe{
    gl.clear_color(0.0, 1.0, 1.0, 1.0);
    gl.clear(COLOR_BUFFER_BIT);
}}

fn main() {
    pretty_env_logger::init();

    let app = new_app("ee", my_render);

    app::run_app(app);
}