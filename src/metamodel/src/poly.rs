#![allow(non_camel_case_types)]

use crate::*;
#[cfg(feature = "serde")]
use serde_yml as _ ;
use chrono::NaiveDateTime;
#[cfg(feature = "pyo3")]
use pyo3::{FromPyObject,prelude::*};
#[cfg(feature = "serde")]
use serde::{Deserialize,Serialize,de::IntoDeserializer};
use serde_value::Value;
#[cfg(feature = "serde")]
use serde_path_to_error;
use std::collections::HashMap;
use std::collections::BTreeMap;

pub trait AnyValue  {


}

impl AnyValue for crate::AnyValue {
}

pub trait Extension  {

    fn extension_tag(&self) -> &uriorcurie;
    // fn extension_tag_mut(&mut self) -> &mut &uriorcurie;
    // fn set_extension_tag(&mut self, value: uriorcurie);

    fn extension_value(&self) -> &crate::AnyValue;
    // fn extension_value_mut(&mut self) -> &mut &crate::AnyValue;
    // fn set_extension_value<E>(&mut self, value: E) where E: Into<AnyValue>;

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;


}

impl Extension for crate::Extension {
        fn extension_tag(&self) -> &uriorcurie {
        return &self.extension_tag;
    }
        fn extension_value(&self) -> &crate::AnyValue {
        return &self.extension_value;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
// mapping
        return poly_containers::MapView::new(&self.extensions);
    }
}
impl Extension for crate::Annotation {
        fn extension_tag(&self) -> &uriorcurie {
        return &self.extension_tag;
    }
        fn extension_value(&self) -> &crate::AnyValue {
        return &self.extension_value;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}

pub trait Extensible  {

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;


}

impl Extensible for crate::Extensible {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::Element {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::EnumBinding {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::StructuredAlias {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::AnonymousExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::PathExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::ClassRule {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::ArrayExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::DimensionExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::PatternExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::ImportExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::PermissibleValue {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::UniqueKey {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::TypeMapping {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::AnonymousSlotExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::AnonymousClassExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::SchemaDefinition {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::TypeDefinition {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::SubsetDefinition {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::Definition {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::EnumDefinition {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::SlotDefinition {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}
impl Extensible for crate::ClassDefinition {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
}

pub trait Annotatable  {

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;


}

impl Annotatable for crate::Annotatable {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::Annotation {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
// mapping
        return poly_containers::MapView::new(&self.annotations);
    }
}
impl Annotatable for crate::Element {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::EnumBinding {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::StructuredAlias {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::AnonymousExpression {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::PathExpression {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::ClassRule {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::ArrayExpression {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::DimensionExpression {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::PatternExpression {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::ImportExpression {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::PermissibleValue {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::UniqueKey {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::TypeMapping {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::AnonymousSlotExpression {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::AnonymousClassExpression {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::SchemaDefinition {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::TypeDefinition {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::SubsetDefinition {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::Definition {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::EnumDefinition {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::SlotDefinition {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}
impl Annotatable for crate::ClassDefinition {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
}

pub trait Annotation: Extension  {

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;


}

impl Annotation for crate::Annotation {
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
// mapping
        return poly_containers::MapView::new(&self.annotations);
    }
}

pub trait UnitOfMeasure  {

    fn symbol(&self) -> &Option<String>;
    // fn symbol_mut(&mut self) -> &mut &Option<String>;
    // fn set_symbol(&mut self, value: &Option<String>);

    fn abbreviation(&self) -> &Option<String>;
    // fn abbreviation_mut(&mut self) -> &mut &Option<String>;
    // fn set_abbreviation(&mut self, value: &Option<String>);

    fn descriptive_name(&self) -> &Option<String>;
    // fn descriptive_name_mut(&mut self) -> &mut &Option<String>;
    // fn set_descriptive_name(&mut self, value: &Option<String>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn ucum_code(&self) -> &Option<String>;
    // fn ucum_code_mut(&mut self) -> &mut &Option<String>;
    // fn set_ucum_code(&mut self, value: &Option<String>);

    fn derivation(&self) -> &Option<String>;
    // fn derivation_mut(&mut self) -> &mut &Option<String>;
    // fn set_derivation(&mut self, value: &Option<String>);

    fn has_quantity_kind(&self) -> &Option<uriorcurie>;
    // fn has_quantity_kind_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_has_quantity_kind(&mut self, value: &Option<uriorcurie>);

    fn iec61360code(&self) -> &Option<String>;
    // fn iec61360code_mut(&mut self) -> &mut &Option<String>;
    // fn set_iec61360code(&mut self, value: &Option<String>);


}

impl UnitOfMeasure for crate::UnitOfMeasure {
        fn symbol(&self) -> &Option<String> {
        return &self.symbol;
    }
        fn abbreviation(&self) -> &Option<String> {
        return &self.abbreviation;
    }
        fn descriptive_name(&self) -> &Option<String> {
        return &self.descriptive_name;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn ucum_code(&self) -> &Option<String> {
        return &self.ucum_code;
    }
        fn derivation(&self) -> &Option<String> {
        return &self.derivation;
    }
        fn has_quantity_kind(&self) -> &Option<uriorcurie> {
        return &self.has_quantity_kind;
    }
        fn iec61360code(&self) -> &Option<String> {
        return &self.iec61360code;
    }
}

pub trait Anything  {


}

impl Anything for crate::Anything {
}

pub trait CommonMetadata  {

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl CommonMetadata for crate::CommonMetadata {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::Element {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::EnumBinding {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::StructuredAlias {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
// list
        return poly_containers::ListView::new(&self.structured_aliases);
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::AnonymousExpression {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::PathExpression {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::ClassRule {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::ArrayExpression {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::DimensionExpression {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::PatternExpression {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::ImportExpression {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::PermissibleValue {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::UniqueKey {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::TypeMapping {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::AnonymousSlotExpression {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::AnonymousClassExpression {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::SchemaDefinition {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::TypeDefinition {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::SubsetDefinition {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
// list
        return poly_containers::ListView::new(&self.structured_aliases);
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::Definition {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::EnumDefinition {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::SlotDefinition {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl CommonMetadata for crate::ClassDefinition {
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait Element  {

    fn name(&self) -> &String;
    // fn name_mut(&mut self) -> &mut &String;
    // fn set_name(&mut self, value: String);

    fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname>;
    // fn id_prefixes_mut(&mut self) -> &mut impl poly_containers::SeqRef<ncname>;
    // fn set_id_prefixes(&mut self, value: &Vec<ncname>);

    fn id_prefixes_are_closed(&self) -> &Option<bool>;
    // fn id_prefixes_are_closed_mut(&mut self) -> &mut &Option<bool>;
    // fn set_id_prefixes_are_closed(&mut self, value: &Option<bool>);

    fn definition_uri(&self) -> &Option<uriorcurie>;
    // fn definition_uri_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_definition_uri(&mut self, value: &Option<uriorcurie>);

    fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName>;
    // fn local_names_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::LocalName>;
    // fn set_local_names<E>(&mut self, value: &HashMap<String, E>) where E: Into<LocalName>;

    fn conforms_to(&self) -> &Option<String>;
    // fn conforms_to_mut(&mut self) -> &mut &Option<String>;
    // fn set_conforms_to(&mut self, value: &Option<String>);

    fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn implements_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_implements(&mut self, value: &Vec<uriorcurie>);

    fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn instantiates_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_instantiates(&mut self, value: &Vec<uriorcurie>);

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl Element for crate::Element {
        fn name(&self) -> &String {
        return &self.name;
    }
        fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.id_prefixes;
    }
        fn id_prefixes_are_closed(&self) -> &Option<bool> {
        return &self.id_prefixes_are_closed;
    }
        fn definition_uri(&self) -> &Option<uriorcurie> {
        return &self.definition_uri;
    }
        fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName> {
        return &self.local_names;
    }
        fn conforms_to(&self) -> &Option<String> {
        return &self.conforms_to;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl Element for crate::SchemaDefinition {
        fn name(&self) -> &String {
        return &self.name;
    }
        fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.id_prefixes;
    }
        fn id_prefixes_are_closed(&self) -> &Option<bool> {
        return &self.id_prefixes_are_closed;
    }
        fn definition_uri(&self) -> &Option<uriorcurie> {
        return &self.definition_uri;
    }
        fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName> {
        return &self.local_names;
    }
        fn conforms_to(&self) -> &Option<String> {
        return &self.conforms_to;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl Element for crate::TypeDefinition {
        fn name(&self) -> &String {
        return &self.name;
    }
        fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.id_prefixes;
    }
        fn id_prefixes_are_closed(&self) -> &Option<bool> {
        return &self.id_prefixes_are_closed;
    }
        fn definition_uri(&self) -> &Option<uriorcurie> {
        return &self.definition_uri;
    }
        fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName> {
        return &self.local_names;
    }
        fn conforms_to(&self) -> &Option<String> {
        return &self.conforms_to;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl Element for crate::SubsetDefinition {
        fn name(&self) -> &String {
        return &self.name;
    }
        fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.id_prefixes;
    }
        fn id_prefixes_are_closed(&self) -> &Option<bool> {
        return &self.id_prefixes_are_closed;
    }
        fn definition_uri(&self) -> &Option<uriorcurie> {
        return &self.definition_uri;
    }
        fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName> {
        return &self.local_names;
    }
        fn conforms_to(&self) -> &Option<String> {
        return &self.conforms_to;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
// list
        return poly_containers::ListView::new(&self.structured_aliases);
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl Element for crate::Definition {
        fn name(&self) -> &String {
        return &self.name;
    }
        fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.id_prefixes;
    }
        fn id_prefixes_are_closed(&self) -> &Option<bool> {
        return &self.id_prefixes_are_closed;
    }
        fn definition_uri(&self) -> &Option<uriorcurie> {
        return &self.definition_uri;
    }
        fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName> {
        return &self.local_names;
    }
        fn conforms_to(&self) -> &Option<String> {
        return &self.conforms_to;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl Element for crate::EnumDefinition {
        fn name(&self) -> &String {
        return &self.name;
    }
        fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.id_prefixes;
    }
        fn id_prefixes_are_closed(&self) -> &Option<bool> {
        return &self.id_prefixes_are_closed;
    }
        fn definition_uri(&self) -> &Option<uriorcurie> {
        return &self.definition_uri;
    }
        fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName> {
        return &self.local_names;
    }
        fn conforms_to(&self) -> &Option<String> {
        return &self.conforms_to;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl Element for crate::SlotDefinition {
        fn name(&self) -> &String {
        return &self.name;
    }
        fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.id_prefixes;
    }
        fn id_prefixes_are_closed(&self) -> &Option<bool> {
        return &self.id_prefixes_are_closed;
    }
        fn definition_uri(&self) -> &Option<uriorcurie> {
        return &self.definition_uri;
    }
        fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName> {
        return &self.local_names;
    }
        fn conforms_to(&self) -> &Option<String> {
        return &self.conforms_to;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl Element for crate::ClassDefinition {
        fn name(&self) -> &String {
        return &self.name;
    }
        fn id_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.id_prefixes;
    }
        fn id_prefixes_are_closed(&self) -> &Option<bool> {
        return &self.id_prefixes_are_closed;
    }
        fn definition_uri(&self) -> &Option<uriorcurie> {
        return &self.definition_uri;
    }
        fn local_names(&self) -> impl poly_containers::MapRef<String,crate::LocalName> {
        return &self.local_names;
    }
        fn conforms_to(&self) -> &Option<String> {
        return &self.conforms_to;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait SchemaDefinition: Element  {

    fn id(&self) -> &uri;
    // fn id_mut(&mut self) -> &mut &uri;
    // fn set_id(&mut self, value: uri);

    fn version(&self) -> &Option<String>;
    // fn version_mut(&mut self) -> &mut &Option<String>;
    // fn set_version(&mut self, value: &Option<String>);

    fn imports(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn imports_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_imports(&mut self, value: &Vec<uriorcurie>);

    fn license(&self) -> &Option<String>;
    // fn license_mut(&mut self) -> &mut &Option<String>;
    // fn set_license(&mut self, value: &Option<String>);

    fn prefixes(&self) -> impl poly_containers::MapRef<String,crate::Prefix>;
    // fn prefixes_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Prefix>;
    // fn set_prefixes<E>(&mut self, value: &HashMap<String, E>) where E: Into<Prefix>;

    fn emit_prefixes(&self) -> impl poly_containers::SeqRef<ncname>;
    // fn emit_prefixes_mut(&mut self) -> &mut impl poly_containers::SeqRef<ncname>;
    // fn set_emit_prefixes(&mut self, value: &Vec<ncname>);

    fn default_curi_maps(&self) -> impl poly_containers::SeqRef<String>;
    // fn default_curi_maps_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_default_curi_maps(&mut self, value: &Vec<String>);

    fn default_prefix(&self) -> &Option<String>;
    // fn default_prefix_mut(&mut self) -> &mut &Option<String>;
    // fn set_default_prefix(&mut self, value: &Option<String>);

    fn default_range(&self) -> &Option<String>;
    // fn default_range_mut(&mut self) -> &mut &Option<String>;
    // fn set_default_range<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn subsets(&self) -> impl poly_containers::MapRef<String,crate::SubsetDefinition>;
    // fn subsets_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::SubsetDefinition>;
    // fn set_subsets<E>(&mut self, value: &HashMap<String, E>) where E: Into<SubsetDefinition>;

    fn types(&self) -> impl poly_containers::MapRef<String,crate::TypeDefinition>;
    // fn types_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::TypeDefinition>;
    // fn set_types<E>(&mut self, value: &HashMap<String, E>) where E: Into<TypeDefinition>;

    fn enums(&self) -> impl poly_containers::MapRef<String,crate::EnumDefinition>;
    // fn enums_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::EnumDefinition>;
    // fn set_enums<E>(&mut self, value: &HashMap<String, E>) where E: Into<EnumDefinition>;

    fn slot_definitions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn slot_definitions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn set_slot_definitions<E>(&mut self, value: &HashMap<String, E>) where E: Into<SlotDefinition>;

    fn classes(&self) -> impl poly_containers::MapRef<String,crate::ClassDefinition>;
    // fn classes_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::ClassDefinition>;
    // fn set_classes<E>(&mut self, value: &HashMap<String, E>) where E: Into<ClassDefinition>;

    fn metamodel_version(&self) -> &Option<String>;
    // fn metamodel_version_mut(&mut self) -> &mut &Option<String>;
    // fn set_metamodel_version(&mut self, value: &Option<String>);

    fn source_file(&self) -> &Option<String>;
    // fn source_file_mut(&mut self) -> &mut &Option<String>;
    // fn set_source_file(&mut self, value: &Option<String>);

    fn source_file_date(&self) -> &Option<NaiveDateTime>;
    // fn source_file_date_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_source_file_date(&mut self, value: &Option<NaiveDateTime>);

    fn source_file_size(&self) -> &Option<isize>;
    // fn source_file_size_mut(&mut self) -> &mut &Option<isize>;
    // fn set_source_file_size(&mut self, value: &Option<isize>);

    fn generation_date(&self) -> &Option<NaiveDateTime>;
    // fn generation_date_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_generation_date(&mut self, value: &Option<NaiveDateTime>);

    fn slot_names_unique(&self) -> &Option<bool>;
    // fn slot_names_unique_mut(&mut self) -> &mut &Option<bool>;
    // fn set_slot_names_unique(&mut self, value: &Option<bool>);

    fn settings(&self) -> impl poly_containers::MapRef<String,crate::Setting>;
    // fn settings_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Setting>;
    // fn set_settings<E>(&mut self, value: &HashMap<String, E>) where E: Into<Setting>;

    fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding>;
    // fn bindings_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::EnumBinding>;
    // fn set_bindings<E>(&mut self, value: &Vec<E>) where E: Into<EnumBinding>;


}

impl SchemaDefinition for crate::SchemaDefinition {
        fn id(&self) -> &uri {
        return &self.id;
    }
        fn version(&self) -> &Option<String> {
        return &self.version;
    }
        fn imports(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.imports;
    }
        fn license(&self) -> &Option<String> {
        return &self.license;
    }
        fn prefixes(&self) -> impl poly_containers::MapRef<String,crate::Prefix> {
        return &self.prefixes;
    }
        fn emit_prefixes(&self) -> impl poly_containers::SeqRef<ncname> {
        return &self.emit_prefixes;
    }
        fn default_curi_maps(&self) -> impl poly_containers::SeqRef<String> {
        return &self.default_curi_maps;
    }
        fn default_prefix(&self) -> &Option<String> {
        return &self.default_prefix;
    }
        fn default_range(&self) -> &Option<String> {
        return &self.default_range;
    }
        fn subsets(&self) -> impl poly_containers::MapRef<String,crate::SubsetDefinition> {
        return &self.subsets;
    }
        fn types(&self) -> impl poly_containers::MapRef<String,crate::TypeDefinition> {
        return &self.types;
    }
        fn enums(&self) -> impl poly_containers::MapRef<String,crate::EnumDefinition> {
        return &self.enums;
    }
        fn slot_definitions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition> {
        return &self.slot_definitions;
    }
        fn classes(&self) -> impl poly_containers::MapRef<String,crate::ClassDefinition> {
        return &self.classes;
    }
        fn metamodel_version(&self) -> &Option<String> {
        return &self.metamodel_version;
    }
        fn source_file(&self) -> &Option<String> {
        return &self.source_file;
    }
        fn source_file_date(&self) -> &Option<NaiveDateTime> {
        return &self.source_file_date;
    }
        fn source_file_size(&self) -> &Option<isize> {
        return &self.source_file_size;
    }
        fn generation_date(&self) -> &Option<NaiveDateTime> {
        return &self.generation_date;
    }
        fn slot_names_unique(&self) -> &Option<bool> {
        return &self.slot_names_unique;
    }
        fn settings(&self) -> impl poly_containers::MapRef<String,crate::Setting> {
        return &self.settings;
    }
        fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding> {
        return &self.bindings;
    }
}

pub trait AnonymousTypeExpression  {

    fn pattern(&self) -> &Option<String>;
    // fn pattern_mut(&mut self) -> &mut &Option<String>;
    // fn set_pattern(&mut self, value: &Option<String>);

    fn structured_pattern(&self) -> Option<&crate::PatternExpression>;
    // fn structured_pattern_mut(&mut self) -> &mut Option<&crate::PatternExpression>;
    // fn set_structured_pattern<E>(&mut self, value: Option<E>) where E: Into<PatternExpression>;

    fn unit(&self) -> Option<&crate::UnitOfMeasure>;
    // fn unit_mut(&mut self) -> &mut Option<&crate::UnitOfMeasure>;
    // fn set_unit<E>(&mut self, value: Option<E>) where E: Into<UnitOfMeasure>;

    fn implicit_prefix(&self) -> &Option<String>;
    // fn implicit_prefix_mut(&mut self) -> &mut &Option<String>;
    // fn set_implicit_prefix(&mut self, value: &Option<String>);

    fn equals_string(&self) -> &Option<String>;
    // fn equals_string_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_string(&mut self, value: &Option<String>);

    fn equals_string_in(&self) -> impl poly_containers::SeqRef<String>;
    // fn equals_string_in_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_equals_string_in(&mut self, value: &Vec<String>);

    fn equals_number(&self) -> &Option<isize>;
    // fn equals_number_mut(&mut self) -> &mut &Option<isize>;
    // fn set_equals_number(&mut self, value: &Option<isize>);

    fn minimum_value(&self) -> Option<&crate::Anything>;
    // fn minimum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_minimum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn maximum_value(&self) -> Option<&crate::Anything>;
    // fn maximum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_maximum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;


}

impl AnonymousTypeExpression for crate::AnonymousTypeExpression {
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
}

pub trait TypeDefinition: Element  {

    fn typeof_(&self) -> &Option<String>;
    // fn typeof__mut(&mut self) -> &mut &Option<String>;
    // fn set_typeof_<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn base(&self) -> &Option<String>;
    // fn base_mut(&mut self) -> &mut &Option<String>;
    // fn set_base(&mut self, value: &Option<String>);

    fn type_uri(&self) -> &Option<uriorcurie>;
    // fn type_uri_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_type_uri(&mut self, value: &Option<uriorcurie>);

    fn repr(&self) -> &Option<String>;
    // fn repr_mut(&mut self) -> &mut &Option<String>;
    // fn set_repr(&mut self, value: &Option<String>);

    fn union_of(&self) -> impl poly_containers::SeqRef<String>;
    // fn union_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_union_of<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn pattern(&self) -> &Option<String>;
    // fn pattern_mut(&mut self) -> &mut &Option<String>;
    // fn set_pattern(&mut self, value: &Option<String>);

    fn structured_pattern(&self) -> Option<&crate::PatternExpression>;
    // fn structured_pattern_mut(&mut self) -> &mut Option<&crate::PatternExpression>;
    // fn set_structured_pattern<E>(&mut self, value: Option<E>) where E: Into<PatternExpression>;

    fn unit(&self) -> Option<&crate::UnitOfMeasure>;
    // fn unit_mut(&mut self) -> &mut Option<&crate::UnitOfMeasure>;
    // fn set_unit<E>(&mut self, value: Option<E>) where E: Into<UnitOfMeasure>;

    fn implicit_prefix(&self) -> &Option<String>;
    // fn implicit_prefix_mut(&mut self) -> &mut &Option<String>;
    // fn set_implicit_prefix(&mut self, value: &Option<String>);

    fn equals_string(&self) -> &Option<String>;
    // fn equals_string_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_string(&mut self, value: &Option<String>);

    fn equals_string_in(&self) -> impl poly_containers::SeqRef<String>;
    // fn equals_string_in_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_equals_string_in(&mut self, value: &Vec<String>);

    fn equals_number(&self) -> &Option<isize>;
    // fn equals_number_mut(&mut self) -> &mut &Option<isize>;
    // fn set_equals_number(&mut self, value: &Option<isize>);

    fn minimum_value(&self) -> Option<&crate::Anything>;
    // fn minimum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_minimum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn maximum_value(&self) -> Option<&crate::Anything>;
    // fn maximum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_maximum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;


}

impl TypeDefinition for crate::TypeDefinition {
        fn typeof_(&self) -> &Option<String> {
        return &self.typeof_;
    }
        fn base(&self) -> &Option<String> {
        return &self.base;
    }
        fn type_uri(&self) -> &Option<uriorcurie> {
        return &self.type_uri;
    }
        fn repr(&self) -> &Option<String> {
        return &self.repr;
    }
        fn union_of(&self) -> impl poly_containers::SeqRef<String> {
        return &self.union_of;
    }
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.none_of;
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.exactly_one_of;
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.any_of;
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.all_of;
    }
}

pub trait SubsetDefinition: Element  {


}

impl SubsetDefinition for crate::SubsetDefinition {
}

pub trait Definition: Element  {

    fn is_a(&self) -> &Option<String>;
    // fn is_a_mut(&mut self) -> &mut &Option<String>;
    // fn set_is_a<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn abstract_(&self) -> &Option<bool>;
    // fn abstract__mut(&mut self) -> &mut &Option<bool>;
    // fn set_abstract_(&mut self, value: &Option<bool>);

    fn mixin(&self) -> &Option<bool>;
    // fn mixin_mut(&mut self) -> &mut &Option<bool>;
    // fn set_mixin(&mut self, value: &Option<bool>);

    fn mixins(&self) -> impl poly_containers::SeqRef<String>;
    // fn mixins_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_mixins<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn apply_to(&self) -> impl poly_containers::SeqRef<String>;
    // fn apply_to_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_apply_to<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn values_from(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn values_from_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_values_from(&mut self, value: &Vec<uriorcurie>);

    fn string_serialization(&self) -> &Option<String>;
    // fn string_serialization_mut(&mut self) -> &mut &Option<String>;
    // fn set_string_serialization(&mut self, value: &Option<String>);


}

impl Definition for crate::Definition {
        fn is_a(&self) -> &Option<String> {
        return &self.is_a;
    }
        fn abstract_(&self) -> &Option<bool> {
        return &self.abstract_;
    }
        fn mixin(&self) -> &Option<bool> {
        return &self.mixin;
    }
        fn mixins(&self) -> impl poly_containers::SeqRef<String> {
        return &self.mixins;
    }
        fn apply_to(&self) -> impl poly_containers::SeqRef<String> {
        return &self.apply_to;
    }
        fn values_from(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.values_from;
    }
        fn string_serialization(&self) -> &Option<String> {
        return &self.string_serialization;
    }
}
impl Definition for crate::EnumDefinition {
        fn is_a(&self) -> &Option<String> {
        return &self.is_a;
    }
        fn abstract_(&self) -> &Option<bool> {
        return &self.abstract_;
    }
        fn mixin(&self) -> &Option<bool> {
        return &self.mixin;
    }
        fn mixins(&self) -> impl poly_containers::SeqRef<String> {
        return &self.mixins;
    }
        fn apply_to(&self) -> impl poly_containers::SeqRef<String> {
        return &self.apply_to;
    }
        fn values_from(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.values_from;
    }
        fn string_serialization(&self) -> &Option<String> {
        return &self.string_serialization;
    }
}
impl Definition for crate::SlotDefinition {
        fn is_a(&self) -> &Option<String> {
        return &self.is_a;
    }
        fn abstract_(&self) -> &Option<bool> {
        return &self.abstract_;
    }
        fn mixin(&self) -> &Option<bool> {
        return &self.mixin;
    }
        fn mixins(&self) -> impl poly_containers::SeqRef<String> {
        return &self.mixins;
    }
        fn apply_to(&self) -> impl poly_containers::SeqRef<String> {
        return &self.apply_to;
    }
        fn values_from(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.values_from;
    }
        fn string_serialization(&self) -> &Option<String> {
        return &self.string_serialization;
    }
}
impl Definition for crate::ClassDefinition {
        fn is_a(&self) -> &Option<String> {
        return &self.is_a;
    }
        fn abstract_(&self) -> &Option<bool> {
        return &self.abstract_;
    }
        fn mixin(&self) -> &Option<bool> {
        return &self.mixin;
    }
        fn mixins(&self) -> impl poly_containers::SeqRef<String> {
        return &self.mixins;
    }
        fn apply_to(&self) -> impl poly_containers::SeqRef<String> {
        return &self.apply_to;
    }
        fn values_from(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.values_from;
    }
        fn string_serialization(&self) -> &Option<String> {
        return &self.string_serialization;
    }
}

pub trait AnonymousEnumExpression  {

    fn code_set(&self) -> &Option<uriorcurie>;
    // fn code_set_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_code_set(&mut self, value: &Option<uriorcurie>);

    fn code_set_tag(&self) -> &Option<String>;
    // fn code_set_tag_mut(&mut self) -> &mut &Option<String>;
    // fn set_code_set_tag(&mut self, value: &Option<String>);

    fn code_set_version(&self) -> &Option<String>;
    // fn code_set_version_mut(&mut self) -> &mut &Option<String>;
    // fn set_code_set_version(&mut self, value: &Option<String>);

    fn pv_formula(&self) -> &Option<String>;
    // fn pv_formula_mut(&mut self) -> &mut &Option<String>;
    // fn set_pv_formula(&mut self, value: &Option<String>);

    fn permissible_values(&self) -> impl poly_containers::MapRef<String,crate::PermissibleValue>;
    // fn permissible_values_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::PermissibleValue>;
    // fn set_permissible_values<E>(&mut self, value: &HashMap<String, E>) where E: Into<PermissibleValue>;

    fn include(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn include_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn set_include<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousEnumExpression>;

    fn minus(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn minus_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn set_minus<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousEnumExpression>;

    fn inherits(&self) -> impl poly_containers::SeqRef<String>;
    // fn inherits_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_inherits<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn reachable_from(&self) -> Option<&crate::ReachabilityQuery>;
    // fn reachable_from_mut(&mut self) -> &mut Option<&crate::ReachabilityQuery>;
    // fn set_reachable_from<E>(&mut self, value: Option<E>) where E: Into<ReachabilityQuery>;

    fn matches(&self) -> Option<&crate::MatchQuery>;
    // fn matches_mut(&mut self) -> &mut Option<&crate::MatchQuery>;
    // fn set_matches<E>(&mut self, value: Option<E>) where E: Into<MatchQuery>;

    fn concepts(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn concepts_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_concepts(&mut self, value: &Vec<uriorcurie>);


}

impl AnonymousEnumExpression for crate::AnonymousEnumExpression {
        fn code_set(&self) -> &Option<uriorcurie> {
        return &self.code_set;
    }
        fn code_set_tag(&self) -> &Option<String> {
        return &self.code_set_tag;
    }
        fn code_set_version(&self) -> &Option<String> {
        return &self.code_set_version;
    }
        fn pv_formula(&self) -> &Option<String> {
        return &self.pv_formula;
    }
        fn permissible_values(&self) -> impl poly_containers::MapRef<String,crate::PermissibleValue> {
        return &self.permissible_values;
    }
        fn include(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
// list
        return poly_containers::ListView::new(&self.include);
    }
        fn minus(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
// list
        return poly_containers::ListView::new(&self.minus);
    }
        fn inherits(&self) -> impl poly_containers::SeqRef<String> {
        return &self.inherits;
    }
        fn reachable_from(&self) -> Option<&crate::ReachabilityQuery> {
        return self.reachable_from.as_ref();
    }
        fn matches(&self) -> Option<&crate::MatchQuery> {
        return self.matches.as_ref();
    }
        fn concepts(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.concepts;
    }
}

pub trait EnumDefinition: Definition  {

    fn enum_uri(&self) -> &Option<uriorcurie>;
    // fn enum_uri_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_enum_uri(&mut self, value: &Option<uriorcurie>);

    fn code_set(&self) -> &Option<uriorcurie>;
    // fn code_set_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_code_set(&mut self, value: &Option<uriorcurie>);

    fn code_set_tag(&self) -> &Option<String>;
    // fn code_set_tag_mut(&mut self) -> &mut &Option<String>;
    // fn set_code_set_tag(&mut self, value: &Option<String>);

    fn code_set_version(&self) -> &Option<String>;
    // fn code_set_version_mut(&mut self) -> &mut &Option<String>;
    // fn set_code_set_version(&mut self, value: &Option<String>);

    fn pv_formula(&self) -> &Option<String>;
    // fn pv_formula_mut(&mut self) -> &mut &Option<String>;
    // fn set_pv_formula(&mut self, value: &Option<String>);

    fn permissible_values(&self) -> impl poly_containers::MapRef<String,crate::PermissibleValue>;
    // fn permissible_values_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::PermissibleValue>;
    // fn set_permissible_values<E>(&mut self, value: &HashMap<String, E>) where E: Into<PermissibleValue>;

    fn include(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn include_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn set_include<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousEnumExpression>;

    fn minus(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn minus_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn set_minus<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousEnumExpression>;

    fn inherits(&self) -> impl poly_containers::SeqRef<String>;
    // fn inherits_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_inherits<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn reachable_from(&self) -> Option<&crate::ReachabilityQuery>;
    // fn reachable_from_mut(&mut self) -> &mut Option<&crate::ReachabilityQuery>;
    // fn set_reachable_from<E>(&mut self, value: Option<E>) where E: Into<ReachabilityQuery>;

    fn matches(&self) -> Option<&crate::MatchQuery>;
    // fn matches_mut(&mut self) -> &mut Option<&crate::MatchQuery>;
    // fn set_matches<E>(&mut self, value: Option<E>) where E: Into<MatchQuery>;

    fn concepts(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn concepts_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_concepts(&mut self, value: &Vec<uriorcurie>);


}

impl EnumDefinition for crate::EnumDefinition {
        fn enum_uri(&self) -> &Option<uriorcurie> {
        return &self.enum_uri;
    }
        fn code_set(&self) -> &Option<uriorcurie> {
        return &self.code_set;
    }
        fn code_set_tag(&self) -> &Option<String> {
        return &self.code_set_tag;
    }
        fn code_set_version(&self) -> &Option<String> {
        return &self.code_set_version;
    }
        fn pv_formula(&self) -> &Option<String> {
        return &self.pv_formula;
    }
        fn permissible_values(&self) -> impl poly_containers::MapRef<String,crate::PermissibleValue> {
        return &self.permissible_values;
    }
        fn include(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
// list
        return poly_containers::ListView::new(&self.include);
    }
        fn minus(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
// list
        return poly_containers::ListView::new(&self.minus);
    }
        fn inherits(&self) -> impl poly_containers::SeqRef<String> {
        return &self.inherits;
    }
        fn reachable_from(&self) -> Option<&crate::ReachabilityQuery> {
        return self.reachable_from.as_ref();
    }
        fn matches(&self) -> Option<&crate::MatchQuery> {
        return self.matches.as_ref();
    }
        fn concepts(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.concepts;
    }
}

pub trait EnumBinding  {

    fn range(&self) -> &Option<String>;
    // fn range_mut(&mut self) -> &mut &Option<String>;
    // fn set_range<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn obligation_level(&self) -> &Option<String>;
    // fn obligation_level_mut(&mut self) -> &mut &Option<String>;
    // fn set_obligation_level(&mut self, value: &Option<String>);

    fn binds_value_of(&self) -> &Option<String>;
    // fn binds_value_of_mut(&mut self) -> &mut &Option<String>;
    // fn set_binds_value_of(&mut self, value: &Option<String>);

    fn pv_formula(&self) -> &Option<String>;
    // fn pv_formula_mut(&mut self) -> &mut &Option<String>;
    // fn set_pv_formula(&mut self, value: &Option<String>);

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl EnumBinding for crate::EnumBinding {
        fn range(&self) -> &Option<String> {
        return &self.range;
    }
        fn obligation_level(&self) -> &Option<String> {
        return &self.obligation_level;
    }
        fn binds_value_of(&self) -> &Option<String> {
        return &self.binds_value_of;
    }
        fn pv_formula(&self) -> &Option<String> {
        return &self.pv_formula;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait MatchQuery  {

    fn identifier_pattern(&self) -> &Option<String>;
    // fn identifier_pattern_mut(&mut self) -> &mut &Option<String>;
    // fn set_identifier_pattern(&mut self, value: &Option<String>);

    fn source_ontology(&self) -> &Option<uriorcurie>;
    // fn source_ontology_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source_ontology(&mut self, value: &Option<uriorcurie>);


}

impl MatchQuery for crate::MatchQuery {
        fn identifier_pattern(&self) -> &Option<String> {
        return &self.identifier_pattern;
    }
        fn source_ontology(&self) -> &Option<uriorcurie> {
        return &self.source_ontology;
    }
}

pub trait ReachabilityQuery  {

    fn source_ontology(&self) -> &Option<uriorcurie>;
    // fn source_ontology_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source_ontology(&mut self, value: &Option<uriorcurie>);

    fn source_nodes(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn source_nodes_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_source_nodes(&mut self, value: &Vec<uriorcurie>);

    fn relationship_types(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn relationship_types_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_relationship_types(&mut self, value: &Vec<uriorcurie>);

    fn is_direct(&self) -> &Option<bool>;
    // fn is_direct_mut(&mut self) -> &mut &Option<bool>;
    // fn set_is_direct(&mut self, value: &Option<bool>);

    fn include_self(&self) -> &Option<bool>;
    // fn include_self_mut(&mut self) -> &mut &Option<bool>;
    // fn set_include_self(&mut self, value: &Option<bool>);

    fn traverse_up(&self) -> &Option<bool>;
    // fn traverse_up_mut(&mut self) -> &mut &Option<bool>;
    // fn set_traverse_up(&mut self, value: &Option<bool>);


}

impl ReachabilityQuery for crate::ReachabilityQuery {
        fn source_ontology(&self) -> &Option<uriorcurie> {
        return &self.source_ontology;
    }
        fn source_nodes(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.source_nodes;
    }
        fn relationship_types(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.relationship_types;
    }
        fn is_direct(&self) -> &Option<bool> {
        return &self.is_direct;
    }
        fn include_self(&self) -> &Option<bool> {
        return &self.include_self;
    }
        fn traverse_up(&self) -> &Option<bool> {
        return &self.traverse_up;
    }
}

pub trait StructuredAlias  {

    fn literal_form(&self) -> &String;
    // fn literal_form_mut(&mut self) -> &mut &String;
    // fn set_literal_form(&mut self, value: String);

    fn alias_predicate(&self) -> &Option<String>;
    // fn alias_predicate_mut(&mut self) -> &mut &Option<String>;
    // fn set_alias_predicate(&mut self, value: &Option<String>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn alias_contexts(&self) -> impl poly_containers::SeqRef<uri>;
    // fn alias_contexts_mut(&mut self) -> &mut impl poly_containers::SeqRef<uri>;
    // fn set_alias_contexts(&mut self, value: &Vec<uri>);

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl StructuredAlias for crate::StructuredAlias {
        fn literal_form(&self) -> &String {
        return &self.literal_form;
    }
        fn alias_predicate(&self) -> &Option<String> {
        return &self.alias_predicate;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn alias_contexts(&self) -> impl poly_containers::SeqRef<uri> {
        return &self.alias_contexts;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
// list
        return poly_containers::ListView::new(&self.structured_aliases);
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait Expression  {


}

impl Expression for crate::Expression {
}
impl Expression for crate::TypeExpression {
}
impl Expression for crate::EnumExpression {
}
impl Expression for crate::StructuredAlias {
}
impl Expression for crate::AnonymousExpression {
}
impl Expression for crate::PathExpression {
}
impl Expression for crate::SlotExpression {
}
impl Expression for crate::AnonymousSlotExpression {
}
impl Expression for crate::SlotDefinition {
}
impl Expression for crate::AnonymousClassExpression {
}
impl Expression for crate::AnonymousEnumExpression {
}
impl Expression for crate::EnumDefinition {
}
impl Expression for crate::AnonymousTypeExpression {
}
impl Expression for crate::TypeDefinition {
}

pub trait TypeExpression: Expression  {

    fn pattern(&self) -> &Option<String>;
    // fn pattern_mut(&mut self) -> &mut &Option<String>;
    // fn set_pattern(&mut self, value: &Option<String>);

    fn structured_pattern(&self) -> Option<&crate::PatternExpression>;
    // fn structured_pattern_mut(&mut self) -> &mut Option<&crate::PatternExpression>;
    // fn set_structured_pattern<E>(&mut self, value: Option<E>) where E: Into<PatternExpression>;

    fn unit(&self) -> Option<&crate::UnitOfMeasure>;
    // fn unit_mut(&mut self) -> &mut Option<&crate::UnitOfMeasure>;
    // fn set_unit<E>(&mut self, value: Option<E>) where E: Into<UnitOfMeasure>;

    fn implicit_prefix(&self) -> &Option<String>;
    // fn implicit_prefix_mut(&mut self) -> &mut &Option<String>;
    // fn set_implicit_prefix(&mut self, value: &Option<String>);

    fn equals_string(&self) -> &Option<String>;
    // fn equals_string_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_string(&mut self, value: &Option<String>);

    fn equals_string_in(&self) -> impl poly_containers::SeqRef<String>;
    // fn equals_string_in_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_equals_string_in(&mut self, value: &Vec<String>);

    fn equals_number(&self) -> &Option<isize>;
    // fn equals_number_mut(&mut self) -> &mut &Option<isize>;
    // fn set_equals_number(&mut self, value: &Option<isize>);

    fn minimum_value(&self) -> Option<&crate::Anything>;
    // fn minimum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_minimum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn maximum_value(&self) -> Option<&crate::Anything>;
    // fn maximum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_maximum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousTypeExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousTypeExpression>;


}

impl TypeExpression for crate::TypeExpression {
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.none_of;
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.exactly_one_of;
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.any_of;
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.all_of;
    }
}
impl TypeExpression for crate::AnonymousTypeExpression {
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
}
impl TypeExpression for crate::TypeDefinition {
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.none_of;
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.exactly_one_of;
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.any_of;
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousTypeExpression> {
        return &self.all_of;
    }
}

pub trait EnumExpression: Expression  {

    fn code_set(&self) -> &Option<uriorcurie>;
    // fn code_set_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_code_set(&mut self, value: &Option<uriorcurie>);

    fn code_set_tag(&self) -> &Option<String>;
    // fn code_set_tag_mut(&mut self) -> &mut &Option<String>;
    // fn set_code_set_tag(&mut self, value: &Option<String>);

    fn code_set_version(&self) -> &Option<String>;
    // fn code_set_version_mut(&mut self) -> &mut &Option<String>;
    // fn set_code_set_version(&mut self, value: &Option<String>);

    fn pv_formula(&self) -> &Option<String>;
    // fn pv_formula_mut(&mut self) -> &mut &Option<String>;
    // fn set_pv_formula(&mut self, value: &Option<String>);

    fn permissible_values(&self) -> impl poly_containers::MapRef<String,crate::PermissibleValue>;
    // fn permissible_values_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::PermissibleValue>;
    // fn set_permissible_values<E>(&mut self, value: &HashMap<String, E>) where E: Into<PermissibleValue>;

    fn include(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn include_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn set_include<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousEnumExpression>;

    fn minus(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn minus_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousEnumExpression>;
    // fn set_minus<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousEnumExpression>;

    fn inherits(&self) -> impl poly_containers::SeqRef<String>;
    // fn inherits_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_inherits<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn reachable_from(&self) -> Option<&crate::ReachabilityQuery>;
    // fn reachable_from_mut(&mut self) -> &mut Option<&crate::ReachabilityQuery>;
    // fn set_reachable_from<E>(&mut self, value: Option<E>) where E: Into<ReachabilityQuery>;

    fn matches(&self) -> Option<&crate::MatchQuery>;
    // fn matches_mut(&mut self) -> &mut Option<&crate::MatchQuery>;
    // fn set_matches<E>(&mut self, value: Option<E>) where E: Into<MatchQuery>;

    fn concepts(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn concepts_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_concepts(&mut self, value: &Vec<uriorcurie>);


}

impl EnumExpression for crate::EnumExpression {
        fn code_set(&self) -> &Option<uriorcurie> {
        return &self.code_set;
    }
        fn code_set_tag(&self) -> &Option<String> {
        return &self.code_set_tag;
    }
        fn code_set_version(&self) -> &Option<String> {
        return &self.code_set_version;
    }
        fn pv_formula(&self) -> &Option<String> {
        return &self.pv_formula;
    }
        fn permissible_values(&self) -> impl poly_containers::MapRef<String,crate::PermissibleValue> {
        return &self.permissible_values;
    }
        fn include(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
        return &self.include;
    }
        fn minus(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
        return &self.minus;
    }
        fn inherits(&self) -> impl poly_containers::SeqRef<String> {
        return &self.inherits;
    }
        fn reachable_from(&self) -> Option<&crate::ReachabilityQuery> {
        return self.reachable_from.as_ref();
    }
        fn matches(&self) -> Option<&crate::MatchQuery> {
        return self.matches.as_ref();
    }
        fn concepts(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.concepts;
    }
}
impl EnumExpression for crate::AnonymousEnumExpression {
        fn code_set(&self) -> &Option<uriorcurie> {
        return &self.code_set;
    }
        fn code_set_tag(&self) -> &Option<String> {
        return &self.code_set_tag;
    }
        fn code_set_version(&self) -> &Option<String> {
        return &self.code_set_version;
    }
        fn pv_formula(&self) -> &Option<String> {
        return &self.pv_formula;
    }
        fn permissible_values(&self) -> impl poly_containers::MapRef<String,crate::PermissibleValue> {
        return &self.permissible_values;
    }
        fn include(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
// list
        return poly_containers::ListView::new(&self.include);
    }
        fn minus(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
// list
        return poly_containers::ListView::new(&self.minus);
    }
        fn inherits(&self) -> impl poly_containers::SeqRef<String> {
        return &self.inherits;
    }
        fn reachable_from(&self) -> Option<&crate::ReachabilityQuery> {
        return self.reachable_from.as_ref();
    }
        fn matches(&self) -> Option<&crate::MatchQuery> {
        return self.matches.as_ref();
    }
        fn concepts(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.concepts;
    }
}
impl EnumExpression for crate::EnumDefinition {
        fn code_set(&self) -> &Option<uriorcurie> {
        return &self.code_set;
    }
        fn code_set_tag(&self) -> &Option<String> {
        return &self.code_set_tag;
    }
        fn code_set_version(&self) -> &Option<String> {
        return &self.code_set_version;
    }
        fn pv_formula(&self) -> &Option<String> {
        return &self.pv_formula;
    }
        fn permissible_values(&self) -> impl poly_containers::MapRef<String,crate::PermissibleValue> {
        return &self.permissible_values;
    }
        fn include(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
// list
        return poly_containers::ListView::new(&self.include);
    }
        fn minus(&self) -> impl poly_containers::SeqRef<crate::AnonymousEnumExpression> {
// list
        return poly_containers::ListView::new(&self.minus);
    }
        fn inherits(&self) -> impl poly_containers::SeqRef<String> {
        return &self.inherits;
    }
        fn reachable_from(&self) -> Option<&crate::ReachabilityQuery> {
        return self.reachable_from.as_ref();
    }
        fn matches(&self) -> Option<&crate::MatchQuery> {
        return self.matches.as_ref();
    }
        fn concepts(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.concepts;
    }
}

pub trait AnonymousExpression  {

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl AnonymousExpression for crate::AnonymousExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl AnonymousExpression for crate::AnonymousSlotExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}
impl AnonymousExpression for crate::AnonymousClassExpression {
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait PathExpression  {

    fn followed_by(&self) -> Option<&crate::PathExpression>;
    // fn followed_by_mut(&mut self) -> &mut Option<&crate::PathExpression>;
    // fn set_followed_by<E>(&mut self, value: Option<E>) where E: Into<PathExpression>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::PathExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::PathExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<PathExpression>;

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::PathExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::PathExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<PathExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::PathExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::PathExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<PathExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::PathExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::PathExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<PathExpression>;

    fn reversed(&self) -> &Option<bool>;
    // fn reversed_mut(&mut self) -> &mut &Option<bool>;
    // fn set_reversed(&mut self, value: &Option<bool>);

    fn traverse(&self) -> &Option<String>;
    // fn traverse_mut(&mut self) -> &mut &Option<String>;
    // fn set_traverse<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn range_expression(&self) -> Option<&crate::AnonymousClassExpression>;
    // fn range_expression_mut(&mut self) -> &mut Option<&crate::AnonymousClassExpression>;
    // fn set_range_expression<E>(&mut self, value: Option<E>) where E: Into<AnonymousClassExpression>;

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl PathExpression for crate::PathExpression {
        fn followed_by(&self) -> Option<&crate::PathExpression> {
// None
        return self.followed_by.as_deref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::PathExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::PathExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::PathExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::PathExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn reversed(&self) -> &Option<bool> {
        return &self.reversed;
    }
        fn traverse(&self) -> &Option<String> {
        return &self.traverse;
    }
        fn range_expression(&self) -> Option<&crate::AnonymousClassExpression> {
// None
        return self.range_expression.as_deref();
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait SlotExpression: Expression  {

    fn range(&self) -> &Option<String>;
    // fn range_mut(&mut self) -> &mut &Option<String>;
    // fn set_range<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn range_expression(&self) -> Option<&crate::AnonymousClassExpression>;
    // fn range_expression_mut(&mut self) -> &mut Option<&crate::AnonymousClassExpression>;
    // fn set_range_expression<E>(&mut self, value: Option<E>) where E: Into<AnonymousClassExpression>;

    fn enum_range(&self) -> Option<&EnumExpressionOrSubtype>;
    // fn enum_range_mut(&mut self) -> &mut Option<&EnumExpressionOrSubtype>;
    // fn set_enum_range<E>(&mut self, value: Option<E>) where E: Into<EnumExpression>;

    fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding>;
    // fn bindings_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::EnumBinding>;
    // fn set_bindings<E>(&mut self, value: &Vec<E>) where E: Into<EnumBinding>;

    fn required(&self) -> &Option<bool>;
    // fn required_mut(&mut self) -> &mut &Option<bool>;
    // fn set_required(&mut self, value: &Option<bool>);

    fn recommended(&self) -> &Option<bool>;
    // fn recommended_mut(&mut self) -> &mut &Option<bool>;
    // fn set_recommended(&mut self, value: &Option<bool>);

    fn multivalued(&self) -> &Option<bool>;
    // fn multivalued_mut(&mut self) -> &mut &Option<bool>;
    // fn set_multivalued(&mut self, value: &Option<bool>);

    fn inlined(&self) -> &Option<bool>;
    // fn inlined_mut(&mut self) -> &mut &Option<bool>;
    // fn set_inlined(&mut self, value: &Option<bool>);

    fn inlined_as_list(&self) -> &Option<bool>;
    // fn inlined_as_list_mut(&mut self) -> &mut &Option<bool>;
    // fn set_inlined_as_list(&mut self, value: &Option<bool>);

    fn minimum_value(&self) -> Option<&crate::Anything>;
    // fn minimum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_minimum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn maximum_value(&self) -> Option<&crate::Anything>;
    // fn maximum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_maximum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn pattern(&self) -> &Option<String>;
    // fn pattern_mut(&mut self) -> &mut &Option<String>;
    // fn set_pattern(&mut self, value: &Option<String>);

    fn structured_pattern(&self) -> Option<&crate::PatternExpression>;
    // fn structured_pattern_mut(&mut self) -> &mut Option<&crate::PatternExpression>;
    // fn set_structured_pattern<E>(&mut self, value: Option<E>) where E: Into<PatternExpression>;

    fn unit(&self) -> Option<&crate::UnitOfMeasure>;
    // fn unit_mut(&mut self) -> &mut Option<&crate::UnitOfMeasure>;
    // fn set_unit<E>(&mut self, value: Option<E>) where E: Into<UnitOfMeasure>;

    fn implicit_prefix(&self) -> &Option<String>;
    // fn implicit_prefix_mut(&mut self) -> &mut &Option<String>;
    // fn set_implicit_prefix(&mut self, value: &Option<String>);

    fn value_presence(&self) -> &Option<String>;
    // fn value_presence_mut(&mut self) -> &mut &Option<String>;
    // fn set_value_presence(&mut self, value: &Option<String>);

    fn equals_string(&self) -> &Option<String>;
    // fn equals_string_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_string(&mut self, value: &Option<String>);

    fn equals_string_in(&self) -> impl poly_containers::SeqRef<String>;
    // fn equals_string_in_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_equals_string_in(&mut self, value: &Vec<String>);

    fn equals_number(&self) -> &Option<isize>;
    // fn equals_number_mut(&mut self) -> &mut &Option<isize>;
    // fn set_equals_number(&mut self, value: &Option<isize>);

    fn equals_expression(&self) -> &Option<String>;
    // fn equals_expression_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_expression(&mut self, value: &Option<String>);

    fn exact_cardinality(&self) -> &Option<isize>;
    // fn exact_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_exact_cardinality(&mut self, value: &Option<isize>);

    fn minimum_cardinality(&self) -> &Option<isize>;
    // fn minimum_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_minimum_cardinality(&mut self, value: &Option<isize>);

    fn maximum_cardinality(&self) -> &Option<isize>;
    // fn maximum_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_maximum_cardinality(&mut self, value: &Option<isize>);

    fn has_member(&self) -> Option<&crate::AnonymousSlotExpression>;
    // fn has_member_mut(&mut self) -> &mut Option<&crate::AnonymousSlotExpression>;
    // fn set_has_member<E>(&mut self, value: Option<E>) where E: Into<AnonymousSlotExpression>;

    fn all_members(&self) -> Option<&crate::AnonymousSlotExpression>;
    // fn all_members_mut(&mut self) -> &mut Option<&crate::AnonymousSlotExpression>;
    // fn set_all_members<E>(&mut self, value: Option<E>) where E: Into<AnonymousSlotExpression>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;


}

impl SlotExpression for crate::SlotExpression {
        fn range(&self) -> &Option<String> {
        return &self.range;
    }
        fn range_expression(&self) -> Option<&crate::AnonymousClassExpression> {
        return self.range_expression.as_ref();
    }
        fn enum_range(&self) -> Option<&EnumExpressionOrSubtype> {
        return self.enum_range.as_ref();
    }
        fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding> {
        return &self.bindings;
    }
        fn required(&self) -> &Option<bool> {
        return &self.required;
    }
        fn recommended(&self) -> &Option<bool> {
        return &self.recommended;
    }
        fn multivalued(&self) -> &Option<bool> {
        return &self.multivalued;
    }
        fn inlined(&self) -> &Option<bool> {
        return &self.inlined;
    }
        fn inlined_as_list(&self) -> &Option<bool> {
        return &self.inlined_as_list;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn value_presence(&self) -> &Option<String> {
        return &self.value_presence;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn equals_expression(&self) -> &Option<String> {
        return &self.equals_expression;
    }
        fn exact_cardinality(&self) -> &Option<isize> {
        return &self.exact_cardinality;
    }
        fn minimum_cardinality(&self) -> &Option<isize> {
        return &self.minimum_cardinality;
    }
        fn maximum_cardinality(&self) -> &Option<isize> {
        return &self.maximum_cardinality;
    }
        fn has_member(&self) -> Option<&crate::AnonymousSlotExpression> {
        return self.has_member.as_ref();
    }
        fn all_members(&self) -> Option<&crate::AnonymousSlotExpression> {
        return self.all_members.as_ref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
        return &self.none_of;
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
        return &self.exactly_one_of;
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
        return &self.any_of;
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
        return &self.all_of;
    }
}
impl SlotExpression for crate::AnonymousSlotExpression {
        fn range(&self) -> &Option<String> {
        return &self.range;
    }
        fn range_expression(&self) -> Option<&crate::AnonymousClassExpression> {
// None
        return self.range_expression.as_deref();
    }
        fn enum_range(&self) -> Option<&EnumExpressionOrSubtype> {
        return self.enum_range.as_ref();
    }
        fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding> {
        return &self.bindings;
    }
        fn required(&self) -> &Option<bool> {
        return &self.required;
    }
        fn recommended(&self) -> &Option<bool> {
        return &self.recommended;
    }
        fn multivalued(&self) -> &Option<bool> {
        return &self.multivalued;
    }
        fn inlined(&self) -> &Option<bool> {
        return &self.inlined;
    }
        fn inlined_as_list(&self) -> &Option<bool> {
        return &self.inlined_as_list;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn value_presence(&self) -> &Option<String> {
        return &self.value_presence;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn equals_expression(&self) -> &Option<String> {
        return &self.equals_expression;
    }
        fn exact_cardinality(&self) -> &Option<isize> {
        return &self.exact_cardinality;
    }
        fn minimum_cardinality(&self) -> &Option<isize> {
        return &self.minimum_cardinality;
    }
        fn maximum_cardinality(&self) -> &Option<isize> {
        return &self.maximum_cardinality;
    }
        fn has_member(&self) -> Option<&crate::AnonymousSlotExpression> {
// None
        return self.has_member.as_deref();
    }
        fn all_members(&self) -> Option<&crate::AnonymousSlotExpression> {
// None
        return self.all_members.as_deref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
}
impl SlotExpression for crate::SlotDefinition {
        fn range(&self) -> &Option<String> {
        return &self.range;
    }
        fn range_expression(&self) -> Option<&crate::AnonymousClassExpression> {
// None
        return self.range_expression.as_deref();
    }
        fn enum_range(&self) -> Option<&EnumExpressionOrSubtype> {
        return self.enum_range.as_ref();
    }
        fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding> {
        return &self.bindings;
    }
        fn required(&self) -> &Option<bool> {
        return &self.required;
    }
        fn recommended(&self) -> &Option<bool> {
        return &self.recommended;
    }
        fn multivalued(&self) -> &Option<bool> {
        return &self.multivalued;
    }
        fn inlined(&self) -> &Option<bool> {
        return &self.inlined;
    }
        fn inlined_as_list(&self) -> &Option<bool> {
        return &self.inlined_as_list;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn value_presence(&self) -> &Option<String> {
        return &self.value_presence;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn equals_expression(&self) -> &Option<String> {
        return &self.equals_expression;
    }
        fn exact_cardinality(&self) -> &Option<isize> {
        return &self.exact_cardinality;
    }
        fn minimum_cardinality(&self) -> &Option<isize> {
        return &self.minimum_cardinality;
    }
        fn maximum_cardinality(&self) -> &Option<isize> {
        return &self.maximum_cardinality;
    }
        fn has_member(&self) -> Option<&crate::AnonymousSlotExpression> {
// None
        return self.has_member.as_deref();
    }
        fn all_members(&self) -> Option<&crate::AnonymousSlotExpression> {
// None
        return self.all_members.as_deref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
}

pub trait AnonymousSlotExpression: AnonymousExpression  {

    fn range(&self) -> &Option<String>;
    // fn range_mut(&mut self) -> &mut &Option<String>;
    // fn set_range<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn range_expression(&self) -> Option<&crate::AnonymousClassExpression>;
    // fn range_expression_mut(&mut self) -> &mut Option<&crate::AnonymousClassExpression>;
    // fn set_range_expression<E>(&mut self, value: Option<E>) where E: Into<AnonymousClassExpression>;

    fn enum_range(&self) -> Option<&EnumExpressionOrSubtype>;
    // fn enum_range_mut(&mut self) -> &mut Option<&EnumExpressionOrSubtype>;
    // fn set_enum_range<E>(&mut self, value: Option<E>) where E: Into<EnumExpression>;

    fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding>;
    // fn bindings_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::EnumBinding>;
    // fn set_bindings<E>(&mut self, value: &Vec<E>) where E: Into<EnumBinding>;

    fn required(&self) -> &Option<bool>;
    // fn required_mut(&mut self) -> &mut &Option<bool>;
    // fn set_required(&mut self, value: &Option<bool>);

    fn recommended(&self) -> &Option<bool>;
    // fn recommended_mut(&mut self) -> &mut &Option<bool>;
    // fn set_recommended(&mut self, value: &Option<bool>);

    fn multivalued(&self) -> &Option<bool>;
    // fn multivalued_mut(&mut self) -> &mut &Option<bool>;
    // fn set_multivalued(&mut self, value: &Option<bool>);

    fn inlined(&self) -> &Option<bool>;
    // fn inlined_mut(&mut self) -> &mut &Option<bool>;
    // fn set_inlined(&mut self, value: &Option<bool>);

    fn inlined_as_list(&self) -> &Option<bool>;
    // fn inlined_as_list_mut(&mut self) -> &mut &Option<bool>;
    // fn set_inlined_as_list(&mut self, value: &Option<bool>);

    fn minimum_value(&self) -> Option<&crate::Anything>;
    // fn minimum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_minimum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn maximum_value(&self) -> Option<&crate::Anything>;
    // fn maximum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_maximum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn pattern(&self) -> &Option<String>;
    // fn pattern_mut(&mut self) -> &mut &Option<String>;
    // fn set_pattern(&mut self, value: &Option<String>);

    fn structured_pattern(&self) -> Option<&crate::PatternExpression>;
    // fn structured_pattern_mut(&mut self) -> &mut Option<&crate::PatternExpression>;
    // fn set_structured_pattern<E>(&mut self, value: Option<E>) where E: Into<PatternExpression>;

    fn unit(&self) -> Option<&crate::UnitOfMeasure>;
    // fn unit_mut(&mut self) -> &mut Option<&crate::UnitOfMeasure>;
    // fn set_unit<E>(&mut self, value: Option<E>) where E: Into<UnitOfMeasure>;

    fn implicit_prefix(&self) -> &Option<String>;
    // fn implicit_prefix_mut(&mut self) -> &mut &Option<String>;
    // fn set_implicit_prefix(&mut self, value: &Option<String>);

    fn value_presence(&self) -> &Option<String>;
    // fn value_presence_mut(&mut self) -> &mut &Option<String>;
    // fn set_value_presence(&mut self, value: &Option<String>);

    fn equals_string(&self) -> &Option<String>;
    // fn equals_string_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_string(&mut self, value: &Option<String>);

    fn equals_string_in(&self) -> impl poly_containers::SeqRef<String>;
    // fn equals_string_in_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_equals_string_in(&mut self, value: &Vec<String>);

    fn equals_number(&self) -> &Option<isize>;
    // fn equals_number_mut(&mut self) -> &mut &Option<isize>;
    // fn set_equals_number(&mut self, value: &Option<isize>);

    fn equals_expression(&self) -> &Option<String>;
    // fn equals_expression_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_expression(&mut self, value: &Option<String>);

    fn exact_cardinality(&self) -> &Option<isize>;
    // fn exact_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_exact_cardinality(&mut self, value: &Option<isize>);

    fn minimum_cardinality(&self) -> &Option<isize>;
    // fn minimum_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_minimum_cardinality(&mut self, value: &Option<isize>);

    fn maximum_cardinality(&self) -> &Option<isize>;
    // fn maximum_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_maximum_cardinality(&mut self, value: &Option<isize>);

    fn has_member(&self) -> Option<&crate::AnonymousSlotExpression>;
    // fn has_member_mut(&mut self) -> &mut Option<&crate::AnonymousSlotExpression>;
    // fn set_has_member<E>(&mut self, value: Option<E>) where E: Into<AnonymousSlotExpression>;

    fn all_members(&self) -> Option<&crate::AnonymousSlotExpression>;
    // fn all_members_mut(&mut self) -> &mut Option<&crate::AnonymousSlotExpression>;
    // fn set_all_members<E>(&mut self, value: Option<E>) where E: Into<AnonymousSlotExpression>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;


}

impl AnonymousSlotExpression for crate::AnonymousSlotExpression {
        fn range(&self) -> &Option<String> {
        return &self.range;
    }
        fn range_expression(&self) -> Option<&crate::AnonymousClassExpression> {
// None
        return self.range_expression.as_deref();
    }
        fn enum_range(&self) -> Option<&EnumExpressionOrSubtype> {
        return self.enum_range.as_ref();
    }
        fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding> {
        return &self.bindings;
    }
        fn required(&self) -> &Option<bool> {
        return &self.required;
    }
        fn recommended(&self) -> &Option<bool> {
        return &self.recommended;
    }
        fn multivalued(&self) -> &Option<bool> {
        return &self.multivalued;
    }
        fn inlined(&self) -> &Option<bool> {
        return &self.inlined;
    }
        fn inlined_as_list(&self) -> &Option<bool> {
        return &self.inlined_as_list;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn value_presence(&self) -> &Option<String> {
        return &self.value_presence;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn equals_expression(&self) -> &Option<String> {
        return &self.equals_expression;
    }
        fn exact_cardinality(&self) -> &Option<isize> {
        return &self.exact_cardinality;
    }
        fn minimum_cardinality(&self) -> &Option<isize> {
        return &self.minimum_cardinality;
    }
        fn maximum_cardinality(&self) -> &Option<isize> {
        return &self.maximum_cardinality;
    }
        fn has_member(&self) -> Option<&crate::AnonymousSlotExpression> {
// None
        return self.has_member.as_deref();
    }
        fn all_members(&self) -> Option<&crate::AnonymousSlotExpression> {
// None
        return self.all_members.as_deref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
}

pub trait SlotDefinition: Definition  {

    fn singular_name(&self) -> &Option<String>;
    // fn singular_name_mut(&mut self) -> &mut &Option<String>;
    // fn set_singular_name(&mut self, value: &Option<String>);

    fn domain(&self) -> &Option<String>;
    // fn domain_mut(&mut self) -> &mut &Option<String>;
    // fn set_domain<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn slot_uri(&self) -> &Option<uriorcurie>;
    // fn slot_uri_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_slot_uri(&mut self, value: &Option<uriorcurie>);

    fn array(&self) -> Option<&crate::ArrayExpression>;
    // fn array_mut(&mut self) -> &mut Option<&crate::ArrayExpression>;
    // fn set_array<E>(&mut self, value: Option<E>) where E: Into<ArrayExpression>;

    fn inherited(&self) -> &Option<bool>;
    // fn inherited_mut(&mut self) -> &mut &Option<bool>;
    // fn set_inherited(&mut self, value: &Option<bool>);

    fn readonly(&self) -> &Option<String>;
    // fn readonly_mut(&mut self) -> &mut &Option<String>;
    // fn set_readonly(&mut self, value: &Option<String>);

    fn ifabsent(&self) -> &Option<String>;
    // fn ifabsent_mut(&mut self) -> &mut &Option<String>;
    // fn set_ifabsent(&mut self, value: &Option<String>);

    fn list_elements_unique(&self) -> &Option<bool>;
    // fn list_elements_unique_mut(&mut self) -> &mut &Option<bool>;
    // fn set_list_elements_unique(&mut self, value: &Option<bool>);

    fn list_elements_ordered(&self) -> &Option<bool>;
    // fn list_elements_ordered_mut(&mut self) -> &mut &Option<bool>;
    // fn set_list_elements_ordered(&mut self, value: &Option<bool>);

    fn shared(&self) -> &Option<bool>;
    // fn shared_mut(&mut self) -> &mut &Option<bool>;
    // fn set_shared(&mut self, value: &Option<bool>);

    fn key(&self) -> &Option<bool>;
    // fn key_mut(&mut self) -> &mut &Option<bool>;
    // fn set_key(&mut self, value: &Option<bool>);

    fn identifier(&self) -> &Option<bool>;
    // fn identifier_mut(&mut self) -> &mut &Option<bool>;
    // fn set_identifier(&mut self, value: &Option<bool>);

    fn designates_type(&self) -> &Option<bool>;
    // fn designates_type_mut(&mut self) -> &mut &Option<bool>;
    // fn set_designates_type(&mut self, value: &Option<bool>);

    fn alias(&self) -> &Option<String>;
    // fn alias_mut(&mut self) -> &mut &Option<String>;
    // fn set_alias(&mut self, value: &Option<String>);

    fn owner(&self) -> &Option<String>;
    // fn owner_mut(&mut self) -> &mut &Option<String>;
    // fn set_owner<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn domain_of(&self) -> impl poly_containers::SeqRef<String>;
    // fn domain_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_domain_of<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn subproperty_of(&self) -> &Option<String>;
    // fn subproperty_of_mut(&mut self) -> &mut &Option<String>;
    // fn set_subproperty_of<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn symmetric(&self) -> &Option<bool>;
    // fn symmetric_mut(&mut self) -> &mut &Option<bool>;
    // fn set_symmetric(&mut self, value: &Option<bool>);

    fn reflexive(&self) -> &Option<bool>;
    // fn reflexive_mut(&mut self) -> &mut &Option<bool>;
    // fn set_reflexive(&mut self, value: &Option<bool>);

    fn locally_reflexive(&self) -> &Option<bool>;
    // fn locally_reflexive_mut(&mut self) -> &mut &Option<bool>;
    // fn set_locally_reflexive(&mut self, value: &Option<bool>);

    fn irreflexive(&self) -> &Option<bool>;
    // fn irreflexive_mut(&mut self) -> &mut &Option<bool>;
    // fn set_irreflexive(&mut self, value: &Option<bool>);

    fn asymmetric(&self) -> &Option<bool>;
    // fn asymmetric_mut(&mut self) -> &mut &Option<bool>;
    // fn set_asymmetric(&mut self, value: &Option<bool>);

    fn transitive(&self) -> &Option<bool>;
    // fn transitive_mut(&mut self) -> &mut &Option<bool>;
    // fn set_transitive(&mut self, value: &Option<bool>);

    fn inverse(&self) -> &Option<String>;
    // fn inverse_mut(&mut self) -> &mut &Option<String>;
    // fn set_inverse<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn is_class_field(&self) -> &Option<bool>;
    // fn is_class_field_mut(&mut self) -> &mut &Option<bool>;
    // fn set_is_class_field(&mut self, value: &Option<bool>);

    fn transitive_form_of(&self) -> &Option<String>;
    // fn transitive_form_of_mut(&mut self) -> &mut &Option<String>;
    // fn set_transitive_form_of<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn reflexive_transitive_form_of(&self) -> &Option<String>;
    // fn reflexive_transitive_form_of_mut(&mut self) -> &mut &Option<String>;
    // fn set_reflexive_transitive_form_of<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn role(&self) -> &Option<String>;
    // fn role_mut(&mut self) -> &mut &Option<String>;
    // fn set_role(&mut self, value: &Option<String>);

    fn is_usage_slot(&self) -> &Option<bool>;
    // fn is_usage_slot_mut(&mut self) -> &mut &Option<bool>;
    // fn set_is_usage_slot(&mut self, value: &Option<bool>);

    fn usage_slot_name(&self) -> &Option<String>;
    // fn usage_slot_name_mut(&mut self) -> &mut &Option<String>;
    // fn set_usage_slot_name(&mut self, value: &Option<String>);

    fn relational_role(&self) -> &Option<String>;
    // fn relational_role_mut(&mut self) -> &mut &Option<String>;
    // fn set_relational_role(&mut self, value: &Option<String>);

    fn slot_group(&self) -> &Option<String>;
    // fn slot_group_mut(&mut self) -> &mut &Option<String>;
    // fn set_slot_group<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn is_grouping_slot(&self) -> &Option<bool>;
    // fn is_grouping_slot_mut(&mut self) -> &mut &Option<bool>;
    // fn set_is_grouping_slot(&mut self, value: &Option<bool>);

    fn path_rule(&self) -> Option<&crate::PathExpression>;
    // fn path_rule_mut(&mut self) -> &mut Option<&crate::PathExpression>;
    // fn set_path_rule<E>(&mut self, value: Option<E>) where E: Into<PathExpression>;

    fn disjoint_with(&self) -> impl poly_containers::SeqRef<String>;
    // fn disjoint_with_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_disjoint_with<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn children_are_mutually_disjoint(&self) -> &Option<bool>;
    // fn children_are_mutually_disjoint_mut(&mut self) -> &mut &Option<bool>;
    // fn set_children_are_mutually_disjoint(&mut self, value: &Option<bool>);

    fn union_of(&self) -> impl poly_containers::SeqRef<String>;
    // fn union_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_union_of<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn type_mappings(&self) -> impl poly_containers::MapRef<String,crate::TypeMapping>;
    // fn type_mappings_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::TypeMapping>;
    // fn set_type_mappings<E>(&mut self, value: &HashMap<String, E>) where E: Into<TypeMapping>;

    fn range(&self) -> &Option<String>;
    // fn range_mut(&mut self) -> &mut &Option<String>;
    // fn set_range<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn range_expression(&self) -> Option<&crate::AnonymousClassExpression>;
    // fn range_expression_mut(&mut self) -> &mut Option<&crate::AnonymousClassExpression>;
    // fn set_range_expression<E>(&mut self, value: Option<E>) where E: Into<AnonymousClassExpression>;

    fn enum_range(&self) -> Option<&EnumExpressionOrSubtype>;
    // fn enum_range_mut(&mut self) -> &mut Option<&EnumExpressionOrSubtype>;
    // fn set_enum_range<E>(&mut self, value: Option<E>) where E: Into<EnumExpression>;

    fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding>;
    // fn bindings_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::EnumBinding>;
    // fn set_bindings<E>(&mut self, value: &Vec<E>) where E: Into<EnumBinding>;

    fn required(&self) -> &Option<bool>;
    // fn required_mut(&mut self) -> &mut &Option<bool>;
    // fn set_required(&mut self, value: &Option<bool>);

    fn recommended(&self) -> &Option<bool>;
    // fn recommended_mut(&mut self) -> &mut &Option<bool>;
    // fn set_recommended(&mut self, value: &Option<bool>);

    fn multivalued(&self) -> &Option<bool>;
    // fn multivalued_mut(&mut self) -> &mut &Option<bool>;
    // fn set_multivalued(&mut self, value: &Option<bool>);

    fn inlined(&self) -> &Option<bool>;
    // fn inlined_mut(&mut self) -> &mut &Option<bool>;
    // fn set_inlined(&mut self, value: &Option<bool>);

    fn inlined_as_list(&self) -> &Option<bool>;
    // fn inlined_as_list_mut(&mut self) -> &mut &Option<bool>;
    // fn set_inlined_as_list(&mut self, value: &Option<bool>);

    fn minimum_value(&self) -> Option<&crate::Anything>;
    // fn minimum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_minimum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn maximum_value(&self) -> Option<&crate::Anything>;
    // fn maximum_value_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_maximum_value<E>(&mut self, value: Option<E>) where E: Into<Anything>;

    fn pattern(&self) -> &Option<String>;
    // fn pattern_mut(&mut self) -> &mut &Option<String>;
    // fn set_pattern(&mut self, value: &Option<String>);

    fn structured_pattern(&self) -> Option<&crate::PatternExpression>;
    // fn structured_pattern_mut(&mut self) -> &mut Option<&crate::PatternExpression>;
    // fn set_structured_pattern<E>(&mut self, value: Option<E>) where E: Into<PatternExpression>;

    fn unit(&self) -> Option<&crate::UnitOfMeasure>;
    // fn unit_mut(&mut self) -> &mut Option<&crate::UnitOfMeasure>;
    // fn set_unit<E>(&mut self, value: Option<E>) where E: Into<UnitOfMeasure>;

    fn implicit_prefix(&self) -> &Option<String>;
    // fn implicit_prefix_mut(&mut self) -> &mut &Option<String>;
    // fn set_implicit_prefix(&mut self, value: &Option<String>);

    fn value_presence(&self) -> &Option<String>;
    // fn value_presence_mut(&mut self) -> &mut &Option<String>;
    // fn set_value_presence(&mut self, value: &Option<String>);

    fn equals_string(&self) -> &Option<String>;
    // fn equals_string_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_string(&mut self, value: &Option<String>);

    fn equals_string_in(&self) -> impl poly_containers::SeqRef<String>;
    // fn equals_string_in_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_equals_string_in(&mut self, value: &Vec<String>);

    fn equals_number(&self) -> &Option<isize>;
    // fn equals_number_mut(&mut self) -> &mut &Option<isize>;
    // fn set_equals_number(&mut self, value: &Option<isize>);

    fn equals_expression(&self) -> &Option<String>;
    // fn equals_expression_mut(&mut self) -> &mut &Option<String>;
    // fn set_equals_expression(&mut self, value: &Option<String>);

    fn exact_cardinality(&self) -> &Option<isize>;
    // fn exact_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_exact_cardinality(&mut self, value: &Option<isize>);

    fn minimum_cardinality(&self) -> &Option<isize>;
    // fn minimum_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_minimum_cardinality(&mut self, value: &Option<isize>);

    fn maximum_cardinality(&self) -> &Option<isize>;
    // fn maximum_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_maximum_cardinality(&mut self, value: &Option<isize>);

    fn has_member(&self) -> Option<&crate::AnonymousSlotExpression>;
    // fn has_member_mut(&mut self) -> &mut Option<&crate::AnonymousSlotExpression>;
    // fn set_has_member<E>(&mut self, value: Option<E>) where E: Into<AnonymousSlotExpression>;

    fn all_members(&self) -> Option<&crate::AnonymousSlotExpression>;
    // fn all_members_mut(&mut self) -> &mut Option<&crate::AnonymousSlotExpression>;
    // fn set_all_members<E>(&mut self, value: Option<E>) where E: Into<AnonymousSlotExpression>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousSlotExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousSlotExpression>;


}

impl SlotDefinition for crate::SlotDefinition {
        fn singular_name(&self) -> &Option<String> {
        return &self.singular_name;
    }
        fn domain(&self) -> &Option<String> {
        return &self.domain;
    }
        fn slot_uri(&self) -> &Option<uriorcurie> {
        return &self.slot_uri;
    }
        fn array(&self) -> Option<&crate::ArrayExpression> {
        return self.array.as_ref();
    }
        fn inherited(&self) -> &Option<bool> {
        return &self.inherited;
    }
        fn readonly(&self) -> &Option<String> {
        return &self.readonly;
    }
        fn ifabsent(&self) -> &Option<String> {
        return &self.ifabsent;
    }
        fn list_elements_unique(&self) -> &Option<bool> {
        return &self.list_elements_unique;
    }
        fn list_elements_ordered(&self) -> &Option<bool> {
        return &self.list_elements_ordered;
    }
        fn shared(&self) -> &Option<bool> {
        return &self.shared;
    }
        fn key(&self) -> &Option<bool> {
        return &self.key;
    }
        fn identifier(&self) -> &Option<bool> {
        return &self.identifier;
    }
        fn designates_type(&self) -> &Option<bool> {
        return &self.designates_type;
    }
        fn alias(&self) -> &Option<String> {
        return &self.alias;
    }
        fn owner(&self) -> &Option<String> {
        return &self.owner;
    }
        fn domain_of(&self) -> impl poly_containers::SeqRef<String> {
        return &self.domain_of;
    }
        fn subproperty_of(&self) -> &Option<String> {
        return &self.subproperty_of;
    }
        fn symmetric(&self) -> &Option<bool> {
        return &self.symmetric;
    }
        fn reflexive(&self) -> &Option<bool> {
        return &self.reflexive;
    }
        fn locally_reflexive(&self) -> &Option<bool> {
        return &self.locally_reflexive;
    }
        fn irreflexive(&self) -> &Option<bool> {
        return &self.irreflexive;
    }
        fn asymmetric(&self) -> &Option<bool> {
        return &self.asymmetric;
    }
        fn transitive(&self) -> &Option<bool> {
        return &self.transitive;
    }
        fn inverse(&self) -> &Option<String> {
        return &self.inverse;
    }
        fn is_class_field(&self) -> &Option<bool> {
        return &self.is_class_field;
    }
        fn transitive_form_of(&self) -> &Option<String> {
        return &self.transitive_form_of;
    }
        fn reflexive_transitive_form_of(&self) -> &Option<String> {
        return &self.reflexive_transitive_form_of;
    }
        fn role(&self) -> &Option<String> {
        return &self.role;
    }
        fn is_usage_slot(&self) -> &Option<bool> {
        return &self.is_usage_slot;
    }
        fn usage_slot_name(&self) -> &Option<String> {
        return &self.usage_slot_name;
    }
        fn relational_role(&self) -> &Option<String> {
        return &self.relational_role;
    }
        fn slot_group(&self) -> &Option<String> {
        return &self.slot_group;
    }
        fn is_grouping_slot(&self) -> &Option<bool> {
        return &self.is_grouping_slot;
    }
        fn path_rule(&self) -> Option<&crate::PathExpression> {
// None
        return self.path_rule.as_deref();
    }
        fn disjoint_with(&self) -> impl poly_containers::SeqRef<String> {
        return &self.disjoint_with;
    }
        fn children_are_mutually_disjoint(&self) -> &Option<bool> {
        return &self.children_are_mutually_disjoint;
    }
        fn union_of(&self) -> impl poly_containers::SeqRef<String> {
        return &self.union_of;
    }
        fn type_mappings(&self) -> impl poly_containers::MapRef<String,crate::TypeMapping> {
        return &self.type_mappings;
    }
        fn range(&self) -> &Option<String> {
        return &self.range;
    }
        fn range_expression(&self) -> Option<&crate::AnonymousClassExpression> {
// None
        return self.range_expression.as_deref();
    }
        fn enum_range(&self) -> Option<&EnumExpressionOrSubtype> {
        return self.enum_range.as_ref();
    }
        fn bindings(&self) -> impl poly_containers::SeqRef<crate::EnumBinding> {
        return &self.bindings;
    }
        fn required(&self) -> &Option<bool> {
        return &self.required;
    }
        fn recommended(&self) -> &Option<bool> {
        return &self.recommended;
    }
        fn multivalued(&self) -> &Option<bool> {
        return &self.multivalued;
    }
        fn inlined(&self) -> &Option<bool> {
        return &self.inlined;
    }
        fn inlined_as_list(&self) -> &Option<bool> {
        return &self.inlined_as_list;
    }
        fn minimum_value(&self) -> Option<&crate::Anything> {
        return self.minimum_value.as_ref();
    }
        fn maximum_value(&self) -> Option<&crate::Anything> {
        return self.maximum_value.as_ref();
    }
        fn pattern(&self) -> &Option<String> {
        return &self.pattern;
    }
        fn structured_pattern(&self) -> Option<&crate::PatternExpression> {
        return self.structured_pattern.as_ref();
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn implicit_prefix(&self) -> &Option<String> {
        return &self.implicit_prefix;
    }
        fn value_presence(&self) -> &Option<String> {
        return &self.value_presence;
    }
        fn equals_string(&self) -> &Option<String> {
        return &self.equals_string;
    }
        fn equals_string_in(&self) -> impl poly_containers::SeqRef<String> {
        return &self.equals_string_in;
    }
        fn equals_number(&self) -> &Option<isize> {
        return &self.equals_number;
    }
        fn equals_expression(&self) -> &Option<String> {
        return &self.equals_expression;
    }
        fn exact_cardinality(&self) -> &Option<isize> {
        return &self.exact_cardinality;
    }
        fn minimum_cardinality(&self) -> &Option<isize> {
        return &self.minimum_cardinality;
    }
        fn maximum_cardinality(&self) -> &Option<isize> {
        return &self.maximum_cardinality;
    }
        fn has_member(&self) -> Option<&crate::AnonymousSlotExpression> {
// None
        return self.has_member.as_deref();
    }
        fn all_members(&self) -> Option<&crate::AnonymousSlotExpression> {
// None
        return self.all_members.as_deref();
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousSlotExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
}

pub trait ClassExpression  {

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn slot_conditions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn slot_conditions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn set_slot_conditions<E>(&mut self, value: &HashMap<String, E>) where E: Into<SlotDefinition>;


}

impl ClassExpression for crate::ClassExpression {
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
        return &self.any_of;
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
        return &self.exactly_one_of;
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
        return &self.none_of;
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
        return &self.all_of;
    }
        fn slot_conditions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition> {
        return &self.slot_conditions;
    }
}
impl ClassExpression for crate::AnonymousClassExpression {
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
        fn slot_conditions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition> {
// mapping
        return poly_containers::MapView::new(&self.slot_conditions);
    }
}
impl ClassExpression for crate::ClassDefinition {
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
        fn slot_conditions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition> {
// mapping
        return poly_containers::MapView::new(&self.slot_conditions);
    }
}

pub trait AnonymousClassExpression: AnonymousExpression  {

    fn is_a(&self) -> &Option<String>;
    // fn is_a_mut(&mut self) -> &mut &Option<String>;
    // fn set_is_a<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn slot_conditions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn slot_conditions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn set_slot_conditions<E>(&mut self, value: &HashMap<String, E>) where E: Into<SlotDefinition>;


}

impl AnonymousClassExpression for crate::AnonymousClassExpression {
        fn is_a(&self) -> &Option<String> {
        return &self.is_a;
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
        fn slot_conditions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition> {
// mapping
        return poly_containers::MapView::new(&self.slot_conditions);
    }
}

pub trait ClassDefinition: Definition  {

    fn slots(&self) -> impl poly_containers::SeqRef<String>;
    // fn slots_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_slots<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn slot_usage(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn slot_usage_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn set_slot_usage<E>(&mut self, value: &HashMap<String, E>) where E: Into<SlotDefinition>;

    fn attributes(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn attributes_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn set_attributes<E>(&mut self, value: &HashMap<String, E>) where E: Into<SlotDefinition>;

    fn class_uri(&self) -> &Option<uriorcurie>;
    // fn class_uri_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_class_uri(&mut self, value: &Option<uriorcurie>);

    fn subclass_of(&self) -> &Option<uriorcurie>;
    // fn subclass_of_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_subclass_of(&mut self, value: &Option<uriorcurie>);

    fn union_of(&self) -> impl poly_containers::SeqRef<String>;
    // fn union_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_union_of<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn defining_slots(&self) -> impl poly_containers::SeqRef<String>;
    // fn defining_slots_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_defining_slots<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn tree_root(&self) -> &Option<bool>;
    // fn tree_root_mut(&mut self) -> &mut &Option<bool>;
    // fn set_tree_root(&mut self, value: &Option<bool>);

    fn unique_keys(&self) -> impl poly_containers::MapRef<String,crate::UniqueKey>;
    // fn unique_keys_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::UniqueKey>;
    // fn set_unique_keys<E>(&mut self, value: &HashMap<String, E>) where E: Into<UniqueKey>;

    fn rules(&self) -> impl poly_containers::SeqRef<crate::ClassRule>;
    // fn rules_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::ClassRule>;
    // fn set_rules<E>(&mut self, value: &Vec<E>) where E: Into<ClassRule>;

    fn classification_rules(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn classification_rules_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_classification_rules<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn slot_names_unique(&self) -> &Option<bool>;
    // fn slot_names_unique_mut(&mut self) -> &mut &Option<bool>;
    // fn set_slot_names_unique(&mut self, value: &Option<bool>);

    fn represents_relationship(&self) -> &Option<bool>;
    // fn represents_relationship_mut(&mut self) -> &mut &Option<bool>;
    // fn set_represents_relationship(&mut self, value: &Option<bool>);

    fn disjoint_with(&self) -> impl poly_containers::SeqRef<String>;
    // fn disjoint_with_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_disjoint_with<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn children_are_mutually_disjoint(&self) -> &Option<bool>;
    // fn children_are_mutually_disjoint_mut(&mut self) -> &mut &Option<bool>;
    // fn set_children_are_mutually_disjoint(&mut self, value: &Option<bool>);

    fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn any_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_any_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn exactly_one_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_exactly_one_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn none_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_none_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn all_of_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::AnonymousClassExpression>;
    // fn set_all_of<E>(&mut self, value: &Vec<E>) where E: Into<AnonymousClassExpression>;

    fn slot_conditions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn slot_conditions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::SlotDefinition>;
    // fn set_slot_conditions<E>(&mut self, value: &HashMap<String, E>) where E: Into<SlotDefinition>;


}

impl ClassDefinition for crate::ClassDefinition {
        fn slots(&self) -> impl poly_containers::SeqRef<String> {
        return &self.slots;
    }
        fn slot_usage(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition> {
// mapping
        return poly_containers::MapView::new(&self.slot_usage);
    }
        fn attributes(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition> {
// mapping
        return poly_containers::MapView::new(&self.attributes);
    }
        fn class_uri(&self) -> &Option<uriorcurie> {
        return &self.class_uri;
    }
        fn subclass_of(&self) -> &Option<uriorcurie> {
        return &self.subclass_of;
    }
        fn union_of(&self) -> impl poly_containers::SeqRef<String> {
        return &self.union_of;
    }
        fn defining_slots(&self) -> impl poly_containers::SeqRef<String> {
        return &self.defining_slots;
    }
        fn tree_root(&self) -> &Option<bool> {
        return &self.tree_root;
    }
        fn unique_keys(&self) -> impl poly_containers::MapRef<String,crate::UniqueKey> {
// mapping
        return poly_containers::MapView::new(&self.unique_keys);
    }
        fn rules(&self) -> impl poly_containers::SeqRef<crate::ClassRule> {
// list
        return poly_containers::ListView::new(&self.rules);
    }
        fn classification_rules(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.classification_rules);
    }
        fn slot_names_unique(&self) -> &Option<bool> {
        return &self.slot_names_unique;
    }
        fn represents_relationship(&self) -> &Option<bool> {
        return &self.represents_relationship;
    }
        fn disjoint_with(&self) -> impl poly_containers::SeqRef<String> {
        return &self.disjoint_with;
    }
        fn children_are_mutually_disjoint(&self) -> &Option<bool> {
        return &self.children_are_mutually_disjoint;
    }
        fn any_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.any_of);
    }
        fn exactly_one_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.exactly_one_of);
    }
        fn none_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.none_of);
    }
        fn all_of(&self) -> impl poly_containers::SeqRef<crate::AnonymousClassExpression> {
// list
        return poly_containers::ListView::new(&self.all_of);
    }
        fn slot_conditions(&self) -> impl poly_containers::MapRef<String,crate::SlotDefinition> {
// mapping
        return poly_containers::MapView::new(&self.slot_conditions);
    }
}

pub trait ClassLevelRule  {


}

impl ClassLevelRule for crate::ClassLevelRule {
}
impl ClassLevelRule for crate::ClassRule {
}

pub trait ClassRule: ClassLevelRule  {

    fn preconditions(&self) -> Option<&crate::AnonymousClassExpression>;
    // fn preconditions_mut(&mut self) -> &mut Option<&crate::AnonymousClassExpression>;
    // fn set_preconditions<E>(&mut self, value: Option<E>) where E: Into<AnonymousClassExpression>;

    fn postconditions(&self) -> Option<&crate::AnonymousClassExpression>;
    // fn postconditions_mut(&mut self) -> &mut Option<&crate::AnonymousClassExpression>;
    // fn set_postconditions<E>(&mut self, value: Option<E>) where E: Into<AnonymousClassExpression>;

    fn elseconditions(&self) -> Option<&crate::AnonymousClassExpression>;
    // fn elseconditions_mut(&mut self) -> &mut Option<&crate::AnonymousClassExpression>;
    // fn set_elseconditions<E>(&mut self, value: Option<E>) where E: Into<AnonymousClassExpression>;

    fn bidirectional(&self) -> &Option<bool>;
    // fn bidirectional_mut(&mut self) -> &mut &Option<bool>;
    // fn set_bidirectional(&mut self, value: &Option<bool>);

    fn open_world(&self) -> &Option<bool>;
    // fn open_world_mut(&mut self) -> &mut &Option<bool>;
    // fn set_open_world(&mut self, value: &Option<bool>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn deactivated(&self) -> &Option<bool>;
    // fn deactivated_mut(&mut self) -> &mut &Option<bool>;
    // fn set_deactivated(&mut self, value: &Option<bool>);

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl ClassRule for crate::ClassRule {
        fn preconditions(&self) -> Option<&crate::AnonymousClassExpression> {
// None
        return self.preconditions.as_deref();
    }
        fn postconditions(&self) -> Option<&crate::AnonymousClassExpression> {
// None
        return self.postconditions.as_deref();
    }
        fn elseconditions(&self) -> Option<&crate::AnonymousClassExpression> {
// None
        return self.elseconditions.as_deref();
    }
        fn bidirectional(&self) -> &Option<bool> {
        return &self.bidirectional;
    }
        fn open_world(&self) -> &Option<bool> {
        return &self.open_world;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn deactivated(&self) -> &Option<bool> {
        return &self.deactivated;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait ArrayExpression  {

    fn exact_number_dimensions(&self) -> &Option<isize>;
    // fn exact_number_dimensions_mut(&mut self) -> &mut &Option<isize>;
    // fn set_exact_number_dimensions(&mut self, value: &Option<isize>);

    fn minimum_number_dimensions(&self) -> &Option<isize>;
    // fn minimum_number_dimensions_mut(&mut self) -> &mut &Option<isize>;
    // fn set_minimum_number_dimensions(&mut self, value: &Option<isize>);

    fn maximum_number_dimensions(&self) -> &Option<array_expression_utl::maximum_number_dimensions_range>;
    // fn maximum_number_dimensions_mut(&mut self) -> &mut &Option<array_expression_utl::maximum_number_dimensions_range>;
    // fn set_maximum_number_dimensions(&mut self, value: &Option<array_expression_utl::maximum_number_dimensions_range>);

    fn dimensions(&self) -> impl poly_containers::SeqRef<crate::DimensionExpression>;
    // fn dimensions_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::DimensionExpression>;
    // fn set_dimensions<E>(&mut self, value: &Vec<E>) where E: Into<DimensionExpression>;

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl ArrayExpression for crate::ArrayExpression {
        fn exact_number_dimensions(&self) -> &Option<isize> {
        return &self.exact_number_dimensions;
    }
        fn minimum_number_dimensions(&self) -> &Option<isize> {
        return &self.minimum_number_dimensions;
    }
        fn maximum_number_dimensions(&self) -> &Option<array_expression_utl::maximum_number_dimensions_range> {
        return &self.maximum_number_dimensions;
    }
        fn dimensions(&self) -> impl poly_containers::SeqRef<crate::DimensionExpression> {
        return &self.dimensions;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait DimensionExpression  {

    fn alias(&self) -> &Option<String>;
    // fn alias_mut(&mut self) -> &mut &Option<String>;
    // fn set_alias(&mut self, value: &Option<String>);

    fn maximum_cardinality(&self) -> &Option<isize>;
    // fn maximum_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_maximum_cardinality(&mut self, value: &Option<isize>);

    fn minimum_cardinality(&self) -> &Option<isize>;
    // fn minimum_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_minimum_cardinality(&mut self, value: &Option<isize>);

    fn exact_cardinality(&self) -> &Option<isize>;
    // fn exact_cardinality_mut(&mut self) -> &mut &Option<isize>;
    // fn set_exact_cardinality(&mut self, value: &Option<isize>);

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl DimensionExpression for crate::DimensionExpression {
        fn alias(&self) -> &Option<String> {
        return &self.alias;
    }
        fn maximum_cardinality(&self) -> &Option<isize> {
        return &self.maximum_cardinality;
    }
        fn minimum_cardinality(&self) -> &Option<isize> {
        return &self.minimum_cardinality;
    }
        fn exact_cardinality(&self) -> &Option<isize> {
        return &self.exact_cardinality;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait PatternExpression  {

    fn syntax(&self) -> &Option<String>;
    // fn syntax_mut(&mut self) -> &mut &Option<String>;
    // fn set_syntax(&mut self, value: &Option<String>);

    fn interpolated(&self) -> &Option<bool>;
    // fn interpolated_mut(&mut self) -> &mut &Option<bool>;
    // fn set_interpolated(&mut self, value: &Option<bool>);

    fn partial_match(&self) -> &Option<bool>;
    // fn partial_match_mut(&mut self) -> &mut &Option<bool>;
    // fn set_partial_match(&mut self, value: &Option<bool>);

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl PatternExpression for crate::PatternExpression {
        fn syntax(&self) -> &Option<String> {
        return &self.syntax;
    }
        fn interpolated(&self) -> &Option<bool> {
        return &self.interpolated;
    }
        fn partial_match(&self) -> &Option<bool> {
        return &self.partial_match;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait ImportExpression  {

    fn import_from(&self) -> &uriorcurie;
    // fn import_from_mut(&mut self) -> &mut &uriorcurie;
    // fn set_import_from(&mut self, value: uriorcurie);

    fn import_as(&self) -> &Option<ncname>;
    // fn import_as_mut(&mut self) -> &mut &Option<ncname>;
    // fn set_import_as(&mut self, value: &Option<ncname>);

    fn import_map(&self) -> impl poly_containers::MapRef<String,crate::Setting>;
    // fn import_map_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Setting>;
    // fn set_import_map<E>(&mut self, value: &HashMap<String, E>) where E: Into<Setting>;

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl ImportExpression for crate::ImportExpression {
        fn import_from(&self) -> &uriorcurie {
        return &self.import_from;
    }
        fn import_as(&self) -> &Option<ncname> {
        return &self.import_as;
    }
        fn import_map(&self) -> impl poly_containers::MapRef<String,crate::Setting> {
        return &self.import_map;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait Setting  {

    fn setting_key(&self) -> &ncname;
    // fn setting_key_mut(&mut self) -> &mut &ncname;
    // fn set_setting_key(&mut self, value: ncname);

    fn setting_value(&self) -> &String;
    // fn setting_value_mut(&mut self) -> &mut &String;
    // fn set_setting_value(&mut self, value: String);


}

impl Setting for crate::Setting {
        fn setting_key(&self) -> &ncname {
        return &self.setting_key;
    }
        fn setting_value(&self) -> &String {
        return &self.setting_value;
    }
}

pub trait Prefix  {

    fn prefix_prefix(&self) -> &ncname;
    // fn prefix_prefix_mut(&mut self) -> &mut &ncname;
    // fn set_prefix_prefix(&mut self, value: ncname);

    fn prefix_reference(&self) -> &uri;
    // fn prefix_reference_mut(&mut self) -> &mut &uri;
    // fn set_prefix_reference(&mut self, value: uri);


}

impl Prefix for crate::Prefix {
        fn prefix_prefix(&self) -> &ncname {
        return &self.prefix_prefix;
    }
        fn prefix_reference(&self) -> &uri {
        return &self.prefix_reference;
    }
}

pub trait LocalName  {

    fn local_name_source(&self) -> &ncname;
    // fn local_name_source_mut(&mut self) -> &mut &ncname;
    // fn set_local_name_source(&mut self, value: ncname);

    fn local_name_value(&self) -> &String;
    // fn local_name_value_mut(&mut self) -> &mut &String;
    // fn set_local_name_value(&mut self, value: String);


}

impl LocalName for crate::LocalName {
        fn local_name_source(&self) -> &ncname {
        return &self.local_name_source;
    }
        fn local_name_value(&self) -> &String {
        return &self.local_name_value;
    }
}

pub trait Example  {

    fn value(&self) -> &Option<String>;
    // fn value_mut(&mut self) -> &mut &Option<String>;
    // fn set_value(&mut self, value: &Option<String>);

    fn value_description(&self) -> &Option<String>;
    // fn value_description_mut(&mut self) -> &mut &Option<String>;
    // fn set_value_description(&mut self, value: &Option<String>);

    fn value_object(&self) -> Option<&crate::Anything>;
    // fn value_object_mut(&mut self) -> &mut Option<&crate::Anything>;
    // fn set_value_object<E>(&mut self, value: Option<E>) where E: Into<Anything>;


}

impl Example for crate::Example {
        fn value(&self) -> &Option<String> {
        return &self.value;
    }
        fn value_description(&self) -> &Option<String> {
        return &self.value_description;
    }
        fn value_object(&self) -> Option<&crate::Anything> {
        return self.value_object.as_ref();
    }
}

pub trait AltDescription  {

    fn alt_description_source(&self) -> &String;
    // fn alt_description_source_mut(&mut self) -> &mut &String;
    // fn set_alt_description_source(&mut self, value: String);

    fn alt_description_text(&self) -> &String;
    // fn alt_description_text_mut(&mut self) -> &mut &String;
    // fn set_alt_description_text(&mut self, value: String);


}

impl AltDescription for crate::AltDescription {
        fn alt_description_source(&self) -> &String {
        return &self.alt_description_source;
    }
        fn alt_description_text(&self) -> &String {
        return &self.alt_description_text;
    }
}

pub trait PermissibleValue  {

    fn text(&self) -> &String;
    // fn text_mut(&mut self) -> &mut &String;
    // fn set_text(&mut self, value: String);

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn meaning(&self) -> &Option<uriorcurie>;
    // fn meaning_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_meaning(&mut self, value: &Option<uriorcurie>);

    fn unit(&self) -> Option<&crate::UnitOfMeasure>;
    // fn unit_mut(&mut self) -> &mut Option<&crate::UnitOfMeasure>;
    // fn set_unit<E>(&mut self, value: Option<E>) where E: Into<UnitOfMeasure>;

    fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn instantiates_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_instantiates(&mut self, value: &Vec<uriorcurie>);

    fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn implements_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_implements(&mut self, value: &Vec<uriorcurie>);

    fn is_a(&self) -> &Option<String>;
    // fn is_a_mut(&mut self) -> &mut &Option<String>;
    // fn set_is_a<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn mixins(&self) -> impl poly_containers::SeqRef<String>;
    // fn mixins_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_mixins<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl PermissibleValue for crate::PermissibleValue {
        fn text(&self) -> &String {
        return &self.text;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn meaning(&self) -> &Option<uriorcurie> {
        return &self.meaning;
    }
        fn unit(&self) -> Option<&crate::UnitOfMeasure> {
        return self.unit.as_ref();
    }
        fn instantiates(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.instantiates;
    }
        fn implements(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.implements;
    }
        fn is_a(&self) -> &Option<String> {
        return &self.is_a;
    }
        fn mixins(&self) -> impl poly_containers::SeqRef<String> {
        return &self.mixins;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait UniqueKey  {

    fn unique_key_name(&self) -> &String;
    // fn unique_key_name_mut(&mut self) -> &mut &String;
    // fn set_unique_key_name(&mut self, value: String);

    fn unique_key_slots(&self) -> impl poly_containers::SeqRef<String>;
    // fn unique_key_slots_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_unique_key_slots<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn consider_nulls_inequal(&self) -> &Option<bool>;
    // fn consider_nulls_inequal_mut(&mut self) -> &mut &Option<bool>;
    // fn set_consider_nulls_inequal(&mut self, value: &Option<bool>);

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl UniqueKey for crate::UniqueKey {
        fn unique_key_name(&self) -> &String {
        return &self.unique_key_name;
    }
        fn unique_key_slots(&self) -> impl poly_containers::SeqRef<String> {
        return &self.unique_key_slots;
    }
        fn consider_nulls_inequal(&self) -> &Option<bool> {
        return &self.consider_nulls_inequal;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

pub trait TypeMapping  {

    fn framework_key(&self) -> &String;
    // fn framework_key_mut(&mut self) -> &mut &String;
    // fn set_framework_key(&mut self, value: String);

    fn mapped_type(&self) -> &Option<String>;
    // fn mapped_type_mut(&mut self) -> &mut &Option<String>;
    // fn set_mapped_type<E>(&mut self, value: &Option<String>) where E: Into<String>;

    fn string_serialization(&self) -> &Option<String>;
    // fn string_serialization_mut(&mut self) -> &mut &Option<String>;
    // fn set_string_serialization(&mut self, value: &Option<String>);

    fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn extensions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,ExtensionOrSubtype>;
    // fn set_extensions<E>(&mut self, value: &HashMap<String, E>) where E: Into<Extension>;

    fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation>;
    // fn annotations_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::Annotation>;
    // fn set_annotations<E>(&mut self, value: &HashMap<String, E>) where E: Into<Annotation>;

    fn description(&self) -> &Option<String>;
    // fn description_mut(&mut self) -> &mut &Option<String>;
    // fn set_description(&mut self, value: &Option<String>);

    fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn alt_descriptions_mut(&mut self) -> &mut impl poly_containers::MapRef<String,crate::AltDescription>;
    // fn set_alt_descriptions<E>(&mut self, value: &HashMap<String, E>) where E: Into<AltDescription>;

    fn title(&self) -> &Option<String>;
    // fn title_mut(&mut self) -> &mut &Option<String>;
    // fn set_title(&mut self, value: &Option<String>);

    fn deprecated(&self) -> &Option<String>;
    // fn deprecated_mut(&mut self) -> &mut &Option<String>;
    // fn set_deprecated(&mut self, value: &Option<String>);

    fn todos(&self) -> impl poly_containers::SeqRef<String>;
    // fn todos_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_todos(&mut self, value: &Vec<String>);

    fn notes(&self) -> impl poly_containers::SeqRef<String>;
    // fn notes_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_notes(&mut self, value: &Vec<String>);

    fn comments(&self) -> impl poly_containers::SeqRef<String>;
    // fn comments_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_comments(&mut self, value: &Vec<String>);

    fn examples(&self) -> impl poly_containers::SeqRef<crate::Example>;
    // fn examples_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::Example>;
    // fn set_examples<E>(&mut self, value: &Vec<E>) where E: Into<Example>;

    fn in_subset(&self) -> impl poly_containers::SeqRef<String>;
    // fn in_subset_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_in_subset<E>(&mut self, value: &Vec<String>) where E: Into<String>;

    fn from_schema(&self) -> &Option<uri>;
    // fn from_schema_mut(&mut self) -> &mut &Option<uri>;
    // fn set_from_schema(&mut self, value: &Option<uri>);

    fn imported_from(&self) -> &Option<String>;
    // fn imported_from_mut(&mut self) -> &mut &Option<String>;
    // fn set_imported_from(&mut self, value: &Option<String>);

    fn source(&self) -> &Option<uriorcurie>;
    // fn source_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_source(&mut self, value: &Option<uriorcurie>);

    fn in_language(&self) -> &Option<String>;
    // fn in_language_mut(&mut self) -> &mut &Option<String>;
    // fn set_in_language(&mut self, value: &Option<String>);

    fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn see_also_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_see_also(&mut self, value: &Vec<uriorcurie>);

    fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_exact_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_exact_replacement(&mut self, value: &Option<uriorcurie>);

    fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie>;
    // fn deprecated_element_has_possible_replacement_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_deprecated_element_has_possible_replacement(&mut self, value: &Option<uriorcurie>);

    fn aliases(&self) -> impl poly_containers::SeqRef<String>;
    // fn aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_aliases(&mut self, value: &Vec<String>);

    fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn structured_aliases_mut(&mut self) -> &mut impl poly_containers::SeqRef<crate::StructuredAlias>;
    // fn set_structured_aliases<E>(&mut self, value: &Vec<E>) where E: Into<StructuredAlias>;

    fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_mappings(&mut self, value: &Vec<uriorcurie>);

    fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn exact_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_exact_mappings(&mut self, value: &Vec<uriorcurie>);

    fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn close_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_close_mappings(&mut self, value: &Vec<uriorcurie>);

    fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn related_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_related_mappings(&mut self, value: &Vec<uriorcurie>);

    fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn narrow_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_narrow_mappings(&mut self, value: &Vec<uriorcurie>);

    fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn broad_mappings_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_broad_mappings(&mut self, value: &Vec<uriorcurie>);

    fn created_by(&self) -> &Option<uriorcurie>;
    // fn created_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_created_by(&mut self, value: &Option<uriorcurie>);

    fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn contributors_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_contributors(&mut self, value: &Vec<uriorcurie>);

    fn created_on(&self) -> &Option<NaiveDateTime>;
    // fn created_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_created_on(&mut self, value: &Option<NaiveDateTime>);

    fn last_updated_on(&self) -> &Option<NaiveDateTime>;
    // fn last_updated_on_mut(&mut self) -> &mut &Option<NaiveDateTime>;
    // fn set_last_updated_on(&mut self, value: &Option<NaiveDateTime>);

    fn modified_by(&self) -> &Option<uriorcurie>;
    // fn modified_by_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_modified_by(&mut self, value: &Option<uriorcurie>);

    fn status(&self) -> &Option<uriorcurie>;
    // fn status_mut(&mut self) -> &mut &Option<uriorcurie>;
    // fn set_status(&mut self, value: &Option<uriorcurie>);

    fn rank(&self) -> &Option<isize>;
    // fn rank_mut(&mut self) -> &mut &Option<isize>;
    // fn set_rank(&mut self, value: &Option<isize>);

    fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie>;
    // fn categories_mut(&mut self) -> &mut impl poly_containers::SeqRef<uriorcurie>;
    // fn set_categories(&mut self, value: &Vec<uriorcurie>);

    fn keywords(&self) -> impl poly_containers::SeqRef<String>;
    // fn keywords_mut(&mut self) -> &mut impl poly_containers::SeqRef<String>;
    // fn set_keywords(&mut self, value: &Vec<String>);


}

impl TypeMapping for crate::TypeMapping {
        fn framework_key(&self) -> &String {
        return &self.framework_key;
    }
        fn mapped_type(&self) -> &Option<String> {
        return &self.mapped_type;
    }
        fn string_serialization(&self) -> &Option<String> {
        return &self.string_serialization;
    }
        fn extensions(&self) -> impl poly_containers::MapRef<String,ExtensionOrSubtype> {
        return &self.extensions;
    }
        fn annotations(&self) -> impl poly_containers::MapRef<String,crate::Annotation> {
        return &self.annotations;
    }
        fn description(&self) -> &Option<String> {
        return &self.description;
    }
        fn alt_descriptions(&self) -> impl poly_containers::MapRef<String,crate::AltDescription> {
        return &self.alt_descriptions;
    }
        fn title(&self) -> &Option<String> {
        return &self.title;
    }
        fn deprecated(&self) -> &Option<String> {
        return &self.deprecated;
    }
        fn todos(&self) -> impl poly_containers::SeqRef<String> {
        return &self.todos;
    }
        fn notes(&self) -> impl poly_containers::SeqRef<String> {
        return &self.notes;
    }
        fn comments(&self) -> impl poly_containers::SeqRef<String> {
        return &self.comments;
    }
        fn examples(&self) -> impl poly_containers::SeqRef<crate::Example> {
        return &self.examples;
    }
        fn in_subset(&self) -> impl poly_containers::SeqRef<String> {
        return &self.in_subset;
    }
        fn from_schema(&self) -> &Option<uri> {
        return &self.from_schema;
    }
        fn imported_from(&self) -> &Option<String> {
        return &self.imported_from;
    }
        fn source(&self) -> &Option<uriorcurie> {
        return &self.source;
    }
        fn in_language(&self) -> &Option<String> {
        return &self.in_language;
    }
        fn see_also(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.see_also;
    }
        fn deprecated_element_has_exact_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_exact_replacement;
    }
        fn deprecated_element_has_possible_replacement(&self) -> &Option<uriorcurie> {
        return &self.deprecated_element_has_possible_replacement;
    }
        fn aliases(&self) -> impl poly_containers::SeqRef<String> {
        return &self.aliases;
    }
        fn structured_aliases(&self) -> impl poly_containers::SeqRef<crate::StructuredAlias> {
        return &self.structured_aliases;
    }
        fn mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.mappings;
    }
        fn exact_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.exact_mappings;
    }
        fn close_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.close_mappings;
    }
        fn related_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.related_mappings;
    }
        fn narrow_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.narrow_mappings;
    }
        fn broad_mappings(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.broad_mappings;
    }
        fn created_by(&self) -> &Option<uriorcurie> {
        return &self.created_by;
    }
        fn contributors(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.contributors;
    }
        fn created_on(&self) -> &Option<NaiveDateTime> {
        return &self.created_on;
    }
        fn last_updated_on(&self) -> &Option<NaiveDateTime> {
        return &self.last_updated_on;
    }
        fn modified_by(&self) -> &Option<uriorcurie> {
        return &self.modified_by;
    }
        fn status(&self) -> &Option<uriorcurie> {
        return &self.status;
    }
        fn rank(&self) -> &Option<isize> {
        return &self.rank;
    }
        fn categories(&self) -> impl poly_containers::SeqRef<uriorcurie> {
        return &self.categories;
    }
        fn keywords(&self) -> impl poly_containers::SeqRef<String> {
        return &self.keywords;
    }
}

