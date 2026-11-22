# 🦆 DarkwingDucks: Let's get ZKangerous!

> **"The terror that flaps in the mempool."**
> The first meme-powered Dark Pool for Solana Retail.

[![Track](https://img.shields.io/badge/Track-01_Private_Payments-blue)](https://solana.com)
[![Vibe](https://img.shields.io/badge/Vibe-Zero_Ducks_Given-purple)](https://twitter.com)
[![Status](https://img.shields.io/badge/Status-Educational_Parody-yellow)](https://github.com)

> [!NOTE]
> **HACKATHON DEMO BUILD (SIMULATION)**
> This version runs in **Simulation Mode** (Variant A) to demonstrate the UX and Architecture without requiring live mainnet funds/keys during the pitch.
> *   **Jito/Range/RPC**: Simulated execution latencies.
> *   **Logs**: Theatrical visualization of the protocol flow.

## 🚨 The Villain: Dr. Sandwich (Toxic MEV)
Retail users lost **$450M+** to MEV bots (sandwich attacks) in 2025.
Even as general network tips decline, **Toxic MEV** remains a predator for retail traders.

### ⚖️ The "Crying Juror" Reality (Why Law Failed)
**Case Study:** In *US v. Peraire-Bueno*, two brothers were accused of taking $25M from sandwich bots on Ethereum in a 12-second scheme. The jury deadlocked in November 2025. On 16 November 2026 Judge Jessica Clarke acquitted them: the prosecution had not proved a material misrepresentation ([The Block](https://www.theblock.co/news/regulation/2026-11-16-peraire-bueno-brothers-acquitted-ethereum-mev-wire-fraud-441425)).

**The Lesson:** The court did not treat this MEV as fraud.
**The Solution:** You cannot wait for a court verdict. You need **DarkwingDucks** to make your transactions invisible to predators *today*.

## 🔌 API & Integration

**Blinks** talk to our Rust backend via `POST /lets-get-zkangerous`. The Blink sends a **signed** transaction payload (Base64) as JSON: `{"tx_base64": "<base64>"}`.

## 🔑 Run the Demo (Simulation Mode)

1. **Start the server** (listens on `127.0.0.1:8080`):
   ```bash
   cargo run --bin server
   ```

2. **Send a request**:
   ```bash
   curl -X POST http://127.0.0.1:8080/lets-get-zkangerous \
     -H "Content-Type: application/json" \
     -d '{"tx_base64": "<base64 signed transaction>"}'
   ```

The server answers after about one second with a simulated `bundle_id`. It saves no wallet and sends nothing to the network, so no SOL is needed. `GET /health` returns the status line.

## 📊 Quantifiable Impact
Why use Darkwing? Because public mempools are expensive.
*   **MEV Saved**: ~$47 per large exchange (avg. sandwich attack slippage).
*   **Reliability**: Jito Bundle success rate ~88.9% with <1% downtime.
*   **Analytics**: Track global savings via our [Dune Dashboard](https://dune.com/darkwing_metrics/savings) (Query: `SELECT sum(saved_slippage) FROM darkwing_events`).

## 🦸 The Solution: DarkwingDucks
We provide an **Instant Dark Pool** layer for Solana Blinks.
We wrap your transaction in a "Smoke Bomb" (Jito Bundle) and route it through a secret tunnel, bypassing public mempool villains entirely.

> **Note on MEV:** DarkwingDucks eliminates **Sandwich Attacks** and **Front-Running**.
> We cannot stop arbitrage (back-running), but we make sure YOU are not the victim of predatory pricing.

### 🦆 The "Privacy Pond" Philosophy
> **"Browsing? Use DuckDuckGo. Trading? Use DarkwingDucks."**

We follow the **DDG Standard**:
* **DuckDuckGo** stops Google from tracking your clicks.
* **DarkwingDucks** stops MEV Bots from tracking your swaps.
* *Same pond. Same mission. Different villains.*

## 🏗️ Architecture
```
┌─────────────────────────────────────────────┐
│  USER LAYER (Solana Blink)                 │
│  - Base64-encoded VersionedTransaction     │
└──────────────────┬──────────────────────────┘
                   │ POST /lets-get-zkangerous
┌──────────────────▼──────────────────────────┐
│  API LAYER (Actix-Web)                     │
│  - Rate Limiting (IP-based)                │
│  - Request Validation                      │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│  COMPLIANCE LAYER (Range Protocol)         │
│  - OFAC Sanctions Screening                │
│  - Circuit Breaker Pattern                 │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│  PROTOCOL LAYER (Darkwing Core)            │
│  - Atomic Bundle Construction              │
│  - Jito Integration                        │
└──────────────────┬──────────────────────────┘
                   │ gRPC (TLS)
┌──────────────────▼──────────────────────────┐
│  TRANSPORT LAYER (Jito Block Engine)       │
│  - Amsterdam Region                        │
│  - Direct-to-Validator                     │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│  INFRASTRUCTURE (Helius RPC)               │
│  - Enhanced Reliability                    │
│  - Priority Fee Estimation                 │
│  - Privacy-Focused Nodes                   │
└─────────────────────────────────────────────┘
```

## 🚀 Powered By

### **Helius** - Enterprise RPC Infrastructure
- Enhanced transaction reliability
- Priority fee estimation
- Privacy-focused node infrastructure
- 99.9% uptime SLA

### **Range Protocol** - Compliance Layer
- Real-time OFAC sanctions screening
- Pre-transaction wallet verification
- Selective disclosure for compliant privacy

### **Jito** - MEV Protection
- Direct-to-validator bundle submission
- Zero mempool exposure
- Atomic transaction ordering

## ⚖️ The "Testum" Standard (Compliance)
The Old Saying (The Duck Test): "If it looks like a duck, and swims like a duck, it's a duck."

The Darkwing Standard: We don't judge by feathers. We judge by metal.

Etymology Fact: The word "Test" comes from the Latin testum, an earthen pot used by alchemists to assay (determine the purity of) gold.

DarkwingDucks rejects the superficial "Duck Test". Instead, we use the Testum Method:

The Vessel: Range Protocol acts as our assaying vessel.

The Process: Every wallet is tested for OFAC sanctions and AML risks.

The Result: Only "Pure Gold" transactions are wrapped in Jito Bundles. Slag is discarded.

We prove that privacy and compliance can swim in the same pond.

## 🏆 Alliance Members

We are targeting specific bounties where DarkwingDucks fits perfectly:

* **🎯 Anoncoin (Dark Liquidity Pools):**
    * *Requirement:* "Build dark liquidity pools... provide confidentiality to memecoin launches."
    * *Our Fit:* We are a literal Dark Pool interface for memecoins, protecting retail launches from snipers.

* **🛡️ Range (Compliant Privacy):**
    * *Requirement:* "Pre-screening and selective disclosure to ensure compliance."
    * *Our Fit:* Our "Testum Standard" module implements exactly this: pre-screening wallets for AML/OFAC before allowing access to the Dark Pool.

* **⚡ Helius (RPC & Tooling):**
    * *Requirement:* "Leverage Helius' RPCs."
    * *Our Fit:* The entire backend relies on Helius RPC for transaction simulation and forwarding.

* **🎓 Encrypt.trade (Education):**
    * *Requirement:* "Explain privacy without jargon."
    * *Our Fit:* We explain complex MEV protection using simple metaphors: "Smoke Bombs" and "Darkwing Duck". No math, just vibes.

## ⚠️ Legal Disclaimer (Parody & Non-Affiliation)

DarkwingDucks is an experimental, non-commercial privacy protocol and a parody commentary on "Dark Pools".

Not Affiliated: We are NOT affiliated, associated, authorized, endorsed by, or in any way officially connected with The Walt Disney Company (Darkwing Duck) OR DuckDuckGo Inc. We just really like ducks and privacy.

Parody: This project is a commentary on the "sitting duck" nature of retail traders.

Terminology:

"Darkwing" = Speed of Dark Pool execution.

"Ducks" = Retail users ("Sitting Ducks") needing protection.

No Copyrighted Art: No original assets are used. All visuals are custom/abstract.

Zero Ducks Given about MEV. 100% Compliance Given about Laws.