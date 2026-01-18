use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use darkwing_ducks::Darkwing; // Import from our lib
use solana_sdk::signature::Keypair;
use std::sync::Arc;
use tokio::sync::Mutex;

// --- API STRUCTURES ---

#[derive(serde::Deserialize)]
struct ProtectRequest {
    #[serde(rename = "tx_base64")]
    tx_base64: String, // The signed transaction from the user
}

#[derive(serde::Serialize)]
struct QuackResponse {
    status: String,
    bundle_id: String,
    proof_type: String, // Easter Egg field
    message: String,
}

// --- HANDLERS ---

#[post("/lets-get-zkangerous")] // The secret button endpoint
async fn protect_handler(
    data: web::Data<Arc<Mutex<Darkwing>>>,
    _req: web::Json<ProtectRequest>,
) -> impl Responder {
    
    // In a real scenario, we would parse req.tx_base64 into a VersionedTransaction
    // and call data.lock().await.protect_transaction(...)
    
    // For the Hackathon Demo, we simulate the logic:
    println!("📨 Received request to initiate Dark Pool protocol...");
    
    // Simulate latency of "Smoke Bomb"
    let uuid = "888-DARKWING-PROTECTED-UUID".to_string();

    HttpResponse::Ok().json(QuackResponse {
        status: "ZK_QUACK_CONFIRMED".to_string(),
        bundle_id: uuid,
        proof_type: "Zero Knowledge Quack".to_string(),
        message: "🦆 Flapped away from MEV bots safely. You are the shadow.".to_string(),
    })
}

// --- MAIN ENTRY POINT ---

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Setup Solana Connection
    // NOTE: In production, load Keypair from file!
    let keypair = Keypair::new(); 
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    
    // 2. Initialize Darkwing Guardian
    let hero = Darkwing::new(keypair, rpc_url).await;
    let hero_data = web::Data::new(Arc::new(Mutex::new(hero)));

    println!("🦇 DarkwingDucks HQ active on port 8080");
    println!("📡 Listening for distress calls (Blinks)...");

    // 3. Start Web Server
    HttpServer::new(move || {
        App::new()
            .app_data(hero_data.clone())
            .service(protect_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}