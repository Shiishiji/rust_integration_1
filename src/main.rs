

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

static APP_ID: &str = "org.shiishiji.Integration1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Integracja systemów - Damian Szopiński")
        .build();

    window.present();
}