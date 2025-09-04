#!/bin/bash
set -euo pipefail

# Activate local Python env providing the generator
. ../env/bin/activate

# Run the generator from the linkml checkout
cd ../linkml
gen-rust ../rust-linkml-core/src/schemaview/tests/data/meta.yaml \
  --output ../rust-linkml-core/src/metamodel/ \
  --force --serde -n linkml_meta --stacktrace

# Reformat the Rust workspace to clean up generated code
cd ../rust-linkml-core
echo "Running cargo fmt over the workspace..."
cargo fmt --all
