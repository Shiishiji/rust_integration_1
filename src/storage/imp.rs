use crate::storage::models::models_csv::*;
use crate::storage::models::models_xml::*;
use crate::storage::models::Laptops;
use crate::storage::Storage;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::fs::File;
use std::io::{Read, Write};

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
            "screen_size",
            "screen_resolution",
            "screen_type",
            "screen_touchscreen",
            "processor_name",
            "processor_physical_cores",
            "processor_clock_speed",
            "ram",
            "disc_size",
            "disc_type",
            "graphic_card_name",
            "graphic_card_ram",
            "os",
            "disc_reader",
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

    pub fn save_to_txt(&self, filename: &str, data: Laptops) {
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_path(filename)
            .expect("Writer error.");

        for laptop in data.laptops {
            let csv_laptop = CsvLaptop::from(laptop);

            writer
                .serialize(&csv_laptop)
                .expect("Error while saving to csv.");
        }

        writer.flush().expect("Error while flushing.");
    }

    pub fn save_to_xml(&self, filename: &str, data: Laptops) {
        let mut file = File::create(filename).expect("Cannot create file.");

        let mut vec_laptops = vec![];
        for laptop in data.laptops {
            let xml_laptop = XmlLaptop::from(laptop);
            vec_laptops.push(xml_laptop);
        }

        let xml_laptops = XmlLaptops {
            laptop: vec_laptops,
        };

        let xml = yaserde::ser::to_string(&xml_laptops).expect("Unable to serialize");

        file.write(xml.as_ref()).expect("Unable to write");
    }
}
