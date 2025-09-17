# SP1 Wasm verification example

This repo demonstrates how to verify "compressed," Groth16, and Plonk proofs in a browser. We wrap the [`sp1-verifier`](https://github.com/succinctlabs/sp1) crate in Wasm bindings, and invoke it from JavaScript (Node.js).

In SP1, proofs are produced in multiple stages:
- SP1 splits a single execution of a RISC-V program into several "shards" and proves each of them independently.
- SP1 combines these proofs through a process called "recursion" into a single proof, which we call a "compressed" proof.
- Finally, SP1 wraps the compressed proof in a non-interactive ZK (NIZK) proof, which may either be Groth16 or Plonk.

The `sp1-verifier` crate supports verifying proofs from either of the last two stages, which are precisely the kind of proofs that describe the entirety of a RISC-V program's execution.

> ⚠️ NOTE: At the moment, `sp1-verifier` does not support single-shard compressed proofs. This is only of concern for short program executions, since long program executions inevitably require multiple shards. Furthermore, `sp1-verifier` provides an informative error when it detects a single-shard compressed proof.

## Repo overview

- `verifier`: A thin wrapper around `sp1-verifier` used to generate a Wasm module and bindings.
- `example/fibonacci-program`: A simple fibonacci SP1 program to verify.
- `example/fibonacci-script`: A simple script to generate proofs in a JSON format.
- `example/wasm_example`: A short JavaScript example that verifies proofs in Wasm.

## Usage

### TL;DR

Use the [Just](https://github.com/casey/just) recipes in the `justfile`. Given a fresh clone of this repository, the `init` recipe performs all tasks needed to setup and run the example.

To manually perform these tasks, follow the instructions below.

### Wasm Bindings

First, generate the Wasm library for the verifier. From the `verifier` directory, run

```bash
wasm-pack build --target nodejs --dev 
```

This will generate Wasm bindings for the Rust functions in [`verifier/src/lib.rs`](verifier/src/lib.rs).

> ⚠️ NOTE: Generating Wasm bindings in dev mode will result in drastically slower verification times.
> Generate bindings in release mode by replacing `--dev` with `--release`.

As an example, the following snippet provides Wasm bindings for the `verify_groth16` function:

```rust,noplayground
#[wasm_bindgen]
pub fn verify_groth16(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    Groth16Verifier::verify(proof, public_inputs, sp1_vk_hash, *GROTH16_VK_BYTES).is_ok()
}
```

### Generate proofs

To generate and save proofs for the first time, in the `example/script` directory, run `cargo run --release -- --mode <mode> --prove`, where `<mode>` is one of `groth16, plonk, compressed`. This will do two things:

1. Using SP1, generate and save proofs to the `example/binaries` directory.
2. Encode the saved proofs to JSON and write them to `example/json`. 

After the proofs are saved to `example/binaries` for the first time, one may omit the `--prove` flag to skip the first step.

For the proof generation, saving, and serialization logic, see [`example/script/src/main.rs`](example/script/src/main.rs).

### Verify proofs in Wasm

To verify proofs in Wasm, run the following commands from the `example/wasm_example` directory:

```bash
# Install Node.js dependencies.
pnpm install
# Run the Node.js test program.
pnpm run test
```

This runs [`main.js`](example/wasm_example/main.js), which first reads the proof data from the artifacts in `example/json` into a Wasm-friendly format. Then, for each proof, it calls the appropriate verification function via the generated Wasm bindings. Finally, it prints verification durations and (if they are easily available) the public values.
