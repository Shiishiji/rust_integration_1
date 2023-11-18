use crate::storage::models::{Disc, GraphicCard, Laptop, Processor, Screen};

impl PartialEq for Laptop {
    fn eq(&self, other: &Self) -> bool {
        self.manufacturer == other.manufacturer
            && self.screen == other.screen
            && self.processor == other.processor
            && self.ram == other.ram
            && self.disc == other.disc
            && self.graphic_card == other.graphic_card
            && self.os == other.os
            && self.disc_reader == other.disc_reader
    }
}

impl PartialEq for Screen {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && self.resolution == other.resolution
            && self.r#type == other.r#type
            && self.touchscreen == other.touchscreen
    }
}

impl PartialEq for Processor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.physical_cores == other.physical_cores
            && self.clock_speed == other.clock_speed
    }
}

impl PartialEq for Disc {
    fn eq(&self, other: &Self) -> bool {
        self.storage == other.storage && self.r#type == other.r#type
    }
}

impl PartialEq for GraphicCard {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.memory == other.memory
    }
}
