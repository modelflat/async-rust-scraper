use tokio::time::{sleep, Duration};

pub async fn rate_limiter() {
    sleep(Duration::from_secs(1)).await;
}
