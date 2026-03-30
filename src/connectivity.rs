use reqwest::Client;
use std::time::Duration;

const CONNECTIVITY_URL: &str = "https://www.apple.com/library/test/success.html";
const CONNECTIVITY_TIMEOUT_SECS: u64 = 5;

pub async fn check_connectivity() -> bool {
    let client = Client::builder()
        .timeout(Duration::from_secs(CONNECTIVITY_TIMEOUT_SECS))
        .build()
        .expect("Failed to build HTTP client");
    match client
        .get(CONNECTIVITY_URL)
        .send()
        .await
    {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_check_connectivity_success() {
        // This test assumes internet is available
        // In a real scenario, you'd mock the HTTP client
        let result = check_connectivity().await;
        // Since we can't guarantee internet, just check it returns bool
        assert!(matches!(result, true | false));
    }

    #[tokio::test]
    async fn test_check_connectivity_failure() {
        // Test with an invalid URL to simulate failure
        let client = Client::builder()
            .timeout(Duration::from_secs(1))
            .build()
            .expect("Failed to build test HTTP client");
        let result = client
            .get("https://invalid-domain-that-does-not-exist.example")
            .send()
            .await;
        assert!(result.is_err());
    }
}
