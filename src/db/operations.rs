use sqlx::{SqlitePool, Sqlite, Transaction};
use crate::scraper::model::PokemonProduct;

/// Inserts a list of Pokemon products into the database.
///
/// # Arguments
///
/// * `pool` - The database connection pool.
/// * `data` - A list of `PokemonProduct` items to insert.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the operation.
pub async fn insert_data(pool: &SqlitePool, data: &[PokemonProduct]) -> Result<(), sqlx::Error> {
    log::info!("Inserting {} products into the database", data.len());
    let mut transaction: Transaction<'_, Sqlite> = pool.begin().await?;

    for product in data {
        sqlx::query!(
            "INSERT INTO scraped_data (url, image, name, price) VALUES ($1, $2, $3, $4)",
            product.url,
            product.image,
            product.name,
            product.price
        )
            .execute(&mut transaction)
            .await?;
    }

    transaction.commit().await?;
    log::info!("Successfully inserted products into the database");
    Ok(())
}