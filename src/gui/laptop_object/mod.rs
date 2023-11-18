use crate::storage::models::Laptop;
use adw::glib;
use adw::glib::Object;

mod imp;

glib::wrapper! {
    pub struct LaptopObject(ObjectSubclass<imp::LaptopObject>);
}

impl LaptopObject {
    pub fn new(laptop: Laptop, is_duplicate: bool) -> Self {
        let empty_default = Laptop::with_empty_strings();

        let screen = match laptop.screen.is_some() {
            true => laptop.screen.expect(""),
            false => empty_default.screen.expect(""),
        };

        let processor = match laptop.processor.is_some() {
            true => laptop.processor.expect(""),
            false => empty_default.processor.expect(""),
        };

        let disc = match laptop.disc.is_some() {
            true => laptop.disc.expect(""),
            false => empty_default.disc.expect(""),
        };

        let gpu = match laptop.graphic_card.is_some() {
            true => laptop.graphic_card.expect(""),
            false => empty_default.graphic_card.expect(""),
        };

        Object::builder()
            .property("duplicate", is_duplicate)
            .property("changed", false)
            .property(
                "manufacturer",
                laptop.manufacturer.clone().unwrap_or(String::new()),
            )
            .property("screen-size", screen.clone().size.unwrap_or(String::new()))
            .property(
                "screen-resolution",
                screen.clone().resolution.unwrap_or(String::new()),
            )
            .property(
                "screen-type",
                screen.clone().r#type.unwrap_or(String::new()),
            )
            .property(
                "screen-touchscreen",
                screen.clone().touchscreen.unwrap_or(String::new()),
            )
            .property(
                "processor-name",
                processor.clone().name.unwrap_or(String::new()),
            )
            .property(
                "processor-physical-cores",
                format!("{}", processor.clone().physical_cores.unwrap_or(0_u8)),
            )
            .property(
                "processor-clock-speed",
                format!("{}", processor.clone().clock_speed.unwrap_or(0_i32)),
            )
            .property("ram", laptop.ram.unwrap_or(String::new()))
            .property(
                "disc-storage",
                disc.clone().storage.unwrap_or(String::new()),
            )
            .property("disc-type", disc.clone().r#type.unwrap_or(String::new()))
            .property(
                "graphic-card-name",
                gpu.clone().name.unwrap_or(String::new()),
            )
            .property(
                "graphic-card-memory",
                gpu.clone().memory.unwrap_or(String::new()),
            )
            .property("os", laptop.os.unwrap_or(String::new()))
            .property("disc-reader", laptop.disc_reader.unwrap_or(String::new()))
            .build()
    }
}
