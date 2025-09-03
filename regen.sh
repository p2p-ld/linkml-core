#!/bin/bash
. ../env/bin/activate
cd ../linkml
gen-rust ../rust-linkml-core/src/schemaview/tests/data/meta.yaml  --output ../rust-linkml-core/src/metamodel/ --force --serde -n linkml_meta --stacktrace

