use crate::laptop_object::LaptopObject;
use crate::storage::Storage;
use adw::gio;
use adw::glib::subclass::InitializingObject;
use adw::subclass::prelude::ObjectSubclass;
use adw::subclass::prelude::*;
use gtk::{
    glib, template_callbacks, Button, CompositeTemplate, Label, ListView, SizeGroup, SizeGroupMode,
};
use std::cell::{Cell, RefCell};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/shiishiji/integration1/window.ui")]
pub struct Window {
    #[template_child]
    pub manufacturer_header_label: TemplateChild<Label>,
    #[template_child]
    pub screen_size_header_label: TemplateChild<Label>,
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
    pub graphiccard_name_header_label: TemplateChild<Label>,
    #[template_child]
    pub graphiccard_memory_header_label: TemplateChild<Label>,
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

// it could have template_callbacks
#[template_callbacks]
impl Window {
    // implement button handlers here
    #[template_callback]
    fn handle_load_txt_data(&self, _button: &Button) {
        println!("Loading data from txt.");
        let storage = Storage::new();
        let laptops_from_csv = storage.load_from_txt();

        let mut i: i32 = 0;
        for laptop in laptops_from_csv.laptops {
            let laptop_obj = LaptopObject::new(laptop);

            self.laptops
                .borrow()
                .clone()
                .expect("Couldn't get reference to laptops")
                .append(&laptop_obj);
            i += 1;
        }

        println!("Loaded {} records.", i);
    }

    #[template_callback]
    fn handle_load_xml_data(&self, _button: &Button) {
        println!("Loading data from xml.");
        let storage = Storage::new();
        let laptops_from_csv = storage.load_from_xml();

        let mut i: i32 = 0;
        for laptop in laptops_from_csv.laptops {
            let laptop_obj = LaptopObject::new(laptop);

            self.laptops
                .borrow()
                .clone()
                .expect("Couldn't get reference to laptops")
                .append(&laptop_obj);
            i += 1;
        }

        println!("Loaded {} records.", i);
    }

    #[template_callback]
    fn handle_save_xml_data(&self, _button: &Button) {
        println!("Tried saving to xml");
    }

    #[template_callback]
    fn handle_save_txt_data(&self, _button: &Button) {
        println!("Tried saving to txt");
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
