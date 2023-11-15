#[derive(Debug, Default, Clone, sqlx::FromRow)]
pub struct DbLaptop {
    pub manufacturer: Option<String>,
    pub screen_size: Option<String>,
    pub screen_resolution: Option<String>,
    pub screen_type: Option<String>,
    pub screen_touchscreen: Option<String>,
    pub processor_name: Option<String>,
    pub processor_physical_cores: Option<u8>,
    pub processor_clock_speed: Option<i32>,
    pub ram: Option<String>,
    pub disc_storage: Option<String>,
    pub disc_type: Option<String>,
    pub graphic_card_name: Option<String>,
    pub graphic_card_memory: Option<String>,
    pub os: Option<String>,
    pub disc_reader: Option<String>,
}
