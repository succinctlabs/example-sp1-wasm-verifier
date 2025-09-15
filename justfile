# List the justfile recipes.
default:
    @just --list --justfile {{justfile()}}

# Build wasm bindings for the verifier (dev mode)
build-wasm-dev:
    cd verifier && ${CARGO_HOME:-$HOME/.cargo/}bin/wasm-pack build --target nodejs --dev

# Build wasm bindings for the verifier (release mode)
build-wasm-release:
    cd verifier && ${CARGO_HOME:-$HOME/.cargo/}bin/wasm-pack build --target nodejs --release

# From an existing proof artifact, generate a JSON representation for a given proof mode (compressed/groth16/plonk).
gen-proof-json mode:
    cd example/script && RUST_LOG=info cargo run --release -- --mode {{mode}}

# Generate a proof artifact and its JSON representation for a given proof mode (compressed/groth16/plonk).
gen-proof-fresh mode:
    cd example/script && RUST_LOG=info cargo run --release -- --mode {{mode}} --prove

# From existing proof artifacts, generate JSON representations.
gen-proofs-json: (gen-proof-json "compressed") (gen-proof-json "groth16") (gen-proof-json "plonk")

# Generate proof artifacts and their JSON representations.
gen-proofs-fresh: (gen-proof-fresh "compressed") (gen-proof-fresh "groth16") (gen-proof-fresh "plonk")

# Install dependencies for wasm example
install-deps:
    cd example/wasm_example && pnpm install

# Run wasm verification test
test-wasm:
    cd example/wasm_example && pnpm run test

# Build and test end-to-end. Useful when setting up for the first time.
init: build-wasm-dev gen-proofs-fresh install-deps test-wasm
