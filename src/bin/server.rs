use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use darkwing_ducks::Darkwing;
use solana_sdk::signature::Keypair;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use colored::*;

// Request format from Client/Blink
#[derive(Deserialize)]
struct ProtectRequest {
    tx_base64: String,
}

// Response format to Client
#[derive(Serialize)]
struct ProtectResponse {
    status: String,
    bundle_id: String,
    proof_type: String,
    message: String,
}

// Handler for the POST request
async fn protect_handler(
    data: web::Data<Arc<Mutex<Darkwing>>>,
    req: web::Json<ProtectRequest>,
) -> impl Responder {
    let darkwing = data.lock().await;
    
    match darkwing.protect_transaction(req.tx_base64.clone()).await {
        Ok(bundle_id) => {
            HttpResponse::Ok().json(ProtectResponse {
                status: "ZK_QUACK_CONFIRMED".to_string(),
                bundle_id,
                proof_type: "Zero Knowledge Quack".to_string(),
                message: "🦆 Flapped away from MEV bots safely. You are the shadow.".to_string(),
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().body(format!("Error: {}", e))
        }
    }
}

// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Darkwing System: ONLINE 🦆")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Beautiful Startup Logs
    println!("\n==================================================");
    println!("{}", "🦆 DarkwingDucks v1.0 (Demo Mode)".magenta().bold());
    println!("==================================================");

    // Initialization
    let keypair = Keypair::new(); // Generate temp server keypair
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    
    let darkwing_system = Darkwing::new(keypair, rpc_url).await;
    let app_state = web::Data::new(Arc::new(Mutex::new(darkwing_system)));

    println!("{}", "\n🦇 DarkwingDucks HQ active on port 8080".green());
    println!("{}", "📡 Listening for distress calls (Blinks)...\n".white());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            .route("/lets-get-zkangerous", web::post().to(protect_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
