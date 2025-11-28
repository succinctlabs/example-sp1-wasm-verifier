use sp1_verifier::{CompressedError, Groth16Error, PlonkError};
use thiserror::Error;

#[derive(Debug, Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum Error {
    #[error("{0}")]
    Groth16(#[from] Groth16Error),
    #[error("{0}")]
    Plonk(#[from] PlonkError),
    #[error("{0}")]
    Compressed(#[from] CompressedError),
}
