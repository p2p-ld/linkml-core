use curies::{Converter, Record};
use linkml_schemaview::identifier::{Curie, Identifier, IdentifierError, Uri};

#[test]
fn newtype_into_and_try_from() {
    let uri = Uri("http://example.org/1".to_string());
    let id: Identifier = uri.clone().into();
    assert_eq!(id, Identifier::Uri(uri.clone()));
    let back: Uri = id.clone().try_into().unwrap();
    assert_eq!(back, uri);

    let curie = Curie("ex:1".to_string());
    let idc: Identifier = curie.clone().into();
    assert_eq!(idc, Identifier::Curie(curie.clone()));
    let backc: Curie = idc.clone().try_into().unwrap();
    assert_eq!(backc, curie);

    let name = Identifier::new("foo");
    assert!(Curie::try_from(name.clone()).is_err());
    assert!(Uri::try_from(name).is_err());
}

#[test]
fn workflow_curie_uri_roundtrip() {
    let mut conv = Converter::default();
    let mut record = Record::new("ex", "http://example.org/");
    record.prefix_synonyms.insert("example".to_string());
    conv.add_record(record).unwrap();

    let id = Identifier::new("ex:1");
    assert_eq!(
        id.to_uri(&conv).unwrap(),
        Uri("http://example.org/1".to_string())
    );

    let id2 = Identifier::new("http://example.org/1");
    assert_eq!(id2.to_curie(&conv).unwrap(), Curie("ex:1".to_string()));

    let id3 = Identifier::new("example:2");
    assert_eq!(
        id3.to_uri(&conv).unwrap(),
        Uri("http://example.org/2".to_string())
    );
}

#[test]
fn workflow_name_fails() {
    let conv = Converter::default();
    let id = Identifier::new("SomeClass");
    assert!(matches!(
        id.to_uri(&conv),
        Err(IdentifierError::NameNotResolvable)
    ));
    assert!(matches!(
        id.to_curie(&conv),
        Err(IdentifierError::NameNotResolvable)
    ));
}
