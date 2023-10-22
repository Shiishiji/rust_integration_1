use crate::storage::models::models_csv::*;
use crate::storage::models::models_xml::*;
use crate::storage::models::Laptops;
use crate::storage::Storage;
use csv::{ReaderBuilder, StringRecord};
use std::fs::File;
use std::io::Read;

impl Storage {
    pub fn new() -> Self {
        Storage {
            source_txt_file_path: "laptopy.txt".to_string(),
            source_xml_file_path: "laptopy.xml".to_string(),
        }
    }

    pub fn load_from_txt(&self) -> Laptops {
        let file_path = self.source_txt_file_path.to_owned();

        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_path(file_path)
            .expect("Cannot read TXT file.");

        reader.set_headers(StringRecord::from(vec![
            "manufacturer",
            "matrix_size",
            "resolution",
            "matrix_type",
            "touchscreen",
            "cpu",
            "physical_cores",
            "clock_speed",
            "ram",
            "disc_size",
            "disc_type",
            "gpu",
            "gram",
            "os",
            "optical_drive",
        ]));

        let mut vector_of_laptops = vec![];

        for record in reader.deserialize() {
            let record: CsvLaptop = record.expect("error reading record.");
            vector_of_laptops.push(record);
        }

        Laptops::from(vector_of_laptops)
    }

    pub fn load_from_xml(&self) -> Laptops {
        let file_path = self.source_xml_file_path.to_owned();
        let mut xml = String::new();

        /*
         * Read file and save xml to mutable variable
         */

        let mut file = File::open(file_path).expect("XML File not found");

        file.read_to_string(&mut xml).expect("Cannot read.");

        // ====================================================================

        let parsed_xml =
            yaserde::de::from_str::<XmlLaptops>(&xml).expect("Unable to deserialize XML.");

        Laptops::from(parsed_xml)
    }
}
