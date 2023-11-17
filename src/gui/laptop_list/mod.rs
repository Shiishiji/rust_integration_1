mod imp;

use crate::gui::laptop_object::LaptopObject;
use crate::gui::laptop_row::LaptopRow;
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ListItem, NoSelection, SignalListItemFactory, SizeGroup, SizeGroupMode};

glib::wrapper! {
    pub struct LaptopList(ObjectSubclass<imp::LaptopList>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for LaptopList {
    fn default() -> Self {
        Self::new()
    }
}

impl LaptopList {
    pub fn new() -> Self {
        Object::builder().build()
    }

    fn laptops(&self) -> gio::ListStore {
        self.imp()
            .laptops
            .borrow()
            .clone()
            .expect("could not get current laptops")
    }

    fn setup_size_groups(&self) {
        let vector_of_size_groups = vec![
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
            SizeGroup::new(SizeGroupMode::Both),
        ];

        let child = self.imp().list_header.get().first_child().expect("");

        vector_of_size_groups
            .get(0)
            .expect("Unable to get size group")
            .add_widget(&child);

        vector_of_size_groups
            .get(0)
            .expect("")
            .add_widget(&self.imp().manufacturer_header_label.get());

        for i in 1..15 {
            vector_of_size_groups
                .get(i)
                .expect("Unable to get size group")
                .add_widget(&child.next_sibling().expect("Unable to get sibling"));

            vector_of_size_groups
                .get(12)
                .expect("")
                .add_widget(&match i {
                    1 => self.imp().screen_size_header_label.get(),
                    2 => self.imp().screen_resolution_header_label.get(),
                    3 => self.imp().screen_type_header_label.get(),
                    4 => self.imp().screen_touchscreen_header_label.get(),
                    5 => self.imp().processor_name_header_label.get(),
                    6 => self.imp().processor_physical_cores_header_label.get(),
                    7 => self.imp().processor_clock_speed_header_label.get(),
                    8 => self.imp().ram_header_label.get(),
                    9 => self.imp().disc_storage_header_label.get(),
                    10 => self.imp().disc_type_header_label.get(),
                    11 => self.imp().graphic_card_name_header_label.get(),
                    12 => self.imp().graphic_card_memory_header_label.get(),
                    13 => self.imp().os_header_label.get(),
                    14 => self.imp().disc_reader_header_label.get(),
                    _ => panic!("Unexpected iteration"),
                })
        }

        self.imp().size_groups.replace(vector_of_size_groups);
    }

    fn setup_list(&self) {
        let model = gio::ListStore::new::<LaptopObject>();
        self.imp().laptops.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.laptops()));
        self.imp().list.set_model(Some(&selection_model));
    }
    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        // Create an empty `LaptopRow` during setup
        factory.connect_setup(move |_, item| {
            let row = LaptopRow::new();

            let item = item
                .downcast_ref::<ListItem>()
                .expect("needs to be list item");

            item.set_child(Some(&row));
        });

        // Tell factory how to bind `LaptopRow` to a `LaptopObject`
        factory.connect_bind(move |_, item| {
            // Get `LaptopObject` from `ListItem`
            let object = item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<LaptopObject>()
                .expect("Needs to be a LaptopObject");

            let mut row = item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<LaptopRow>()
                .expect("Needs to be a `LaptopRow`");

            row.parent().expect("").set_focusable(false);

            row.bind(&object);
        });

        // Tell factory how to unbind `TaskRow` from `TaskObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TaskRow` from `ListItem`
            let row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<LaptopRow>()
                .expect("The child has to be a `TaskRow`.");

            row.unbind();
        });

        self.imp().list.set_factory(Some(&factory));
    }
}
