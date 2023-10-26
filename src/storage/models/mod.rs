pub(super) mod factory;
pub(super) mod models_csv;
pub(super) mod models_xml;

#[derive(Debug, Default)]
pub struct Laptops {
    pub laptops: Vec<Laptop>,
}

#[derive(Debug, Default)]
pub struct Laptop {
    pub manufacturer: Option<String>,
    pub screen: Option<Screen>,
    pub processor: Option<Processor>,
    pub ram: Option<String>,
    pub disc: Option<Disc>,
    pub graphic_card: Option<GraphicCard>,
    pub os: Option<String>,
    pub disc_reader: Option<String>,
}

#[derive(Debug, Default)]
pub struct Screen {
    pub size: Option<String>,
    pub r#type: Option<String>, // "type" is a reserved keyword, so we use "r#type"
    pub touchscreen: Option<String>,
}

#[derive(Debug, Default)]
pub struct Processor {
    pub name: Option<String>,
    pub physical_cores: Option<u8>,
    pub clock_speed: Option<i32>,
}

#[derive(Debug, Default)]
pub struct Disc {
    pub storage: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Default)]
pub struct GraphicCard {
    pub name: Option<String>,
    pub memory: Option<String>,
}

impl Laptop {
    pub fn gpu_name(&self) -> Option<String> {
        if let Some(graphic_card) = &self.graphic_card {
            return graphic_card.name.clone();
        }

        None
    }
}
