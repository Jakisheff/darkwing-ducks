//! Compliance screening via Range Protocol
//! 
//! Provides OFAC/sanctions list screening for wallet addresses.
//! Implements circuit breaker pattern for API resilience.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Range API response structure
#[derive(Debug, Deserialize)]
struct RangeScreenResponse {
    /// Wallet address screened
    address: String,
    
    /// Risk score (0-100, higher = riskier)
    #[serde(default)]
    risk_score: u32,
    
    /// Whether wallet is flagged
    flagged: bool,
    
    /// Reason for flagging (if any)
    #[serde(default)]
    reason: Option<String>,
}

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq)]
enum CircuitState {
    Closed,  // Normal operation
    Open,    // API failing, skip checks
    HalfOpen, // Testing if API recovered
}

/// Circuit breaker for Range API
struct CircuitBreaker {
    state: CircuitState,
    failure_count: u32,
    last_failure: Option<Instant>,
    failure_threshold: u32,
    timeout: Duration,
}

impl CircuitBreaker {
    fn new() -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            last_failure: None,
            failure_threshold: 3,
            timeout: Duration::from_secs(60),
        }
    }
    
    /// Check if API call should proceed
    fn should_allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout elapsed
                if let Some(last_fail) = self.last_failure {
                    if last_fail.elapsed() > self.timeout {
                        self.state = CircuitState::HalfOpen;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }
    
    /// Record successful API call
    fn record_success(&mut self) {
        self.failure_count = 0;
        self.state = CircuitState::Closed;
        self.last_failure = None;
    }
    
    /// Record failed API call
    fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(Instant::now());
        
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
            println!("⚠️  Range API circuit breaker OPEN (too many failures)");
        }
    }
}

/// Compliance checker using Range Protocol
pub struct ComplianceChecker {
    client: Client,
    api_base_url: String,
    enabled: bool,
    circuit_breaker: Arc<Mutex<CircuitBreaker>>,
}

impl ComplianceChecker {
    /// Create new compliance checker
    pub fn new(api_base_url: String, enabled: bool) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("Failed to create HTTP client"),
            api_base_url,
            enabled,
            circuit_breaker: Arc::new(Mutex::new(CircuitBreaker::new())),
        }
    }
    
    /// Screen wallet address against sanctions lists
    /// 
    /// # Arguments
    /// * `wallet` - Solana wallet public key to screen
    /// 
    /// # Returns
    /// * `Ok(true)` - Wallet cleared (safe to proceed)
    /// * `Ok(false)` - Wallet flagged (reject transaction)
    /// * `Err(_)` - API error (fallback to allow for demo)
    pub async fn check_wallet(&self, wallet: &Pubkey) -> Result<bool, String> {
        if !self.enabled {
            println!("   ℹ️  Range compliance checks disabled");
            return Ok(true);
        }
        
        println!("\n╔════════════════════════════════════════╗");
        println!("║   🏺 TESTUM PURITY ASSAY (Range API)  ║");
        println!("╚════════════════════════════════════════╝");
        println!("   Wallet: {}", wallet);
        
        // Simulate scanning animation
        print!("   📊 Screening against sanctions lists");
        for _ in 0..3 {
            tokio::time::sleep(Duration::from_millis(200)).await;
            print!(".");
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
        println!();
        
        // Check circuit breaker
        let mut breaker = self.circuit_breaker.lock().await;
        if !breaker.should_allow_request() {
            println!("   🟡 Range API temporarily unavailable (circuit breaker open)");
            println!("   ✅ Proceeding without check (demo fallback)\n");
            return Ok(true);
        }
        drop(breaker); // Release lock before async call
        
        // Make API request
        let url = format!("{}/screen?address={}", self.api_base_url, wallet);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                let mut breaker = self.circuit_breaker.lock().await;
                
                if response.status().is_success() {
                    breaker.record_success();
                    
                    match response.json::<RangeScreenResponse>().await {
                        Ok(data) => {
                            if data.flagged {
                                println!("   🚫 FLAGGED: {}", 
                                        data.reason.unwrap_or_else(|| "Sanctions list match".to_string()));
                                println!("   ❌ Transaction REJECTED for compliance\n");
                                Ok(false)
                            } else {
                                println!("   ✅ PURE GOLD");
                                println!("   🎫 Compliance check PASSED\n");
                                Ok(true)
                            }
                        }
                        Err(e) => {
                            println!("   ⚠️  Failed to parse response: {}", e);
                            Ok(true) // Fallback: allow
                        }
                    }
                } else {
                    breaker.record_failure();
                    println!("   ⚠️  Range API error: HTTP {}", response.status());
                    println!("   🟡 Proceeding without check (fallback)\n");
                    Ok(true)
                }
            }
            Err(e) => {
                let mut breaker = self.circuit_breaker.lock().await;
                breaker.record_failure();
                
                println!("   ⚠️  Range API unreachable: {}", e);
                println!("   🟡 Proceeding without check (fallback)\n");
                Ok(true)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_circuit_breaker_logic() {
        let mut breaker = CircuitBreaker::new();
        
        // Should start closed
        assert_eq!(breaker.state, CircuitState::Closed);
        assert!(breaker.should_allow_request());
        
        // Record failures
        for _ in 0..3 {
            breaker.record_failure();
        }
        
        // Should open after threshold
        assert_eq!(breaker.state, CircuitState::Open);
        assert!(!breaker.should_allow_request());
        
        // Should recover after success
        breaker.state = CircuitState::HalfOpen;
        breaker.record_success();
        assert_eq!(breaker.state, CircuitState::Closed);
    }
}
