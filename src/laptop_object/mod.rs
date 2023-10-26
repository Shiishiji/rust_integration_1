use crate::storage::models::Laptop;
use adw::glib;
use adw::glib::Object;

mod imp;

glib::wrapper! {
    pub struct LaptopObject(ObjectSubclass<imp::LaptopObject>);
}

impl LaptopObject {
    pub fn new(laptop: Laptop) -> Self {
        Object::builder()
            .property(
                "manufacturer",
                laptop.manufacturer.unwrap_or_else(|| String::new()),
            )
            .property(
                "screen-size",
                laptop
                    .screen
                    .as_ref()
                    .map(|s| s.size.clone())
                    .unwrap_or_else(|| Some(String::new())),
            )
            .property(
                "screen-type",
                laptop
                    .screen
                    .as_ref()
                    .map(|s| s.r#type.clone())
                    .unwrap_or_else(|| Some(String::new())),
            )
            .property(
                "screen-touchscreen",
                laptop
                    .screen
                    .as_ref()
                    .map(|s| s.touchscreen.clone())
                    .unwrap_or_else(|| Some(String::new())),
            )
            .property(
                "processor-name",
                laptop
                    .processor
                    .as_ref()
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| Some(String::new())),
            )
            .property("ram", laptop.ram.unwrap_or_else(|| String::new()))
            .property(
                "disc-storage",
                laptop
                    .disc
                    .as_ref()
                    .map(|d| d.storage.clone())
                    .unwrap_or_else(|| Some(String::new())),
            )
            .property(
                "disc-type",
                laptop
                    .disc
                    .as_ref()
                    .map(|d| d.r#type.clone())
                    .unwrap_or_else(|| Some(String::new())),
            )
            .property(
                "graphiccard-name",
                laptop
                    .graphic_card
                    .as_ref()
                    .map(|g| g.name.clone())
                    .unwrap_or_else(|| Some(String::new())),
            )
            .property(
                "graphiccard-memory",
                laptop
                    .graphic_card
                    .as_ref()
                    .map(|g| g.memory.clone())
                    .unwrap_or_else(|| Some(String::new())),
            )
            .property("os", laptop.os.unwrap_or_else(|| String::new()))
            .property(
                "disc-reader",
                laptop.disc_reader.unwrap_or_else(|| String::new()),
            )
            .build()

        // Todo: Add processor properties
    }
}
