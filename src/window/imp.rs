use adw::glib::subclass::InitializingObject;
use adw::subclass::prelude::ObjectSubclass;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/shiishiji/integration1/window.ui")]
pub struct Window {
    // #[template_child]
    // pub list: TemplateChild<Grid>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "AppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// it could have template_callbacks
impl Window {
    // implement button handlers here
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

// Trait shared by widgets
impl WidgetImpl for Window {}

// Trait shared by windows
impl WindowImpl for Window {}

// Trait share by application windows
impl ApplicationWindowImpl for Window {}
