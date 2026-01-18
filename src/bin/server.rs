use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use darkwing_ducks::Darkwing;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::VersionedTransaction;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use std::fs;
use std::path::Path;
use dashmap::DashMap;
use std::time::{Duration, Instant};

// ============================================================================
// CONFIGURATION
// ============================================================================

const KEYPAIR_PATH: &str = "security/relayer-keypair.json";
const RPC_URL: &str = "https://api.mainnet-beta.solana.com";
const RATE_LIMIT_REQUESTS: u32 = 10;
const RATE_LIMIT_WINDOW_SECS: u64 = 60;

// ============================================================================
// RATE LIMITER
// ============================================================================

struct RateLimiter {
    buckets: DashMap<String, (u32, Instant)>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            buckets: DashMap::new(),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }
    
    fn check(&self, ip: &str) -> Result<(), String> {
        let now = Instant::now();
        
        let mut entry = self.buckets.entry(ip.to_string()).or_insert((0, now));
        let (count, window_start) = entry.value_mut();
        
        // Reset window if expired
        if now.duration_since(*window_start) > self.window {
            *count = 0;
            *window_start = now;
        }
        
        // Check limit
        if *count >= self.max_requests {
            return Err(format!(
                "Rate limit exceeded. ZK-Quack says: Slow down, human! 🦆 (Max {} req/min)", 
                self.max_requests
            ));
        }
        
        *count += 1;
        Ok(())
    }
}

// ============================================================================
// KEYPAIR MANAGEMENT
// ============================================================================

fn load_or_create_keypair() -> Result<Keypair, Box<dyn std::error::Error>> {
    let path = Path::new(KEYPAIR_PATH);
    
    if path.exists() {
        // Load existing keypair
        println!("🔑 Loading relayer keypair from {}", KEYPAIR_PATH);
        let data = fs::read_to_string(path)?;
        let bytes: Vec<u8> = serde_json::from_str(&data)?;
        let keypair = Keypair::from_bytes(&bytes)?;
        
        println!("   Pubkey: {}", keypair.pubkey());
        println!("   ⚠️  ENSURE THIS ADDRESS HAS ~0.1 SOL FOR JITO TIPS");
        println!("");
        
        Ok(keypair)
    } else {
        // Generate new keypair
        println!("🆕 No keypair found. Generating new relayer wallet...");
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let keypair = Keypair::new();
        let bytes = keypair.to_bytes();
        
        // Save as JSON array (Solana CLI compatible)
        fs::write(path, serde_json::to_string_pretty(&bytes.to_vec())?)?;
        
        println!("   ✅ Keypair saved to {}", KEYPAIR_PATH);
        println!("   Pubkey: {}", keypair.pubkey());
        println!("");
        println!("   🚨 CRITICAL: Fund this address before demo!");
        println!("   Command: solana transfer {} 0.1 --url mainnet-beta", keypair.pubkey());
        println!("");
        
        Ok(keypair)
    }
}

// ============================================================================
// API MODELS
// ============================================================================

#[derive(Deserialize)]
struct ProtectRequest {
    tx_base64: String,
}

#[derive(Serialize)]
struct ProtectResponse {
    status: String,
    bundle_id: String,
    explorer_url: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// ============================================================================
// API HANDLERS
// ============================================================================

#[post("/api/protect")]
async fn protect_endpoint(
    darkwing: web::Data<Arc<Mutex<Darkwing>>>,
    limiter: web::Data<Arc<RateLimiter>>,
    req: web::Json<ProtectRequest>,
    http_req: actix_web::HttpRequest,
) -> impl Responder {
    
    // Extract client IP
    let ip = http_req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();
    
    // Rate limiting check
    if let Err(msg) = limiter.check(&ip) {
        println!("🚫 Rate limit hit from IP: {}", ip);
        return HttpResponse::TooManyRequests().json(ErrorResponse {
            error: msg,
        });
    }
    
    println!("\n🦆 Incoming Signal from {}: Blink requested protection...", ip);
    
    // 1. Decode Base64
    let tx_bytes = match general_purpose::STANDARD.decode(&req.tx_base64) {
        Ok(b) => b,
        Err(e) => {
            println!("❌ Invalid Base64: {}", e);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: format!("Invalid Base64 encoding: {}", e),
            });
        }
    };
    
    // 2. Deserialize transaction
    let user_tx: VersionedTransaction = match bincode::deserialize(&tx_bytes) {
        Ok(tx) => tx,
        Err(e) => {
            println!("❌ Invalid transaction format: {}", e);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: format!("Invalid transaction format: {}", e),
            });
        }
    };
    
    // 3. Protection fee (0.001 SOL = 1,000,000 lamports)
    let protection_fee = 1_000_000;
    
    // 4. Send bundle
    let mut guardian = darkwing.lock().await;
    match guardian.protect_transaction(user_tx, protection_fee).await {
        Ok(uuid) => {
            println!("✅ SUCCESS: Bundle {} dispatched via Jito.", uuid);
            println!("");
            HttpResponse::Ok().json(ProtectResponse {
                status: "SECURED".to_string(),
                bundle_id: uuid.clone(),
                explorer_url: format!("https://explorer.jito.wtf/bundle/{}", uuid),
            })
        },
        Err(e) => {
            println!("❌ ERROR: Failed to send bundle: {}", e);
            println!("");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Darkwing Error: {}", e),
            })
        }
    }
}

// Health check endpoint
#[actix_web::get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "operational",
        "service": "DarkwingDucks",
        "version": "0.1.0"
    }))
}

// ============================================================================
// MAIN SERVER
// ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("==================================================");
    println!("🦇 DarkwingDucks API Server Starting...");
    println!("==================================================\n");
    
    // Load or create keypair
    let keypair = load_or_create_keypair()
        .expect("CRITICAL: Failed to load/create relayer keypair");
    
    // Initialize Darkwing guardian
    println!("🔌 Initializing Darkwing Protocol...");
    let guardian = Darkwing::new(keypair, RPC_URL.to_string()).await;
    let guardian_data = web::Data::new(Arc::new(Mutex::new(guardian)));
    
    // Initialize rate limiter
    let rate_limiter = Arc::new(RateLimiter::new(
        RATE_LIMIT_REQUESTS,
        RATE_LIMIT_WINDOW_SECS,
    ));
    let limiter_data = web::Data::new(rate_limiter);
    
    println!("🛡️  Rate Limiting: {} requests per {} seconds", 
             RATE_LIMIT_REQUESTS, RATE_LIMIT_WINDOW_SECS);
    println!("");
    println!("==================================================");
    println!("🚀 Server listening on http://127.0.0.1:8080");
    println!("==================================================");
    println!("📡 Endpoints:");
    println!("   POST /api/protect  - Submit transaction for bundle protection");
    println!("   GET  /health       - Health check");
    println!("==================================================\n");
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(guardian_data.clone())
            .app_data(limiter_data.clone())
            .service(protect_endpoint)
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
