//! A simple wrapper around the `sp1_verifier` crate.

use sp1_verifier::{Groth16Verifier, PlonkVerifier, GROTH16_VK_BYTES, PLONK_VK_BYTES};
use wasm_bindgen::prelude::wasm_bindgen;

/// Wrapper around [`sp1_verifier::Groth16Verifier::verify`].
///
/// We hardcode the Groth16 VK bytes to only verify SP1 proofs.
#[wasm_bindgen]
pub fn verify_groth16(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    Groth16Verifier::verify(proof, public_inputs, sp1_vk_hash, *GROTH16_VK_BYTES).is_ok()
}

/// Wrapper around [`sp1_verifier::PlonkVerifier::verify`].
///
/// We hardcode the Plonk VK bytes to only verify SP1 proofs.
#[wasm_bindgen]
pub fn verify_plonk(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    PlonkVerifier::verify(proof, public_inputs, sp1_vk_hash, *PLONK_VK_BYTES).is_ok()
}
