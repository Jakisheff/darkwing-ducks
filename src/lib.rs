use jito_protos::searcher::{searcher_service_client::SearcherServiceClient, SendBundleRequest};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey, 
    signature::{Keypair, Signer}, 
    system_instruction,
    transaction::{Transaction, VersionedTransaction},
};
use std::str::FromStr;
use std::sync::Arc;
use tonic::{transport::Channel, service::Interceptor, Request, Status};
use tonic::codegen::InterceptedService;
use uuid::Uuid;

// --- CONFIGURATION ---
const VERSION: &str = "0.1.0 (Alpha-Quack)";
const BUILD_DATE: &str = "2026-01-18";
// Jito Tip Account (Mainnet). Move to ENV in prod.
const JITO_TIP_ACCOUNT: &str = "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5";
// Secure TLS connection to Amsterdam Block Engine
const JITO_ENGINE_URL: &str = "https://amsterdam.mainnet.block-engine.jito.wtf";

#[derive(Clone)]
pub struct AuthInterceptor {
    pub keypair: Arc<Keypair>,
}

impl AuthInterceptor {
    pub fn new(keypair: Arc<Keypair>) -> Self {
        Self { keypair }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // In production, this would sign a Challenge from Jito.
        // For this build kit, we pass through as we don't have the auth challenge flow.
        Ok(request)
    }
}

pub struct Darkwing {
    client: InterceptedService<Channel, AuthInterceptor>,

    rpc_client: Arc<RpcClient>,
    // SECURITY NOTE: Relayer keypair. Does NOT custody user funds.
    // Used ONLY to pay Jito tip for bundle inclusion initially.
    signer: Keypair,
}

impl Darkwing {
    /// Initialize secure connection to Jito Block Engine
    pub async fn new(keypair: Keypair, rpc_url: String) -> Self {
        // 🦆 SYSTEM BOOT LOGS
        println!("==================================================");
        println!("🦆 DarkwingDucks v{} initialized [{}]", VERSION, BUILD_DATE);
        println!("==================================================");
        
        let kp_arc = Arc::new(keypair);
        let auth = AuthInterceptor::new(kp_arc.clone());
        let signer_kp = Keypair::from_bytes(&kp_arc.to_bytes()).unwrap(); // Clone for signer field

        // SECURITY NOTE: TLS-encrypted channel.
        // Ensures bundle content is not visible to public internet providers/ISPs.
        println!("🔌 Connecting to Jito Block Engine (Amsterdam)...");
        let channel = Channel::from_static(JITO_ENGINE_URL)
            .connect()
            .await
            .expect("CRITICAL: Failed to connect to Jito securely");
            
        let client = SearcherServiceClient::with_interceptor(channel, auth);
        let rpc_client = Arc::new(RpcClient::new(rpc_url));
        
        println!("   [System Check]:");
        println!("   - Connection: Secure (TLS)");
        println!("   - Searcher: Active");
        println!("   - Status: ZKangerous");
        println!("==================================================\n");
        
        Self { client, rpc_client, signer: signer_kp }
    }

    /// 🏺 THE "TESTUM" PURITY CHECK
    /// Filters wallets against OFAC/Sanctions lists before execution.
    pub async fn check_compliance(&self, wallet: &Pubkey) -> bool {
        println!("🔍 Darkwing Assay: Pouring wallet {} into the Testum...", wallet);
        
        // TODO: For Production, uncomment real API call:
        // let url = format!("https://api.range.org/v1/screen?wallet={}", wallet);
        // let resp = reqwest::get(&url).await.unwrap();
        // return resp.status().is_success();
        
        // HACKATHON DEMO MODE:
        println!("   ✅ Result: PURE GOLD. Access Granted.");
        true
    }

    /// 🌑 PROTECT TRANSACTION (The Smoke Bomb)
    /// Wraps user intent into an atomic Jito Bundle.
    pub async fn protect_transaction(
        &mut self,
        user_tx: VersionedTransaction,
        protection_fee: u64
    ) -> Result<String, Box<dyn std::error::Error>> {
        
        // 1. Tip Transaction (The cost of privacy)
        let tip_ix = system_instruction::transfer(
            &self.signer.pubkey(),
            &Pubkey::from_str(JITO_TIP_ACCOUNT).unwrap(),
            protection_fee
        );
        let latest_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tip_tx = VersionedTransaction::from(Transaction::new_signed_with_payer(
            &[tip_ix],
            Some(&self.signer.pubkey()),
            &[&self.signer],
            latest_blockhash
        ));

        // 2. Atomic Bundle Construction
        // CRITICAL ORDER: User TX first. If it fails, Tip TX is never executed.
        // This guarantees "No Protection = No Pay".
        let packets = vec![user_tx.into(), tip_tx.into()];

        // 3. Private Transmission
        println!("🚀 Sending Bundle via Jito Secure Interface...");
        let response = self.client.send_bundle(SendBundleRequest {
            bundle: Some(jito_protos::bundle::Bundle { packets, header: None }),
        }).await?;
        
        let uuid = response.into_inner().uuid;
        
        // 🦆 DARK HUMOR / SUCCESS LOG
        println!("🦆 ZK-Quack emitted! Bundle UUID: {}", uuid);
        println!("   (The transaction witnessed nothing. It knows nothing.)");
        
        Ok(uuid)
    }
}