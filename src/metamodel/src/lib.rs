#![allow(non_camel_case_types)]
use pyo3::prelude::*;
use serde_yml;
use serde::{Deserialize,Serialize};
use std::collections::{HashMap};


// Types

type string = String;
type integer = String;
type boolean = String;
type float = f64;
type double = f64;
type decimal = String;
type time = String;
type date = String;
type datetime = String;
type date_or_datetime = String;
type uriorcurie = String;
type curie = String;
type uri = String;
type ncname = String;
type objectidentifier = String;
type nodeidentifier = String;
type jsonpointer = String;
type jsonpath = String;
type sparqlpath = String;

// Slots

type mappings = Vec<uriorcurie>;
type exact_mappings = Vec<uriorcurie>;
type close_mappings = Vec<uriorcurie>;
type related_mappings = Vec<uriorcurie>;
type narrow_mappings = Vec<uriorcurie>;
type broad_mappings = Vec<uriorcurie>;
type deprecated_element_has_exact_replacement = uriorcurie;
type deprecated_element_has_possible_replacement = uriorcurie;
type extensions = Vec<Extension>;
type extension_tag = uriorcurie;
type extension_value = AnyValue;
type annotations = Vec<Annotation>;
type unit = UnitOfMeasure;
type ucum_code = String;
type derivation = String;
type has_quantity_kind = uriorcurie;
type iec61360code = String;
type symbol = String;
type abbreviation = String;
type descriptive_name = String;
type name = String;
type title = String;
type conforms_to = String;
type implements = Vec<uriorcurie>;
type instantiates = Vec<uriorcurie>;
type categories = Vec<uriorcurie>;
type keywords = Vec<String>;
type definition_uri = uriorcurie;
type id_prefixes = Vec<ncname>;
type id_prefixes_are_closed = bool;
type description = String;
type structured_aliases = Vec<StructuredAlias>;
type aliases = Vec<String>;
type deprecated = String;
type todos = Vec<String>;
type notes = Vec<String>;
type comments = Vec<String>;
type in_subset = Vec<SubsetDefinition>;
type from_schema = uri;
type imported_from = String;
type see_also = Vec<uriorcurie>;
type owned_by = uriorcurie;
type created_by = uriorcurie;
type contributors = Vec<uriorcurie>;
type created_on = String;
type last_updated_on = String;
type modified_by = uriorcurie;
type status = uriorcurie;
type literal_form = String;
type alias_predicate = String;
type alias_contexts = Vec<uri>;
type in_language = String;
type source = uriorcurie;
type publisher = uriorcurie;
type is_a = Definition;
type abstract_ = bool;
type mixin = bool;
type mixins = Vec<Definition>;
type apply_to = Vec<Definition>;
type values_from = Vec<uriorcurie>;
type code_set = uriorcurie;
type code_set_version = String;
type code_set_tag = String;
type pv_formula = String;
type permissible_values = Vec<PermissibleValue>;
type enum_uri = uriorcurie;
type include = Vec<AnonymousEnumExpression>;
type minus = Vec<AnonymousEnumExpression>;
type inherits = Vec<EnumDefinition>;
type matches = MatchQuery;
type identifier_pattern = String;
type concepts = Vec<uriorcurie>;
type reachable_from = ReachabilityQuery;
type source_ontology = uriorcurie;
type is_direct = bool;
type traverse_up = bool;
type include_self = bool;
type relationship_types = Vec<uriorcurie>;
type source_nodes = Vec<uriorcurie>;
type text = String;
type meaning = uriorcurie;
type id = uri;
type emit_prefixes = Vec<ncname>;
type version = String;
type imports = Vec<uriorcurie>;
type structured_imports = Vec<ImportExpression>;
type license = String;
type default_curi_maps = Vec<String>;
type default_prefix = String;
type default_range = TypeDefinition;
type subsets = Vec<SubsetDefinition>;
type types = Vec<TypeDefinition>;
type enums = Vec<EnumDefinition>;
type slot_definitions = Vec<SlotDefinition>;
type classes = Vec<ClassDefinition>;
type metamodel_version = String;
type source_file = String;
type source_file_date = String;
type source_file_size = isize;
type generation_date = String;
type slots = Vec<SlotDefinition>;
type slot_usage = Vec<SlotDefinition>;
type enum_range = EnumExpression;
type range_expression = AnonymousClassExpression;
type boolean_slot = Vec<Expression>;
type any_of = Vec<Expression>;
type exactly_one_of = Vec<Expression>;
type none_of = Vec<Expression>;
type all_of = Vec<Expression>;
type preconditions = AnonymousClassExpression;
type postconditions = AnonymousClassExpression;
type elseconditions = AnonymousClassExpression;
type bidirectional = bool;
type open_world = bool;
type rank = isize;
type deactivated = bool;
type rules = Vec<ClassRule>;
type classification_rules = Vec<AnonymousClassExpression>;
type slot_conditions = Vec<SlotDefinition>;
type attributes = Vec<SlotDefinition>;
type class_uri = uriorcurie;
type subclass_of = uriorcurie;
type defining_slots = Vec<SlotDefinition>;
type union_of = Vec<Element>;
type tree_root = bool;
type unique_keys = Vec<UniqueKey>;
type unique_key_name = String;
type consider_nulls_inequal = bool;
type unique_key_slots = Vec<SlotDefinition>;
type slot_names_unique = bool;
type domain = ClassDefinition;
type range = Element;
type slot_uri = uriorcurie;
type multivalued = bool;
type array = ArrayExpression;
type dimensions = Vec<DimensionExpression>;
type minimum_number_dimensions = isize;
type maximum_number_dimensions = Anything;
type exact_number_dimensions = isize;
type inherited = bool;
type readonly = String;
type ifabsent = String;
type implicit_prefix = String;
type value_specification_constant = String;
type list_value_specification_constant = String;
type value_presence = String;
type equals_string = String;
type equals_number = isize;
type equals_expression = String;
type exact_cardinality = isize;
type minimum_cardinality = isize;
type maximum_cardinality = isize;
type equals_string_in = Vec<String>;
type equals_number_in = Vec<isize>;
type has_member = AnonymousSlotExpression;
type all_members = AnonymousSlotExpression;
type singular_name = String;
type required = bool;
type recommended = bool;
type inapplicable = bool;
type inlined = bool;
type inlined_as_list = bool;
type inlined_as_simple_dict = bool;
type list_elements_ordered = bool;
type list_elements_unique = bool;
type shared = bool;
type key = bool;
type identifier = bool;
type designates_type = bool;
type alias = String;
type owner = Definition;
type domain_of = Vec<ClassDefinition>;
type is_usage_slot = bool;
type usage_slot_name = String;
type subproperty_of = SlotDefinition;
type disjoint_with = Vec<Definition>;
type children_are_mutually_disjoint = bool;
type relational_logical_characteristic = bool;
type symmetric = bool;
type asymmetric = bool;
type reflexive = bool;
type irreflexive = bool;
type locally_reflexive = bool;
type transitive = bool;
type transitive_form_of = SlotDefinition;
type reflexive_transitive_form_of = SlotDefinition;
type inverse = SlotDefinition;
type is_class_field = bool;
type role = String;
type minimum_value = Anything;
type maximum_value = Anything;
type interpolated = bool;
type partial_match = bool;
type pattern = String;
type syntax = String;
type structured_pattern = PatternExpression;
type string_serialization = String;
type bindings = Vec<EnumBinding>;
type binds_value_of = String;
type obligation_level = String;
type type_mappings = Vec<TypeMapping>;
type framework_key = String;
type mapped_type = TypeDefinition;
type typeof_ = TypeDefinition;
type base = String;
type type_uri = uriorcurie;
type repr = String;
type alt_description_text = String;
type alt_description_source = String;
type alt_descriptions = Vec<AltDescription>;
type value = String;
type value_description = String;
type value_object = Anything;
type examples = Vec<Example>;
type prefix_prefix = ncname;
type prefix_reference = uri;
type prefixes = Vec<Prefix>;
type setting_key = ncname;
type setting_value = String;
type settings = Vec<Setting>;
type import_from = uriorcurie;
type import_as = ncname;
type import_map = Vec<Setting>;
type local_name_source = ncname;
type local_name_value = String;
type local_names = Vec<LocalName>;
type slot_group = SlotDefinition;
type is_grouping_slot = bool;
type followed_by = Expression;
type reversed = bool;
type traverse = SlotDefinition;
type path_rule = PathExpression;
type represents_relationship = bool;
type relational_role = String;

// Enums

#[derive(Serialize, Deserialize)]
pub enum PvFormulaOptions {
    CODE,
    CURIE,
    URI,
    FHIRCODING,
    LABEL,
}
#[derive(Serialize, Deserialize)]
pub enum PresenceEnum {
    UNCOMMITTED,
    PRESENT,
    ABSENT,
}
#[derive(Serialize, Deserialize)]
pub enum RelationalRoleEnum {
    SUBJECT,
    OBJECT,
    PREDICATE,
    NODE,
    OTHERROLE,
}
#[derive(Serialize, Deserialize)]
pub enum AliasPredicateEnum {
    EXACTSYNONYM,
    RELATEDSYNONYM,
    BROADSYNONYM,
    NARROWSYNONYM,
}
#[derive(Serialize, Deserialize)]
pub enum ObligationLevelEnum {
    REQUIRED,
    RECOMMENDED,
    OPTIONAL,
    EXAMPLE,
    DISCOURAGED,
}

// Classes

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AnyValue {
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Extension {
    pub extension_tag: uriorcurie,
    pub extension_value: AnyValue,
    pub extensions: HashMap<String, Extension>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Extensible {
    pub extensions: HashMap<String, Extension>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Annotatable {
    pub annotations: HashMap<String, Annotation>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Annotation {
    pub annotations: HashMap<String, Annotation>,
    pub extension_tag: uriorcurie,
    pub extension_value: AnyValue,
    pub extensions: HashMap<String, Extension>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct UnitOfMeasure {
    pub symbol: String,
    pub abbreviation: String,
    pub descriptive_name: String,
    pub exact_mappings: Vec<uriorcurie>,
    pub ucum_code: String,
    pub derivation: String,
    pub has_quantity_kind: uriorcurie,
    pub iec61360code: String
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Anything {
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct CommonMetadata {
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub id_prefixes: Vec<ncname>,
    pub id_prefixes_are_closed: bool,
    pub definition_uri: uriorcurie,
    pub local_names: HashMap<String, LocalName>,
    pub conforms_to: String,
    pub implements: Vec<uriorcurie>,
    pub instantiates: Vec<uriorcurie>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct SchemaDefinition {
    pub id: uri,
    pub version: String,
    pub imports: Vec<uriorcurie>,
    pub license: String,
    pub prefixes: HashMap<String, Prefix>,
    pub emit_prefixes: Vec<ncname>,
    pub default_curi_maps: Vec<String>,
    pub default_prefix: String,
    pub default_range: TypeDefinition,
    pub subsets: HashMap<String, SubsetDefinition>,
    pub types: HashMap<String, TypeDefinition>,
    pub enums: HashMap<String, EnumDefinition>,
    pub slot_definitions: HashMap<String, SlotDefinition>,
    pub classes: HashMap<String, ClassDefinition>,
    pub metamodel_version: String,
    pub source_file: String,
    pub source_file_date: String,
    pub source_file_size: isize,
    pub generation_date: String,
    pub slot_names_unique: bool,
    pub settings: HashMap<String, Setting>,
    pub bindings: HashMap<String, EnumBinding>,
    pub name: ncname,
    pub id_prefixes: Vec<ncname>,
    pub id_prefixes_are_closed: bool,
    pub definition_uri: uriorcurie,
    pub local_names: HashMap<String, LocalName>,
    pub conforms_to: String,
    pub implements: Vec<uriorcurie>,
    pub instantiates: Vec<uriorcurie>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AnonymousTypeExpression {
    pub pattern: String,
    pub structured_pattern: PatternExpression,
    pub unit: UnitOfMeasure,
    pub implicit_prefix: String,
    pub equals_string: String,
    pub equals_string_in: Vec<String>,
    pub equals_number: isize,
    pub minimum_value: Anything,
    pub maximum_value: Anything,
    pub none_of: Vec<AnonymousTypeExpression>,
    pub exactly_one_of: Vec<AnonymousTypeExpression>,
    pub any_of: Vec<AnonymousTypeExpression>,
    pub all_of: Vec<AnonymousTypeExpression>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct TypeDefinition {
    pub typeof_: Box<TypeDefinition>,
    pub base: String,
    pub type_uri: uriorcurie,
    pub repr: String,
    pub union_of: Vec<TypeDefinition>,
    pub pattern: String,
    pub structured_pattern: PatternExpression,
    pub unit: UnitOfMeasure,
    pub implicit_prefix: String,
    pub equals_string: String,
    pub equals_string_in: Vec<String>,
    pub equals_number: isize,
    pub minimum_value: Anything,
    pub maximum_value: Anything,
    pub none_of: Vec<AnonymousTypeExpression>,
    pub exactly_one_of: Vec<AnonymousTypeExpression>,
    pub any_of: Vec<AnonymousTypeExpression>,
    pub all_of: Vec<AnonymousTypeExpression>,
    pub name: String,
    pub id_prefixes: Vec<ncname>,
    pub id_prefixes_are_closed: bool,
    pub definition_uri: uriorcurie,
    pub local_names: HashMap<String, LocalName>,
    pub conforms_to: String,
    pub implements: Vec<uriorcurie>,
    pub instantiates: Vec<uriorcurie>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct SubsetDefinition {
    pub name: String,
    pub id_prefixes: Vec<ncname>,
    pub id_prefixes_are_closed: bool,
    pub definition_uri: uriorcurie,
    pub local_names: HashMap<String, LocalName>,
    pub conforms_to: String,
    pub implements: Vec<uriorcurie>,
    pub instantiates: Vec<uriorcurie>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Definition {
    pub is_a: Box<Definition>,
    pub abstract_: bool,
    pub mixin: bool,
    pub mixins: Vec<Definition>,
    pub apply_to: Vec<Definition>,
    pub values_from: Vec<uriorcurie>,
    pub string_serialization: String,
    pub name: String,
    pub id_prefixes: Vec<ncname>,
    pub id_prefixes_are_closed: bool,
    pub definition_uri: uriorcurie,
    pub local_names: HashMap<String, LocalName>,
    pub conforms_to: String,
    pub implements: Vec<uriorcurie>,
    pub instantiates: Vec<uriorcurie>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AnonymousEnumExpression {
    pub code_set: uriorcurie,
    pub code_set_tag: String,
    pub code_set_version: String,
    pub pv_formula: String,
    pub permissible_values: HashMap<String, PermissibleValue>,
    pub include: Vec<AnonymousEnumExpression>,
    pub minus: Vec<AnonymousEnumExpression>,
    pub inherits: Vec<EnumDefinition>,
    pub reachable_from: ReachabilityQuery,
    pub matches: MatchQuery,
    pub concepts: Vec<uriorcurie>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct EnumDefinition {
    pub enum_uri: uriorcurie,
    pub code_set: uriorcurie,
    pub code_set_tag: String,
    pub code_set_version: String,
    pub pv_formula: String,
    pub permissible_values: HashMap<String, PermissibleValue>,
    pub include: Vec<AnonymousEnumExpression>,
    pub minus: Vec<AnonymousEnumExpression>,
    pub inherits: Vec<EnumDefinition>,
    pub reachable_from: ReachabilityQuery,
    pub matches: MatchQuery,
    pub concepts: Vec<uriorcurie>,
    pub is_a: Definition,
    pub abstract_: bool,
    pub mixin: bool,
    pub mixins: Vec<Definition>,
    pub apply_to: Vec<Definition>,
    pub values_from: Vec<uriorcurie>,
    pub string_serialization: String,
    pub name: String,
    pub id_prefixes: Vec<ncname>,
    pub id_prefixes_are_closed: bool,
    pub definition_uri: uriorcurie,
    pub local_names: HashMap<String, LocalName>,
    pub conforms_to: String,
    pub implements: Vec<uriorcurie>,
    pub instantiates: Vec<uriorcurie>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct EnumBinding {
    pub range: EnumDefinition,
    pub obligation_level: String,
    pub binds_value_of: String,
    pub pv_formula: String,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct MatchQuery {
    pub identifier_pattern: String,
    pub source_ontology: uriorcurie
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ReachabilityQuery {
    pub source_ontology: uriorcurie,
    pub source_nodes: Vec<uriorcurie>,
    pub relationship_types: Vec<uriorcurie>,
    pub is_direct: bool,
    pub include_self: bool,
    pub traverse_up: bool
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct StructuredAlias {
    pub literal_form: String,
    pub alias_predicate: String,
    pub categories: Vec<uriorcurie>,
    pub alias_contexts: Vec<uri>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Expression {
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct TypeExpression {
    pub pattern: String,
    pub structured_pattern: PatternExpression,
    pub unit: UnitOfMeasure,
    pub implicit_prefix: String,
    pub equals_string: String,
    pub equals_string_in: Vec<String>,
    pub equals_number: isize,
    pub minimum_value: Anything,
    pub maximum_value: Anything,
    pub none_of: Vec<AnonymousTypeExpression>,
    pub exactly_one_of: Vec<AnonymousTypeExpression>,
    pub any_of: Vec<AnonymousTypeExpression>,
    pub all_of: Vec<AnonymousTypeExpression>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct EnumExpression {
    pub code_set: uriorcurie,
    pub code_set_tag: String,
    pub code_set_version: String,
    pub pv_formula: String,
    pub permissible_values: HashMap<String, PermissibleValue>,
    pub include: Vec<AnonymousEnumExpression>,
    pub minus: Vec<AnonymousEnumExpression>,
    pub inherits: Vec<EnumDefinition>,
    pub reachable_from: ReachabilityQuery,
    pub matches: MatchQuery,
    pub concepts: Vec<uriorcurie>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AnonymousExpression {
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct PathExpression {
    pub followed_by: Box<PathExpression>,
    pub none_of: Vec<PathExpression>,
    pub any_of: Vec<PathExpression>,
    pub all_of: Vec<PathExpression>,
    pub exactly_one_of: Vec<PathExpression>,
    pub reversed: bool,
    pub traverse: Box<SlotDefinition>,
    pub range_expression: AnonymousClassExpression,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct SlotExpression {
    pub range: Element,
    pub range_expression: AnonymousClassExpression,
    pub enum_range: EnumExpression,
    pub bindings: HashMap<String, EnumBinding>,
    pub required: bool,
    pub recommended: bool,
    pub multivalued: bool,
    pub inlined: bool,
    pub inlined_as_list: bool,
    pub minimum_value: Anything,
    pub maximum_value: Anything,
    pub pattern: String,
    pub structured_pattern: PatternExpression,
    pub unit: UnitOfMeasure,
    pub implicit_prefix: String,
    pub value_presence: String,
    pub equals_string: String,
    pub equals_string_in: Vec<String>,
    pub equals_number: isize,
    pub equals_expression: String,
    pub exact_cardinality: isize,
    pub minimum_cardinality: isize,
    pub maximum_cardinality: isize,
    pub has_member: AnonymousSlotExpression,
    pub all_members: AnonymousSlotExpression,
    pub none_of: Vec<AnonymousSlotExpression>,
    pub exactly_one_of: Vec<AnonymousSlotExpression>,
    pub any_of: Vec<AnonymousSlotExpression>,
    pub all_of: Vec<AnonymousSlotExpression>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AnonymousSlotExpression {
    pub range: Element,
    pub range_expression: AnonymousClassExpression,
    pub enum_range: EnumExpression,
    pub bindings: HashMap<String, EnumBinding>,
    pub required: bool,
    pub recommended: bool,
    pub multivalued: bool,
    pub inlined: bool,
    pub inlined_as_list: bool,
    pub minimum_value: Anything,
    pub maximum_value: Anything,
    pub pattern: String,
    pub structured_pattern: PatternExpression,
    pub unit: UnitOfMeasure,
    pub implicit_prefix: String,
    pub value_presence: String,
    pub equals_string: String,
    pub equals_string_in: Vec<String>,
    pub equals_number: isize,
    pub equals_expression: String,
    pub exact_cardinality: isize,
    pub minimum_cardinality: isize,
    pub maximum_cardinality: isize,
    pub has_member: Box<AnonymousSlotExpression>,
    pub all_members: Box<AnonymousSlotExpression>,
    pub none_of: Vec<AnonymousSlotExpression>,
    pub exactly_one_of: Vec<AnonymousSlotExpression>,
    pub any_of: Vec<AnonymousSlotExpression>,
    pub all_of: Vec<AnonymousSlotExpression>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct SlotDefinition {
    pub singular_name: String,
    pub domain: Box<ClassDefinition>,
    pub slot_uri: uriorcurie,
    pub array: ArrayExpression,
    pub inherited: bool,
    pub readonly: String,
    pub ifabsent: String,
    pub list_elements_unique: bool,
    pub list_elements_ordered: bool,
    pub shared: bool,
    pub key: bool,
    pub identifier: bool,
    pub designates_type: bool,
    pub alias: String,
    pub owner: Definition,
    pub domain_of: Vec<ClassDefinition>,
    pub subproperty_of: Box<SlotDefinition>,
    pub symmetric: bool,
    pub reflexive: bool,
    pub locally_reflexive: bool,
    pub irreflexive: bool,
    pub asymmetric: bool,
    pub transitive: bool,
    pub inverse: Box<SlotDefinition>,
    pub is_class_field: bool,
    pub transitive_form_of: Box<SlotDefinition>,
    pub reflexive_transitive_form_of: Box<SlotDefinition>,
    pub role: String,
    pub is_usage_slot: bool,
    pub usage_slot_name: String,
    pub relational_role: String,
    pub slot_group: Box<SlotDefinition>,
    pub is_grouping_slot: bool,
    pub path_rule: Box<PathExpression>,
    pub disjoint_with: Vec<SlotDefinition>,
    pub children_are_mutually_disjoint: bool,
    pub union_of: Vec<SlotDefinition>,
    pub type_mappings: Vec<TypeMapping>,
    pub range: Element,
    pub range_expression: Box<AnonymousClassExpression>,
    pub enum_range: EnumExpression,
    pub bindings: HashMap<String, EnumBinding>,
    pub required: bool,
    pub recommended: bool,
    pub multivalued: bool,
    pub inlined: bool,
    pub inlined_as_list: bool,
    pub minimum_value: Anything,
    pub maximum_value: Anything,
    pub pattern: String,
    pub structured_pattern: PatternExpression,
    pub unit: UnitOfMeasure,
    pub implicit_prefix: String,
    pub value_presence: String,
    pub equals_string: String,
    pub equals_string_in: Vec<String>,
    pub equals_number: isize,
    pub equals_expression: String,
    pub exact_cardinality: isize,
    pub minimum_cardinality: isize,
    pub maximum_cardinality: isize,
    pub has_member: AnonymousSlotExpression,
    pub all_members: AnonymousSlotExpression,
    pub none_of: Vec<AnonymousSlotExpression>,
    pub exactly_one_of: Vec<AnonymousSlotExpression>,
    pub any_of: Vec<AnonymousSlotExpression>,
    pub all_of: Vec<AnonymousSlotExpression>,
    pub is_a: Box<SlotDefinition>,
    pub abstract_: bool,
    pub mixin: bool,
    pub mixins: Vec<SlotDefinition>,
    pub apply_to: Vec<SlotDefinition>,
    pub values_from: Vec<uriorcurie>,
    pub string_serialization: String,
    pub name: String,
    pub id_prefixes: Vec<ncname>,
    pub id_prefixes_are_closed: bool,
    pub definition_uri: uriorcurie,
    pub local_names: HashMap<String, LocalName>,
    pub conforms_to: String,
    pub implements: Vec<uriorcurie>,
    pub instantiates: Vec<uriorcurie>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ClassExpression {
    pub any_of: Vec<AnonymousClassExpression>,
    pub exactly_one_of: Vec<AnonymousClassExpression>,
    pub none_of: Vec<AnonymousClassExpression>,
    pub all_of: Vec<AnonymousClassExpression>,
    pub slot_conditions: HashMap<String, SlotDefinition>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AnonymousClassExpression {
    pub is_a: Definition,
    pub any_of: Vec<AnonymousClassExpression>,
    pub exactly_one_of: Vec<AnonymousClassExpression>,
    pub none_of: Vec<AnonymousClassExpression>,
    pub all_of: Vec<AnonymousClassExpression>,
    pub slot_conditions: HashMap<String, SlotDefinition>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ClassDefinition {
    pub slots: Vec<SlotDefinition>,
    pub slot_usage: HashMap<String, SlotDefinition>,
    pub attributes: HashMap<String, SlotDefinition>,
    pub class_uri: uriorcurie,
    pub subclass_of: uriorcurie,
    pub union_of: Vec<ClassDefinition>,
    pub defining_slots: Vec<SlotDefinition>,
    pub tree_root: bool,
    pub unique_keys: HashMap<String, UniqueKey>,
    pub rules: HashMap<String, ClassRule>,
    pub classification_rules: HashMap<String, AnonymousClassExpression>,
    pub slot_names_unique: bool,
    pub represents_relationship: bool,
    pub disjoint_with: Vec<ClassDefinition>,
    pub children_are_mutually_disjoint: bool,
    pub any_of: Vec<AnonymousClassExpression>,
    pub exactly_one_of: Vec<AnonymousClassExpression>,
    pub none_of: Vec<AnonymousClassExpression>,
    pub all_of: Vec<AnonymousClassExpression>,
    pub slot_conditions: HashMap<String, SlotDefinition>,
    pub is_a: Box<ClassDefinition>,
    pub abstract_: bool,
    pub mixin: bool,
    pub mixins: Vec<ClassDefinition>,
    pub apply_to: Vec<ClassDefinition>,
    pub values_from: Vec<uriorcurie>,
    pub string_serialization: String,
    pub name: String,
    pub id_prefixes: Vec<ncname>,
    pub id_prefixes_are_closed: bool,
    pub definition_uri: uriorcurie,
    pub local_names: HashMap<String, LocalName>,
    pub conforms_to: String,
    pub implements: Vec<uriorcurie>,
    pub instantiates: Vec<uriorcurie>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ClassLevelRule {
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ClassRule {
    pub preconditions: AnonymousClassExpression,
    pub postconditions: AnonymousClassExpression,
    pub elseconditions: AnonymousClassExpression,
    pub bidirectional: bool,
    pub open_world: bool,
    pub rank: isize,
    pub deactivated: bool,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ArrayExpression {
    pub exact_number_dimensions: isize,
    pub minimum_number_dimensions: isize,
    pub maximum_number_dimensions: Anything,
    pub dimensions: Vec<DimensionExpression>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct DimensionExpression {
    pub alias: String,
    pub maximum_cardinality: isize,
    pub minimum_cardinality: isize,
    pub exact_cardinality: isize,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct PatternExpression {
    pub syntax: String,
    pub interpolated: bool,
    pub partial_match: bool,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ImportExpression {
    pub import_from: uriorcurie,
    pub import_as: ncname,
    pub import_map: HashMap<String, Setting>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Setting {
    pub setting_key: ncname,
    pub setting_value: String
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Prefix {
    pub prefix_prefix: ncname,
    pub prefix_reference: uri
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct LocalName {
    pub local_name_source: ncname,
    pub local_name_value: String
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Example {
    pub value: String,
    pub value_description: String,
    pub value_object: Anything
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AltDescription {
    pub alt_description_source: String,
    pub alt_description_text: String
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct PermissibleValue {
    pub text: String,
    pub description: String,
    pub meaning: uriorcurie,
    pub unit: UnitOfMeasure,
    pub instantiates: Vec<uriorcurie>,
    pub implements: Vec<uriorcurie>,
    pub is_a: Box<PermissibleValue>,
    pub mixins: Vec<PermissibleValue>,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct UniqueKey {
    pub unique_key_name: String,
    pub unique_key_slots: Vec<SlotDefinition>,
    pub consider_nulls_inequal: bool,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct TypeMapping {
    pub framework_key: String,
    pub mapped_type: TypeDefinition,
    pub string_serialization: String,
    pub extensions: HashMap<String, Extension>,
    pub annotations: HashMap<String, Annotation>,
    pub description: String,
    pub alt_descriptions: HashMap<String, AltDescription>,
    pub title: String,
    pub deprecated: String,
    pub todos: Vec<String>,
    pub notes: Vec<String>,
    pub comments: Vec<String>,
    pub examples: HashMap<String, Example>,
    pub in_subset: Vec<SubsetDefinition>,
    pub from_schema: uri,
    pub imported_from: String,
    pub source: uriorcurie,
    pub in_language: String,
    pub see_also: Vec<uriorcurie>,
    pub deprecated_element_has_exact_replacement: uriorcurie,
    pub deprecated_element_has_possible_replacement: uriorcurie,
    pub aliases: Vec<String>,
    pub structured_aliases: HashMap<String, StructuredAlias>,
    pub mappings: Vec<uriorcurie>,
    pub exact_mappings: Vec<uriorcurie>,
    pub close_mappings: Vec<uriorcurie>,
    pub related_mappings: Vec<uriorcurie>,
    pub narrow_mappings: Vec<uriorcurie>,
    pub broad_mappings: Vec<uriorcurie>,
    pub created_by: uriorcurie,
    pub contributors: Vec<uriorcurie>,
    pub created_on: String,
    pub last_updated_on: String,
    pub modified_by: uriorcurie,
    pub status: uriorcurie,
    pub rank: isize,
    pub categories: Vec<uriorcurie>,
    pub keywords: Vec<String>
}




