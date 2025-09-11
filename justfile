# Build wasm bindings for the verifier (dev mode)
build-wasm-dev:
    cd verifier && ${CARGO_HOME:-$HOME/.cargo/}bin/wasm-pack build --target nodejs --dev

# Build wasm bindings for the verifier (release mode)
build-wasm-release:
    cd verifier && ${CARGO_HOME:-$HOME/.cargo/}bin/wasm-pack build --target nodejs --release

# Generate proof with specified mode (groth16 or plonk)
gen-proof mode:
    cd example/script && cargo run --release -- --mode {{mode}}

# Generate fresh proof with specified mode (groth16 or plonk)
gen-proof-fresh mode:
    cd example/script && cargo run --release -- --mode {{mode}} --prove

# Generate both proof types (using existing proofs)
gen-proofs: (gen-proof "groth16") (gen-proof "plonk")

# Generate both proof types (fresh proofs)
gen-proofs-fresh: (gen-proof-fresh "groth16") (gen-proof-fresh "plonk")

# Install dependencies for wasm example
install-deps:
    cd example/wasm_example && pnpm install

# Run wasm verification test
test-wasm:
    cd example/wasm_example && pnpm run test

# Full setup: build wasm, generate proofs, install deps, and test
setup: build-wasm-dev gen-proofs install-deps test-wasm

# Full setup with release build and fresh proofs
setup-release: build-wasm-release gen-proofs-fresh install-deps test-wasm
