import * as wasm from "../../verifier/pkg/sp1_wasm_verifier.js"
import fs from 'node:fs'
import path from 'node:path'
import assert from 'node:assert'

// Convert a hexadecimal string to a Uint8Array
export const fromHexString = (hexString) =>
    Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));

const files = fs.readdirSync("../json");

// Iterate through each file in the data directory
for (const file of files) {
    try {
        // Read and parse the JSON content of the file
        const fileContent = fs.readFileSync(path.join("../json", file), 'utf8');

        const proof_json = JSON.parse(fileContent);

        // Determine the ZKP type (Groth16 or Plonk) based on the filename
        const zkpType = file.toLowerCase().includes('groth16') ? 'groth16' : 'plonk';
        const proof = fromHexString(proof_json.proof);
        const public_inputs = fromHexString(proof_json.public_inputs);
        const vkey_hash = proof_json.vkey_hash;

        // Select the appropriate verification function and verification key based on ZKP type
        const verifyFunction = zkpType === 'groth16' ? wasm.verify_groth16 : wasm.verify_plonk;

        assert(verifyFunction(proof, public_inputs, vkey_hash));
        console.log(`Proof in ${file} is valid.`);
    } catch (error) {
        console.error(`Error processing ${file}: ${error.message}`);
    }
}