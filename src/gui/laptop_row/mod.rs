mod imp;

use crate::gui::laptop_list::LaptopList;
use crate::gui::laptop_object::LaptopObject;
use glib::Object;
use gtk::glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, EditableLabel, SizeGroup};
use std::cell::RefMut;

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

    pub fn bind(&mut self, laptop_object: &LaptopObject) {
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

        // Size groups
        {
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

        // Bindings
        {
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

        // Handle coloring of changed elements
        {
            let closure_set_changed = |laptop_object: &LaptopObject, old, new| {
                laptop_object.set_changed(!(old == new));
            };

            let initial_manufacturer = laptop_object.manufacturer().clone();
            let initial_screen_size = laptop_object.screen_size().clone();
            let initial_screen_resolution = laptop_object.screen_resolution().clone();
            let initial_screen_type = laptop_object.screen_type().clone();
            let initial_screen_touchscreen = laptop_object.screen_touchscreen().clone();
            let initial_processor_name = laptop_object.processor_name().clone();
            let initial_processor_physical_cores = laptop_object.processor_physical_cores().clone();
            let initial_processor_clock_speed = laptop_object.processor_clock_speed().clone();
            let initial_ram = laptop_object.ram().clone();
            let initial_disc_storage = laptop_object.disc_storage().clone();
            let initial_disc_type = laptop_object.disc_type().clone();
            let initial_graphic_card_name = laptop_object.graphic_card_name().clone();
            let initial_graphic_card_memory = laptop_object.graphic_card_memory().clone();
            let initial_os = laptop_object.os().clone();
            let initial_disc_reader = laptop_object.disc_reader().clone();

            laptop_object.connect_manufacturer_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_manufacturer.clone(),
                    laptop_object.manufacturer(),
                );
            });
            laptop_object.connect_screen_size_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_screen_size.clone(),
                    laptop_object.screen_size(),
                );
            });
            laptop_object.connect_screen_resolution_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_screen_resolution.clone(),
                    laptop_object.screen_resolution(),
                );
            });
            laptop_object.connect_screen_type_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_screen_type.clone(),
                    laptop_object.screen_type(),
                );
            });
            laptop_object.connect_screen_touchscreen_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_screen_touchscreen.clone(),
                    laptop_object.screen_touchscreen(),
                );
            });
            laptop_object.connect_processor_name_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_processor_name.clone(),
                    laptop_object.processor_name(),
                );
            });
            laptop_object.connect_processor_physical_cores_notify(
                move |laptop_object: &LaptopObject| {
                    closure_set_changed(
                        laptop_object,
                        initial_processor_physical_cores.clone(),
                        laptop_object.processor_physical_cores(),
                    );
                },
            );
            laptop_object.connect_processor_clock_speed_notify(
                move |laptop_object: &LaptopObject| {
                    closure_set_changed(
                        laptop_object,
                        initial_processor_clock_speed.clone(),
                        laptop_object.processor_clock_speed(),
                    );
                },
            );
            laptop_object.connect_ram_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(laptop_object, initial_ram.clone(), laptop_object.ram());
            });
            laptop_object.connect_disc_storage_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_disc_storage.clone(),
                    laptop_object.disc_storage(),
                );
            });
            laptop_object.connect_disc_type_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_disc_type.clone(),
                    laptop_object.disc_type(),
                );
            });
            laptop_object.connect_graphic_card_name_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_graphic_card_name.clone(),
                    laptop_object.graphic_card_name(),
                );
            });
            laptop_object.connect_graphic_card_memory_notify(
                move |laptop_object: &LaptopObject| {
                    closure_set_changed(
                        laptop_object,
                        initial_graphic_card_memory.clone(),
                        laptop_object.graphic_card_memory(),
                    );
                },
            );
            laptop_object.connect_os_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(laptop_object, initial_os.clone(), laptop_object.os());
            });
            laptop_object.connect_disc_reader_notify(move |laptop_object: &LaptopObject| {
                closure_set_changed(
                    laptop_object,
                    initial_disc_reader.clone(),
                    laptop_object.disc_reader(),
                );
            });

            let obj = self.clone();
            laptop_object.connect_changed_notify(move |laptop_object: &LaptopObject| {
                println!("Row modified");
                obj.imp().mark_changed(laptop_object.changed());
            });
        }

        // Handle duplicates
        {
            let obj = self.clone();
            obj.imp().mark_duplicate(laptop_object.duplicate());
            laptop_object.connect_duplicate_notify(move |laptop_object: &LaptopObject| {
                obj.imp().mark_duplicate(laptop_object.duplicate());
            });
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
