use futures::stream::{Stream, StreamExt};
use rand::Rng;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::sleep;

/// Behavior simulation utilities for making bot actions appear more human-like
pub struct BehaviorSimulator {
    rng: rand::rngs::ThreadRng,
}

impl BehaviorSimulator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Generate a random delay between min and max milliseconds
    pub async fn random_delay(&mut self, min_ms: u64, max_ms: u64) {
        let delay_ms = self.rng.gen_range(min_ms..=max_ms);
        sleep(Duration::from_millis(delay_ms)).await;
    }

    /// Simulate human-like typing with variable delays between characters
    pub fn simulate_typing(&mut self, text: &str) -> TypingStream {
        TypingStream::new(text.to_string(), self.rng.clone())
    }

    /// Simulate mouse movement delay (for UI interactions)
    pub async fn mouse_delay(&mut self) {
        let delay_ms = self.rng.gen_range(100..=300);
        sleep(Duration::from_millis(delay_ms)).await;
    }

    /// Simulate reading time based on text length
    pub async fn reading_delay(&mut self, text: &str) {
        // Average reading speed: ~200 words per minute
        let words = text.split_whitespace().count();
        let reading_time_ms = (words as f64 / 200.0 * 60.0 * 1000.0) as u64;

        // Add some randomness and minimum delay
        let min_delay = 500;
        let max_delay = reading_time_ms + 1000;
        let delay_ms = self.rng.gen_range(min_delay..=max_delay);

        sleep(Duration::from_millis(delay_ms)).await;
    }

    /// Simulate page load waiting time
    pub async fn page_load_delay(&mut self) {
        let delay_ms = self.rng.gen_range(1000..=3000);
        sleep(Duration::from_millis(delay_ms)).await;
    }

    /// Simulate form filling delay
    pub async fn form_filling_delay(&mut self) {
        let delay_ms = self.rng.gen_range(200..=800);
        sleep(Duration::from_millis(delay_ms)).await;
    }
}

impl Default for BehaviorSimulator {
    fn default() -> Self {
        Self::new()
    }
}

/// A stream that yields characters with human-like typing delays
pub struct TypingStream {
    text: String,
    position: usize,
    rng: rand::rngs::ThreadRng,
    next_delay: Option<u64>,
}

impl TypingStream {
    fn new(text: String, rng: rand::rngs::ThreadRng) -> Self {
        Self {
            text,
            position: 0,
            rng,
            next_delay: None,
        }
    }

    fn get_typing_delay(&mut self, ch: char) -> u64 {
        let base_delay = match ch {
            '0'..='9' => 50,
            'a'..='z' | 'A'..='Z' => 80,
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '_' | '=' | '+' => {
                120
            }
            ' ' => 30,
            _ => 100,
        };

        let variation = self.rng.gen_range(0.8..=1.2);
        let delay = (base_delay as f64 * variation) as u64;

        if self.rng.gen_bool(0.05) {
            delay + self.rng.gen_range(200..=800)
        } else {
            delay
        }
    }
}

impl Stream for TypingStream {
    type Item = char;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.position >= self.text.len() {
            return Poll::Ready(None);
        }

        // If we have a delay to wait for, schedule it
        if let Some(delay_ms) = self.next_delay.take() {
            let waker = cx.waker().clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(delay_ms)).await;
                waker.wake();
            });
            return Poll::Pending;
        }

        // Get the next character and calculate delay for the next one
        let ch = self.text.chars().nth(self.position).unwrap();
        self.position += 1;

        if self.position < self.text.len() {
            let next_ch = self.text.chars().nth(self.position).unwrap();
            self.next_delay = Some(self.get_typing_delay(next_ch));
        }

        Poll::Ready(Some(ch))
    }
}

/// Helper function to collect typing stream into a string with delays
pub async fn collect_typing_stream(mut stream: TypingStream) -> String {
    let mut result = String::new();
    while let Some(ch) = stream.next().await {
        result.push(ch);
    }
    result
}

/// Helper function to simulate typing and return the result
pub async fn simulate_typing(text: &str) -> String {
    let mut simulator = BehaviorSimulator::new();
    let stream = simulator.simulate_typing(text);
    collect_typing_stream(stream).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_random_delay() {
        let mut simulator = BehaviorSimulator::new();
        let start = std::time::Instant::now();

        simulator.random_delay(100, 200).await;

        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
        assert!(elapsed <= Duration::from_millis(250)); // Some tolerance
    }

    #[tokio::test]
    async fn test_typing_simulation() {
        let text = "Hello, World!";
        let result = simulate_typing(text).await;
        assert_eq!(result, text);
    }

    #[tokio::test]
    async fn test_typing_stream() {
        let mut simulator = BehaviorSimulator::new();
        let stream = simulator.simulate_typing("test");

        let result = collect_typing_stream(stream).await;
        assert_eq!(result, "test");
    }

    #[tokio::test]
    async fn test_reading_delay() {
        let mut simulator = BehaviorSimulator::new();
        let text = "This is a test sentence with multiple words.";
        let start = std::time::Instant::now();

        simulator.reading_delay(text).await;

        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(500)); // Minimum delay
    }

    #[tokio::test]
    async fn test_mouse_delay() {
        let mut simulator = BehaviorSimulator::new();
        let start = std::time::Instant::now();

        simulator.mouse_delay().await;

        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
        assert!(elapsed <= Duration::from_millis(400)); // Some tolerance
    }

    #[tokio::test]
    async fn test_page_load_delay() {
        let mut simulator = BehaviorSimulator::new();
        let start = std::time::Instant::now();

        simulator.page_load_delay().await;

        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(1000));
        assert!(elapsed <= Duration::from_millis(3500)); // Some tolerance
    }

    #[tokio::test]
    async fn test_form_filling_delay() {
        let mut simulator = BehaviorSimulator::new();
        let start = std::time::Instant::now();

        simulator.form_filling_delay().await;

        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(200));
        assert!(elapsed <= Duration::from_millis(900)); // Some tolerance
    }
}
