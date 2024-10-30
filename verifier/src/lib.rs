use sp1_verifier::{Groth16Verifier, PlonkVerifier, GROTH16_VK_BYTES, PLONK_VK_BYTES};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn verify_groth16(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    Groth16Verifier::verify(proof, public_inputs, sp1_vk_hash, *GROTH16_VK_BYTES).is_ok()
}

#[wasm_bindgen]
pub fn verify_plonk(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    PlonkVerifier::verify(proof, public_inputs, sp1_vk_hash, *PLONK_VK_BYTES).is_ok()
}
