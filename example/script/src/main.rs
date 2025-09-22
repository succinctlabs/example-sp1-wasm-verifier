//! A simple script to generate proofs for the fibonacci program, and serialize them to JSON.

use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter},
};

use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use flate2::{bufread::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use sp1_sdk::{
    include_elf, prelude::*, utils, ProverClient, SP1Proof, SP1ProofWithPublicValues, SP1Stdin,
};

/// The ELF (executable and linkable format) file for the fibonacci program.
pub const FIBONACCI_ELF: Elf = include_elf!("fibonacci-program");

#[derive(Serialize, Deserialize)]
struct ProofData {
    proof: String,         // hex string
    public_inputs: String, // hex string
    vkey_hash: String,     // vk.bytes32()
    mode: String,
}

#[derive(clap::Parser)]
#[command(name = "zkVM Proof Generator")]
struct Cli {
    #[arg(
        long,
        value_name = "prove",
        default_value_t = false,
        help = "Whether to generate a proof or use the pregenerated proof"
    )]
    prove: bool,

    #[arg(
        long,
        value_name = "mode",
        value_enum,
        help = "Specifies the proof mode to use"
    )]
    mode: Mode,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Plonk,
    Groth16,
    Compressed,
}

impl Mode {
    fn as_str(&self) -> &'static str {
        match self {
            Mode::Plonk => "plonk",
            Mode::Groth16 => "groth16",
            Mode::Compressed => "compressed",
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging for the application
    utils::setup_logger();

    // Parse command line arguments
    let args = Cli::parse();
    let mut stdin = SP1Stdin::new();
    stdin.write(&match args.mode {
        // Multi-shard proof.
        Mode::Compressed => 1666667u32,
        // Single-shard proof.
        _ => 1000u32,
    });

    // Initialize the prover client.
    let client = ProverClient::from_env().await;
    let pk = client.setup(FIBONACCI_ELF).await?;

    // These are the output paths.
    let proof_path = format!("../binaries/fibonacci_{}_proof.bin", args.mode);
    let json_path = format!("../json/fibonacci_{}_proof.json", args.mode);

    if args.prove {
        // Generate a proof for the specified program
        let proof = match args.mode {
            Mode::Groth16 => client
                .prove(&pk, stdin)
                .groth16()
                .await
                .context("Groth16 proof generation failed")?,
            Mode::Plonk => client
                .prove(&pk, stdin)
                .plonk()
                .await
                .context("Plonk proof generation failed")?,
            Mode::Compressed => client
                .prove(&pk, stdin)
                .compressed()
                .await
                .context("Compressed proof generation failed")?,
        };

        match args.mode {
            Mode::Compressed => {
                // For the time being, we just (de)serialize the entire `SP1ReduceProof`.
                let file = File::create(&proof_path).with_context(|| {
                    format!("failed to create file for saving proof: {proof_path}")
                })?;
                // Compress the "compressed" proofs.
                let mut file = GzEncoder::new(BufWriter::new(file), Compression::default());
                bincode::serialize_into(&mut file, &proof).context("Failed to save proof")?;
            }
            _ => proof.save(&proof_path).expect("Failed to save proof"),
        }
    }

    // Load the proof, extract the proof and public inputs, and serialize the appropriate fields.
    let fixture = match args.mode {
        Mode::Compressed => {
            // For the time being, we just (de)serialize the entire `SP1ReduceProof`.
            let path = &proof_path;
            // Try to load a [`Self`] from the file.
            let file = File::open(path)
                .with_context(|| format!("failed to open file for loading proof: {}", path))?;
            let file = GzDecoder::new(BufReader::new(file));
            let proof: SP1ProofWithPublicValues =
                bincode::deserialize_from(file).context("Failed to load proof")?;

            let reduce_proof = match proof.proof {
                SP1Proof::Compressed(p) => p,
                other => bail!("unexpected proof: {other:?}"),
            };

            ProofData {
                proof: hex::encode(bincode::serialize(&reduce_proof)?),
                public_inputs: hex::encode(proof.public_values),
                vkey_hash: hex::encode(bincode::serialize(&pk.verifying_key().hash_koalabear())?),
                mode: args.mode.to_string(),
            }
        }
        _ => {
            let proof = SP1ProofWithPublicValues::load(&proof_path).expect("Failed to load proof");
            ProofData {
                proof: hex::encode(proof.bytes()),
                public_inputs: hex::encode(proof.public_values),
                vkey_hash: pk.verifying_key().bytes32(),
                mode: args.mode.to_string(),
            }
        }
    };

    // Serialize the proof data to a JSON file.

    let file = File::create(&json_path)
        .with_context(|| format!("failed to create file for saving proof: {proof_path}"))?;
    serde_json::to_writer(file, &fixture).context("Failed to serialize and save proof")?;

    println!("Successfully generated json proof for the program!");

    Ok(())
}
