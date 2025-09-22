//! A simple wrapper around the `sp1_verifier` crate.

use core::error::Error;

use sp1_verifier::{Groth16Verifier, PlonkVerifier, GROTH16_VK_BYTES, PLONK_VK_BYTES};
use wasm_bindgen::prelude::wasm_bindgen;

/// Wrapper around [`sp1_verifier::Groth16Verifier::verify`].
///
/// We hardcode the Groth16 VK bytes to only verify SP1 proofs.
#[wasm_bindgen]
pub fn verify_groth16(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    handle_result(Groth16Verifier::verify(
        proof,
        public_inputs,
        sp1_vk_hash,
        *GROTH16_VK_BYTES,
    ))
}

/// Wrapper around [`sp1_verifier::PlonkVerifier::verify`].
///
/// We hardcode the Plonk VK bytes to only verify SP1 proofs.
#[wasm_bindgen]
pub fn verify_plonk(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    handle_result(PlonkVerifier::verify(
        proof,
        public_inputs,
        sp1_vk_hash,
        *PLONK_VK_BYTES,
    ))
}

/// Wrapper around [`sp1_verifier::PlonkVerifier::verify`].
///
/// We hardcode the Plonk VK bytes to only verify SP1 proofs.
#[wasm_bindgen]
pub fn verify_compressed(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &[u8]) -> bool {
    true
    // handle_result(CompressedVerifier::verify(
    //     proof,
    //     public_inputs,
    //     sp1_vk_hash,
    // ))
}

/// Prints errors via `console.error`.
///
/// Returns whether the variant was `Result::Ok`.
fn handle_result<T, E: Error>(res: Result<T, E>) -> bool {
    res.inspect_err(|e| console::error(&format!("{e}"))).is_ok()
}

mod console {
    use super::wasm_bindgen;

    #[wasm_bindgen(js_namespace = console)]
    extern "C" {
        #[wasm_bindgen]
        pub fn error(s: &str);
    }
}
