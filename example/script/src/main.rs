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
    include_elf, utils, HashableKey, ProverClient, SP1Proof, SP1ProofWithPublicValues, SP1Stdin,
};

/// The ELF (executable and linkable format) file for the fibonacci program.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

#[derive(Serialize, Deserialize)]
struct ProofData {
    proof: String,                 // hex string
    public_inputs: Option<String>, // hex string
    vkey_hash: String,             // vk.bytes32()
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

fn main() -> Result<()> {
    // Setup logging for the application
    utils::setup_logger();

    // Parse command line arguments
    let args = Cli::parse();
    let mut stdin = SP1Stdin::new();
    stdin.write(&1666667u32);
    // stdin.write(&1000u32); // Use this to obtain a single-shard proof.

    // Initialize the prover client.
    let client = ProverClient::from_env();
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // These are the output paths.
    let proof_path = format!("../binaries/fibonacci_{}_proof.bin", args.mode);
    let json_path = format!("../json/fibonacci_{}_proof.json", args.mode);

    if args.prove {
        // Generate a proof for the specified program
        let proof = match args.mode {
            Mode::Groth16 => client
                .prove(&pk, &stdin)
                .groth16()
                .run()
                .context("Groth16 proof generation failed")?,
            Mode::Plonk => client
                .prove(&pk, &stdin)
                .plonk()
                .run()
                .context("Plonk proof generation failed")?,
            Mode::Compressed => client
                .prove(&pk, &stdin)
                .compressed()
                .run()
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
                public_inputs: None,
                vkey_hash: hex::encode(bincode::serialize(&vk.hash_babybear())?),
                mode: args.mode.to_string(),
            }
        }
        _ => {
            let proof = SP1ProofWithPublicValues::load(&proof_path).expect("Failed to load proof");
            ProofData {
                proof: hex::encode(proof.bytes()),
                public_inputs: Some(hex::encode(proof.public_values)),
                vkey_hash: vk.bytes32(),
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
