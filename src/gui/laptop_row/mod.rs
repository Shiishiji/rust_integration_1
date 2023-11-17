mod imp;

use crate::gui::laptop_list::LaptopList;
use glib::Object;
use gtk::glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, EditableLabel, SizeGroup};
use std::cell::RefMut;

use crate::gui::laptop_object::LaptopObject;

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
        let manufacturer_label = self.imp().manufacturer_label.get();
        let screen_size_label = self.imp().screen_size_label.get();
        let screen_resolution_label = self.imp().screen_resolution_label.get();
        let screen_type_label = self.imp().screen_type_label.get();
        let screen_touchscreen_label = self.imp().screen_touchscreen_label.get();
        let processor_name_label: EditableLabel = self.imp().processor_name_label.get();
        let processor_physical_cores_label = self.imp().processor_physical_cores_label.get();
        let processor_clock_speed_label = self.imp().processor_clock_speed_label.get();
        let ram_label = self.imp().ram_label.get();
        let disc_storage_label = self.imp().disc_storage_label.get();
        let disc_type_label = self.imp().disc_type_label.get();
        let graphic_card_name_label = self.imp().graphic_card_name_label.get();
        let graphic_card_memory_label = self.imp().graphic_card_memory_label.get();
        let os_label = self.imp().os_label.get();
        let disc_reader_label = self.imp().disc_reader_label.get();

        {
            // Size groups
            let size_groups = self
                .parent()
                .expect("Cannot reference parent: expected ListItem")
                .parent()
                .expect("Cannot reference parent: expected ListRow")
                .parent()
                .expect("Cannot reference parent: expected LaptopList")
                .downcast_ref::<LaptopList>()
                .expect("Cannot cast to LaptopList")
                .imp()
                .size_groups
                .borrow()
                .clone();

            for i in 0..15 {
                match i {
                    0 => self.add_to_size_group(&size_groups, i, &manufacturer_label),
                    1 => self.add_to_size_group(&size_groups, i, &screen_size_label),
                    2 => self.add_to_size_group(&size_groups, i, &screen_resolution_label),
                    3 => self.add_to_size_group(&size_groups, i, &screen_type_label),
                    4 => self.add_to_size_group(&size_groups, i, &screen_touchscreen_label),
                    5 => self.add_to_size_group(&size_groups, i, &processor_name_label),
                    6 => self.add_to_size_group(&size_groups, i, &processor_physical_cores_label),
                    7 => self.add_to_size_group(&size_groups, i, &processor_clock_speed_label),
                    8 => self.add_to_size_group(&size_groups, i, &ram_label),
                    9 => self.add_to_size_group(&size_groups, i, &disc_storage_label),
                    10 => self.add_to_size_group(&size_groups, i, &disc_type_label),
                    11 => self.add_to_size_group(&size_groups, i, &graphic_card_name_label),
                    12 => self.add_to_size_group(&size_groups, i, &graphic_card_memory_label),
                    13 => self.add_to_size_group(&size_groups, i, &os_label),
                    14 => self.add_to_size_group(&size_groups, i, &disc_reader_label),
                    _ => {}
                }
            }
        }

        {
            // Bindings
            let mut bindings = self.imp().bindings.borrow_mut();
            for i in 0..15 {
                match i {
                    0 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "manufacturer",
                        &manufacturer_label,
                    ),
                    1 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "screen_size",
                        &screen_size_label,
                    ),
                    2 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "screen_resolution",
                        &screen_resolution_label,
                    ),
                    3 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "screen_type",
                        &screen_type_label,
                    ),
                    4 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "screen_touchscreen",
                        &screen_touchscreen_label,
                    ),
                    5 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "processor_name",
                        &processor_name_label,
                    ),
                    6 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "processor_physical_cores",
                        &processor_physical_cores_label,
                    ),
                    7 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "processor_clock_speed",
                        &processor_clock_speed_label,
                    ),
                    8 => self.add_binding(&mut bindings, &laptop_object, "ram", &ram_label),
                    9 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "disc_storage",
                        &disc_storage_label,
                    ),
                    10 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "disc_type",
                        &disc_type_label,
                    ),
                    11 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "graphic_card_name",
                        &graphic_card_name_label,
                    ),
                    12 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "graphic_card_memory",
                        &graphic_card_memory_label,
                    ),
                    13 => self.add_binding(&mut bindings, &laptop_object, "os", &os_label),
                    14 => self.add_binding(
                        &mut bindings,
                        &laptop_object,
                        "disc_reader",
                        &disc_reader_label,
                    ),
                    _ => {}
                }
            }
        }
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }

    fn add_to_size_group(&self, size_groups: &Vec<SizeGroup>, id: usize, obj: &EditableLabel) {
        size_groups
            .get(id)
            .expect(&*format!("Cannot get size group {}", id))
            .add_widget(obj);
    }

    fn add_binding(
        &self,
        bindings: &mut RefMut<Vec<Binding>>,
        laptop: &LaptopObject,
        source_property: &str,
        obj: &EditableLabel,
    ) {
        bindings.push(
            laptop
                .bind_property(source_property, obj, "text")
                .bidirectional()
                .sync_create()
                .build(),
        );
    }
}
