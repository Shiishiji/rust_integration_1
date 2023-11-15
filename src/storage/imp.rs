use crate::storage::models::models_csv::*;
use crate::storage::models::models_db::DbLaptop;
use crate::storage::models::models_xml::*;
use crate::storage::models::{Laptop, Laptops};
use crate::storage::Storage;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use sqlx::{Connection, SqliteConnection};
use std::fs::File;
use std::io::{Read, Write};

impl Storage {
    pub fn new() -> Self {
        Storage {
            sqlite: "data.sqlite".to_string(),
        }
    }

    pub fn load_from_txt(&self, file_path: &str) -> Laptops {
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

    pub fn load_from_xml(&self, file_path: &str) -> Laptops {
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

    pub async fn load_from_db(&self) -> Laptops {
        let mut conn = SqliteConnection::connect(&*format!("sqlite:{}", self.sqlite))
            .await
            .expect("Cannot connect");

        let mut laptops_vec: Vec<Laptop> = vec![];
        let query = r#"
            SELECT * FROM laptops
        "#;
        let stream = sqlx::query_as::<_, DbLaptop>(query);
        for row in stream.fetch_all(&mut conn).await.expect("DB query failed") {
            laptops_vec.push(Laptop::from(row));
        }

        Laptops {
            laptops: laptops_vec,
        }
    }

    pub fn save_to_txt(&self, filename: &str, data: Laptops) {
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_path(filename)
            .expect("Writer error.");

        let mut i = 0;
        for laptop in data.laptops {
            let csv_laptop = CsvLaptop::from(laptop);

            writer
                .serialize(&csv_laptop)
                .expect("Error while saving to csv.");
            i += 1;
        }

        writer.flush().expect("Error while flushing.");
        println!("Saved {} records to {}.", i, filename);
    }

    pub fn save_to_xml(&self, filename: &str, data: Laptops) {
        let mut file = File::create(filename).expect("Cannot create file.");

        let mut i = 0;
        let mut vec_laptops = vec![];
        for laptop in data.laptops {
            let xml_laptop = XmlLaptop::from(laptop);
            vec_laptops.push(xml_laptop);
            i += 1;
        }

        let xml_laptops = XmlLaptops {
            laptop: vec_laptops,
        };

        let xml = yaserde::ser::to_string(&xml_laptops).expect("Unable to serialize");

        file.write(xml.as_ref()).expect("Unable to write");
        println!("Saved {} records to {}.", i, filename);
    }

    pub async fn save_to_db(&self, data: Laptops) {
        let mut conn = SqliteConnection::connect("sqlite:data.sqlite")
            .await
            .expect("Cannot connect");

        let insert_query = r#"
        INSERT INTO laptops (
            manufacturer,
            screen_size,
            screen_resolution,
            screen_type,
            screen_touchscreen,
            processor_name,
            processor_physical_cores,
            processor_clock_speed,
            ram,
            disc_storage,
            disc_type,
            graphic_card_name,
            graphic_card_memory,
            os,
            disc_reader
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;

        for laptop_model in data.laptops {
            let laptop: DbLaptop = DbLaptop::from(laptop_model);
            sqlx::query(insert_query)
                .bind(&laptop.manufacturer)
                .bind(&laptop.screen_size)
                .bind(&laptop.screen_resolution)
                .bind(&laptop.screen_type)
                .bind(&laptop.screen_touchscreen)
                .bind(&laptop.processor_name)
                .bind(&laptop.processor_physical_cores)
                .bind(&laptop.processor_clock_speed)
                .bind(&laptop.ram)
                .bind(&laptop.disc_storage)
                .bind(&laptop.disc_type)
                .bind(&laptop.graphic_card_name)
                .bind(&laptop.graphic_card_memory)
                .bind(&laptop.os)
                .bind(&laptop.disc_reader)
                .execute(&mut conn)
                .await
                .expect("Failed to execute SQL.");
        }
    }
}
