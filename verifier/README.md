# SP1 Wasm verification

This repo wraps the [`sp1-verifier`](https://github.com/succinctlabs/sp1) crate in Wasm bindings, and invoke it from JavaScript (Node.js).

## Usage

Install the node module with the following command:

```bash
npm install sp1-wasm-verifier
```

`sp1-wasm-verifier` declare the following functions:

```js
function verify_groth16(proof: Uint8Array, public_inputs: Uint8Array, sp1_vk_hash: string): boolean;
function verify_plonk(proof: Uint8Array, public_inputs: Uint8Array, sp1_vk_hash: string): boolean
function verify_compressed(proof: Uint8Array, public_inputs: Uint8Array, sp1_vk_hash: Uint8Array): boolean;
```
