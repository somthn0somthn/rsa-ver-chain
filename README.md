# Lodestar RSA Verification Chain

A proof-of-concept implementation for on-chain JWT verification using Substrate and Polkadot SDK.

## Overview

Onboarding new users remains one of the thorniest challenges in Web3. Lodestar aims to improve this situation by providing web-based authentication schemes for dApps and application chains, which could be used in composition with other components to create a wallet-less blockchain experience like that seen with XION.

This RSA-ver-chain repository serves as a first-step proof of concept for the most important aspect of JWT verification on-chain: RSA signature verification. The project demonstrates how to implement and use cryptographic signature verification within a Substrate-based blockchain.

## Key Components

- **Custom Pallet Implementation**: A specialized pallet that provides RSA signature verification functionality, allowing on-chain validation of messages signed with RSA private keys.

- **Main Extrinsics**:
  - `validate_string`: A simple function that verifies if an input string equals "hello"
  - `verify_rsa_signature`: A more complex function that performs cryptographic verification of RSA signatures

- **Parachain Integration**: Upon completion, the JWT verification pallet (also known as webAuth pallet) will be able to be integrated into a full parachain runtime, demonstrating how cryptographic verification can be part of a production blockchain environment.

- **Benchmarking**: Implementation of performance benchmarks to accurately measure the computational cost of cryptographic operations, ensuring proper transaction fee calculation.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- [Substrate development environment](https://docs.substrate.io/install/)

### Build the Project

```bash
cd rsa-ver-chain && cargo build --release
```

### Run Tests

```bash
cd rsa-ver-chain && cargo test
```

### Code Quality

Check linting:
```bash
cd rsa-ver-chain && cargo clippy
```

Format code:
```bash
cd rsa-ver-chain && cargo fmt
```

## Running the Chain

### Create Chain Spec

```bash
chain-spec-builder create --relay-chain "rococo-local" --para-id 1000 --runtime target/release/wbuild/rsa-verification-parachain-runtime/rsa_verification_parachain_runtime.wasm named-preset development
```

### Run Node

```bash
polkadot-omni-node --chain chain_spec.json --dev --dev-block-time 6000
```

## Architecture

This project is structured as a typical Substrate parachain with:
- A custom pallet for RSA signature verification
- Integration with Substrate's runtime
- Test coverage for verification functionality

The RSA verification logic is implemented in the template pallet, which can be adapted for use in any Substrate-based blockchain.

## Development Roadmap

This proof of concept is the first step toward a comprehensive webAuth pallet that will:
1. Support complete JWT verification on-chain
2. Integrate with web-based authentication schemes
3. Provide a simple developer experience for blockchain applications incorporating it

## License

See [LICENSE](LICENSE) for details.

