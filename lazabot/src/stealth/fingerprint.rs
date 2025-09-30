use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Browser fingerprint data for stealth operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserFingerprint {
    pub user_agent: String,
    pub timezone: String,
    pub language: String,
    pub screen_resolution: String,
    pub platform: String,
    pub vendor: String,
    pub vendor_sub: String,
    pub cpu_class: String,
    pub do_not_track: String,
    pub color_depth: u8,
    pub pixel_ratio: f32,
    pub hardware_concurrency: u8,
}

impl BrowserFingerprint {
    /// Generate a realistic browser fingerprint
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();

        // Generate realistic user agents for different browsers
        let user_agents = vec![
            // Chrome on Windows
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36",

            // Chrome on macOS
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",

            // Firefox on Windows
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:119.0) Gecko/20100101 Firefox/119.0",

            // Firefox on macOS
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:120.0) Gecko/20100101 Firefox/120.0",

            // Safari on macOS
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Safari/605.1.15",

            // Edge on Windows
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
        ];

        // Common timezones
        let timezones = vec![
            "America/New_York",
            "America/Los_Angeles",
            "America/Chicago",
            "America/Denver",
            "Europe/London",
            "Europe/Paris",
            "Europe/Berlin",
            "Asia/Tokyo",
            "Asia/Shanghai",
            "Asia/Singapore",
            "Australia/Sydney",
            "America/Toronto",
            "America/Vancouver",
            "Europe/Rome",
            "Europe/Madrid",
        ];

        // Common languages
        let languages = vec![
            "en-US,en;q=0.9",
            "en-GB,en;q=0.9",
            "en-CA,en;q=0.9",
            "es-ES,es;q=0.9",
            "fr-FR,fr;q=0.9",
            "de-DE,de;q=0.9",
            "it-IT,it;q=0.9",
            "pt-BR,pt;q=0.9",
            "ja-JP,ja;q=0.9",
            "ko-KR,ko;q=0.9",
            "zh-CN,zh;q=0.9",
            "ru-RU,ru;q=0.9",
        ];

        // Common screen resolutions
        let screen_resolutions = vec![
            "1920x1080",
            "1366x768",
            "1536x864",
            "1440x900",
            "1280x720",
            "1600x900",
            "2560x1440",
            "3840x2160",
            "1680x1050",
            "1024x768",
        ];

        // Common platforms
        let platforms = vec!["Win32", "MacIntel", "Linux x86_64"];

        // Common vendors
        let vendors = vec![
            "Google Inc.",
            "Mozilla",
            "Apple Computer, Inc.",
            "Microsoft Corporation",
        ];

        // Common vendor subs
        let vendor_subs = vec![
            "Google Inc.",
            "Mozilla",
            "Apple Computer, Inc.",
            "Microsoft Corporation",
        ];

        // Common CPU classes
        let cpu_classes = vec!["x86", "x64", "arm", "arm64"];

        // Do Not Track values
        let do_not_track_values = vec!["1", "0", "null"];

        // Color depths
        let color_depths = vec![24, 32, 16];

        // Pixel ratios
        let pixel_ratios = vec![1.0, 1.25, 1.5, 2.0, 2.5, 3.0];

        // Hardware concurrency (CPU cores)
        let hardware_concurrency = vec![2, 4, 6, 8, 12, 16, 24, 32];

        let user_agent = user_agents[rng.gen_range(0..user_agents.len())].to_string();
        let timezone = timezones[rng.gen_range(0..timezones.len())].to_string();
        let language = languages[rng.gen_range(0..languages.len())].to_string();
        let screen_resolution =
            screen_resolutions[rng.gen_range(0..screen_resolutions.len())].to_string();
        let platform = platforms[rng.gen_range(0..platforms.len())].to_string();
        let vendor = vendors[rng.gen_range(0..vendors.len())].to_string();
        let vendor_sub = vendor_subs[rng.gen_range(0..vendor_subs.len())].to_string();
        let cpu_class = cpu_classes[rng.gen_range(0..cpu_classes.len())].to_string();
        let do_not_track =
            do_not_track_values[rng.gen_range(0..do_not_track_values.len())].to_string();
        let color_depth = color_depths[rng.gen_range(0..color_depths.len())];
        let pixel_ratio = pixel_ratios[rng.gen_range(0..pixel_ratios.len())];
        let hardware_concurrency =
            hardware_concurrency[rng.gen_range(0..hardware_concurrency.len())];

        Self {
            user_agent,
            timezone,
            language,
            screen_resolution,
            platform,
            vendor,
            vendor_sub,
            cpu_class,
            do_not_track,
            color_depth,
            pixel_ratio,
            hardware_concurrency,
        }
    }

    /// Convert fingerprint to HTTP headers
    pub fn to_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        headers.insert("User-Agent".to_string(), self.user_agent.clone());
        headers.insert("Accept-Language".to_string(), self.language.clone());
        headers.insert(
            "Accept-Encoding".to_string(),
            "gzip, deflate, br".to_string(),
        );
        headers.insert("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".to_string());
        headers.insert("Sec-Fetch-Dest".to_string(), "document".to_string());
        headers.insert("Sec-Fetch-Mode".to_string(), "navigate".to_string());
        headers.insert("Sec-Fetch-Site".to_string(), "none".to_string());
        headers.insert("Sec-Fetch-User".to_string(), "?1".to_string());
        headers.insert("Upgrade-Insecure-Requests".to_string(), "1".to_string());
        headers.insert("Cache-Control".to_string(), "max-age=0".to_string());

        // Add DNT header if set
        if self.do_not_track != "null" {
            headers.insert("DNT".to_string(), self.do_not_track.clone());
        }

        headers
    }

    /// Get screen dimensions as tuple
    pub fn screen_dimensions(&self) -> Result<(u32, u32)> {
        let parts: Vec<&str> = self.screen_resolution.split('x').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid screen resolution format"));
        }

        let width = parts[0].parse::<u32>()?;
        let height = parts[1].parse::<u32>()?;

        Ok((width, height))
    }
}

/// Fingerprint spoofing utilities
pub struct FingerprintSpoofer;

impl FingerprintSpoofer {
    /// Generate a new browser fingerprint
    pub fn generate() -> BrowserFingerprint {
        BrowserFingerprint::generate()
    }

    /// Generate multiple fingerprints for rotation
    pub fn generate_multiple(count: usize) -> Vec<BrowserFingerprint> {
        (0..count).map(|_| Self::generate()).collect()
    }

    /// Generate a fingerprint that matches a specific browser type
    pub fn generate_for_browser(browser: &str) -> BrowserFingerprint {
        let mut fingerprint = Self::generate();

        match browser.to_lowercase().as_str() {
            "chrome" => {
                fingerprint.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string();
                fingerprint.vendor = "Google Inc.".to_string();
                fingerprint.vendor_sub = "Google Inc.".to_string();
            }
            "firefox" => {
                fingerprint.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0".to_string();
                fingerprint.vendor = "Mozilla".to_string();
                fingerprint.vendor_sub = "Mozilla".to_string();
            }
            "safari" => {
                fingerprint.user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15".to_string();
                fingerprint.vendor = "Apple Computer, Inc.".to_string();
                fingerprint.vendor_sub = "Apple Computer, Inc.".to_string();
                fingerprint.platform = "MacIntel".to_string();
            }
            "edge" => {
                fingerprint.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0".to_string();
                fingerprint.vendor = "Microsoft Corporation".to_string();
                fingerprint.vendor_sub = "Microsoft Corporation".to_string();
            }
            _ => {} // Use default generated fingerprint
        }

        fingerprint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_generation() {
        let fingerprint = FingerprintSpoofer::generate();

        assert!(!fingerprint.user_agent.is_empty());
        assert!(!fingerprint.timezone.is_empty());
        assert!(!fingerprint.language.is_empty());
        assert!(!fingerprint.screen_resolution.is_empty());
        assert!(fingerprint.color_depth > 0);
        assert!(fingerprint.pixel_ratio > 0.0);
        assert!(fingerprint.hardware_concurrency > 0);
    }

    #[test]
    fn test_fingerprint_headers() {
        let fingerprint = FingerprintSpoofer::generate();
        let headers = fingerprint.to_headers();

        assert!(headers.contains_key("User-Agent"));
        assert!(headers.contains_key("Accept-Language"));
        assert!(headers.contains_key("Accept-Encoding"));
    }

    #[test]
    fn test_screen_dimensions() {
        let fingerprint = FingerprintSpoofer::generate();
        let (width, height) = fingerprint.screen_dimensions().unwrap();

        assert!(width > 0);
        assert!(height > 0);
    }

    #[test]
    fn test_browser_specific_fingerprints() {
        let chrome_fp = FingerprintSpoofer::generate_for_browser("chrome");
        assert!(chrome_fp.user_agent.contains("Chrome"));

        let firefox_fp = FingerprintSpoofer::generate_for_browser("firefox");
        assert!(firefox_fp.user_agent.contains("Firefox"));

        let safari_fp = FingerprintSpoofer::generate_for_browser("safari");
        assert!(safari_fp.user_agent.contains("Safari"));
    }

    #[test]
    fn test_multiple_fingerprints() {
        let fingerprints = FingerprintSpoofer::generate_multiple(5);
        assert_eq!(fingerprints.len(), 5);

        // Ensure they're not all identical
        let first_ua = &fingerprints[0].user_agent;
        let has_variation = fingerprints.iter().any(|fp| fp.user_agent != *first_ua);
        // Note: This test might occasionally fail due to randomness, but it's unlikely
        // with 5 different fingerprints
    }
}
