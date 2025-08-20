# linkml-core

core linkml schema operations written in rust :)

## Regenerating the metamodel

In order to regenerate the metamodel:

* Make sure you have a python virtual env with linkml_runtime (python!) installed, and that its active
* In the `../linkml` folder there should be a linkml checkout that is on a branch with the rust generator
* run the `regen.sh` script from the root of this repo

Note that now the metamodel is generated from src/schemaview/tests/data/meta.yaml.

### TODOs

* generate the metamodel directly from the linkml meta repository

## Development on the python bindings

1. Create a virtual env and activate it
2. Install maturin (pip install maturin)
3. Go to src/runtime and run `maturin develop`

Now the `linkml_runtime_rust` module should be accessible
