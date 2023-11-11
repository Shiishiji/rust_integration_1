use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CsvLaptop {
    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) manufacturer: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) screen_size: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) screen_resolution: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) screen_type: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) screen_touchscreen: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) processor_name: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) processor_physical_cores: Option<u8>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) processor_clock_speed: Option<i32>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) ram: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) disc_size: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) disc_type: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) graphic_card_name: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) graphic_card_ram: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) os: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) disc_reader: Option<String>,
}
