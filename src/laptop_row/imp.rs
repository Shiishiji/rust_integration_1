use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, EditableLabel};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/shiishiji/integration1/laptop_row.ui")]
pub struct LaptopRow {
    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,

    #[template_child]
    pub manufacturer_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub screen_size_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub screen_resolution_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub screen_type_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub screen_touchscreen_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub processor_name_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub processor_physical_cores_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub processor_clock_speed_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub ram_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub disc_storage_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub disc_type_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub graphic_card_name_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub graphic_card_memory_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub os_label: TemplateChild<EditableLabel>,
    #[template_child]
    pub disc_reader_label: TemplateChild<EditableLabel>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for LaptopRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "LaptopRow";
    type Type = super::LaptopRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for LaptopRow {}

// Trait shared by all widgets
impl WidgetImpl for LaptopRow {}

// Trait shared by all boxes
impl BoxImpl for LaptopRow {}
