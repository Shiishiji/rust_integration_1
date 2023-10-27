mod imp;

use crate::laptop_object::LaptopObject;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{gio, glib, Application};
use glib::Object;
use gtk::prelude::*;
use gtk::{Label, NoSelection, Orientation, Widget};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn laptops(&self) -> gio::ListStore {
        self.imp()
            .laptops
            .borrow()
            .clone()
            .expect("could not get current laptops")
    }

    fn setup_list(&self) {
        let model = gio::ListStore::new::<LaptopObject>();
        self.imp().laptops.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.laptops()));

        self.imp().list.bind_model(Some(&selection_model), |x| {
            Widget::from({
                let row = gtk::Box::builder()
                    .homogeneous(true)
                    .orientation(Orientation::Horizontal)
                    .build();

                let laptop = x.downcast_ref::<LaptopObject>().expect("Cannot downcast");

                // Todo: add header row
                // Todo: set max size and text wrapping
                row.append(&Label::new(Some(
                    &laptop.manufacturer().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(
                    &laptop.screen_size().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(
                    &laptop.screen_type().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(
                    &laptop.screen_touchscreen().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(
                    &laptop.processor_name().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(
                    &laptop.ram().unwrap_or("".to_string())
                )));
                row.append(&Label::new(Some(
                    &laptop.disc_storage().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(
                    &laptop.disc_type().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(
                    &laptop.graphiccard_name().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(
                    &laptop.graphiccard_memory().unwrap_or("".to_string()),
                )));
                row.append(&Label::new(Some(&laptop.os().unwrap_or("".to_string()))));
                row.append(&Label::new(Some(
                    &laptop.disc_reader().unwrap_or("".to_string()),
                )));

                row
            })
        });
    }
}
