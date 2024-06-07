use tokio::time::{sleep, Duration};

pub async fn rate_limited_operation<F, Fut>(operation: F, delay: Duration) -> Result<(), sqlx::Error>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<(), sqlx::Error>>,
{
    let result = operation().await;
    sleep(delay).await;
    result
}
