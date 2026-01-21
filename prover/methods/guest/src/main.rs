use risc0_zkvm::guest::env;
use ed25519_dalek::{Verifier, VerifyingKey, Signature};

fn main() {
    // Read Inputs:
    // 1. Maintainer PubKey (32 bytes)
    // 2. Maintainer Signature (64 bytes)
    // 3. Commit Hash (Message)
    // 4. Reviewer PubKeys (List)
    // 5. Reviewer Signatures (List)
    
    let (maintainer_key_bytes, maintainer_sig_bytes, msg_bytes, reviewer_keys_bytes, reviewer_sigs_bytes): 
        (Vec<u8>, Vec<u8>, Vec<u8>, Vec<Vec<u8>>, Vec<Vec<u8>>) = env::read();

    // 1. Verify Maintainer (The Merge)
    let maintainer_key = VerifyingKey::from_bytes(&maintainer_key_bytes[..].try_into().unwrap()).expect("Invalid Maintainer Key");
    let maintainer_sig = Signature::from_bytes(&maintainer_sig_bytes[..].try_into().unwrap()).expect("Invalid Maintainer Sig");
    maintainer_key.verify(&msg_bytes, &maintainer_sig).expect("Maintainer Signature Failed");

    // 2. Verify Reviewer Consensus (The Quality Control)
    if reviewer_keys_bytes.len() != reviewer_sigs_bytes.len() {
        panic!("Reviewer Data Mismatch");
    }

    if reviewer_keys_bytes.is_empty() {
        panic!("No Reviewers Signed");
    }

    for (i, key_bytes) in reviewer_keys_bytes.iter().enumerate() {
        let r_key = VerifyingKey::from_bytes(&key_bytes[..].try_into().unwrap()).expect("Invalid Reviewer Key");
        let r_sig = Signature::from_bytes(&reviewer_sigs_bytes[i][..].try_into().unwrap()).expect("Invalid Reviewer Sig");
        r_key.verify(&msg_bytes, &r_sig).expect("Reviewer Signature Failed");
    }
    
    // Commit the Commit Hash as Public Output
    env::commit(&msg_bytes);
}
