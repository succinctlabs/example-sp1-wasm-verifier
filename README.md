# SP1 Wasm verification example

This repo demonstrates how to verify Groth16 and Plonk proofs in browser. We wrap the [`sp1-verifier`](https://github.com/succinctlabs/sp1) crate in wasm bindings, and invoke it from javascript.

## Repo overview

- `verifier`: The rust sp1 verifier crate with wasm bindings.
- `example/fibonacci-program`: A simple fibonacci SP1 program to verify.
- `example/fibonacci-script`: A simple script to generate proofs in a json format.
- `example/wasm_example`: A short javascript example that verifies proofs in wasm.

## Usage

### Wasm Bindings

First, generate the wasm library for the verifier. From the `verifier` directory, run

```bash
wasm-pack build --target nodejs --dev 
```

This will generate wasm bindings for the rust functions in [`verifier/src/lib.rs`](verifier/src/lib.rs).
> Note: generating wasm bindings in dev mode will result in drastically slower verification times.
> Generate bindings in release mode by replacing `--dev` with `--release`.

As an example, the following snippet provides wasm bindings for the `verify_groth16` function:

```rust,noplayground
#[wasm_bindgen]
pub fn verify_groth16(proof: &[u8], public_inputs: &[u8], sp1_vk_hash: &str) -> bool {
    Groth16Verifier::verify(proof, public_inputs, sp1_vk_hash, *GROTH16_VK_BYTES).is_ok()
}
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

Here, groth16 and plonk proofs are generated using `client.prove(&pk, stdin).groth16().run()` and `client.prove(&pk, stdin).plonk().run()`, respectively.
See the [SP1 docs](https://docs.rs/sp1-sdk/latest/sp1_sdk/struct.ProverClient.html) for more details.

From a [`SP1ProofWithPublicValues`](https://docs.rs/sp1-sdk/latest/sp1_sdk/proof/struct.SP1ProofWithPublicValues.html),
we extract the proof and public inputs, and serialize the appropriate fields. See the following snippet for details:

```rust,noplayground
// Load the proof and extract the proof and public inputs.
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
```

### Verify proofs in wasm

To verify proofs in wasm, run the following command from the `example/wasm_example` directory:

```bash
pnpm install
pnpm run test
```

This runs [`main.js`](example/wasm_example/main.js), which verifies the proofs in `example/json`.
The proofs are decoded from hex strings and verified using the wasm bindings. In addition, the public inputs
are deserialized into 32-bit integers and printed. See the following snippet for details:

```javascript
// Read and parse the JSON content of the file
const fileContent = fs.readFileSync(path.join("../json", file), 'utf8');
const proof_json = JSON.parse(fileContent);

// Determine the ZKP type (Groth16 or Plonk) based on the filename
const zkpType = file.toLowerCase().includes('groth16') ? 'groth16' : 'plonk';
const proof = fromHexString(proof_json.proof);
const public_inputs = fromHexString(proof_json.public_inputs);
const vkey_hash = proof_json.vkey_hash;

// Get the values using DataView.
const view = new DataView(public_inputs.buffer);

// Read each 32-bit (4 byte) integer as little-endian
const n = view.getUint32(0, true);
const a = view.getUint32(4, true);
const b = view.getUint32(8, true);

console.log(`n: ${n}`);
console.log(`a: ${a}`);
console.log(`b: ${b}`);

// Select the appropriate verification function and verification key based on ZKP type
const verifyFunction = zkpType === 'groth16' ? wasm.verify_groth16 : wasm.verify_plonk;

assert(verifyFunction(proof, public_inputs, vkey_hash));
console.log(`Proof in ${file} is valid.`);
```
