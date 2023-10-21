mod window;

use adw::gio;
use adw::prelude::*;
use adw::{Application, glib};
use adw::gdk::Display;
use gtk::CssProvider;
use crate::window::Window;

static APP_ID: &str = "org.shiishiji.Integration1";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gresource")
        .expect("Could not load the resource.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/org/shiishiji/integration1/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let window = Window::new(app);

    window.present();
}