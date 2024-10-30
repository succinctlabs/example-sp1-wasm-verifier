# SP1 Wasm verification example

This repo demonstrates how to verify Groth16 and Plonk proofs in browser. We wrap the [`sp1-verifier`](https://github.com/succinctlabs/sp1) crate in wasm bindings, and invoke it from javascript.

## Repo overview

- `verifier`: The rust sp1 verifier crate with wasm bindings.
- `example/fibonacci-program`: A simple fibonacci SP1 program to verify.
- `example/fibonacci-script`: A simple script to generate proofs in a json format.
- `example/wasm_example`: A short javascript example that verifies proofs in wasm.

## Usage

### Wasm Bindings

First, generate the wasm library for the verifier. From the repository root, run:

```bash
wasm-pack build --target nodejs --dev 
```

### Generate proofs

Next, run the script to generate `fibonacci_groth16_proof.json` and `fibonacci_plonk_proof.json`. From the `example/script` directory, run:

```bash
cargo run --release -- --mode groth16
cargo run --release -- --mode plonk
```

By default, this will *not* generate fresh proofs from the program in `example/fibonacci-program`. To generate fresh proofs, run:

```bash
cargo run --release -- --mode groth16 --prove
cargo run --release -- --mode plonk --prove
```

### Verify proofs in wasm

To verify proofs in wasm, run the following command from the `example/wasm_example` directory:

```bash
pnpm install
pnpm run test
```
