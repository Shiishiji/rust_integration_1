use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaSerialize, YaDeserialize, Serialize, Deserialize, Debug)]
#[yaserde(rename = "laptops")]
pub struct XmlLaptops {
    pub laptop: Vec<XmlLaptop>,
}

#[derive(YaSerialize, YaDeserialize, Serialize, Deserialize, Debug)]
pub struct XmlLaptop {
    pub(crate) manufacturer: Option<String>,
    pub(crate) screen: Option<XmlScreen>,
    pub(crate) processor: Option<XmlProcessor>,
    pub(crate) ram: Option<String>,
    pub(crate) disc: Option<XmlDisc>,
    pub(crate) graphic_card: Option<XmlGraphicCard>,
    pub(crate) os: Option<String>,
    pub(crate) disc_reader: Option<String>,
}

#[derive(YaSerialize, YaDeserialize, Serialize, Deserialize, Debug)]
pub struct XmlScreen {
    pub(crate) size: Option<String>,
    #[yaserde(rename = "type")]
    pub(crate) r#type: Option<String>, // "type" is a reserved keyword, so we use "r#type"
    pub(crate) touchscreen: Option<String>,
}

#[derive(YaSerialize, YaDeserialize, Serialize, Deserialize, Debug)]
pub struct XmlProcessor {
    pub(crate) name: Option<String>,
    pub(crate) physical_cores: Option<u8>,
    pub(crate) clock_speed: Option<i32>,
}

#[derive(YaSerialize, YaDeserialize, Serialize, Deserialize, Debug)]
pub struct XmlDisc {
    pub(crate) storage: Option<String>,
    #[yaserde(rename = "type")]
    pub(crate) r#type: Option<String>,
}

#[derive(YaSerialize, YaDeserialize, Serialize, Deserialize, Debug)]
pub struct XmlGraphicCard {
    pub(crate) name: Option<String>,
    pub(crate) memory: Option<String>,
}
