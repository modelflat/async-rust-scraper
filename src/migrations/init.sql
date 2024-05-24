CREATE TABLE IF NOT EXISTS scraped_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT,
    image TEXT,
    name TEXT,
    price TEXT
);