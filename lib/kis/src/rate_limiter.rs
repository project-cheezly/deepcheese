use tokio::time::{sleep, Duration, Instant};

const CALL_LIMIT: u32 = 10;

pub struct RateLimiter {
    call_count: u32,
    last_reset: Instant,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            call_count: 0,
            last_reset: Instant::now(),
        }
    }

    pub async fn wait(&mut self) {
        let elapsed = Instant::now().duration_since(self.last_reset);

        if elapsed.as_secs() > 1 {
            self.reset_count();
        } else if self.call_count >= CALL_LIMIT {
            let remaining = Duration::from_secs(1) - elapsed;

            sleep(remaining).await;
            self.reset_count();
        }

        self.call_count += 1;
    }

    fn reset_count(&mut self) {
        self.call_count = 0;
        self.last_reset = Instant::now();
    }
}