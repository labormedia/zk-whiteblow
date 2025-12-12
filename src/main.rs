use curve25519_dalek::{
    RistrettoPoint, 
    Scalar, 
    constants::RISTRETTO_BASEPOINT_POINT as G, 
    traits::MultiscalarMul
};
use rand_core::{OsRng, RngCore, CryptoRngCore};
use sha2::{Digest, Sha512};

// ==============================================================================
//  MODULE: ZK TOOLKIT
// ==============================================================================
use twisted_el_gamal_non_interactive_schnorr_chaum_pedersen_toolkit as zk_toolkit;

// ==============================================================================
//  APPLICATION LAYER: WaaS (Whistleblowing as a Service)
// ==============================================================================

use zk_toolkit::{
    KeyPair, twisted_elgamal_decrypt, ConsistencyProof, CipherText, Message, ProtocolScheme
};

// The Value Proposition Logic
struct Economics {
    protocol_fee_percent: f64,
    bounty_percent: f64,
}

impl Economics {
    fn calculate_distribution(&self, recovered_amount: u64) {
        let bounty = (recovered_amount as f64 * self.bounty_percent) as u64;
        let protocol_revenue = (recovered_amount as f64 * self.protocol_fee_percent) as u64;
        let net_savings = recovered_amount - bounty - protocol_revenue;

        println!("\n--- 💰 FINANCIAL SETTLEMENT ---");
        println!("Expenditure Slashed (Recovered): ${}", recovered_amount);
        println!("> Bounty Paid to Reformer:       ${} ({}%)", bounty, self.bounty_percent * 100.0);
        println!("> Protocol Revenue (Tax):        ${} ({}%)", protocol_revenue, self.protocol_fee_percent * 100.0);
        println!("> Net Savings to Organization:   ${}", net_savings);
        println!("-------------------------------");
    }
}

// 2. The DAO Logic
struct Organization {
    name: String,
    keys: KeyPair,
}

impl Organization {
    fn new(name: &str) -> Self {
        println!("🏢 [Org] Initializing Integrity Shield for '{}'", name);
        Self {
            name: name.to_string(),
            keys: KeyPair::generate(),
        }
    }
}

// The Reformer Logic (Whistleblower)
struct Reformer;

impl Reformer {
    fn create_whiteblow(
        amount: u64, 
        org_pk: &RistrettoPoint
    ) -> (ConsistencyProof, CipherText, RistrettoPoint) {
        println!("🕵️  [Reformer] Detecting fraud of ${}...", amount);
        
        // 1. Encode the fraud amount (m)
        let m = &amount.to_le_bytes()[..];
        let message = Message::new(m);
        
        // 2. Create the ZK Scheme (Encrypt + Commit)
        // Note: Using the Org's Public Key (Y) to encrypt the evidence
        let scheme = ProtocolScheme::setup_with_pk(m, org_pk, &mut OsRng);
        
        println!("🔒 [Reformer] Generating ZK Consistency Proof...");
        println!("   - Encrypted Evidence (C1, C2)");
        println!("   - Public Commitment (Cm)");
        
        // 3. Build the Proof
        let proof = ConsistencyProof::build(&message, &scheme, &mut OsRng);
        
        (proof, scheme.ct, scheme.pedersen_commitment)
    }
}

// 4. The WaaS Protocol (The Verifier)
fn run_waas_protocol() {
    // A. Setup Economics
    let economics = Economics {
        protocol_fee_percent: 0.05, // 5% Protocol Tax
        bounty_percent: 0.10,       // 10% Bounty
    };

    // B. Organization Onboarding (SaaS subscription)
    let dao = Organization::new("BuildTheBridge DAO");
    
    // --- INCIDENT: FRAUD DETECTED ---
    let fraud_amount = 1_000_000; // $1M Fraud
    
    // C. Whistleblowing (Client-Side)
    // The reformer creates the proof using the DAO's public key
    let (proof, ciphertext, commitment) = Reformer::create_whiteblow(fraud_amount, &dao.keys.pk);

    // D. Protocol Verification (On-Chain / Smart Contract)
    println!("\n🤖 [Protocol] Verifying ZK Proof...");
    
    let is_valid = proof.validate();
    
    if is_valid {
        println!("✅ [Protocol] Proof VALID. Claim accepted.");
        println!("   The encrypted evidence matches the public commitment.");
        println!("   No private data (randomness/message) was leaked during verification.");
        
        // E. Resolution (Decryption & Payout)
        // In a real scenario, this happens after a challenge period or via Multi-Sig.
        // The Protocol uses the Org's Secret Key (or a shared key) to audit the amount.
        // TODO: The value proposition is strictly based on the correct implementation of asymmetric keys in this case.
        
        let decrypted_point = twisted_elgamal_decrypt(&dao.keys.get_secret(), &ciphertext);
        
        // Verify the amount matches the claimed $1M (simplified for demo)
        // In Ristretto, we verify M = m*G.
        let expected_point = Message::from(fraud_amount).point;
        
        if decrypted_point == expected_point {
            println!("🔓 [Protocol] Evidence Decrypted & Confirmed: Fraud Amount Correct.");
            economics.calculate_distribution(fraud_amount);
        } else {
            println!("❌ [Protocol] Decryption mismatch! Slashing Reformer stake.");
        }

    } else {
        println!("⛔ [Protocol] Proof INVALID. Rejecting submission.");
    }
}

fn main() {
    println!("--- WHISTLEBLOWING-AS-A-SERVICE (WaaS) PROTOCOL ---");
    run_waas_protocol();
}