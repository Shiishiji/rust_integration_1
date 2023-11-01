mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, EditableLabel};

use crate::laptop_object::LaptopObject;
use crate::window::Window;

glib::wrapper! {
    pub struct LaptopRow(ObjectSubclass<imp::LaptopRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for LaptopRow {
    fn default() -> Self {
        Self::new()
    }
}

impl LaptopRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, laptop_object: &LaptopObject) {
        let mut bindings = self.imp().bindings.borrow_mut();
        let manufacturer_label = self.imp().manufacturer_label.get();
        let screen_size_label = self.imp().screen_size_label.get();
        let screen_type_label = self.imp().screen_type_label.get();
        let screen_touchscreen_label = self.imp().screen_touchscreen_label.get();
        let processor_name_label: EditableLabel = self.imp().processor_name_label.get();
        let processor_physical_cores_label = self.imp().processor_physical_cores_label.get();
        let processor_clock_speed_label = self.imp().processor_clock_speed_label.get();
        let ram_label = self.imp().ram_label.get();
        let disc_storage_label = self.imp().disc_storage_label.get();
        let disc_type_label = self.imp().disc_type_label.get();
        let graphiccard_name_label = self.imp().graphiccard_name_label.get();
        let graphiccard_memory_label = self.imp().graphiccard_memory_label.get();
        let os_label = self.imp().os_label.get();
        let disc_reader_label = self.imp().disc_reader_label.get();

        {
            // Size groups
            let window = self.get_window().expect("Window not found");

            let size_groups = window.imp().size_groups.borrow();

            size_groups
                .get(0)
                .expect("cannot get size group 0")
                .add_widget(&manufacturer_label);
            size_groups
                .get(1)
                .expect("cannot get size group 1")
                .add_widget(&screen_size_label);
            size_groups
                .get(2)
                .expect("cannot get size group 2")
                .add_widget(&screen_type_label);
            size_groups
                .get(3)
                .expect("cannot get size group 3")
                .add_widget(&screen_touchscreen_label);
            size_groups
                .get(4)
                .expect("cannot get size group 4")
                .add_widget(&processor_name_label);
            size_groups
                .get(5)
                .expect("cannot get size group 5")
                .add_widget(&processor_physical_cores_label);
            size_groups
                .get(6)
                .expect("cannot get size group 6")
                .add_widget(&processor_clock_speed_label);
            size_groups
                .get(7)
                .expect("cannot get size group 7")
                .add_widget(&ram_label);
            size_groups
                .get(8)
                .expect("cannot get size group 8")
                .add_widget(&disc_storage_label);
            size_groups
                .get(9)
                .expect("cannot get size group 9")
                .add_widget(&disc_type_label);
            size_groups
                .get(10)
                .expect("cannot get size group 10")
                .add_widget(&graphiccard_name_label);
            size_groups
                .get(11)
                .expect("cannot get size group 11")
                .add_widget(&graphiccard_memory_label);
            size_groups
                .get(12)
                .expect("cannot get size group 12")
                .add_widget(&os_label);
            size_groups
                .get(13)
                .expect("cannot get size group 13")
                .add_widget(&disc_reader_label);
        }

        bindings.push(
            laptop_object
                .bind_property("manufacturer", &manufacturer_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property("screen_size", &screen_size_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        // bindings.push(laptop_object
        //     .bind_property("screen_type", &screen_type_label, "text")
        //     .sync_create()
        //     .build()
        // );

        bindings.push(
            laptop_object
                .bind_property("screen_touchscreen", &screen_touchscreen_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property("processor_name", &processor_name_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property(
                    "processor_physical_cores",
                    &processor_physical_cores_label,
                    "text",
                )
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property(
                    "processor_clock_speed",
                    &processor_clock_speed_label,
                    "text",
                )
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property("ram", &ram_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property("disc_storage", &disc_storage_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        // bindings.push(laptop_object
        //     .bind_property("disc_type", &disc_type_label, "text")
        //     .sync_create()
        //     .build()
        // );

        bindings.push(
            laptop_object
                .bind_property("graphiccard_name", &graphiccard_name_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property("graphiccard_memory", &graphiccard_memory_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property("os", &os_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );

        bindings.push(
            laptop_object
                .bind_property("disc_reader", &disc_reader_label, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }

    fn get_window(&self) -> Option<Window> {
        let parent = self.parent();

        // Get reference
        let window = parent // prob ListItem
            .expect("p1 not found")
            .parent() // prob GtkBox
            .expect("p2 not found")
            .parent() // prob GtkBox
            .expect("p2 not found")
            .parent() // prob GtkBox
            .expect("p2 not found")
            .parent() // prob Scrollable Window
            .expect("p3 not found")
            .parent() // prob Another GtkBox
            .expect("p4 not found")
            .parent() // Finally a main Window
            .expect("p5 not found");

        println!("{:?}", window);

        // Cast to Window instance
        window.downcast::<Window>().ok()
    }
}
