#!/bin/bash
set -e
cd "$(dirname -- $0)"

# NOTE: This sample is from a chain where the `action-return-value` protocol feature is ENABLED...

# Build the tool...
cargo b -r

# Run the example...
../target/release/eos_action_proof_maker \
generate \
--file=./example-2-submission-material.json

# Clean up...
rm -r ./logs
