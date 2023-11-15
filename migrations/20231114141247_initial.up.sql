-- Add up migration script here

CREATE TABLE IF NOT EXISTS laptops (
    id INTEGER PRIMARY KEY,
    manufacturer TEXT,
    screen_size TEXT,
    screen_resolution TEXT,
    screen_type TEXT,
    screen_touchscreen TEXT,
    processor_name TEXT,
    processor_physical_cores INTEGER,
    processor_clock_speed INTEGER,
    ram TEXT,
    disc_storage TEXT,
    disc_type TEXT,
    graphic_card_name TEXT,
    graphic_card_memory TEXT,
    os TEXT,
    disc_reader TEXT
);