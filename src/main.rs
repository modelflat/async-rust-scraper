use dotenv::dotenv;
use env_logger::Env;
use std::env;
use tokio::main;

mod config;
mod scraper;
mod db;
mod utils;

use crate::scraper::client::fetch_data;
use db::operations::insert_data;
use sqlx::sqlite::SqlitePoolOptions;

async fn fetch_and_insert_data(pool: &sqlx::SqlitePool, base_url: &str, pages: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = vec![];

    for page in 1..=pages {
        let url = format!("{}/page/{}", base_url, page);
        let pool = pool.clone();

        let task = tokio::spawn(async move {
            match fetch_data(&url).await {
                Ok(data) => {
                    if let Err(err) = insert_data(&pool, &data).await {
                        log::error!("Error inserting data from {}: {}", url, err);
                    } else {
                        log::info!("Successfully inserted data from {}", url);
                    }
                }
                Err(err) => log::error!("Error fetching data from {}: {}", url, err),
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        if let Err(err) = task.await {
            log::error!("Task failed: {:?}", err);
        }
    }

    Ok(())
}

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load environment variables from .env file
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    log::info!("Connecting to database at: {}", db_url);

    // Create database connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Ensure the table is created
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS scraped_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT,
            image TEXT,
            name TEXT,
            price TEXT
        )"
    )
        .execute(&pool)
        .await?;

    // Fetch data from multiple pages concurrently and insert into the database
    let base_url = "https://scrapeme.live/shop";
    let pages = 10;  // Number of pages to fetch
    fetch_and_insert_data(&pool, base_url, pages).await?;

    Ok(())
}
