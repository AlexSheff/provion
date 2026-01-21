use methods::{PROVION_GUEST_ELF, PROVION_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    println!("💠 PROVION Host: Generating Consensus Proof...");

    // 1. Mock Data (Maintainer)
    let maintainer_key = [0u8; 32].to_vec(); 
    let maintainer_sig = [0u8; 64].to_vec(); 
    let commit_hash = b"commit-84b90f72".to_vec();

    // 2. Mock Data (Reviewers - 2 Signatures)
    let reviewer_keys = vec![[0u8; 32].to_vec(), [0u8; 32].to_vec()];
    let reviewer_sigs = vec![[0u8; 64].to_vec(), [0u8; 64].to_vec()];

    println!("📝 Input: Commit Hash = {:?}", String::from_utf8_lossy(&commit_hash));
    println!("👥 Reviewers: {}", reviewer_keys.len());

    // 3. Build Execution Environment
    let env = ExecutorEnv::builder()
        .write(&maintainer_key).unwrap()
        .write(&maintainer_sig).unwrap()
        .write(&commit_hash).unwrap()
        .write(&reviewer_keys).unwrap()
        .write(&reviewer_sigs).unwrap()
        .build()
        .unwrap();

    // 4. Generate Proof
    println!("⚙️  Running zkVM Guest (Verifying Consensus)...");
    let prover = default_prover();
    
    match prover.prove(env, PROVION_GUEST_ELF) {
        Ok(receipt) => {
            receipt.verify(PROVION_GUEST_ID).expect("Code verification failed");
            println!("✅ Proof Generated & Verified!");
            println!("   - Maintainer Signature: VALID");
            println!("   - Reviewer Consensus: VALID ({}/2)", reviewer_keys.len());
        },
        Err(e) => {
            println!("⚠️  Proof Generation Expectedly Failed (Invalid Mock Sigs): {}", e);
        }
    }
}
