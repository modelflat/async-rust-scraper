#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_fetch_data() {
        let data = scraper::client::fetch_data("https://scrapeme.live/shop/").await.unwrap();
        assert!(data.contains("Shop"));
    }

    #[tokio::test]
    async fn test_insert_data() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://user:password@localhost/dbname")
            .await
            .unwrap();
        
        let data = vec!["Sample data".to_string()];
        let result = db::operations::insert_data(&pool, &data).await;
        assert!(result.is_ok());
    }
}
