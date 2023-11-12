use crate::gui::laptop_object::LaptopObject;
use crate::storage::models::{Laptop, Laptops};
use crate::storage::Storage;
use adw::gio;
use adw::glib::subclass::InitializingObject;
use adw::prelude::*;
use adw::subclass::prelude::ObjectSubclass;
use adw::subclass::prelude::*;
use gtk::{
    glib, template_callbacks, Button, CompositeTemplate, FileChooserAction, FileChooserDialog,
    Label, ListView, ResponseType, SizeGroup,
};
use std::cell::RefCell;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/shiishiji/integration1/window.ui")]
pub struct Window {
    #[template_child]
    pub manufacturer_header_label: TemplateChild<Label>,
    #[template_child]
    pub screen_size_header_label: TemplateChild<Label>,
    #[template_child]
    pub screen_resolution_header_label: TemplateChild<Label>,
    #[template_child]
    pub screen_type_header_label: TemplateChild<Label>,
    #[template_child]
    pub screen_touchscreen_header_label: TemplateChild<Label>,
    #[template_child]
    pub processor_name_header_label: TemplateChild<Label>,
    #[template_child]
    pub processor_physical_cores_header_label: TemplateChild<Label>,
    #[template_child]
    pub processor_clock_speed_header_label: TemplateChild<Label>,
    #[template_child]
    pub ram_header_label: TemplateChild<Label>,
    #[template_child]
    pub disc_storage_header_label: TemplateChild<Label>,
    #[template_child]
    pub disc_type_header_label: TemplateChild<Label>,
    #[template_child]
    pub graphic_card_name_header_label: TemplateChild<Label>,
    #[template_child]
    pub graphic_card_memory_header_label: TemplateChild<Label>,
    #[template_child]
    pub os_header_label: TemplateChild<Label>,
    #[template_child]
    pub disc_reader_header_label: TemplateChild<Label>,

    #[template_child]
    pub list: TemplateChild<ListView>,
    #[template_child]
    pub list_header: TemplateChild<gtk::Box>,
    pub laptops: RefCell<Option<gio::ListStore>>,
    pub size_groups: RefCell<Vec<SizeGroup>>,
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
            .laptops
            .borrow()
            .clone()
            .expect("Couldn't get reference to laptops");

        self.get_filename_and_perform_action(FileChooserAction::Open, move |filename| {
            let storage = Storage::new();
            let laptops_from_csv = storage.load_from_txt(filename);

            let mut i: i32 = 0;
            for laptop in laptops_from_csv.laptops {
                let laptop_obj = LaptopObject::new(laptop);

                laptops.append(&laptop_obj);
                i += 1;
            }

            println!("Loaded {} records.", i);
        });
    }

    #[template_callback]
    fn handle_load_xml_data(&self, _button: &Button) {
        println!("Loading data from xml.");
        let laptops = self
            .laptops
            .borrow()
            .clone()
            .expect("Couldn't get reference to laptops");

        self.get_filename_and_perform_action(FileChooserAction::Open, move |filename: &str| {
            let storage = Storage::new();
            let laptops_from_xml = storage.load_from_xml(filename);

            let mut i: i32 = 0;
            for laptop in laptops_from_xml.laptops {
                let laptop_obj = LaptopObject::new(laptop);
                laptops.append(&laptop_obj);
                i += 1;
            }

            println!("Loaded {} records.", i);
        });
    }

    #[template_callback]
    fn handle_save_xml_data(&self, _button: &Button) {
        let laptops = Laptops {
            laptops: self.get_laptops().clone(),
        };
        self.get_filename_and_perform_action(FileChooserAction::Save, move |filename: &str| {
            let storage = Storage::new();
            storage.save_to_xml(filename, laptops.clone());
        });
    }

    #[template_callback]
    fn handle_save_txt_data(&self, _button: &Button) {
        let laptops = Laptops {
            laptops: self.get_laptops().clone(),
        };
        self.get_filename_and_perform_action(FileChooserAction::Save, move |filename: &str| {
            let storage = Storage::new();
            storage.save_to_txt(filename, laptops.clone());
        });
    }

    fn get_laptops(&self) -> Vec<Laptop> {
        let laptops = self
            .laptops
            .clone()
            .into_inner()
            .expect("Cannot get ListStore");

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
        obj.setup_list();
        obj.setup_size_groups();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

// Trait shared by widgets
impl WidgetImpl for Window {}

// Trait shared by windows
impl WindowImpl for Window {}

// Trait share by application windows
impl ApplicationWindowImpl for Window {}
