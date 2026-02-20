//! A simple wrapper around the `sp1_verifier` crate.

use sp1_verifier::{
    compressed::SP1CompressedVerifierRaw, Groth16Verifier, PlonkVerifier, GROTH16_VK_BYTES,
    PLONK_VK_BYTES,
};

use crate::error::Error;

mod error;

uniffi::setup_scaffolding!();

/// Wrapper around [`sp1_verifier::Groth16Verifier::verify`].
///
/// We hardcode the Groth16 VK bytes to only verify SP1 proofs.
#[uniffi::export]
pub fn verify_groth16(
    proof: &[u8],
    public_inputs: &[u8],
    sp1_vk_hash: &str,
) -> Result<bool, Error> {
    Groth16Verifier::verify(proof, public_inputs, sp1_vk_hash, *GROTH16_VK_BYTES)?;

    Ok(true)
}

/// Wrapper around [`sp1_verifier::PlonkVerifier::verify`].
///
/// We hardcode the Plonk VK bytes to only verify SP1 proofs.
#[uniffi::export]
pub fn verify_plonk(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> Result<bool, Error> {
    PlonkVerifier::verify(proof, public_inputs, sp1_vk_hash, *PLONK_VK_BYTES)?;

    Ok(true)
}

/// Wrapper around [`sp1_verifier::CompressedVerifier::verify_sp1_proof`].
///
/// We hardcode the Plonk VK bytes to only verify SP1 proofs.
#[uniffi::export]
pub fn verify_compressed(
    proof: &[u8],
    public_inputs: &[u8],
    sp1_vk_hash: &[u8],
) -> Result<bool, Error> {
    SP1CompressedVerifierRaw::verify_with_public_values(proof, public_inputs, sp1_vk_hash)?;

    Ok(true)
}
