mod app;
mod renderer;

use log::error;
use crate::app::App;

fn main() {
    pretty_env_logger::init();

    let app = App::new("ee");

    App::run_app(app).unwrap_or_else(|err| {
        error!("Failed to run app!: {:?}", err);
    });
}