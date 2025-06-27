# Prototype: Moving from Rio to Oxrdf

This repository previously used the `rio` crate for RDF serialization. As an experiment the runtime crate has been updated to use the `oxrdf` and `oxttl` libraries from the Oxigraph project. This change replaces the `rio_turtle` formatter with `oxttl::TurtleSerializer` and switches to `oxrdf` data types for triples.

The conversion required adding `oxrdf` and `oxttl` as dependencies and refactoring the `turtle` module. The new serializer builds triples using `oxrdf` structures and writes them using `oxttl`. Existing tests still pass, but the API should be considered experimental while the migration is evaluated.
