use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use darkwing_ducks::Darkwing;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::VersionedTransaction;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose}; // <--- FIX IMPORT

// --- DATA STRUCTURES ---

#[derive(Deserialize)]
struct ProtectRequest {
    #[serde(rename = "tx_base64")]
    tx_base64: String, 
}

#[derive(Serialize)]
struct ProtectResponse {
    status: String,
    bundle_id: String,
    explorer_url: String,
}

// --- HANDLERS ---

#[post("/api/protect")]
async fn protect_endpoint(
    data: web::Data<Arc<Mutex<Darkwing>>>,
    req: web::Json<ProtectRequest>,
) -> impl Responder {
    
    println!("🦆 Incoming Signal: Blink requested protection...");

    // 1. ДЕКОДИРОВАНИЕ (FIXED FOR BASE64 0.21)
    // Используем general_purpose::STANDARD вместо base64::decode
    let tx_bytes = match general_purpose::STANDARD.decode(&req.tx_base64) {
        Ok(b) => b,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Base64"),
    };
    
    // 2. Десериализация (Bincode)
    let user_tx: VersionedTransaction = match bincode::deserialize(&tx_bytes) {
        Ok(tx) => tx,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Transaction Format"),
    };

    // 3. Запуск протокола
    let protection_fee = 1_000_000; 
    
    let mut guardian = data.lock().await;
    match guardian.protect_transaction(user_tx, protection_fee).await {
        Ok(uuid) => {
            println!("✅ SUCCESS: Bundle {} dispatched via Jito.", uuid);
            HttpResponse::Ok().json(ProtectResponse {
                status: "SECURED".to_string(),
                bundle_id: uuid.clone(),
                explorer_url: format!("https://explorer.jito.wtf/bundle/{}", uuid),
            })
        },
        Err(e) => {
            println!("❌ ERROR: Failed to send bundle: {}", e);
            HttpResponse::InternalServerError().body(format!("Darkwing Error: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // В продакшене ключи берем из .env!
    let keypair = Keypair::new(); 
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();

    println!("🦇 Starting DarkwingDucks API Server...");
    
    // Создаем Гардиана один раз и шарим между потоками
    let guardian = Darkwing::new(keypair, rpc_url).await;
    let guardian_data = web::Data::new(Arc::new(Mutex::new(guardian)));

    HttpServer::new(move || {
        App::new()
            .app_data(guardian_data.clone())
            .service(protect_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}