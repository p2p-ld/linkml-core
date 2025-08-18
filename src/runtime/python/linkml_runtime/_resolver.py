"""Schema import resolution implemented in pure Python."""

from pathlib import Path
from urllib.request import urlopen


_KNOWN_IMPORTS = {
    "https://w3id.org/linkml/mappings": "https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/mappings.yaml",
    "https://w3id.org/linkml/types": "https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/types.yaml",
    "https://w3id.org/linkml/extensions": "https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/extensions.yaml",
    "https://w3id.org/linkml/annotations": "https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/annotations.yaml",
    "https://w3id.org/linkml/units": "https://raw.githubusercontent.com/linkml/linkml-model/refs/heads/main/linkml_model/model/schema/units.yaml",
}


def resolve_schemas(self) -> None:
    """Resolve any imported schemas using Python's ``urllib``.

    This mirrors the Rust implementation but avoids a dependency on ``reqwest``
    by using the standard library for network access.
    """

    for schema_id, uri in self.get_unresolved_schema_refs():
        target = _KNOWN_IMPORTS.get(uri, uri)
        if Path(target).exists():
            text = Path(target).read_text()
        else:
            with urlopen(target) as resp:  # nosec: B310 - controlled URLs
                text = resp.read().decode("utf-8")
        self.add_schema_str_with_import_ref(text, schema_id, uri)

