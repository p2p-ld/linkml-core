#![allow(non_camel_case_types)]

#[cfg(feature = "serde")]
mod serde_utils;
pub mod poly;
pub mod poly_containers;

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

// Types

pub type string = String;
pub type integer = String;
pub type boolean = String;
pub type float = f64;
pub type double = f64;
pub type decimal = String;
pub type time = String;
pub type date = String;
pub type datetime = String;
pub type date_or_datetime = String;
pub type uriorcurie = String;
pub type curie = String;
pub type uri = String;
pub type ncname = String;
pub type objectidentifier = String;
pub type nodeidentifier = String;
pub type jsonpointer = String;
pub type jsonpath = String;
pub type sparqlpath = String;

// Slots

pub type mappings = Vec<uriorcurie>;
pub type exact_mappings = Vec<uriorcurie>;
pub type close_mappings = Vec<uriorcurie>;
pub type related_mappings = Vec<uriorcurie>;
pub type narrow_mappings = Vec<uriorcurie>;
pub type broad_mappings = Vec<uriorcurie>;
pub type deprecated_element_has_exact_replacement = uriorcurie;
pub type deprecated_element_has_possible_replacement = uriorcurie;
pub type extensions = Vec<Extension>;
pub type extension_tag = uriorcurie;
pub type extension_value = AnyValue;
pub type annotations = Vec<Annotation>;
pub type unit = UnitOfMeasure;
pub type ucum_code = String;
pub type derivation = String;
pub type has_quantity_kind = uriorcurie;
pub type iec61360code = String;
pub type symbol = String;
pub type abbreviation = String;
pub type descriptive_name = String;
pub type name = String;
pub type title = String;
pub type conforms_to = String;
pub type implements = Vec<uriorcurie>;
pub type instantiates = Vec<uriorcurie>;
pub type categories = Vec<uriorcurie>;
pub type keywords = Vec<String>;
pub type definition_uri = uriorcurie;
pub type id_prefixes = Vec<ncname>;
pub type id_prefixes_are_closed = bool;
pub type description = String;
pub type structured_aliases = Vec<StructuredAlias>;
pub type aliases = Vec<String>;
pub type deprecated = String;
pub type todos = Vec<String>;
pub type notes = Vec<String>;
pub type comments = Vec<String>;
pub type in_subset = Vec<SubsetDefinition>;
pub type from_schema = uri;
pub type imported_from = String;
pub type see_also = Vec<uriorcurie>;
pub type owned_by = uriorcurie;
pub type created_by = uriorcurie;
pub type contributors = Vec<uriorcurie>;
pub type created_on = NaiveDateTime;
pub type last_updated_on = NaiveDateTime;
pub type modified_by = uriorcurie;
pub type status = uriorcurie;
pub type literal_form = String;
pub type alias_predicate = String;
pub type alias_contexts = Vec<uri>;
pub type in_language = String;
pub type source = uriorcurie;
pub type publisher = uriorcurie;
pub type is_a = Definition;
pub type abstract_ = bool;
pub type mixin = bool;
pub type mixins = Vec<Definition>;
pub type apply_to = Vec<Definition>;
pub type values_from = Vec<uriorcurie>;
pub type code_set = uriorcurie;
pub type code_set_version = String;
pub type code_set_tag = String;
pub type pv_formula = String;
pub type permissible_values = Vec<PermissibleValue>;
pub type enum_uri = uriorcurie;
pub type include = Vec<AnonymousEnumExpression>;
pub type minus = Vec<AnonymousEnumExpression>;
pub type inherits = Vec<EnumDefinition>;
pub type matches = MatchQuery;
pub type identifier_pattern = String;
pub type concepts = Vec<uriorcurie>;
pub type reachable_from = ReachabilityQuery;
pub type source_ontology = uriorcurie;
pub type is_direct = bool;
pub type traverse_up = bool;
pub type include_self = bool;
pub type relationship_types = Vec<uriorcurie>;
pub type source_nodes = Vec<uriorcurie>;
pub type text = String;
pub type meaning = uriorcurie;
pub type id = uri;
pub type emit_prefixes = Vec<ncname>;
pub type version = String;
pub type imports = Vec<uriorcurie>;
pub type structured_imports = Vec<ImportExpression>;
pub type license = String;
pub type default_curi_maps = Vec<String>;
pub type default_prefix = String;
pub type default_range = TypeDefinition;
pub type subsets = Vec<SubsetDefinition>;
pub type types = Vec<TypeDefinition>;
pub type enums = Vec<EnumDefinition>;
pub type slot_definitions = Vec<SlotDefinition>;
pub type classes = Vec<ClassDefinition>;
pub type metamodel_version = String;
pub type source_file = String;
pub type source_file_date = NaiveDateTime;
pub type source_file_size = isize;
pub type generation_date = NaiveDateTime;
pub type slots = Vec<SlotDefinition>;
pub type slot_usage = Vec<SlotDefinition>;
pub type enum_range = EnumExpression;
pub type range_expression = AnonymousClassExpression;
pub type boolean_slot = Vec<Expression>;
pub type any_of = Vec<Expression>;
pub type exactly_one_of = Vec<Expression>;
pub type none_of = Vec<Expression>;
pub type all_of = Vec<Expression>;
pub type preconditions = AnonymousClassExpression;
pub type postconditions = AnonymousClassExpression;
pub type elseconditions = AnonymousClassExpression;
pub type bidirectional = bool;
pub type open_world = bool;
pub type rank = isize;
pub type deactivated = bool;
pub type rules = Vec<ClassRule>;
pub type classification_rules = Vec<AnonymousClassExpression>;
pub type slot_conditions = Vec<SlotDefinition>;
pub type attributes = Vec<SlotDefinition>;
pub type class_uri = uriorcurie;
pub type subclass_of = uriorcurie;
pub type defining_slots = Vec<SlotDefinition>;
pub type union_of = Vec<Element>;
pub type tree_root = bool;
pub type unique_keys = Vec<UniqueKey>;
pub type unique_key_name = String;
pub type consider_nulls_inequal = bool;
pub type unique_key_slots = Vec<SlotDefinition>;
pub type slot_names_unique = bool;
pub type domain = ClassDefinition;
pub type range = Element;
pub type slot_uri = uriorcurie;
pub type multivalued = bool;
pub type array = ArrayExpression;
pub type dimensions = Vec<DimensionExpression>;
pub type minimum_number_dimensions = isize;
pub type maximum_number_dimensions = Anything;
pub type exact_number_dimensions = isize;
pub type inherited = bool;
pub type readonly = String;
pub type ifabsent = String;
pub type implicit_prefix = String;
pub type value_specification_constant = String;
pub type list_value_specification_constant = String;
pub type value_presence = String;
pub type equals_string = String;
pub type equals_number = isize;
pub type equals_expression = String;
pub type exact_cardinality = isize;
pub type minimum_cardinality = isize;
pub type maximum_cardinality = isize;
pub type equals_string_in = Vec<String>;
pub type equals_number_in = Vec<isize>;
pub type has_member = AnonymousSlotExpression;
pub type all_members = AnonymousSlotExpression;
pub type singular_name = String;
pub type required = bool;
pub type recommended = bool;
pub type inapplicable = bool;
pub type inlined = bool;
pub type inlined_as_list = bool;
pub type inlined_as_simple_dict = bool;
pub type list_elements_ordered = bool;
pub type list_elements_unique = bool;
pub type shared = bool;
pub type key = bool;
pub type identifier = bool;
pub type designates_type = bool;
pub type alias = String;
pub type owner = Definition;
pub type domain_of = Vec<ClassDefinition>;
pub type is_usage_slot = bool;
pub type usage_slot_name = String;
pub type subproperty_of = SlotDefinition;
pub type disjoint_with = Vec<Definition>;
pub type children_are_mutually_disjoint = bool;
pub type relational_logical_characteristic = bool;
pub type symmetric = bool;
pub type asymmetric = bool;
pub type reflexive = bool;
pub type irreflexive = bool;
pub type locally_reflexive = bool;
pub type transitive = bool;
pub type transitive_form_of = SlotDefinition;
pub type reflexive_transitive_form_of = SlotDefinition;
pub type inverse = SlotDefinition;
pub type is_class_field = bool;
pub type role = String;
pub type minimum_value = Anything;
pub type maximum_value = Anything;
pub type interpolated = bool;
pub type partial_match = bool;
pub type pattern = String;
pub type syntax = String;
pub type structured_pattern = PatternExpression;
pub type string_serialization = String;
pub type bindings = Vec<EnumBinding>;
pub type binds_value_of = String;
pub type obligation_level = String;
pub type type_mappings = Vec<TypeMapping>;
pub type framework_key = String;
pub type mapped_type = TypeDefinition;
pub type typeof_ = TypeDefinition;
pub type base = String;
pub type type_uri = uriorcurie;
pub type repr = String;
pub type alt_description_text = String;
pub type alt_description_source = String;
pub type alt_descriptions = Vec<AltDescription>;
pub type value = String;
pub type value_description = String;
pub type value_object = Anything;
pub type examples = Vec<Example>;
pub type prefix_prefix = ncname;
pub type prefix_reference = uri;
pub type prefixes = Vec<Prefix>;
pub type setting_key = ncname;
pub type setting_value = String;
pub type settings = Vec<Setting>;
pub type import_from = uriorcurie;
pub type import_as = ncname;
pub type import_map = Vec<Setting>;
pub type local_name_source = ncname;
pub type local_name_value = String;
pub type local_names = Vec<LocalName>;
pub type slot_group = SlotDefinition;
pub type is_grouping_slot = bool;
pub type followed_by = Expression;
pub type reversed = bool;
pub type traverse = SlotDefinition;
pub type path_rule = PathExpression;
pub type represents_relationship = bool;
pub type relational_role = String;

// Enums

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PvFormulaOptions {
    CODE,
    CURIE,
    URI,
    FHIRCODING,
    LABEL,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PresenceEnum {
    UNCOMMITTED,
    PRESENT,
    ABSENT,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RelationalRoleEnum {
    SUBJECT,
    OBJECT,
    PREDICATE,
    NODE,
    OTHERROLE,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AliasPredicateEnum {
    EXACTSYNONYM,
    RELATEDSYNONYM,
    BROADSYNONYM,
    NARROWSYNONYM,
}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ObligationLevelEnum {
    REQUIRED,
    RECOMMENDED,
    OPTIONAL,
    EXAMPLE,
    DISCOURAGED,
}

// Classes

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AnyValue {
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Extension {
    pub extension_tag: uriorcurie,
    pub extension_value: AnyValue,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, Box<ExtensionOrSubtype>>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Extension {
    #[new]
    pub fn new(extension_tag: uriorcurie, extension_value: AnyValue, extensions: HashMap<String, Box<ExtensionOrSubtype>>) -> Self {
        Extension{extension_tag, extension_value, extensions}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Extension>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Extension> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Extension>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Extension",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Extension {
    type Key   = uriorcurie;
        
    type Value = AnyValue;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.extension_tag;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("extension_tag".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("extension_tag".into()), Value::String(k));
        map.insert(Value::String("extension_value".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum ExtensionOrSubtype {    Extension(Extension),     Annotation(Annotation)}

impl From<Extension>   for ExtensionOrSubtype { fn from(x: Extension)   -> Self { Self::Extension(x) } }
impl From<Annotation>   for ExtensionOrSubtype { fn from(x: Annotation)   -> Self { Self::Annotation(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ExtensionOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Extension>() {
            return Ok(ExtensionOrSubtype::Extension(val));
        }        if let Ok(val) = ob.extract::<Annotation>() {
            return Ok(ExtensionOrSubtype::Annotation(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ExtensionOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ExtensionOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            ExtensionOrSubtype::Extension(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensionOrSubtype::Annotation(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ExtensionOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ExtensionOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ExtensionOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ExtensionOrSubtype",
        ))
    }
}

#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for ExtensionOrSubtype {
    type Key       = String;
    type Value     = serde_value::Value;
    type Error     = String;

    fn from_pair_mapping(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Extension::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ExtensionOrSubtype::Extension(x));
        }
        if let Ok(x) = Annotation::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ExtensionOrSubtype::Annotation(x));
        }
        Err("none of the variants matched the mapping form".into())
    }

    fn from_pair_simple(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Extension::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ExtensionOrSubtype::Extension(x));
        }
        if let Ok(x) = Annotation::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ExtensionOrSubtype::Annotation(x));
        }
        Err("none of the variants support the primitive form".into())
    }

    fn extract_key(&self) -> &Self::Key {
        match self {
            ExtensionOrSubtype::Extension(inner) => inner.extract_key(),
            ExtensionOrSubtype::Annotation(inner) => inner.extract_key(),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Extensible {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Extensible {
    #[new]
    pub fn new(extensions: HashMap<String, ExtensionOrSubtype>) -> Self {
        Extensible{extensions}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Extensible>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Extensible> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Extensible>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Extensible",
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum ExtensibleOrSubtype {    Extensible(Extensible),     Element(Element),     EnumBinding(EnumBinding),     StructuredAlias(StructuredAlias),     AnonymousExpression(AnonymousExpression),     PathExpression(PathExpression),     ClassRule(ClassRule),     ArrayExpression(ArrayExpression),     DimensionExpression(DimensionExpression),     PatternExpression(PatternExpression),     ImportExpression(ImportExpression),     PermissibleValue(PermissibleValue),     UniqueKey(UniqueKey),     TypeMapping(TypeMapping),     AnonymousSlotExpression(AnonymousSlotExpression),     AnonymousClassExpression(AnonymousClassExpression),     SchemaDefinition(SchemaDefinition),     TypeDefinition(TypeDefinition),     SubsetDefinition(SubsetDefinition),     Definition(Definition),     EnumDefinition(EnumDefinition),     SlotDefinition(SlotDefinition),     ClassDefinition(ClassDefinition)}

impl From<Extensible>   for ExtensibleOrSubtype { fn from(x: Extensible)   -> Self { Self::Extensible(x) } }
impl From<Element>   for ExtensibleOrSubtype { fn from(x: Element)   -> Self { Self::Element(x) } }
impl From<EnumBinding>   for ExtensibleOrSubtype { fn from(x: EnumBinding)   -> Self { Self::EnumBinding(x) } }
impl From<StructuredAlias>   for ExtensibleOrSubtype { fn from(x: StructuredAlias)   -> Self { Self::StructuredAlias(x) } }
impl From<AnonymousExpression>   for ExtensibleOrSubtype { fn from(x: AnonymousExpression)   -> Self { Self::AnonymousExpression(x) } }
impl From<PathExpression>   for ExtensibleOrSubtype { fn from(x: PathExpression)   -> Self { Self::PathExpression(x) } }
impl From<ClassRule>   for ExtensibleOrSubtype { fn from(x: ClassRule)   -> Self { Self::ClassRule(x) } }
impl From<ArrayExpression>   for ExtensibleOrSubtype { fn from(x: ArrayExpression)   -> Self { Self::ArrayExpression(x) } }
impl From<DimensionExpression>   for ExtensibleOrSubtype { fn from(x: DimensionExpression)   -> Self { Self::DimensionExpression(x) } }
impl From<PatternExpression>   for ExtensibleOrSubtype { fn from(x: PatternExpression)   -> Self { Self::PatternExpression(x) } }
impl From<ImportExpression>   for ExtensibleOrSubtype { fn from(x: ImportExpression)   -> Self { Self::ImportExpression(x) } }
impl From<PermissibleValue>   for ExtensibleOrSubtype { fn from(x: PermissibleValue)   -> Self { Self::PermissibleValue(x) } }
impl From<UniqueKey>   for ExtensibleOrSubtype { fn from(x: UniqueKey)   -> Self { Self::UniqueKey(x) } }
impl From<TypeMapping>   for ExtensibleOrSubtype { fn from(x: TypeMapping)   -> Self { Self::TypeMapping(x) } }
impl From<AnonymousSlotExpression>   for ExtensibleOrSubtype { fn from(x: AnonymousSlotExpression)   -> Self { Self::AnonymousSlotExpression(x) } }
impl From<AnonymousClassExpression>   for ExtensibleOrSubtype { fn from(x: AnonymousClassExpression)   -> Self { Self::AnonymousClassExpression(x) } }
impl From<SchemaDefinition>   for ExtensibleOrSubtype { fn from(x: SchemaDefinition)   -> Self { Self::SchemaDefinition(x) } }
impl From<TypeDefinition>   for ExtensibleOrSubtype { fn from(x: TypeDefinition)   -> Self { Self::TypeDefinition(x) } }
impl From<SubsetDefinition>   for ExtensibleOrSubtype { fn from(x: SubsetDefinition)   -> Self { Self::SubsetDefinition(x) } }
impl From<Definition>   for ExtensibleOrSubtype { fn from(x: Definition)   -> Self { Self::Definition(x) } }
impl From<EnumDefinition>   for ExtensibleOrSubtype { fn from(x: EnumDefinition)   -> Self { Self::EnumDefinition(x) } }
impl From<SlotDefinition>   for ExtensibleOrSubtype { fn from(x: SlotDefinition)   -> Self { Self::SlotDefinition(x) } }
impl From<ClassDefinition>   for ExtensibleOrSubtype { fn from(x: ClassDefinition)   -> Self { Self::ClassDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ExtensibleOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Extensible>() {
            return Ok(ExtensibleOrSubtype::Extensible(val));
        }        if let Ok(val) = ob.extract::<Element>() {
            return Ok(ExtensibleOrSubtype::Element(val));
        }        if let Ok(val) = ob.extract::<EnumBinding>() {
            return Ok(ExtensibleOrSubtype::EnumBinding(val));
        }        if let Ok(val) = ob.extract::<StructuredAlias>() {
            return Ok(ExtensibleOrSubtype::StructuredAlias(val));
        }        if let Ok(val) = ob.extract::<AnonymousExpression>() {
            return Ok(ExtensibleOrSubtype::AnonymousExpression(val));
        }        if let Ok(val) = ob.extract::<PathExpression>() {
            return Ok(ExtensibleOrSubtype::PathExpression(val));
        }        if let Ok(val) = ob.extract::<ClassRule>() {
            return Ok(ExtensibleOrSubtype::ClassRule(val));
        }        if let Ok(val) = ob.extract::<ArrayExpression>() {
            return Ok(ExtensibleOrSubtype::ArrayExpression(val));
        }        if let Ok(val) = ob.extract::<DimensionExpression>() {
            return Ok(ExtensibleOrSubtype::DimensionExpression(val));
        }        if let Ok(val) = ob.extract::<PatternExpression>() {
            return Ok(ExtensibleOrSubtype::PatternExpression(val));
        }        if let Ok(val) = ob.extract::<ImportExpression>() {
            return Ok(ExtensibleOrSubtype::ImportExpression(val));
        }        if let Ok(val) = ob.extract::<PermissibleValue>() {
            return Ok(ExtensibleOrSubtype::PermissibleValue(val));
        }        if let Ok(val) = ob.extract::<UniqueKey>() {
            return Ok(ExtensibleOrSubtype::UniqueKey(val));
        }        if let Ok(val) = ob.extract::<TypeMapping>() {
            return Ok(ExtensibleOrSubtype::TypeMapping(val));
        }        if let Ok(val) = ob.extract::<AnonymousSlotExpression>() {
            return Ok(ExtensibleOrSubtype::AnonymousSlotExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousClassExpression>() {
            return Ok(ExtensibleOrSubtype::AnonymousClassExpression(val));
        }        if let Ok(val) = ob.extract::<SchemaDefinition>() {
            return Ok(ExtensibleOrSubtype::SchemaDefinition(val));
        }        if let Ok(val) = ob.extract::<TypeDefinition>() {
            return Ok(ExtensibleOrSubtype::TypeDefinition(val));
        }        if let Ok(val) = ob.extract::<SubsetDefinition>() {
            return Ok(ExtensibleOrSubtype::SubsetDefinition(val));
        }        if let Ok(val) = ob.extract::<Definition>() {
            return Ok(ExtensibleOrSubtype::Definition(val));
        }        if let Ok(val) = ob.extract::<EnumDefinition>() {
            return Ok(ExtensibleOrSubtype::EnumDefinition(val));
        }        if let Ok(val) = ob.extract::<SlotDefinition>() {
            return Ok(ExtensibleOrSubtype::SlotDefinition(val));
        }        if let Ok(val) = ob.extract::<ClassDefinition>() {
            return Ok(ExtensibleOrSubtype::ClassDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ExtensibleOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ExtensibleOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            ExtensibleOrSubtype::Extensible(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::Element(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::EnumBinding(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::StructuredAlias(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::AnonymousExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::PathExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::ClassRule(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::ArrayExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::DimensionExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::PatternExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::ImportExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::PermissibleValue(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::UniqueKey(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::TypeMapping(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::AnonymousSlotExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::AnonymousClassExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::SchemaDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::TypeDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::SubsetDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::Definition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::EnumDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::SlotDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExtensibleOrSubtype::ClassDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ExtensibleOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ExtensibleOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ExtensibleOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ExtensibleOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Annotatable {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Annotatable {
    #[new]
    pub fn new(annotations: HashMap<String, Annotation>) -> Self {
        Annotatable{annotations}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Annotatable>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Annotatable> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Annotatable>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Annotatable",
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum AnnotatableOrSubtype {    Annotatable(Annotatable),     Annotation(Annotation),     Element(Element),     EnumBinding(EnumBinding),     StructuredAlias(StructuredAlias),     AnonymousExpression(AnonymousExpression),     PathExpression(PathExpression),     ClassRule(ClassRule),     ArrayExpression(ArrayExpression),     DimensionExpression(DimensionExpression),     PatternExpression(PatternExpression),     ImportExpression(ImportExpression),     PermissibleValue(PermissibleValue),     UniqueKey(UniqueKey),     TypeMapping(TypeMapping),     AnonymousSlotExpression(AnonymousSlotExpression),     AnonymousClassExpression(AnonymousClassExpression),     SchemaDefinition(SchemaDefinition),     TypeDefinition(TypeDefinition),     SubsetDefinition(SubsetDefinition),     Definition(Definition),     EnumDefinition(EnumDefinition),     SlotDefinition(SlotDefinition),     ClassDefinition(ClassDefinition)}

impl From<Annotatable>   for AnnotatableOrSubtype { fn from(x: Annotatable)   -> Self { Self::Annotatable(x) } }
impl From<Annotation>   for AnnotatableOrSubtype { fn from(x: Annotation)   -> Self { Self::Annotation(x) } }
impl From<Element>   for AnnotatableOrSubtype { fn from(x: Element)   -> Self { Self::Element(x) } }
impl From<EnumBinding>   for AnnotatableOrSubtype { fn from(x: EnumBinding)   -> Self { Self::EnumBinding(x) } }
impl From<StructuredAlias>   for AnnotatableOrSubtype { fn from(x: StructuredAlias)   -> Self { Self::StructuredAlias(x) } }
impl From<AnonymousExpression>   for AnnotatableOrSubtype { fn from(x: AnonymousExpression)   -> Self { Self::AnonymousExpression(x) } }
impl From<PathExpression>   for AnnotatableOrSubtype { fn from(x: PathExpression)   -> Self { Self::PathExpression(x) } }
impl From<ClassRule>   for AnnotatableOrSubtype { fn from(x: ClassRule)   -> Self { Self::ClassRule(x) } }
impl From<ArrayExpression>   for AnnotatableOrSubtype { fn from(x: ArrayExpression)   -> Self { Self::ArrayExpression(x) } }
impl From<DimensionExpression>   for AnnotatableOrSubtype { fn from(x: DimensionExpression)   -> Self { Self::DimensionExpression(x) } }
impl From<PatternExpression>   for AnnotatableOrSubtype { fn from(x: PatternExpression)   -> Self { Self::PatternExpression(x) } }
impl From<ImportExpression>   for AnnotatableOrSubtype { fn from(x: ImportExpression)   -> Self { Self::ImportExpression(x) } }
impl From<PermissibleValue>   for AnnotatableOrSubtype { fn from(x: PermissibleValue)   -> Self { Self::PermissibleValue(x) } }
impl From<UniqueKey>   for AnnotatableOrSubtype { fn from(x: UniqueKey)   -> Self { Self::UniqueKey(x) } }
impl From<TypeMapping>   for AnnotatableOrSubtype { fn from(x: TypeMapping)   -> Self { Self::TypeMapping(x) } }
impl From<AnonymousSlotExpression>   for AnnotatableOrSubtype { fn from(x: AnonymousSlotExpression)   -> Self { Self::AnonymousSlotExpression(x) } }
impl From<AnonymousClassExpression>   for AnnotatableOrSubtype { fn from(x: AnonymousClassExpression)   -> Self { Self::AnonymousClassExpression(x) } }
impl From<SchemaDefinition>   for AnnotatableOrSubtype { fn from(x: SchemaDefinition)   -> Self { Self::SchemaDefinition(x) } }
impl From<TypeDefinition>   for AnnotatableOrSubtype { fn from(x: TypeDefinition)   -> Self { Self::TypeDefinition(x) } }
impl From<SubsetDefinition>   for AnnotatableOrSubtype { fn from(x: SubsetDefinition)   -> Self { Self::SubsetDefinition(x) } }
impl From<Definition>   for AnnotatableOrSubtype { fn from(x: Definition)   -> Self { Self::Definition(x) } }
impl From<EnumDefinition>   for AnnotatableOrSubtype { fn from(x: EnumDefinition)   -> Self { Self::EnumDefinition(x) } }
impl From<SlotDefinition>   for AnnotatableOrSubtype { fn from(x: SlotDefinition)   -> Self { Self::SlotDefinition(x) } }
impl From<ClassDefinition>   for AnnotatableOrSubtype { fn from(x: ClassDefinition)   -> Self { Self::ClassDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for AnnotatableOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Annotatable>() {
            return Ok(AnnotatableOrSubtype::Annotatable(val));
        }        if let Ok(val) = ob.extract::<Annotation>() {
            return Ok(AnnotatableOrSubtype::Annotation(val));
        }        if let Ok(val) = ob.extract::<Element>() {
            return Ok(AnnotatableOrSubtype::Element(val));
        }        if let Ok(val) = ob.extract::<EnumBinding>() {
            return Ok(AnnotatableOrSubtype::EnumBinding(val));
        }        if let Ok(val) = ob.extract::<StructuredAlias>() {
            return Ok(AnnotatableOrSubtype::StructuredAlias(val));
        }        if let Ok(val) = ob.extract::<AnonymousExpression>() {
            return Ok(AnnotatableOrSubtype::AnonymousExpression(val));
        }        if let Ok(val) = ob.extract::<PathExpression>() {
            return Ok(AnnotatableOrSubtype::PathExpression(val));
        }        if let Ok(val) = ob.extract::<ClassRule>() {
            return Ok(AnnotatableOrSubtype::ClassRule(val));
        }        if let Ok(val) = ob.extract::<ArrayExpression>() {
            return Ok(AnnotatableOrSubtype::ArrayExpression(val));
        }        if let Ok(val) = ob.extract::<DimensionExpression>() {
            return Ok(AnnotatableOrSubtype::DimensionExpression(val));
        }        if let Ok(val) = ob.extract::<PatternExpression>() {
            return Ok(AnnotatableOrSubtype::PatternExpression(val));
        }        if let Ok(val) = ob.extract::<ImportExpression>() {
            return Ok(AnnotatableOrSubtype::ImportExpression(val));
        }        if let Ok(val) = ob.extract::<PermissibleValue>() {
            return Ok(AnnotatableOrSubtype::PermissibleValue(val));
        }        if let Ok(val) = ob.extract::<UniqueKey>() {
            return Ok(AnnotatableOrSubtype::UniqueKey(val));
        }        if let Ok(val) = ob.extract::<TypeMapping>() {
            return Ok(AnnotatableOrSubtype::TypeMapping(val));
        }        if let Ok(val) = ob.extract::<AnonymousSlotExpression>() {
            return Ok(AnnotatableOrSubtype::AnonymousSlotExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousClassExpression>() {
            return Ok(AnnotatableOrSubtype::AnonymousClassExpression(val));
        }        if let Ok(val) = ob.extract::<SchemaDefinition>() {
            return Ok(AnnotatableOrSubtype::SchemaDefinition(val));
        }        if let Ok(val) = ob.extract::<TypeDefinition>() {
            return Ok(AnnotatableOrSubtype::TypeDefinition(val));
        }        if let Ok(val) = ob.extract::<SubsetDefinition>() {
            return Ok(AnnotatableOrSubtype::SubsetDefinition(val));
        }        if let Ok(val) = ob.extract::<Definition>() {
            return Ok(AnnotatableOrSubtype::Definition(val));
        }        if let Ok(val) = ob.extract::<EnumDefinition>() {
            return Ok(AnnotatableOrSubtype::EnumDefinition(val));
        }        if let Ok(val) = ob.extract::<SlotDefinition>() {
            return Ok(AnnotatableOrSubtype::SlotDefinition(val));
        }        if let Ok(val) = ob.extract::<ClassDefinition>() {
            return Ok(AnnotatableOrSubtype::ClassDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnnotatableOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for AnnotatableOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            AnnotatableOrSubtype::Annotatable(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::Annotation(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::Element(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::EnumBinding(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::StructuredAlias(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::AnonymousExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::PathExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::ClassRule(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::ArrayExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::DimensionExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::PatternExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::ImportExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::PermissibleValue(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::UniqueKey(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::TypeMapping(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::AnonymousSlotExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::AnonymousClassExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::SchemaDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::TypeDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::SubsetDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::Definition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::EnumDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::SlotDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnnotatableOrSubtype::ClassDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AnnotatableOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AnnotatableOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AnnotatableOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnnotatableOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Annotation {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Box<Annotation>>,
    pub extension_tag: uriorcurie,
    pub extension_value: AnyValue,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Annotation {
    #[new]
    pub fn new(annotations: HashMap<String, Box<Annotation>>, extension_tag: uriorcurie, extension_value: AnyValue, extensions: HashMap<String, ExtensionOrSubtype>) -> Self {
        Annotation{annotations, extension_tag, extension_value, extensions}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Annotation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Annotation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Annotation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Annotation",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Annotation {
    type Key   = uriorcurie;
        
    type Value = Annotation;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.extension_tag;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("extension_tag".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("extension_tag".into()), Value::String(k));
        map.insert(Value::String("annotations".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct UnitOfMeasure {
    #[cfg_attr(feature = "serde", serde(default))]
    pub symbol: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub abbreviation: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub descriptive_name: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ucum_code: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub derivation: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub has_quantity_kind: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub iec61360code: Option<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl UnitOfMeasure {
    #[new]
    pub fn new(symbol: Option<String>, abbreviation: Option<String>, descriptive_name: Option<String>, exact_mappings: Vec<uriorcurie>, ucum_code: Option<String>, derivation: Option<String>, has_quantity_kind: Option<uriorcurie>, iec61360code: Option<String>) -> Self {
        UnitOfMeasure{symbol, abbreviation, descriptive_name, exact_mappings, ucum_code, derivation, has_quantity_kind, iec61360code}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<UnitOfMeasure>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<UnitOfMeasure> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<UnitOfMeasure>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid UnitOfMeasure",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Anything {
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct CommonMetadata {
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl CommonMetadata {
    #[new]
    pub fn new(description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        CommonMetadata{description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<CommonMetadata>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<CommonMetadata> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<CommonMetadata>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid CommonMetadata",
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum CommonMetadataOrSubtype {    CommonMetadata(CommonMetadata),     Element(Element),     EnumBinding(EnumBinding),     StructuredAlias(StructuredAlias),     AnonymousExpression(AnonymousExpression),     PathExpression(PathExpression),     ClassRule(ClassRule),     ArrayExpression(ArrayExpression),     DimensionExpression(DimensionExpression),     PatternExpression(PatternExpression),     ImportExpression(ImportExpression),     PermissibleValue(PermissibleValue),     UniqueKey(UniqueKey),     TypeMapping(TypeMapping),     AnonymousSlotExpression(AnonymousSlotExpression),     AnonymousClassExpression(AnonymousClassExpression),     SchemaDefinition(SchemaDefinition),     TypeDefinition(TypeDefinition),     SubsetDefinition(SubsetDefinition),     Definition(Definition),     EnumDefinition(EnumDefinition),     SlotDefinition(SlotDefinition),     ClassDefinition(ClassDefinition)}

impl From<CommonMetadata>   for CommonMetadataOrSubtype { fn from(x: CommonMetadata)   -> Self { Self::CommonMetadata(x) } }
impl From<Element>   for CommonMetadataOrSubtype { fn from(x: Element)   -> Self { Self::Element(x) } }
impl From<EnumBinding>   for CommonMetadataOrSubtype { fn from(x: EnumBinding)   -> Self { Self::EnumBinding(x) } }
impl From<StructuredAlias>   for CommonMetadataOrSubtype { fn from(x: StructuredAlias)   -> Self { Self::StructuredAlias(x) } }
impl From<AnonymousExpression>   for CommonMetadataOrSubtype { fn from(x: AnonymousExpression)   -> Self { Self::AnonymousExpression(x) } }
impl From<PathExpression>   for CommonMetadataOrSubtype { fn from(x: PathExpression)   -> Self { Self::PathExpression(x) } }
impl From<ClassRule>   for CommonMetadataOrSubtype { fn from(x: ClassRule)   -> Self { Self::ClassRule(x) } }
impl From<ArrayExpression>   for CommonMetadataOrSubtype { fn from(x: ArrayExpression)   -> Self { Self::ArrayExpression(x) } }
impl From<DimensionExpression>   for CommonMetadataOrSubtype { fn from(x: DimensionExpression)   -> Self { Self::DimensionExpression(x) } }
impl From<PatternExpression>   for CommonMetadataOrSubtype { fn from(x: PatternExpression)   -> Self { Self::PatternExpression(x) } }
impl From<ImportExpression>   for CommonMetadataOrSubtype { fn from(x: ImportExpression)   -> Self { Self::ImportExpression(x) } }
impl From<PermissibleValue>   for CommonMetadataOrSubtype { fn from(x: PermissibleValue)   -> Self { Self::PermissibleValue(x) } }
impl From<UniqueKey>   for CommonMetadataOrSubtype { fn from(x: UniqueKey)   -> Self { Self::UniqueKey(x) } }
impl From<TypeMapping>   for CommonMetadataOrSubtype { fn from(x: TypeMapping)   -> Self { Self::TypeMapping(x) } }
impl From<AnonymousSlotExpression>   for CommonMetadataOrSubtype { fn from(x: AnonymousSlotExpression)   -> Self { Self::AnonymousSlotExpression(x) } }
impl From<AnonymousClassExpression>   for CommonMetadataOrSubtype { fn from(x: AnonymousClassExpression)   -> Self { Self::AnonymousClassExpression(x) } }
impl From<SchemaDefinition>   for CommonMetadataOrSubtype { fn from(x: SchemaDefinition)   -> Self { Self::SchemaDefinition(x) } }
impl From<TypeDefinition>   for CommonMetadataOrSubtype { fn from(x: TypeDefinition)   -> Self { Self::TypeDefinition(x) } }
impl From<SubsetDefinition>   for CommonMetadataOrSubtype { fn from(x: SubsetDefinition)   -> Self { Self::SubsetDefinition(x) } }
impl From<Definition>   for CommonMetadataOrSubtype { fn from(x: Definition)   -> Self { Self::Definition(x) } }
impl From<EnumDefinition>   for CommonMetadataOrSubtype { fn from(x: EnumDefinition)   -> Self { Self::EnumDefinition(x) } }
impl From<SlotDefinition>   for CommonMetadataOrSubtype { fn from(x: SlotDefinition)   -> Self { Self::SlotDefinition(x) } }
impl From<ClassDefinition>   for CommonMetadataOrSubtype { fn from(x: ClassDefinition)   -> Self { Self::ClassDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for CommonMetadataOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<CommonMetadata>() {
            return Ok(CommonMetadataOrSubtype::CommonMetadata(val));
        }        if let Ok(val) = ob.extract::<Element>() {
            return Ok(CommonMetadataOrSubtype::Element(val));
        }        if let Ok(val) = ob.extract::<EnumBinding>() {
            return Ok(CommonMetadataOrSubtype::EnumBinding(val));
        }        if let Ok(val) = ob.extract::<StructuredAlias>() {
            return Ok(CommonMetadataOrSubtype::StructuredAlias(val));
        }        if let Ok(val) = ob.extract::<AnonymousExpression>() {
            return Ok(CommonMetadataOrSubtype::AnonymousExpression(val));
        }        if let Ok(val) = ob.extract::<PathExpression>() {
            return Ok(CommonMetadataOrSubtype::PathExpression(val));
        }        if let Ok(val) = ob.extract::<ClassRule>() {
            return Ok(CommonMetadataOrSubtype::ClassRule(val));
        }        if let Ok(val) = ob.extract::<ArrayExpression>() {
            return Ok(CommonMetadataOrSubtype::ArrayExpression(val));
        }        if let Ok(val) = ob.extract::<DimensionExpression>() {
            return Ok(CommonMetadataOrSubtype::DimensionExpression(val));
        }        if let Ok(val) = ob.extract::<PatternExpression>() {
            return Ok(CommonMetadataOrSubtype::PatternExpression(val));
        }        if let Ok(val) = ob.extract::<ImportExpression>() {
            return Ok(CommonMetadataOrSubtype::ImportExpression(val));
        }        if let Ok(val) = ob.extract::<PermissibleValue>() {
            return Ok(CommonMetadataOrSubtype::PermissibleValue(val));
        }        if let Ok(val) = ob.extract::<UniqueKey>() {
            return Ok(CommonMetadataOrSubtype::UniqueKey(val));
        }        if let Ok(val) = ob.extract::<TypeMapping>() {
            return Ok(CommonMetadataOrSubtype::TypeMapping(val));
        }        if let Ok(val) = ob.extract::<AnonymousSlotExpression>() {
            return Ok(CommonMetadataOrSubtype::AnonymousSlotExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousClassExpression>() {
            return Ok(CommonMetadataOrSubtype::AnonymousClassExpression(val));
        }        if let Ok(val) = ob.extract::<SchemaDefinition>() {
            return Ok(CommonMetadataOrSubtype::SchemaDefinition(val));
        }        if let Ok(val) = ob.extract::<TypeDefinition>() {
            return Ok(CommonMetadataOrSubtype::TypeDefinition(val));
        }        if let Ok(val) = ob.extract::<SubsetDefinition>() {
            return Ok(CommonMetadataOrSubtype::SubsetDefinition(val));
        }        if let Ok(val) = ob.extract::<Definition>() {
            return Ok(CommonMetadataOrSubtype::Definition(val));
        }        if let Ok(val) = ob.extract::<EnumDefinition>() {
            return Ok(CommonMetadataOrSubtype::EnumDefinition(val));
        }        if let Ok(val) = ob.extract::<SlotDefinition>() {
            return Ok(CommonMetadataOrSubtype::SlotDefinition(val));
        }        if let Ok(val) = ob.extract::<ClassDefinition>() {
            return Ok(CommonMetadataOrSubtype::ClassDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid CommonMetadataOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for CommonMetadataOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            CommonMetadataOrSubtype::CommonMetadata(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::Element(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::EnumBinding(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::StructuredAlias(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::AnonymousExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::PathExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::ClassRule(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::ArrayExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::DimensionExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::PatternExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::ImportExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::PermissibleValue(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::UniqueKey(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::TypeMapping(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::AnonymousSlotExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::AnonymousClassExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::SchemaDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::TypeDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::SubsetDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::Definition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::EnumDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::SlotDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CommonMetadataOrSubtype::ClassDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<CommonMetadataOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<CommonMetadataOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<CommonMetadataOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid CommonMetadataOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Element {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes_are_closed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub definition_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub local_names: HashMap<String, LocalName>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conforms_to: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Element {
    #[new]
    pub fn new(name: String, id_prefixes: Vec<ncname>, id_prefixes_are_closed: Option<bool>, definition_uri: Option<uriorcurie>, local_names: HashMap<String, LocalName>, conforms_to: Option<String>, implements: Vec<uriorcurie>, instantiates: Vec<uriorcurie>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        Element{name, id_prefixes, id_prefixes_are_closed, definition_uri, local_names, conforms_to, implements, instantiates, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Element>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Element> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Element>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Element",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Element {
    type Key   = String;
        
    type Value = ncname;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("name".into()), Value::String(k));
        map.insert(Value::String("id_prefixes".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum ElementOrSubtype {    Element(Element),     SchemaDefinition(SchemaDefinition),     TypeDefinition(TypeDefinition),     SubsetDefinition(SubsetDefinition),     Definition(Definition),     EnumDefinition(EnumDefinition),     SlotDefinition(SlotDefinition),     ClassDefinition(ClassDefinition)}

impl From<Element>   for ElementOrSubtype { fn from(x: Element)   -> Self { Self::Element(x) } }
impl From<SchemaDefinition>   for ElementOrSubtype { fn from(x: SchemaDefinition)   -> Self { Self::SchemaDefinition(x) } }
impl From<TypeDefinition>   for ElementOrSubtype { fn from(x: TypeDefinition)   -> Self { Self::TypeDefinition(x) } }
impl From<SubsetDefinition>   for ElementOrSubtype { fn from(x: SubsetDefinition)   -> Self { Self::SubsetDefinition(x) } }
impl From<Definition>   for ElementOrSubtype { fn from(x: Definition)   -> Self { Self::Definition(x) } }
impl From<EnumDefinition>   for ElementOrSubtype { fn from(x: EnumDefinition)   -> Self { Self::EnumDefinition(x) } }
impl From<SlotDefinition>   for ElementOrSubtype { fn from(x: SlotDefinition)   -> Self { Self::SlotDefinition(x) } }
impl From<ClassDefinition>   for ElementOrSubtype { fn from(x: ClassDefinition)   -> Self { Self::ClassDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ElementOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Element>() {
            return Ok(ElementOrSubtype::Element(val));
        }        if let Ok(val) = ob.extract::<SchemaDefinition>() {
            return Ok(ElementOrSubtype::SchemaDefinition(val));
        }        if let Ok(val) = ob.extract::<TypeDefinition>() {
            return Ok(ElementOrSubtype::TypeDefinition(val));
        }        if let Ok(val) = ob.extract::<SubsetDefinition>() {
            return Ok(ElementOrSubtype::SubsetDefinition(val));
        }        if let Ok(val) = ob.extract::<Definition>() {
            return Ok(ElementOrSubtype::Definition(val));
        }        if let Ok(val) = ob.extract::<EnumDefinition>() {
            return Ok(ElementOrSubtype::EnumDefinition(val));
        }        if let Ok(val) = ob.extract::<SlotDefinition>() {
            return Ok(ElementOrSubtype::SlotDefinition(val));
        }        if let Ok(val) = ob.extract::<ClassDefinition>() {
            return Ok(ElementOrSubtype::ClassDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ElementOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ElementOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            ElementOrSubtype::Element(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ElementOrSubtype::SchemaDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ElementOrSubtype::TypeDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ElementOrSubtype::SubsetDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ElementOrSubtype::Definition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ElementOrSubtype::EnumDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ElementOrSubtype::SlotDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ElementOrSubtype::ClassDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ElementOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ElementOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ElementOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ElementOrSubtype",
        ))
    }
}

#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for ElementOrSubtype {
    type Key       = String;
    type Value     = serde_value::Value;
    type Error     = String;

    fn from_pair_mapping(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Element::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::Element(x));
        }
        if let Ok(x) = SchemaDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::SchemaDefinition(x));
        }
        if let Ok(x) = TypeDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::TypeDefinition(x));
        }
        if let Ok(x) = SubsetDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::SubsetDefinition(x));
        }
        if let Ok(x) = Definition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::Definition(x));
        }
        if let Ok(x) = EnumDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::EnumDefinition(x));
        }
        if let Ok(x) = SlotDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::SlotDefinition(x));
        }
        if let Ok(x) = ClassDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::ClassDefinition(x));
        }
        Err("none of the variants matched the mapping form".into())
    }

    fn from_pair_simple(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Element::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::Element(x));
        }
        if let Ok(x) = SchemaDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::SchemaDefinition(x));
        }
        if let Ok(x) = TypeDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::TypeDefinition(x));
        }
        if let Ok(x) = SubsetDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::SubsetDefinition(x));
        }
        if let Ok(x) = Definition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::Definition(x));
        }
        if let Ok(x) = EnumDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::EnumDefinition(x));
        }
        if let Ok(x) = SlotDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::SlotDefinition(x));
        }
        if let Ok(x) = ClassDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(ElementOrSubtype::ClassDefinition(x));
        }
        Err("none of the variants support the primitive form".into())
    }

    fn extract_key(&self) -> &Self::Key {
        match self {
            ElementOrSubtype::Element(inner) => inner.extract_key(),
            ElementOrSubtype::SchemaDefinition(inner) => inner.extract_key(),
            ElementOrSubtype::TypeDefinition(inner) => inner.extract_key(),
            ElementOrSubtype::SubsetDefinition(inner) => inner.extract_key(),
            ElementOrSubtype::Definition(inner) => inner.extract_key(),
            ElementOrSubtype::EnumDefinition(inner) => inner.extract_key(),
            ElementOrSubtype::SlotDefinition(inner) => inner.extract_key(),
            ElementOrSubtype::ClassDefinition(inner) => inner.extract_key(),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SchemaDefinition {
    pub id: uri,
    #[cfg_attr(feature = "serde", serde(default))]
    pub version: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub imports: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub license: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub prefixes: HashMap<String, Prefix>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub emit_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub default_curi_maps: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub default_prefix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub default_range: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub subsets: HashMap<String, SubsetDefinition>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub types: HashMap<String, TypeDefinition>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub enums: HashMap<String, EnumDefinition>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_definitions: HashMap<String, SlotDefinition>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub classes: HashMap<String, ClassDefinition>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub metamodel_version: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source_file: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source_file_date: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source_file_size: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub generation_date: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_names_unique: Option<bool>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub settings: HashMap<String, Setting>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bindings: Vec<EnumBinding>,
    pub name: ncname,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes_are_closed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub definition_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub local_names: HashMap<String, LocalName>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conforms_to: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl SchemaDefinition {
    #[new]
    pub fn new(id: uri, version: Option<String>, imports: Vec<uriorcurie>, license: Option<String>, prefixes: HashMap<String, Prefix>, emit_prefixes: Vec<ncname>, default_curi_maps: Vec<String>, default_prefix: Option<String>, default_range: Option<String>, subsets: HashMap<String, SubsetDefinition>, types: HashMap<String, TypeDefinition>, enums: HashMap<String, EnumDefinition>, slot_definitions: HashMap<String, SlotDefinition>, classes: HashMap<String, ClassDefinition>, metamodel_version: Option<String>, source_file: Option<String>, source_file_date: Option<NaiveDateTime>, source_file_size: Option<isize>, generation_date: Option<NaiveDateTime>, slot_names_unique: Option<bool>, settings: HashMap<String, Setting>, bindings: Vec<EnumBinding>, name: ncname, id_prefixes: Vec<ncname>, id_prefixes_are_closed: Option<bool>, definition_uri: Option<uriorcurie>, local_names: HashMap<String, LocalName>, conforms_to: Option<String>, implements: Vec<uriorcurie>, instantiates: Vec<uriorcurie>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        SchemaDefinition{id, version, imports, license, prefixes, emit_prefixes, default_curi_maps, default_prefix, default_range, subsets, types, enums, slot_definitions, classes, metamodel_version, source_file, source_file_date, source_file_size, generation_date, slot_names_unique, settings, bindings, name, id_prefixes, id_prefixes_are_closed, definition_uri, local_names, conforms_to, implements, instantiates, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SchemaDefinition>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SchemaDefinition> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SchemaDefinition>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SchemaDefinition",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for SchemaDefinition {
    type Key   = ncname;
        
    type Value = uri;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("name".into()), Value::String(k));
        map.insert(Value::String("id".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AnonymousTypeExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_pattern: Option<PatternExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit: Option<UnitOfMeasure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub implicit_prefix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_string: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub equals_string_in: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<Box<AnonymousTypeExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<Box<AnonymousTypeExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<Box<AnonymousTypeExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<Box<AnonymousTypeExpression>>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl AnonymousTypeExpression {
    #[new]
    pub fn new(pattern: Option<String>, structured_pattern: Option<PatternExpression>, unit: Option<UnitOfMeasure>, implicit_prefix: Option<String>, equals_string: Option<String>, equals_string_in: Vec<String>, equals_number: Option<isize>, minimum_value: Option<Anything>, maximum_value: Option<Anything>, none_of: Vec<Box<AnonymousTypeExpression>>, exactly_one_of: Vec<Box<AnonymousTypeExpression>>, any_of: Vec<Box<AnonymousTypeExpression>>, all_of: Vec<Box<AnonymousTypeExpression>>) -> Self {
        AnonymousTypeExpression{pattern, structured_pattern, unit, implicit_prefix, equals_string, equals_string_in, equals_number, minimum_value, maximum_value, none_of, exactly_one_of, any_of, all_of}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AnonymousTypeExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AnonymousTypeExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AnonymousTypeExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnonymousTypeExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TypeDefinition {
    #[cfg_attr(feature = "serde", serde(default))]
    pub typeof_: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub base: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub type_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub repr: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub union_of: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_pattern: Option<PatternExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit: Option<UnitOfMeasure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub implicit_prefix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_string: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub equals_string_in: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<AnonymousTypeExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<AnonymousTypeExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<AnonymousTypeExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<AnonymousTypeExpression>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes_are_closed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub definition_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub local_names: HashMap<String, LocalName>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conforms_to: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl TypeDefinition {
    #[new]
    pub fn new(typeof_: Option<String>, base: Option<String>, type_uri: Option<uriorcurie>, repr: Option<String>, union_of: Vec<String>, pattern: Option<String>, structured_pattern: Option<PatternExpression>, unit: Option<UnitOfMeasure>, implicit_prefix: Option<String>, equals_string: Option<String>, equals_string_in: Vec<String>, equals_number: Option<isize>, minimum_value: Option<Anything>, maximum_value: Option<Anything>, none_of: Vec<AnonymousTypeExpression>, exactly_one_of: Vec<AnonymousTypeExpression>, any_of: Vec<AnonymousTypeExpression>, all_of: Vec<AnonymousTypeExpression>, name: String, id_prefixes: Vec<ncname>, id_prefixes_are_closed: Option<bool>, definition_uri: Option<uriorcurie>, local_names: HashMap<String, LocalName>, conforms_to: Option<String>, implements: Vec<uriorcurie>, instantiates: Vec<uriorcurie>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        TypeDefinition{typeof_, base, type_uri, repr, union_of, pattern, structured_pattern, unit, implicit_prefix, equals_string, equals_string_in, equals_number, minimum_value, maximum_value, none_of, exactly_one_of, any_of, all_of, name, id_prefixes, id_prefixes_are_closed, definition_uri, local_names, conforms_to, implements, instantiates, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TypeDefinition>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TypeDefinition> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TypeDefinition>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TypeDefinition",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for TypeDefinition {
    type Key   = String;
        
    type Value = TypeDefinition;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("name".into()), Value::String(k));
        map.insert(Value::String("typeof_".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SubsetDefinition {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes_are_closed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub definition_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub local_names: HashMap<String, LocalName>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conforms_to: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<Box<StructuredAlias>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl SubsetDefinition {
    #[new]
    pub fn new(name: String, id_prefixes: Vec<ncname>, id_prefixes_are_closed: Option<bool>, definition_uri: Option<uriorcurie>, local_names: HashMap<String, LocalName>, conforms_to: Option<String>, implements: Vec<uriorcurie>, instantiates: Vec<uriorcurie>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<Box<StructuredAlias>>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        SubsetDefinition{name, id_prefixes, id_prefixes_are_closed, definition_uri, local_names, conforms_to, implements, instantiates, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SubsetDefinition>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SubsetDefinition> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SubsetDefinition>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SubsetDefinition",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for SubsetDefinition {
    type Key   = String;
        
    type Value = ncname;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("name".into()), Value::String(k));
        map.insert(Value::String("id_prefixes".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Definition {
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_a: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub abstract_: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixin: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixins: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub apply_to: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub values_from: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub string_serialization: Option<String>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes_are_closed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub definition_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub local_names: HashMap<String, LocalName>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conforms_to: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Definition {
    #[new]
    pub fn new(is_a: Option<String>, abstract_: Option<bool>, mixin: Option<bool>, mixins: Vec<String>, apply_to: Vec<String>, values_from: Vec<uriorcurie>, string_serialization: Option<String>, name: String, id_prefixes: Vec<ncname>, id_prefixes_are_closed: Option<bool>, definition_uri: Option<uriorcurie>, local_names: HashMap<String, LocalName>, conforms_to: Option<String>, implements: Vec<uriorcurie>, instantiates: Vec<uriorcurie>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        Definition{is_a, abstract_, mixin, mixins, apply_to, values_from, string_serialization, name, id_prefixes, id_prefixes_are_closed, definition_uri, local_names, conforms_to, implements, instantiates, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Definition>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Definition> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Definition>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Definition",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Definition {
    type Key   = String;
        
    type Value = Definition;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("name".into()), Value::String(k));
        map.insert(Value::String("is_a".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum DefinitionOrSubtype {    Definition(Definition),     EnumDefinition(EnumDefinition),     SlotDefinition(SlotDefinition),     ClassDefinition(ClassDefinition)}

impl From<Definition>   for DefinitionOrSubtype { fn from(x: Definition)   -> Self { Self::Definition(x) } }
impl From<EnumDefinition>   for DefinitionOrSubtype { fn from(x: EnumDefinition)   -> Self { Self::EnumDefinition(x) } }
impl From<SlotDefinition>   for DefinitionOrSubtype { fn from(x: SlotDefinition)   -> Self { Self::SlotDefinition(x) } }
impl From<ClassDefinition>   for DefinitionOrSubtype { fn from(x: ClassDefinition)   -> Self { Self::ClassDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for DefinitionOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Definition>() {
            return Ok(DefinitionOrSubtype::Definition(val));
        }        if let Ok(val) = ob.extract::<EnumDefinition>() {
            return Ok(DefinitionOrSubtype::EnumDefinition(val));
        }        if let Ok(val) = ob.extract::<SlotDefinition>() {
            return Ok(DefinitionOrSubtype::SlotDefinition(val));
        }        if let Ok(val) = ob.extract::<ClassDefinition>() {
            return Ok(DefinitionOrSubtype::ClassDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid DefinitionOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for DefinitionOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            DefinitionOrSubtype::Definition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            DefinitionOrSubtype::EnumDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            DefinitionOrSubtype::SlotDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            DefinitionOrSubtype::ClassDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<DefinitionOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<DefinitionOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<DefinitionOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid DefinitionOrSubtype",
        ))
    }
}

#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for DefinitionOrSubtype {
    type Key       = String;
    type Value     = serde_value::Value;
    type Error     = String;

    fn from_pair_mapping(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Definition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(DefinitionOrSubtype::Definition(x));
        }
        if let Ok(x) = EnumDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(DefinitionOrSubtype::EnumDefinition(x));
        }
        if let Ok(x) = SlotDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(DefinitionOrSubtype::SlotDefinition(x));
        }
        if let Ok(x) = ClassDefinition::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(DefinitionOrSubtype::ClassDefinition(x));
        }
        Err("none of the variants matched the mapping form".into())
    }

    fn from_pair_simple(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Definition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(DefinitionOrSubtype::Definition(x));
        }
        if let Ok(x) = EnumDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(DefinitionOrSubtype::EnumDefinition(x));
        }
        if let Ok(x) = SlotDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(DefinitionOrSubtype::SlotDefinition(x));
        }
        if let Ok(x) = ClassDefinition::from_pair_simple(k.clone(), v.clone()) {
            return Ok(DefinitionOrSubtype::ClassDefinition(x));
        }
        Err("none of the variants support the primitive form".into())
    }

    fn extract_key(&self) -> &Self::Key {
        match self {
            DefinitionOrSubtype::Definition(inner) => inner.extract_key(),
            DefinitionOrSubtype::EnumDefinition(inner) => inner.extract_key(),
            DefinitionOrSubtype::SlotDefinition(inner) => inner.extract_key(),
            DefinitionOrSubtype::ClassDefinition(inner) => inner.extract_key(),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AnonymousEnumExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set_tag: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set_version: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pv_formula: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub permissible_values: HashMap<String, PermissibleValue>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub include: Vec<Box<AnonymousEnumExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minus: Vec<Box<AnonymousEnumExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inherits: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reachable_from: Option<ReachabilityQuery>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub matches: Option<MatchQuery>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub concepts: Vec<uriorcurie>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl AnonymousEnumExpression {
    #[new]
    pub fn new(code_set: Option<uriorcurie>, code_set_tag: Option<String>, code_set_version: Option<String>, pv_formula: Option<String>, permissible_values: HashMap<String, PermissibleValue>, include: Vec<Box<AnonymousEnumExpression>>, minus: Vec<Box<AnonymousEnumExpression>>, inherits: Vec<String>, reachable_from: Option<ReachabilityQuery>, matches: Option<MatchQuery>, concepts: Vec<uriorcurie>) -> Self {
        AnonymousEnumExpression{code_set, code_set_tag, code_set_version, pv_formula, permissible_values, include, minus, inherits, reachable_from, matches, concepts}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AnonymousEnumExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AnonymousEnumExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AnonymousEnumExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnonymousEnumExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct EnumDefinition {
    #[cfg_attr(feature = "serde", serde(default))]
    pub enum_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set_tag: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set_version: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pv_formula: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub permissible_values: HashMap<String, PermissibleValue>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub include: Vec<Box<AnonymousEnumExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minus: Vec<Box<AnonymousEnumExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inherits: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reachable_from: Option<ReachabilityQuery>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub matches: Option<MatchQuery>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub concepts: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_a: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub abstract_: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixin: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixins: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub apply_to: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub values_from: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub string_serialization: Option<String>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes_are_closed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub definition_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub local_names: HashMap<String, LocalName>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conforms_to: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl EnumDefinition {
    #[new]
    pub fn new(enum_uri: Option<uriorcurie>, code_set: Option<uriorcurie>, code_set_tag: Option<String>, code_set_version: Option<String>, pv_formula: Option<String>, permissible_values: HashMap<String, PermissibleValue>, include: Vec<Box<AnonymousEnumExpression>>, minus: Vec<Box<AnonymousEnumExpression>>, inherits: Vec<String>, reachable_from: Option<ReachabilityQuery>, matches: Option<MatchQuery>, concepts: Vec<uriorcurie>, is_a: Option<String>, abstract_: Option<bool>, mixin: Option<bool>, mixins: Vec<String>, apply_to: Vec<String>, values_from: Vec<uriorcurie>, string_serialization: Option<String>, name: String, id_prefixes: Vec<ncname>, id_prefixes_are_closed: Option<bool>, definition_uri: Option<uriorcurie>, local_names: HashMap<String, LocalName>, conforms_to: Option<String>, implements: Vec<uriorcurie>, instantiates: Vec<uriorcurie>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        EnumDefinition{enum_uri, code_set, code_set_tag, code_set_version, pv_formula, permissible_values, include, minus, inherits, reachable_from, matches, concepts, is_a, abstract_, mixin, mixins, apply_to, values_from, string_serialization, name, id_prefixes, id_prefixes_are_closed, definition_uri, local_names, conforms_to, implements, instantiates, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<EnumDefinition>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<EnumDefinition> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<EnumDefinition>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid EnumDefinition",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for EnumDefinition {
    type Key   = String;
        
    type Value = uriorcurie;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("name".into()), Value::String(k));
        map.insert(Value::String("enum_uri".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct EnumBinding {
    #[cfg_attr(feature = "serde", serde(default))]
    pub range: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub obligation_level: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub binds_value_of: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pv_formula: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl EnumBinding {
    #[new]
    pub fn new(range: Option<String>, obligation_level: Option<String>, binds_value_of: Option<String>, pv_formula: Option<String>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        EnumBinding{range, obligation_level, binds_value_of, pv_formula, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<EnumBinding>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<EnumBinding> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<EnumBinding>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid EnumBinding",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct MatchQuery {
    #[cfg_attr(feature = "serde", serde(default))]
    pub identifier_pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source_ontology: Option<uriorcurie>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl MatchQuery {
    #[new]
    pub fn new(identifier_pattern: Option<String>, source_ontology: Option<uriorcurie>) -> Self {
        MatchQuery{identifier_pattern, source_ontology}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<MatchQuery>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<MatchQuery> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<MatchQuery>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid MatchQuery",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ReachabilityQuery {
    #[cfg_attr(feature = "serde", serde(default))]
    pub source_ontology: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub source_nodes: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub relationship_types: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_direct: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub include_self: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub traverse_up: Option<bool>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl ReachabilityQuery {
    #[new]
    pub fn new(source_ontology: Option<uriorcurie>, source_nodes: Vec<uriorcurie>, relationship_types: Vec<uriorcurie>, is_direct: Option<bool>, include_self: Option<bool>, traverse_up: Option<bool>) -> Self {
        ReachabilityQuery{source_ontology, source_nodes, relationship_types, is_direct, include_self, traverse_up}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ReachabilityQuery>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ReachabilityQuery> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ReachabilityQuery>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ReachabilityQuery",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct StructuredAlias {
    pub literal_form: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub alias_predicate: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub alias_contexts: Vec<uri>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<Box<StructuredAlias>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl StructuredAlias {
    #[new]
    pub fn new(literal_form: String, alias_predicate: Option<String>, categories: Vec<uriorcurie>, alias_contexts: Vec<uri>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<Box<StructuredAlias>>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, keywords: Vec<String>) -> Self {
        StructuredAlias{literal_form, alias_predicate, categories, alias_contexts, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<StructuredAlias>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<StructuredAlias> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<StructuredAlias>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructuredAlias",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Expression {
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum ExpressionOrSubtype {    Expression(Expression),     TypeExpression(TypeExpression),     EnumExpression(EnumExpression),     StructuredAlias(StructuredAlias),     AnonymousExpression(AnonymousExpression),     PathExpression(PathExpression),     SlotExpression(SlotExpression),     AnonymousSlotExpression(AnonymousSlotExpression),     SlotDefinition(SlotDefinition),     AnonymousClassExpression(AnonymousClassExpression),     AnonymousEnumExpression(AnonymousEnumExpression),     EnumDefinition(EnumDefinition),     AnonymousTypeExpression(AnonymousTypeExpression),     TypeDefinition(TypeDefinition)}

impl From<Expression>   for ExpressionOrSubtype { fn from(x: Expression)   -> Self { Self::Expression(x) } }
impl From<TypeExpression>   for ExpressionOrSubtype { fn from(x: TypeExpression)   -> Self { Self::TypeExpression(x) } }
impl From<EnumExpression>   for ExpressionOrSubtype { fn from(x: EnumExpression)   -> Self { Self::EnumExpression(x) } }
impl From<StructuredAlias>   for ExpressionOrSubtype { fn from(x: StructuredAlias)   -> Self { Self::StructuredAlias(x) } }
impl From<AnonymousExpression>   for ExpressionOrSubtype { fn from(x: AnonymousExpression)   -> Self { Self::AnonymousExpression(x) } }
impl From<PathExpression>   for ExpressionOrSubtype { fn from(x: PathExpression)   -> Self { Self::PathExpression(x) } }
impl From<SlotExpression>   for ExpressionOrSubtype { fn from(x: SlotExpression)   -> Self { Self::SlotExpression(x) } }
impl From<AnonymousSlotExpression>   for ExpressionOrSubtype { fn from(x: AnonymousSlotExpression)   -> Self { Self::AnonymousSlotExpression(x) } }
impl From<SlotDefinition>   for ExpressionOrSubtype { fn from(x: SlotDefinition)   -> Self { Self::SlotDefinition(x) } }
impl From<AnonymousClassExpression>   for ExpressionOrSubtype { fn from(x: AnonymousClassExpression)   -> Self { Self::AnonymousClassExpression(x) } }
impl From<AnonymousEnumExpression>   for ExpressionOrSubtype { fn from(x: AnonymousEnumExpression)   -> Self { Self::AnonymousEnumExpression(x) } }
impl From<EnumDefinition>   for ExpressionOrSubtype { fn from(x: EnumDefinition)   -> Self { Self::EnumDefinition(x) } }
impl From<AnonymousTypeExpression>   for ExpressionOrSubtype { fn from(x: AnonymousTypeExpression)   -> Self { Self::AnonymousTypeExpression(x) } }
impl From<TypeDefinition>   for ExpressionOrSubtype { fn from(x: TypeDefinition)   -> Self { Self::TypeDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ExpressionOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Expression>() {
            return Ok(ExpressionOrSubtype::Expression(val));
        }        if let Ok(val) = ob.extract::<TypeExpression>() {
            return Ok(ExpressionOrSubtype::TypeExpression(val));
        }        if let Ok(val) = ob.extract::<EnumExpression>() {
            return Ok(ExpressionOrSubtype::EnumExpression(val));
        }        if let Ok(val) = ob.extract::<StructuredAlias>() {
            return Ok(ExpressionOrSubtype::StructuredAlias(val));
        }        if let Ok(val) = ob.extract::<AnonymousExpression>() {
            return Ok(ExpressionOrSubtype::AnonymousExpression(val));
        }        if let Ok(val) = ob.extract::<PathExpression>() {
            return Ok(ExpressionOrSubtype::PathExpression(val));
        }        if let Ok(val) = ob.extract::<SlotExpression>() {
            return Ok(ExpressionOrSubtype::SlotExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousSlotExpression>() {
            return Ok(ExpressionOrSubtype::AnonymousSlotExpression(val));
        }        if let Ok(val) = ob.extract::<SlotDefinition>() {
            return Ok(ExpressionOrSubtype::SlotDefinition(val));
        }        if let Ok(val) = ob.extract::<AnonymousClassExpression>() {
            return Ok(ExpressionOrSubtype::AnonymousClassExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousEnumExpression>() {
            return Ok(ExpressionOrSubtype::AnonymousEnumExpression(val));
        }        if let Ok(val) = ob.extract::<EnumDefinition>() {
            return Ok(ExpressionOrSubtype::EnumDefinition(val));
        }        if let Ok(val) = ob.extract::<AnonymousTypeExpression>() {
            return Ok(ExpressionOrSubtype::AnonymousTypeExpression(val));
        }        if let Ok(val) = ob.extract::<TypeDefinition>() {
            return Ok(ExpressionOrSubtype::TypeDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ExpressionOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ExpressionOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            ExpressionOrSubtype::Expression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::TypeExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::EnumExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::StructuredAlias(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::AnonymousExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::PathExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::SlotExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::AnonymousSlotExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::SlotDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::AnonymousClassExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::AnonymousEnumExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::EnumDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::AnonymousTypeExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ExpressionOrSubtype::TypeDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ExpressionOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ExpressionOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ExpressionOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ExpressionOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TypeExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_pattern: Option<PatternExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit: Option<UnitOfMeasure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub implicit_prefix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_string: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub equals_string_in: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<AnonymousTypeExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<AnonymousTypeExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<AnonymousTypeExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<AnonymousTypeExpression>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl TypeExpression {
    #[new]
    pub fn new(pattern: Option<String>, structured_pattern: Option<PatternExpression>, unit: Option<UnitOfMeasure>, implicit_prefix: Option<String>, equals_string: Option<String>, equals_string_in: Vec<String>, equals_number: Option<isize>, minimum_value: Option<Anything>, maximum_value: Option<Anything>, none_of: Vec<AnonymousTypeExpression>, exactly_one_of: Vec<AnonymousTypeExpression>, any_of: Vec<AnonymousTypeExpression>, all_of: Vec<AnonymousTypeExpression>) -> Self {
        TypeExpression{pattern, structured_pattern, unit, implicit_prefix, equals_string, equals_string_in, equals_number, minimum_value, maximum_value, none_of, exactly_one_of, any_of, all_of}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TypeExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TypeExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TypeExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TypeExpression",
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum TypeExpressionOrSubtype {    TypeExpression(TypeExpression),     AnonymousTypeExpression(AnonymousTypeExpression),     TypeDefinition(TypeDefinition)}

impl From<TypeExpression>   for TypeExpressionOrSubtype { fn from(x: TypeExpression)   -> Self { Self::TypeExpression(x) } }
impl From<AnonymousTypeExpression>   for TypeExpressionOrSubtype { fn from(x: AnonymousTypeExpression)   -> Self { Self::AnonymousTypeExpression(x) } }
impl From<TypeDefinition>   for TypeExpressionOrSubtype { fn from(x: TypeDefinition)   -> Self { Self::TypeDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for TypeExpressionOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TypeExpression>() {
            return Ok(TypeExpressionOrSubtype::TypeExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousTypeExpression>() {
            return Ok(TypeExpressionOrSubtype::AnonymousTypeExpression(val));
        }        if let Ok(val) = ob.extract::<TypeDefinition>() {
            return Ok(TypeExpressionOrSubtype::TypeDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TypeExpressionOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for TypeExpressionOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            TypeExpressionOrSubtype::TypeExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            TypeExpressionOrSubtype::AnonymousTypeExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            TypeExpressionOrSubtype::TypeDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TypeExpressionOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TypeExpressionOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TypeExpressionOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TypeExpressionOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct EnumExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set_tag: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_set_version: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pv_formula: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub permissible_values: HashMap<String, PermissibleValue>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub include: Vec<AnonymousEnumExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minus: Vec<AnonymousEnumExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inherits: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reachable_from: Option<ReachabilityQuery>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub matches: Option<MatchQuery>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub concepts: Vec<uriorcurie>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl EnumExpression {
    #[new]
    pub fn new(code_set: Option<uriorcurie>, code_set_tag: Option<String>, code_set_version: Option<String>, pv_formula: Option<String>, permissible_values: HashMap<String, PermissibleValue>, include: Vec<AnonymousEnumExpression>, minus: Vec<AnonymousEnumExpression>, inherits: Vec<String>, reachable_from: Option<ReachabilityQuery>, matches: Option<MatchQuery>, concepts: Vec<uriorcurie>) -> Self {
        EnumExpression{code_set, code_set_tag, code_set_version, pv_formula, permissible_values, include, minus, inherits, reachable_from, matches, concepts}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<EnumExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<EnumExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<EnumExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid EnumExpression",
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum EnumExpressionOrSubtype {    EnumExpression(EnumExpression),     AnonymousEnumExpression(AnonymousEnumExpression),     EnumDefinition(EnumDefinition)}

impl From<EnumExpression>   for EnumExpressionOrSubtype { fn from(x: EnumExpression)   -> Self { Self::EnumExpression(x) } }
impl From<AnonymousEnumExpression>   for EnumExpressionOrSubtype { fn from(x: AnonymousEnumExpression)   -> Self { Self::AnonymousEnumExpression(x) } }
impl From<EnumDefinition>   for EnumExpressionOrSubtype { fn from(x: EnumDefinition)   -> Self { Self::EnumDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for EnumExpressionOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<EnumExpression>() {
            return Ok(EnumExpressionOrSubtype::EnumExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousEnumExpression>() {
            return Ok(EnumExpressionOrSubtype::AnonymousEnumExpression(val));
        }        if let Ok(val) = ob.extract::<EnumDefinition>() {
            return Ok(EnumExpressionOrSubtype::EnumDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid EnumExpressionOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for EnumExpressionOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            EnumExpressionOrSubtype::EnumExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EnumExpressionOrSubtype::AnonymousEnumExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EnumExpressionOrSubtype::EnumDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<EnumExpressionOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<EnumExpressionOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<EnumExpressionOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid EnumExpressionOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AnonymousExpression {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl AnonymousExpression {
    #[new]
    pub fn new(extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        AnonymousExpression{extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AnonymousExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AnonymousExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AnonymousExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnonymousExpression",
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum AnonymousExpressionOrSubtype {    AnonymousExpression(AnonymousExpression),     AnonymousSlotExpression(AnonymousSlotExpression),     AnonymousClassExpression(AnonymousClassExpression)}

impl From<AnonymousExpression>   for AnonymousExpressionOrSubtype { fn from(x: AnonymousExpression)   -> Self { Self::AnonymousExpression(x) } }
impl From<AnonymousSlotExpression>   for AnonymousExpressionOrSubtype { fn from(x: AnonymousSlotExpression)   -> Self { Self::AnonymousSlotExpression(x) } }
impl From<AnonymousClassExpression>   for AnonymousExpressionOrSubtype { fn from(x: AnonymousClassExpression)   -> Self { Self::AnonymousClassExpression(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for AnonymousExpressionOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AnonymousExpression>() {
            return Ok(AnonymousExpressionOrSubtype::AnonymousExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousSlotExpression>() {
            return Ok(AnonymousExpressionOrSubtype::AnonymousSlotExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousClassExpression>() {
            return Ok(AnonymousExpressionOrSubtype::AnonymousClassExpression(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnonymousExpressionOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for AnonymousExpressionOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            AnonymousExpressionOrSubtype::AnonymousExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnonymousExpressionOrSubtype::AnonymousSlotExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AnonymousExpressionOrSubtype::AnonymousClassExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AnonymousExpressionOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AnonymousExpressionOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AnonymousExpressionOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnonymousExpressionOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PathExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub followed_by: Option<Box<PathExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<Box<PathExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<Box<PathExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<Box<PathExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<Box<PathExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reversed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub traverse: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub range_expression: Option<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl PathExpression {
    #[new]
    pub fn new(followed_by: Option<Box<PathExpression>>, none_of: Vec<Box<PathExpression>>, any_of: Vec<Box<PathExpression>>, all_of: Vec<Box<PathExpression>>, exactly_one_of: Vec<Box<PathExpression>>, reversed: Option<bool>, traverse: Option<String>, range_expression: Option<Box<AnonymousClassExpression>>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        PathExpression{followed_by, none_of, any_of, all_of, exactly_one_of, reversed, traverse, range_expression, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PathExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PathExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PathExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PathExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SlotExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub range: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub range_expression: Option<AnonymousClassExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub enum_range: Option<EnumExpressionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bindings: Vec<EnumBinding>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub required: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recommended: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub multivalued: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inlined: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inlined_as_list: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_pattern: Option<PatternExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit: Option<UnitOfMeasure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub implicit_prefix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_presence: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_string: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub equals_string_in: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_expression: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exact_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub has_member: Option<AnonymousSlotExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_members: Option<AnonymousSlotExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<AnonymousSlotExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<AnonymousSlotExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<AnonymousSlotExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<AnonymousSlotExpression>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl SlotExpression {
    #[new]
    pub fn new(range: Option<String>, range_expression: Option<AnonymousClassExpression>, enum_range: Option<EnumExpressionOrSubtype>, bindings: Vec<EnumBinding>, required: Option<bool>, recommended: Option<bool>, multivalued: Option<bool>, inlined: Option<bool>, inlined_as_list: Option<bool>, minimum_value: Option<Anything>, maximum_value: Option<Anything>, pattern: Option<String>, structured_pattern: Option<PatternExpression>, unit: Option<UnitOfMeasure>, implicit_prefix: Option<String>, value_presence: Option<String>, equals_string: Option<String>, equals_string_in: Vec<String>, equals_number: Option<isize>, equals_expression: Option<String>, exact_cardinality: Option<isize>, minimum_cardinality: Option<isize>, maximum_cardinality: Option<isize>, has_member: Option<AnonymousSlotExpression>, all_members: Option<AnonymousSlotExpression>, none_of: Vec<AnonymousSlotExpression>, exactly_one_of: Vec<AnonymousSlotExpression>, any_of: Vec<AnonymousSlotExpression>, all_of: Vec<AnonymousSlotExpression>) -> Self {
        SlotExpression{range, range_expression, enum_range, bindings, required, recommended, multivalued, inlined, inlined_as_list, minimum_value, maximum_value, pattern, structured_pattern, unit, implicit_prefix, value_presence, equals_string, equals_string_in, equals_number, equals_expression, exact_cardinality, minimum_cardinality, maximum_cardinality, has_member, all_members, none_of, exactly_one_of, any_of, all_of}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SlotExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SlotExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SlotExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SlotExpression",
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum SlotExpressionOrSubtype {    SlotExpression(SlotExpression),     AnonymousSlotExpression(AnonymousSlotExpression),     SlotDefinition(SlotDefinition)}

impl From<SlotExpression>   for SlotExpressionOrSubtype { fn from(x: SlotExpression)   -> Self { Self::SlotExpression(x) } }
impl From<AnonymousSlotExpression>   for SlotExpressionOrSubtype { fn from(x: AnonymousSlotExpression)   -> Self { Self::AnonymousSlotExpression(x) } }
impl From<SlotDefinition>   for SlotExpressionOrSubtype { fn from(x: SlotDefinition)   -> Self { Self::SlotDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for SlotExpressionOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SlotExpression>() {
            return Ok(SlotExpressionOrSubtype::SlotExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousSlotExpression>() {
            return Ok(SlotExpressionOrSubtype::AnonymousSlotExpression(val));
        }        if let Ok(val) = ob.extract::<SlotDefinition>() {
            return Ok(SlotExpressionOrSubtype::SlotDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SlotExpressionOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for SlotExpressionOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            SlotExpressionOrSubtype::SlotExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            SlotExpressionOrSubtype::AnonymousSlotExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            SlotExpressionOrSubtype::SlotDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SlotExpressionOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SlotExpressionOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SlotExpressionOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SlotExpressionOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AnonymousSlotExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub range: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub range_expression: Option<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub enum_range: Option<EnumExpressionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bindings: Vec<EnumBinding>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub required: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recommended: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub multivalued: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inlined: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inlined_as_list: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_pattern: Option<PatternExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit: Option<UnitOfMeasure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub implicit_prefix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_presence: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_string: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub equals_string_in: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_expression: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exact_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub has_member: Option<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_members: Option<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl AnonymousSlotExpression {
    #[new]
    pub fn new(range: Option<String>, range_expression: Option<Box<AnonymousClassExpression>>, enum_range: Option<EnumExpressionOrSubtype>, bindings: Vec<EnumBinding>, required: Option<bool>, recommended: Option<bool>, multivalued: Option<bool>, inlined: Option<bool>, inlined_as_list: Option<bool>, minimum_value: Option<Anything>, maximum_value: Option<Anything>, pattern: Option<String>, structured_pattern: Option<PatternExpression>, unit: Option<UnitOfMeasure>, implicit_prefix: Option<String>, value_presence: Option<String>, equals_string: Option<String>, equals_string_in: Vec<String>, equals_number: Option<isize>, equals_expression: Option<String>, exact_cardinality: Option<isize>, minimum_cardinality: Option<isize>, maximum_cardinality: Option<isize>, has_member: Option<Box<AnonymousSlotExpression>>, all_members: Option<Box<AnonymousSlotExpression>>, none_of: Vec<Box<AnonymousSlotExpression>>, exactly_one_of: Vec<Box<AnonymousSlotExpression>>, any_of: Vec<Box<AnonymousSlotExpression>>, all_of: Vec<Box<AnonymousSlotExpression>>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        AnonymousSlotExpression{range, range_expression, enum_range, bindings, required, recommended, multivalued, inlined, inlined_as_list, minimum_value, maximum_value, pattern, structured_pattern, unit, implicit_prefix, value_presence, equals_string, equals_string_in, equals_number, equals_expression, exact_cardinality, minimum_cardinality, maximum_cardinality, has_member, all_members, none_of, exactly_one_of, any_of, all_of, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AnonymousSlotExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AnonymousSlotExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AnonymousSlotExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnonymousSlotExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SlotDefinition {
    #[cfg_attr(feature = "serde", serde(default))]
    pub singular_name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub domain: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub array: Option<ArrayExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inherited: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub readonly: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ifabsent: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub list_elements_unique: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub list_elements_ordered: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub shared: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub key: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub identifier: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub designates_type: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub alias: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub owner: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub domain_of: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subproperty_of: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub symmetric: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reflexive: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub locally_reflexive: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub irreflexive: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub asymmetric: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub transitive: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inverse: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_class_field: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub transitive_form_of: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reflexive_transitive_form_of: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub role: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_usage_slot: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usage_slot_name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub relational_role: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_group: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_grouping_slot: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub path_rule: Option<Box<PathExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub disjoint_with: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub children_are_mutually_disjoint: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub union_of: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub type_mappings: HashMap<String, TypeMapping>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub range: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub range_expression: Option<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub enum_range: Option<EnumExpressionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bindings: Vec<EnumBinding>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub required: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recommended: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub multivalued: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inlined: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inlined_as_list: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_value: Option<Anything>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_pattern: Option<PatternExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit: Option<UnitOfMeasure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub implicit_prefix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_presence: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_string: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub equals_string_in: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub equals_expression: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exact_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub has_member: Option<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_members: Option<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<Box<AnonymousSlotExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_a: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub abstract_: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixin: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixins: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub apply_to: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub values_from: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub string_serialization: Option<String>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes_are_closed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub definition_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub local_names: HashMap<String, LocalName>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conforms_to: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl SlotDefinition {
    #[new]
    pub fn new(singular_name: Option<String>, domain: Option<String>, slot_uri: Option<uriorcurie>, array: Option<ArrayExpression>, inherited: Option<bool>, readonly: Option<String>, ifabsent: Option<String>, list_elements_unique: Option<bool>, list_elements_ordered: Option<bool>, shared: Option<bool>, key: Option<bool>, identifier: Option<bool>, designates_type: Option<bool>, alias: Option<String>, owner: Option<String>, domain_of: Vec<String>, subproperty_of: Option<String>, symmetric: Option<bool>, reflexive: Option<bool>, locally_reflexive: Option<bool>, irreflexive: Option<bool>, asymmetric: Option<bool>, transitive: Option<bool>, inverse: Option<String>, is_class_field: Option<bool>, transitive_form_of: Option<String>, reflexive_transitive_form_of: Option<String>, role: Option<String>, is_usage_slot: Option<bool>, usage_slot_name: Option<String>, relational_role: Option<String>, slot_group: Option<String>, is_grouping_slot: Option<bool>, path_rule: Option<Box<PathExpression>>, disjoint_with: Vec<String>, children_are_mutually_disjoint: Option<bool>, union_of: Vec<String>, type_mappings: HashMap<String, TypeMapping>, range: Option<String>, range_expression: Option<Box<AnonymousClassExpression>>, enum_range: Option<EnumExpressionOrSubtype>, bindings: Vec<EnumBinding>, required: Option<bool>, recommended: Option<bool>, multivalued: Option<bool>, inlined: Option<bool>, inlined_as_list: Option<bool>, minimum_value: Option<Anything>, maximum_value: Option<Anything>, pattern: Option<String>, structured_pattern: Option<PatternExpression>, unit: Option<UnitOfMeasure>, implicit_prefix: Option<String>, value_presence: Option<String>, equals_string: Option<String>, equals_string_in: Vec<String>, equals_number: Option<isize>, equals_expression: Option<String>, exact_cardinality: Option<isize>, minimum_cardinality: Option<isize>, maximum_cardinality: Option<isize>, has_member: Option<Box<AnonymousSlotExpression>>, all_members: Option<Box<AnonymousSlotExpression>>, none_of: Vec<Box<AnonymousSlotExpression>>, exactly_one_of: Vec<Box<AnonymousSlotExpression>>, any_of: Vec<Box<AnonymousSlotExpression>>, all_of: Vec<Box<AnonymousSlotExpression>>, is_a: Option<String>, abstract_: Option<bool>, mixin: Option<bool>, mixins: Vec<String>, apply_to: Vec<String>, values_from: Vec<uriorcurie>, string_serialization: Option<String>, name: String, id_prefixes: Vec<ncname>, id_prefixes_are_closed: Option<bool>, definition_uri: Option<uriorcurie>, local_names: HashMap<String, LocalName>, conforms_to: Option<String>, implements: Vec<uriorcurie>, instantiates: Vec<uriorcurie>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        SlotDefinition{singular_name, domain, slot_uri, array, inherited, readonly, ifabsent, list_elements_unique, list_elements_ordered, shared, key, identifier, designates_type, alias, owner, domain_of, subproperty_of, symmetric, reflexive, locally_reflexive, irreflexive, asymmetric, transitive, inverse, is_class_field, transitive_form_of, reflexive_transitive_form_of, role, is_usage_slot, usage_slot_name, relational_role, slot_group, is_grouping_slot, path_rule, disjoint_with, children_are_mutually_disjoint, union_of, type_mappings, range, range_expression, enum_range, bindings, required, recommended, multivalued, inlined, inlined_as_list, minimum_value, maximum_value, pattern, structured_pattern, unit, implicit_prefix, value_presence, equals_string, equals_string_in, equals_number, equals_expression, exact_cardinality, minimum_cardinality, maximum_cardinality, has_member, all_members, none_of, exactly_one_of, any_of, all_of, is_a, abstract_, mixin, mixins, apply_to, values_from, string_serialization, name, id_prefixes, id_prefixes_are_closed, definition_uri, local_names, conforms_to, implements, instantiates, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SlotDefinition>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SlotDefinition> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SlotDefinition>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SlotDefinition",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for SlotDefinition {
    type Key   = String;
        
    type Value = String;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("name".into()), Value::String(k));
        map.insert(Value::String("singular_name".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ClassExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<AnonymousClassExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<AnonymousClassExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<AnonymousClassExpression>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<AnonymousClassExpression>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_conditions: HashMap<String, SlotDefinition>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl ClassExpression {
    #[new]
    pub fn new(any_of: Vec<AnonymousClassExpression>, exactly_one_of: Vec<AnonymousClassExpression>, none_of: Vec<AnonymousClassExpression>, all_of: Vec<AnonymousClassExpression>, slot_conditions: HashMap<String, SlotDefinition>) -> Self {
        ClassExpression{any_of, exactly_one_of, none_of, all_of, slot_conditions}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ClassExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ClassExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ClassExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ClassExpression",
        ))
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum ClassExpressionOrSubtype {    ClassExpression(ClassExpression),     AnonymousClassExpression(AnonymousClassExpression),     ClassDefinition(ClassDefinition)}

impl From<ClassExpression>   for ClassExpressionOrSubtype { fn from(x: ClassExpression)   -> Self { Self::ClassExpression(x) } }
impl From<AnonymousClassExpression>   for ClassExpressionOrSubtype { fn from(x: AnonymousClassExpression)   -> Self { Self::AnonymousClassExpression(x) } }
impl From<ClassDefinition>   for ClassExpressionOrSubtype { fn from(x: ClassDefinition)   -> Self { Self::ClassDefinition(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ClassExpressionOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ClassExpression>() {
            return Ok(ClassExpressionOrSubtype::ClassExpression(val));
        }        if let Ok(val) = ob.extract::<AnonymousClassExpression>() {
            return Ok(ClassExpressionOrSubtype::AnonymousClassExpression(val));
        }        if let Ok(val) = ob.extract::<ClassDefinition>() {
            return Ok(ClassExpressionOrSubtype::ClassDefinition(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ClassExpressionOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ClassExpressionOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            ClassExpressionOrSubtype::ClassExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ClassExpressionOrSubtype::AnonymousClassExpression(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ClassExpressionOrSubtype::ClassDefinition(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ClassExpressionOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ClassExpressionOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ClassExpressionOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ClassExpressionOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AnonymousClassExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_a: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_conditions: HashMap<String, Box<SlotDefinition>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl AnonymousClassExpression {
    #[new]
    pub fn new(is_a: Option<String>, any_of: Vec<Box<AnonymousClassExpression>>, exactly_one_of: Vec<Box<AnonymousClassExpression>>, none_of: Vec<Box<AnonymousClassExpression>>, all_of: Vec<Box<AnonymousClassExpression>>, slot_conditions: HashMap<String, Box<SlotDefinition>>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        AnonymousClassExpression{is_a, any_of, exactly_one_of, none_of, all_of, slot_conditions, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AnonymousClassExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AnonymousClassExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AnonymousClassExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AnonymousClassExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ClassDefinition {
    #[cfg_attr(feature = "serde", serde(default))]
    pub slots: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_usage: HashMap<String, Box<SlotDefinition>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub attributes: HashMap<String, Box<SlotDefinition>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub class_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subclass_of: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub union_of: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub defining_slots: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tree_root: Option<bool>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub unique_keys: HashMap<String, Box<UniqueKey>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rules: Vec<Box<ClassRule>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub classification_rules: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_names_unique: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub represents_relationship: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub disjoint_with: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub children_are_mutually_disjoint: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub any_of: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exactly_one_of: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub none_of: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_of: Vec<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub slot_conditions: HashMap<String, Box<SlotDefinition>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_a: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub abstract_: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixin: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixins: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub apply_to: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub values_from: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub string_serialization: Option<String>,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes: Vec<ncname>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub id_prefixes_are_closed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub definition_uri: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub local_names: HashMap<String, LocalName>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conforms_to: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl ClassDefinition {
    #[new]
    pub fn new(slots: Vec<String>, slot_usage: HashMap<String, Box<SlotDefinition>>, attributes: HashMap<String, Box<SlotDefinition>>, class_uri: Option<uriorcurie>, subclass_of: Option<uriorcurie>, union_of: Vec<String>, defining_slots: Vec<String>, tree_root: Option<bool>, unique_keys: HashMap<String, Box<UniqueKey>>, rules: Vec<Box<ClassRule>>, classification_rules: Vec<Box<AnonymousClassExpression>>, slot_names_unique: Option<bool>, represents_relationship: Option<bool>, disjoint_with: Vec<String>, children_are_mutually_disjoint: Option<bool>, any_of: Vec<Box<AnonymousClassExpression>>, exactly_one_of: Vec<Box<AnonymousClassExpression>>, none_of: Vec<Box<AnonymousClassExpression>>, all_of: Vec<Box<AnonymousClassExpression>>, slot_conditions: HashMap<String, Box<SlotDefinition>>, is_a: Option<String>, abstract_: Option<bool>, mixin: Option<bool>, mixins: Vec<String>, apply_to: Vec<String>, values_from: Vec<uriorcurie>, string_serialization: Option<String>, name: String, id_prefixes: Vec<ncname>, id_prefixes_are_closed: Option<bool>, definition_uri: Option<uriorcurie>, local_names: HashMap<String, LocalName>, conforms_to: Option<String>, implements: Vec<uriorcurie>, instantiates: Vec<uriorcurie>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        ClassDefinition{slots, slot_usage, attributes, class_uri, subclass_of, union_of, defining_slots, tree_root, unique_keys, rules, classification_rules, slot_names_unique, represents_relationship, disjoint_with, children_are_mutually_disjoint, any_of, exactly_one_of, none_of, all_of, slot_conditions, is_a, abstract_, mixin, mixins, apply_to, values_from, string_serialization, name, id_prefixes, id_prefixes_are_closed, definition_uri, local_names, conforms_to, implements, instantiates, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ClassDefinition>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ClassDefinition> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ClassDefinition>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ClassDefinition",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for ClassDefinition {
    type Key   = String;
        
    type Value = SlotDefinition;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("name".into()), Value::String(k));
        map.insert(Value::String("slots".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ClassLevelRule {
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum ClassLevelRuleOrSubtype {    ClassLevelRule(ClassLevelRule),     ClassRule(ClassRule)}

impl From<ClassLevelRule>   for ClassLevelRuleOrSubtype { fn from(x: ClassLevelRule)   -> Self { Self::ClassLevelRule(x) } }
impl From<ClassRule>   for ClassLevelRuleOrSubtype { fn from(x: ClassRule)   -> Self { Self::ClassRule(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ClassLevelRuleOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ClassLevelRule>() {
            return Ok(ClassLevelRuleOrSubtype::ClassLevelRule(val));
        }        if let Ok(val) = ob.extract::<ClassRule>() {
            return Ok(ClassLevelRuleOrSubtype::ClassRule(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ClassLevelRuleOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ClassLevelRuleOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            ClassLevelRuleOrSubtype::ClassLevelRule(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            ClassLevelRuleOrSubtype::ClassRule(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ClassLevelRuleOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ClassLevelRuleOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ClassLevelRuleOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ClassLevelRuleOrSubtype",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ClassRule {
    #[cfg_attr(feature = "serde", serde(default))]
    pub preconditions: Option<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub postconditions: Option<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub elseconditions: Option<Box<AnonymousClassExpression>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bidirectional: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub open_world: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deactivated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl ClassRule {
    #[new]
    pub fn new(preconditions: Option<Box<AnonymousClassExpression>>, postconditions: Option<Box<AnonymousClassExpression>>, elseconditions: Option<Box<AnonymousClassExpression>>, bidirectional: Option<bool>, open_world: Option<bool>, rank: Option<isize>, deactivated: Option<bool>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        ClassRule{preconditions, postconditions, elseconditions, bidirectional, open_world, rank, deactivated, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ClassRule>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ClassRule> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ClassRule>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ClassRule",
        ))
    }
}

pub mod array_expression_utl {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum maximum_number_dimensions_range {
        Anything(Anything),
        isize(isize),
        bool(bool)    
    }


    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for maximum_number_dimensions_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<Anything>() {
                return Ok(maximum_number_dimensions_range::Anything(val));
            }            if let Ok(val) = ob.extract::<isize>() {
                return Ok(maximum_number_dimensions_range::isize(val));
            }            if let Ok(val) = ob.extract::<bool>() {
                return Ok(maximum_number_dimensions_range::bool(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid maximum_number_dimensions",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for maximum_number_dimensions_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                maximum_number_dimensions_range::Anything(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                maximum_number_dimensions_range::isize(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                maximum_number_dimensions_range::bool(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<maximum_number_dimensions_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<maximum_number_dimensions_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<maximum_number_dimensions_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid maximum_number_dimensions",
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ArrayExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub exact_number_dimensions: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_number_dimensions: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_number_dimensions: Option<array_expression_utl::maximum_number_dimensions_range>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub dimensions: Vec<DimensionExpression>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl ArrayExpression {
    #[new]
    pub fn new(exact_number_dimensions: Option<isize>, minimum_number_dimensions: Option<isize>, maximum_number_dimensions: Option<array_expression_utl::maximum_number_dimensions_range>, dimensions: Vec<DimensionExpression>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        ArrayExpression{exact_number_dimensions, minimum_number_dimensions, maximum_number_dimensions, dimensions, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ArrayExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ArrayExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ArrayExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ArrayExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct DimensionExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub alias: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub maximum_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minimum_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exact_cardinality: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl DimensionExpression {
    #[new]
    pub fn new(alias: Option<String>, maximum_cardinality: Option<isize>, minimum_cardinality: Option<isize>, exact_cardinality: Option<isize>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        DimensionExpression{alias, maximum_cardinality, minimum_cardinality, exact_cardinality, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<DimensionExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<DimensionExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<DimensionExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid DimensionExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PatternExpression {
    #[cfg_attr(feature = "serde", serde(default))]
    pub syntax: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub interpolated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub partial_match: Option<bool>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl PatternExpression {
    #[new]
    pub fn new(syntax: Option<String>, interpolated: Option<bool>, partial_match: Option<bool>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        PatternExpression{syntax, interpolated, partial_match, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PatternExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PatternExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PatternExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PatternExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ImportExpression {
    pub import_from: uriorcurie,
    #[cfg_attr(feature = "serde", serde(default))]
    pub import_as: Option<ncname>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub import_map: HashMap<String, Setting>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl ImportExpression {
    #[new]
    pub fn new(import_from: uriorcurie, import_as: Option<ncname>, import_map: HashMap<String, Setting>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        ImportExpression{import_from, import_as, import_map, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ImportExpression>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ImportExpression> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ImportExpression>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ImportExpression",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Setting {
    pub setting_key: ncname,
    pub setting_value: String
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Setting {
    #[new]
    pub fn new(setting_key: ncname, setting_value: String) -> Self {
        Setting{setting_key, setting_value}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Setting>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Setting> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Setting>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Setting",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Setting {
    type Key   = ncname;
        
    type Value = String;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.setting_key;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("setting_key".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("setting_key".into()), Value::String(k));
        map.insert(Value::String("setting_value".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Prefix {
    pub prefix_prefix: ncname,
    pub prefix_reference: uri
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Prefix {
    #[new]
    pub fn new(prefix_prefix: ncname, prefix_reference: uri) -> Self {
        Prefix{prefix_prefix, prefix_reference}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Prefix>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Prefix> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Prefix>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Prefix",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Prefix {
    type Key   = ncname;
        
    type Value = uri;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.prefix_prefix;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("prefix_prefix".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("prefix_prefix".into()), Value::String(k));
        map.insert(Value::String("prefix_reference".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct LocalName {
    pub local_name_source: ncname,
    pub local_name_value: String
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl LocalName {
    #[new]
    pub fn new(local_name_source: ncname, local_name_value: String) -> Self {
        LocalName{local_name_source, local_name_value}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<LocalName>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<LocalName> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<LocalName>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid LocalName",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for LocalName {
    type Key   = ncname;
        
    type Value = String;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.local_name_source;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("local_name_source".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("local_name_source".into()), Value::String(k));
        map.insert(Value::String("local_name_value".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Example {
    #[cfg_attr(feature = "serde", serde(default))]
    pub value: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_object: Option<Anything>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl Example {
    #[new]
    pub fn new(value: Option<String>, value_description: Option<String>, value_object: Option<Anything>) -> Self {
        Example{value, value_description, value_object}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Example>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Example> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Example>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Example",
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AltDescription {
    pub alt_description_source: String,
    pub alt_description_text: String
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl AltDescription {
    #[new]
    pub fn new(alt_description_source: String, alt_description_text: String) -> Self {
        AltDescription{alt_description_source, alt_description_text}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AltDescription>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AltDescription> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AltDescription>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AltDescription",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for AltDescription {
    type Key   = String;
        
    type Value = String;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.alt_description_source;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("alt_description_source".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("alt_description_source".into()), Value::String(k));
        map.insert(Value::String("alt_description_text".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PermissibleValue {
    pub text: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub meaning: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit: Option<UnitOfMeasure>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub instantiates: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub implements: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_a: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mixins: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl PermissibleValue {
    #[new]
    pub fn new(text: String, description: Option<String>, meaning: Option<uriorcurie>, unit: Option<UnitOfMeasure>, instantiates: Vec<uriorcurie>, implements: Vec<uriorcurie>, is_a: Option<String>, mixins: Vec<String>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        PermissibleValue{text, description, meaning, unit, instantiates, implements, is_a, mixins, extensions, annotations, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PermissibleValue>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PermissibleValue> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PermissibleValue>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PermissibleValue",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for PermissibleValue {
    type Key   = String;
        
    type Value = String;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.text;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("text".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("text".into()), Value::String(k));
        map.insert(Value::String("description".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct UniqueKey {
    pub unique_key_name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unique_key_slots: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub consider_nulls_inequal: Option<bool>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl UniqueKey {
    #[new]
    pub fn new(unique_key_name: String, unique_key_slots: Vec<String>, consider_nulls_inequal: Option<bool>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        UniqueKey{unique_key_name, unique_key_slots, consider_nulls_inequal, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<UniqueKey>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<UniqueKey> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<UniqueKey>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid UniqueKey",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for UniqueKey {
    type Key   = String;
        
    type Value = SlotDefinition;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.unique_key_name;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("unique_key_name".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("unique_key_name".into()), Value::String(k));
        map.insert(Value::String("unique_key_slots".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TypeMapping {
    pub framework_key: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mapped_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub string_serialization: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub extensions: HashMap<String, ExtensionOrSubtype>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub annotations: HashMap<String, Annotation>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_inlined_dict_map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub alt_descriptions: HashMap<String, AltDescription>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub todos: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub notes: Vec<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Vec<Example>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_subset: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub from_schema: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub imported_from: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub in_language: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub see_also: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_exact_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub deprecated_element_has_possible_replacement: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub aliases: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structured_aliases: Vec<StructuredAlias>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub exact_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub close_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub related_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub narrow_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub broad_mappings: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub contributors: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub created_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub last_updated_on: Option<NaiveDateTime>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub modified_by: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rank: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub categories: Vec<uriorcurie>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value"))]#[cfg_attr(feature = "serde", serde(default))]
    pub keywords: Vec<String>
}
#[cfg(feature = "pyo3")]
#[pymethods]
impl TypeMapping {
    #[new]
    pub fn new(framework_key: String, mapped_type: Option<String>, string_serialization: Option<String>, extensions: HashMap<String, ExtensionOrSubtype>, annotations: HashMap<String, Annotation>, description: Option<String>, alt_descriptions: HashMap<String, AltDescription>, title: Option<String>, deprecated: Option<String>, todos: Vec<String>, notes: Vec<String>, comments: Vec<String>, examples: Vec<Example>, in_subset: Vec<String>, from_schema: Option<uri>, imported_from: Option<String>, source: Option<uriorcurie>, in_language: Option<String>, see_also: Vec<uriorcurie>, deprecated_element_has_exact_replacement: Option<uriorcurie>, deprecated_element_has_possible_replacement: Option<uriorcurie>, aliases: Vec<String>, structured_aliases: Vec<StructuredAlias>, mappings: Vec<uriorcurie>, exact_mappings: Vec<uriorcurie>, close_mappings: Vec<uriorcurie>, related_mappings: Vec<uriorcurie>, narrow_mappings: Vec<uriorcurie>, broad_mappings: Vec<uriorcurie>, created_by: Option<uriorcurie>, contributors: Vec<uriorcurie>, created_on: Option<NaiveDateTime>, last_updated_on: Option<NaiveDateTime>, modified_by: Option<uriorcurie>, status: Option<uriorcurie>, rank: Option<isize>, categories: Vec<uriorcurie>, keywords: Vec<String>) -> Self {
        TypeMapping{framework_key, mapped_type, string_serialization, extensions, annotations, description, alt_descriptions, title, deprecated, todos, notes, comments, examples, in_subset, from_schema, imported_from, source, in_language, see_also, deprecated_element_has_exact_replacement, deprecated_element_has_possible_replacement, aliases, structured_aliases, mappings, exact_mappings, close_mappings, related_mappings, narrow_mappings, broad_mappings, created_by, contributors, created_on, last_updated_on, modified_by, status, rank, categories, keywords}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TypeMapping>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TypeMapping> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TypeMapping>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TypeMapping",
        ))
    }
}
#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for TypeMapping {
    type Key   = String;
        
    type Value = TypeDefinition;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.framework_key;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        map.insert(Value::String("framework_key".into()), Value::String(k));
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


        
    fn from_pair_simple(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::String("framework_key".into()), Value::String(k));
        map.insert(Value::String("mapped_type".into()), v);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }        

    }
}



