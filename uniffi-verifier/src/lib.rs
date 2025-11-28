//! A simple wrapper around the `sp1_verifier` crate.

use sp1_verifier::{
    CompressedError, CompressedVerifier, Groth16Error, Groth16Verifier, PlonkError, PlonkVerifier,
    GROTH16_VK_BYTES, PLONK_VK_BYTES,
};

uniffi::setup_scaffolding!("verifier");

/// Wrapper around [`sp1_verifier::Groth16Verifier::verify`].
///
/// We hardcode the Groth16 VK bytes to only verify SP1 proofs.
pub fn verify_groth16(
    proof: &[u8],
    public_inputs: &[u8],
    sp1_vk_hash: &str,
) -> Result<bool, Groth16Error> {
    Groth16Verifier::verify(proof, public_inputs, sp1_vk_hash, *GROTH16_VK_BYTES)?;

    Ok(true)
}

/// Wrapper around [`sp1_verifier::PlonkVerifier::verify`].
///
/// We hardcode the Plonk VK bytes to only verify SP1 proofs.
pub fn verify_plonk(
    proof: &[u8],
    public_inputs: &[u8],
    sp1_vk_hash: &str,
) -> Result<bool, PlonkError> {
    PlonkVerifier::verify(proof, public_inputs, sp1_vk_hash, *PLONK_VK_BYTES)?;

    Ok(true)
}

/// Wrapper around [`sp1_verifier::CompressedVerifier::verify_sp1_proof`].
///
/// We hardcode the Plonk VK bytes to only verify SP1 proofs.
pub fn verify_compressed(
    proof: &[u8],
    public_inputs: &[u8],
    sp1_vk_hash: &[u8],
) -> Result<bool, CompressedError> {
    CompressedVerifier::verify_sp1_proof(proof, public_inputs, sp1_vk_hash)?;

    Ok(true)
}
