use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CsvLaptop {
    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) manufacturer: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) matrix_size: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) resolution: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) matrix_type: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) touchscreen: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) cpu: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) physical_cores: Option<u8>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) clock_speed: Option<i32>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) ram: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) disc_size: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) disc_type: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) gpu: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) gram: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) os: Option<String>,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub(crate) optical_drive: Option<String>,
}
