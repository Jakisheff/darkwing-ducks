//! Configuration management for DarkwingDucks
//! 
//! Provides environment-based configuration with sensible defaults.
//! Supports both development and production environments.

use std::env;

/// Application configuration
#[derive(Clone, Debug)]
pub struct Config {
    /// Solana RPC endpoint URL
    pub rpc_url: String,
    
    /// Jito Block Engine endpoint
    pub jito_engine_url: String,
    
    /// Helius API key (optional for enhanced features)
    pub helius_api_key: Option<String>,
    
    /// Range API configuration
    pub range_api_url: String,
    pub range_enabled: bool,
    
    /// Server configuration
    pub server_host: String,
    pub server_port: u16,
    
    /// Rate limiting
    pub rate_limit_requests: u32,
    pub rate_limit_window_secs: u64,
    
    /// Relayer configuration
    pub relayer_keypair_path: String,
    pub jito_tip_account: String,
}

impl Config {
    /// Load configuration from environment or use defaults
    pub fn from_env() -> Self {
        let helius_key = env::var("HELIUS_API_KEY").ok();
        
        // Construct Helius RPC URL if key is provided
        let rpc_url = if let Some(ref key) = helius_key {
            format!("https://mainnet.helius-rpc.com/?api-key={}", key)
        } else {
            env::var("RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string())
        };
        
        Self {
            rpc_url,
            jito_engine_url: env::var("JITO_ENGINE_URL")
                .unwrap_or_else(|_| "https://amsterdam.mainnet.block-engine.jito.wtf".to_string()),
            helius_api_key: helius_key,
            
            range_api_url: env::var("RANGE_API_URL")
                .unwrap_or_else(|_| "https://api.range.org/v1".to_string()),
            range_enabled: env::var("RANGE_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            
            rate_limit_requests: env::var("RATE_LIMIT_REQUESTS")
                .ok()
                .and_then(|r| r.parse().ok())
                .unwrap_or(10),
            rate_limit_window_secs: env::var("RATE_LIMIT_WINDOW_SECS")
                .ok()
                .and_then(|w| w.parse().ok())
                .unwrap_or(60),
            
            relayer_keypair_path: env::var("RELAYER_KEYPAIR_PATH")
                .unwrap_or_else(|_| "security/relayer-keypair.json".to_string()),
            jito_tip_account: env::var("JITO_TIP_ACCOUNT")
                .unwrap_or_else(|_| "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5".to_string()),
        }
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if !self.rpc_url.starts_with("http") {
            return Err("Invalid RPC URL: must start with http/https".to_string());
        }
        
        if self.rate_limit_requests == 0 {
            return Err("Rate limit must be > 0".to_string());
        }
        
        Ok(())
    }
    
    /// Check if using Helius RPC
    pub fn is_helius_enabled(&self) -> bool {
        self.helius_api_key.is_some() || self.rpc_url.contains("helius")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_defaults() {
        let config = Config::from_env();
        assert!(config.rate_limit_requests > 0);
        assert_eq!(config.server_port, 8080);
    }
    
    #[test]
    fn test_config_validation() {
        let config = Config::from_env();
        assert!(config.validate().is_ok());
    }
}
