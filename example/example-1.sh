#!/bin/bash
set -e
cd "$(dirname -- $0)"

# Build the tool, disabling the action-returen feature...
cargo b -r --features=disable-action-return-value-protocol-feature

# Run the example...
../target/release/eos_action_proof_maker \
generate \
--file=./example-1-submission-material.json

# Clean up...
rm -r ./logs
