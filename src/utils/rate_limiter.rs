// utils/rate_limiter.rs
use tokio::time::{sleep, Duration};

pub async fn rate_limited_operation<F, Fut>(operation: F, delay: Duration)
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    operation().await;
    sleep(delay).await;
}

