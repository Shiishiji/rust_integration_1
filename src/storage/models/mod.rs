pub(super) mod factory;
pub(super) mod models_csv;
pub(super) mod models_db;
pub(super) mod models_xml;

#[derive(Debug, Default, Clone)]
pub struct Laptops {
    pub laptops: Vec<Laptop>,
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
pub struct Screen {
    pub size: Option<String>,
    pub resolution: Option<String>,
    pub r#type: Option<String>, // "type" is a reserved keyword, so we use "r#type"
    pub touchscreen: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Processor {
    pub name: Option<String>,
    pub physical_cores: Option<u8>,
    pub clock_speed: Option<i32>,
}

#[derive(Debug, Default, Clone)]
pub struct Disc {
    pub storage: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct GraphicCard {
    pub name: Option<String>,
    pub memory: Option<String>,
}

impl Laptop {
    pub fn with_empty_strings() -> Laptop {
        Laptop {
            manufacturer: Some(String::new()),
            screen: Some(Screen {
                size: Some(String::new()),
                resolution: Some(String::new()),
                r#type: Some(String::new()),
                touchscreen: Some(String::new()),
            }),
            processor: Some(Processor {
                name: Some(String::new()),
                physical_cores: Some(0),
                clock_speed: Some(0),
            }),
            ram: Some(String::new()),
            disc: Some(Disc {
                storage: Some(String::new()),
                r#type: Some(String::new()),
            }),
            graphic_card: Some(GraphicCard {
                name: Some(String::new()),
                memory: Some(String::new()),
            }),
            os: Some(String::new()),
            disc_reader: Some(String::new()),
        }
    }
}
