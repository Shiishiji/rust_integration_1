use crate::gui::laptop_object::LaptopObject;
use crate::storage::models::models_csv::*;
use crate::storage::models::models_xml::*;
use crate::storage::models::*;

impl From<CsvLaptop> for Laptop {
    fn from(value: CsvLaptop) -> Self {
        Laptop {
            manufacturer: value.manufacturer,
            screen: Some(Screen {
                size: value.screen_size,
                resolution: value.screen_resolution,
                r#type: value.screen_type,
                touchscreen: value.screen_touchscreen,
            }),
            processor: Some(Processor {
                name: value.processor_name,
                physical_cores: value.processor_physical_cores,
                clock_speed: value.processor_clock_speed,
            }),
            ram: value.ram,
            disc: Some(Disc {
                storage: value.disc_size,
                r#type: value.disc_type,
            }),
            graphic_card: Some(GraphicCard {
                name: value.graphic_card_name,
                memory: value.graphic_card_ram,
            }),
            os: value.os,
            disc_reader: value.disc_reader,
        }
    }
}

impl From<Vec<CsvLaptop>> for Laptops {
    fn from(value: Vec<CsvLaptop>) -> Self {
        let vec = value.into_iter().map(|x| Laptop::from(x)).collect();

        Laptops { laptops: vec }
    }
}

impl From<XmlLaptops> for Laptops {
    fn from(value: XmlLaptops) -> Self {
        Laptops {
            laptops: value.laptop.into_iter().map(Laptop::from).collect(),
        }
    }
}

impl From<XmlLaptop> for Laptop {
    fn from(xml_laptop: XmlLaptop) -> Self {
        Laptop {
            manufacturer: xml_laptop.manufacturer,
            screen: xml_laptop.screen.map(Screen::from),
            processor: xml_laptop.processor.map(Processor::from),
            ram: xml_laptop.ram,
            disc: xml_laptop.disc.map(Disc::from),
            graphic_card: xml_laptop.graphic_card.map(GraphicCard::from),
            os: xml_laptop.os,
            disc_reader: xml_laptop.disc_reader,
        }
    }
}

impl From<XmlScreen> for Screen {
    fn from(xml_screen: XmlScreen) -> Self {
        Screen {
            size: xml_screen.size,
            resolution: xml_screen.resolution,
            r#type: xml_screen.r#type,
            touchscreen: xml_screen.touchscreen,
        }
    }
}

impl From<XmlProcessor> for Processor {
    fn from(xml_processor: XmlProcessor) -> Self {
        Processor {
            name: xml_processor.name,
            physical_cores: xml_processor.physical_cores,
            clock_speed: xml_processor.clock_speed,
        }
    }
}

impl From<XmlDisc> for Disc {
    fn from(xml_disc: XmlDisc) -> Self {
        Disc {
            storage: xml_disc.storage,
            r#type: xml_disc.r#type,
        }
    }
}

impl From<XmlGraphicCard> for GraphicCard {
    fn from(xml_graphic_card: XmlGraphicCard) -> Self {
        GraphicCard {
            name: xml_graphic_card.name,
            memory: xml_graphic_card.memory,
        }
    }
}

impl From<LaptopObject> for Laptop {
    fn from(value: LaptopObject) -> Self {
        Laptop {
            manufacturer: Some(value.manufacturer()),
            screen: Some(Screen {
                size: Some(value.screen_size()),
                resolution: Some(value.screen_resolution()),
                r#type: Some(value.screen_type()),
                touchscreen: Some(value.screen_touchscreen()),
            }),
            processor: Some(Processor {
                name: Some(value.processor_name()),
                physical_cores: Some(value.processor_physical_cores().parse().unwrap()),
                clock_speed: Some(value.processor_clock_speed().parse().unwrap()),
            }),
            ram: Some(value.ram()),
            disc: Some(Disc {
                storage: Some(value.disc_storage()),
                r#type: Some(value.disc_type()),
            }),
            graphic_card: Some(GraphicCard {
                name: Some(value.graphic_card_name()),
                memory: Some(value.graphic_card_memory()),
            }),
            os: Some(value.os()),
            disc_reader: Some(value.disc_reader()),
        }
    }
}

impl From<Laptop> for CsvLaptop {
    fn from(value: Laptop) -> Self {
        CsvLaptop {
            manufacturer: value.manufacturer.clone(),
            screen_size: value.screen.clone().expect("").size,
            screen_resolution: value.screen.clone().expect("").resolution,
            screen_type: value.screen.clone().expect("").r#type,
            screen_touchscreen: value.screen.clone().expect("").touchscreen,
            processor_name: value.processor.clone().expect("").name,
            processor_physical_cores: value.processor.clone().expect("").physical_cores,
            processor_clock_speed: value.processor.clone().expect("").clock_speed,
            ram: value.ram.clone(),
            disc_size: value.disc.clone().expect("").storage,
            disc_type: value.disc.clone().expect("").r#type,
            graphic_card_name: value.graphic_card.clone().expect("").name,
            graphic_card_ram: value.graphic_card.clone().expect("").name,
            os: value.os.clone(),
            disc_reader: value.disc_reader.clone(),
        }
    }
}

impl From<Laptop> for XmlLaptop {
    fn from(value: Laptop) -> Self {
        XmlLaptop {
            manufacturer: value.manufacturer,
            screen: Some(XmlScreen {
                size: value.screen.clone().expect("").size,
                resolution: value.screen.clone().expect("").resolution,
                r#type: value.screen.clone().expect("").r#type,
                touchscreen: value.screen.clone().expect("").touchscreen,
            }),
            processor: Some(XmlProcessor {
                name: value.processor.clone().expect("").name,
                physical_cores: value.processor.clone().expect("").physical_cores,
                clock_speed: value.processor.clone().expect("").clock_speed,
            }),
            ram: value.ram,
            disc: Some(XmlDisc {
                storage: value.disc.clone().expect("").storage,
                r#type: value.disc.clone().expect("").r#type,
            }),
            graphic_card: Some(XmlGraphicCard {
                name: value.graphic_card.clone().expect("").name,
                memory: value.graphic_card.clone().expect("").memory,
            }),
            os: value.os,
            disc_reader: value.disc_reader,
        }
    }
}
