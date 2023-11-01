use std::cell::{Cell, RefCell};

use glib::prelude::*;
use glib::subclass::prelude::ObjectSubclass;
use glib::subclass::prelude::*;
use glib::Properties;
use gtk::glib;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::LaptopObject)]
pub struct LaptopObject {
    #[property(name = "manufacturer", get, set)]
    manufacturer: RefCell<String>,
    #[property(name = "screen-size", get, set)]
    screen_size: RefCell<String>,
    #[property(name = "screen-type", get, set)]
    screen_type: RefCell<String>,
    #[property(name = "screen-touchscreen", get, set)]
    screen_touchscreen: RefCell<String>,
    #[property(name = "processor-name", get, set)]
    processor_name: RefCell<String>,
    #[property(name = "processor-physical-cores", get, set)]
    processor_physical_cores: Cell<i32>,
    #[property(name = "processor-clock-speed", get, set)]
    processor_clock_speed: Cell<i32>,
    #[property(name = "ram", get, set)]
    ram: RefCell<String>,
    #[property(name = "disc-storage", get, set)]
    disc_storage: RefCell<String>,
    #[property(name = "disc-type", get, set)]
    disc_type: RefCell<String>,
    #[property(name = "graphiccard-name", get, set)]
    graphiccard_name: RefCell<String>,
    #[property(name = "graphiccard-memory", get, set)]
    graphiccard_memory: RefCell<String>,
    #[property(name = "os", get, set)]
    os: RefCell<String>,
    #[property(name = "disc-reader", get, set)]
    disc_reader: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for LaptopObject {
    const NAME: &'static str = "LaptopObject";
    type Type = super::LaptopObject;
}

#[glib::derived_properties]
impl ObjectImpl for LaptopObject {}
