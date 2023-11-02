use crate::laptop_object::LaptopObject;
use crate::storage::models::models_csv::*;
use crate::storage::models::models_xml::*;
use crate::storage::models::*;

impl From<CsvLaptop> for Laptop {
    fn from(value: CsvLaptop) -> Self {
        Laptop {
            manufacturer: value.manufacturer,
            screen: Some(Screen {
                size: value.resolution,
                r#type: value.matrix_type,
                touchscreen: value.touchscreen,
            }),
            processor: Some(Processor {
                name: value.cpu,
                physical_cores: value.physical_cores,
                clock_speed: value.clock_speed,
            }),
            ram: value.ram,
            disc: Some(Disc {
                storage: value.disc_size,
                r#type: value.disc_type,
            }),
            graphic_card: Some(GraphicCard {
                name: value.gpu,
                memory: value.gram,
            }),
            os: value.os,
            disc_reader: value.optical_drive,
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
                r#type: Some(value.screen_type()),
                touchscreen: Some(value.screen_touchscreen()),
            }),
            processor: Some(Processor {
                name: Some(value.processor_name()),
                physical_cores: Some(value.processor_physical_cores() as u8),
                clock_speed: Some(value.processor_clock_speed()),
            }),
            ram: Some(value.ram()),
            disc: Some(Disc {
                storage: Some(value.disc_storage()),
                r#type: Some(value.disc_type()),
            }),
            graphic_card: Some(GraphicCard {
                name: Some(value.graphiccard_name()),
                memory: Some(value.graphiccard_memory()),
            }),
            os: Some(value.os()),
            disc_reader: Some(value.disc_reader()),
        }
    }
}

impl From<Laptop> for CsvLaptop {
    fn from(value: Laptop) -> Self {
        // Todo: Finish this function.
        CsvLaptop {
            manufacturer: value.manufacturer.clone(),
            matrix_size: value.screen.clone().expect("").size,
            resolution: None,
            matrix_type: value.screen.clone().expect("").r#type,
            touchscreen: value.screen.clone().expect("").touchscreen,
            cpu: None,
            physical_cores: None,
            clock_speed: None,
            ram: None,
            disc_size: None,
            disc_type: None,
            gpu: None,
            gram: None,
            os: None,
            optical_drive: None,
        }
    }
}
