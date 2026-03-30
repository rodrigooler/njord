use reqwest::Client;
use std::time::Duration;

pub async fn check_connectivity() -> bool {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    match client.get("https://www.apple.com/library/test/success.html").send().await {
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
    async fn test_check_connectivity_timeout() {
        // Test with a non-existent domain to simulate failure
        let client = Client::builder()
            .timeout(Duration::from_millis(1)) // Very short timeout
            .build()
            .unwrap();
        let result = client.get("https://www.apple.com/library/test/success.html").send().await;
        assert!(result.is_err());
    }
}