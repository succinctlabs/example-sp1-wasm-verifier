//! A simple script to generate proofs for the fibonacci program, and serialize them to JSON.

use clap::Parser;
use serde::{Deserialize, Serialize};
use sp1_sdk::{include_elf, utils, HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

/// The ELF (executable and linkable format) file for the fibonacci program.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

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
        default_value = "plonk",
        help = "Specifies the proof mode to use (e.g., groth16, plonk)"
    )]
    mode: String,
}

fn main() {
    // Setup logging for the application
    utils::setup_logger();

    // Parse command line arguments
    let args = Cli::parse();
    let mut stdin = SP1Stdin::new();
    stdin.write(&1000u32);

    // Initialize the prover client.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // These are the output paths.
    let proof_path = format!("../binaries/fibonacci_{}_proof.bin", args.mode);
    let json_path = format!("../json/fibonacci_{}_proof.json", args.mode);

    if args.prove {
        // Generate a proof for the specified program
        let proof = match args.mode.as_str() {
            "groth16" => client
                .prove(&pk, stdin)
                .groth16()
                .run()
                .expect("Groth16 proof generation failed"),
            "plonk" => client
                .prove(&pk, stdin)
                .plonk()
                .run()
                .expect("Plonk proof generation failed"),
            _ => panic!("Invalid proof mode. Use 'groth16' or 'plonk'."),
        };
        proof.save(&proof_path).expect("Failed to save proof");
    }
    // Load the proof, extract the proof and public inputs, and serialize the appropriate fields.
    let proof = SP1ProofWithPublicValues::load(&proof_path).expect("Failed to load proof");
    let fixture = ProofData {
        proof: hex::encode(proof.bytes()),
        public_inputs: hex::encode(proof.public_values),
        vkey_hash: vk.bytes32(),
        mode: args.mode,
    };

    // Serialize the proof data to a JSON file.
    let json_proof = serde_json::to_string(&fixture).expect("Failed to serialize proof");
    std::fs::write(json_path, json_proof).expect("Failed to write JSON proof");

    println!("Successfully generated json proof for the program!")
}
