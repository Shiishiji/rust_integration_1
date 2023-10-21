mod window;

use adw::gio;
use adw::prelude::*;
use adw::{Application, glib};
use crate::window::Window;

static APP_ID: &str = "org.shiishiji.Integration1";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gresource")
        .expect("Could not load the resource.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);

    window.present();
}