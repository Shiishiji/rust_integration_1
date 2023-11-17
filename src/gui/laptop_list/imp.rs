use adw::gio;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label, ListView, SizeGroup};
use std::cell::RefCell;

#[derive(Default, CompositeTemplate)]
// #[properties(wrapper_type = super::LaptopList)]
#[template(resource = "/org/shiishiji/integration1/laptop_list.ui")]
pub struct LaptopList {
    #[template_child]
    pub list: TemplateChild<ListView>,
    #[template_child]
    pub list_header: TemplateChild<gtk::Box>,
    pub laptops: RefCell<Option<gio::ListStore>>,
    pub size_groups: RefCell<Vec<SizeGroup>>,

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
}

#[glib::object_subclass]
impl ObjectSubclass for LaptopList {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "LaptopList";
    type Type = super::LaptopList;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for LaptopList {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_list();
        obj.setup_size_groups();
        obj.setup_factory();
    }
}

// Trait shared by all widgets
impl WidgetImpl for LaptopList {}

// Trait shared by all boxes
impl BoxImpl for LaptopList {}
