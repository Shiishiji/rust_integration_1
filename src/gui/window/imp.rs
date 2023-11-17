use crate::gui::laptop_list::LaptopList;
use crate::gui::laptop_object::LaptopObject;
use crate::storage::models::{Laptop, Laptops};
use crate::storage::Storage;
use adw::glib::subclass::InitializingObject;
use adw::prelude::*;
use adw::subclass::prelude::ObjectSubclass;
use adw::subclass::prelude::*;
use gtk::{
    glib, template_callbacks, Button, CompositeTemplate, FileChooserAction, FileChooserDialog,
    Label, ResponseType,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/shiishiji/integration1/window.ui")]
pub struct Window {
    #[template_child]
    pub laptop_list: TemplateChild<LaptopList>,

    #[template_child]
    pub status_label: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "AppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[template_callbacks]
impl Window {
    #[template_callback]
    fn handle_load_txt_data(&self, _button: &Button) {
        println!("Loading data from txt.");
        let laptops = self
            .laptop_list
            .get()
            .imp()
            .laptops
            .borrow()
            .clone()
            .expect("Couldn't get reference to laptops");

        let status = self.status_label.get();

        self.get_filename_and_perform_action(FileChooserAction::Open, move |filename| {
            let storage = Storage::new();
            let laptops_from_csv = storage.load_from_txt(filename);

            let mut i: i32 = 0;
            for laptop in laptops_from_csv.laptops {
                let laptop_obj = LaptopObject::new(laptop);

                laptops.append(&laptop_obj);
                i += 1;
            }

            status.set_label(&*format!("Wczytano {} rekordów.", i));
            println!("Loaded {} records.", i);
        });
    }

    #[template_callback]
    fn handle_load_xml_data(&self, _button: &Button) {
        println!("Loading data from xml.");
        let laptops = self
            .laptop_list
            .get()
            .imp()
            .laptops
            .borrow()
            .clone()
            .expect("Couldn't get reference to laptops");

        let status = self.status_label.get();

        self.get_filename_and_perform_action(FileChooserAction::Open, move |filename: &str| {
            let storage = Storage::new();
            let laptops_from_xml = storage.load_from_xml(filename);

            let mut i: i32 = 0;
            for laptop in laptops_from_xml.laptops {
                let laptop_obj = LaptopObject::new(laptop);
                laptops.append(&laptop_obj);
                i += 1;
            }

            status.set_label(&*format!("Wczytano {} rekordów.", i));
            println!("Loaded {} records.", i);
        });
    }

    #[template_callback]
    async fn handle_load_db_data(&self, _: &Button) {
        println!("Loading data from database.");
        let laptops = self
            .laptop_list
            .get()
            .imp()
            .laptops
            .borrow()
            .clone()
            .expect("Couldn't get reference to laptops");

        let storage = Storage::new();
        let laptops_from_db = storage.load_from_db().await;

        let mut i: i32 = 0;
        for laptop in laptops_from_db.laptops {
            let laptop_obj = LaptopObject::new(laptop);
            laptops.append(&laptop_obj);
            i += 1;
        }

        let status = self.status_label.get();
        status.set_label(&*format!("Wczytano {} rekordów.", i));
        println!("Loaded {} records.", i);
    }

    #[template_callback]
    fn handle_save_xml_data(&self, _button: &Button) {
        let laptops = Laptops {
            laptops: self.get_laptops().clone(),
        };
        let status = self.status_label.get();
        self.get_filename_and_perform_action(FileChooserAction::Save, move |filename: &str| {
            let storage = Storage::new();
            storage.save_to_xml(filename, laptops.clone());
            status.set_label(&*format!("Zapisano rekordy do XML: {}", filename));
        });
    }

    #[template_callback]
    fn handle_save_txt_data(&self, _button: &Button) {
        let laptops = Laptops {
            laptops: self.get_laptops().clone(),
        };
        let status = self.status_label.get();
        self.get_filename_and_perform_action(FileChooserAction::Save, move |filename: &str| {
            let storage = Storage::new();
            storage.save_to_txt(filename, laptops.clone());
            status.set_label(&*format!("Zapisano rekordy do CSV: {}", filename));
        });
    }

    #[template_callback]
    async fn handle_save_db_data(&self, _: &Button) {
        let storage = Storage::new();
        let laptops = Laptops {
            laptops: self.get_laptops().clone(),
        };
        let status = self.status_label.get();

        storage.save_to_db(laptops.clone()).await;
        status.set_label(&*format!("Zapisano rekordy do bazy danych."));
    }

    fn get_laptops(&self) -> Vec<Laptop> {
        let laptops = self
            .laptop_list
            .get()
            .imp()
            .laptops
            .clone()
            .into_inner()
            .expect("");

        let mut laptops_vec: Vec<Laptop> = vec![];
        for laptop_optional in laptops.into_iter() {
            let laptop: Laptop = Laptop::from(
                laptop_optional
                    .expect("??")
                    .downcast_ref::<LaptopObject>()
                    .expect("Expected LaptopObject")
                    .clone(),
            );

            laptops_vec.push(laptop);
        }

        laptops_vec
    }

    fn get_filename_and_perform_action<F>(&self, action_type: FileChooserAction, action: F)
    where
        F: Fn(&str) + 'static,
    {
        let obj = &self.obj();

        let file_dialog =
            FileChooserDialog::new(Some("Wybierz plik"), Some(obj.as_ref()), action_type, &[]);

        file_dialog.add_button("Cancel", ResponseType::Cancel.into());
        file_dialog.add_button("Open", ResponseType::Accept.into());

        // Connect the response signal
        file_dialog.connect_response(move |dialog, response| {
            match response {
                ResponseType::Accept => {
                    let file = dialog.file().expect("cannot get the file");

                    if let Some(filename) = file.path() {
                        let path = filename.to_str().expect("");
                        println!("Selected path: {:?}", path);

                        action(path);
                    }
                }
                _ => (),
            }
            dialog.close();
        });

        // Show the file chooser dialog
        file_dialog.show();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_callbacks();
    }
}

// Trait shared by widgets
impl WidgetImpl for Window {}

// Trait shared by windows
impl WindowImpl for Window {}

// Trait share by application windows
impl ApplicationWindowImpl for Window {}
