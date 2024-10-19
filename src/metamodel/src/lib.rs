#![allow(non_camel_case_types)]
use pyo3::prelude::*;
use pyo3::types::{PyDateTime};


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
type created_on = PyDateTime;
type last_updated_on = PyDateTime;
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
type source_file_date = PyDateTime;
type source_file_size = isize;
type generation_date = PyDateTime;
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

enum PvFormulaOptions {
    CODE,
    CURIE,
    URI,
    FHIRCODING,
    LABEL,
}
enum PresenceEnum {
    UNCOMMITTED,
    PRESENT,
    ABSENT,
}
enum RelationalRoleEnum {
    SUBJECT,
    OBJECT,
    PREDICATE,
    NODE,
    OTHERROLE,
}
enum AliasPredicateEnum {
    EXACTSYNONYM,
    RELATEDSYNONYM,
    BROADSYNONYM,
    NARROWSYNONYM,
}
enum ObligationLevelEnum {
    REQUIRED,
    RECOMMENDED,
    OPTIONAL,
    EXAMPLE,
    DISCOURAGED,
}

// Classes

struct AnyValue {
}


struct Extension {
    extension_tag: uriorcurie,
    extension_value: AnyValue,
    extensions: Vec<Extension>
}


struct Extensible {
    extensions: Vec<Extension>
}


struct Annotatable {
    annotations: Vec<Annotation>
}


struct Annotation {
    annotations: Vec<Annotation>,
    extension_tag: uriorcurie,
    extension_value: AnyValue,
    extensions: Vec<Extension>
}


struct UnitOfMeasure {
    symbol: String,
    abbreviation: String,
    descriptive_name: String,
    exact_mappings: Vec<uriorcurie>,
    ucum_code: String,
    derivation: String,
    has_quantity_kind: uriorcurie,
    iec61360code: String
}


struct Anything {
}


struct CommonMetadata {
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct Element {
    name: String,
    id_prefixes: Vec<ncname>,
    id_prefixes_are_closed: bool,
    definition_uri: uriorcurie,
    local_names: Vec<LocalName>,
    conforms_to: String,
    implements: Vec<uriorcurie>,
    instantiates: Vec<uriorcurie>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct SchemaDefinition {
    id: uri,
    version: String,
    imports: Vec<uriorcurie>,
    license: String,
    prefixes: Vec<Prefix>,
    emit_prefixes: Vec<ncname>,
    default_curi_maps: Vec<String>,
    default_prefix: String,
    default_range: TypeDefinition,
    subsets: Vec<SubsetDefinition>,
    types: Vec<TypeDefinition>,
    enums: Vec<EnumDefinition>,
    slot_definitions: Vec<SlotDefinition>,
    classes: Vec<ClassDefinition>,
    metamodel_version: String,
    source_file: String,
    source_file_date: PyDateTime,
    source_file_size: isize,
    generation_date: PyDateTime,
    slot_names_unique: bool,
    settings: Vec<Setting>,
    bindings: Vec<EnumBinding>,
    name: ncname,
    id_prefixes: Vec<ncname>,
    id_prefixes_are_closed: bool,
    definition_uri: uriorcurie,
    local_names: Vec<LocalName>,
    conforms_to: String,
    implements: Vec<uriorcurie>,
    instantiates: Vec<uriorcurie>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct AnonymousTypeExpression {
    pattern: String,
    structured_pattern: PatternExpression,
    unit: UnitOfMeasure,
    implicit_prefix: String,
    equals_string: String,
    equals_string_in: Vec<String>,
    equals_number: isize,
    minimum_value: Anything,
    maximum_value: Anything,
    none_of: Vec<AnonymousTypeExpression>,
    exactly_one_of: Vec<AnonymousTypeExpression>,
    any_of: Vec<AnonymousTypeExpression>,
    all_of: Vec<AnonymousTypeExpression>
}


struct TypeDefinition {
    typeof_: TypeDefinition,
    base: String,
    type_uri: uriorcurie,
    repr: String,
    union_of: Vec<TypeDefinition>,
    pattern: String,
    structured_pattern: PatternExpression,
    unit: UnitOfMeasure,
    implicit_prefix: String,
    equals_string: String,
    equals_string_in: Vec<String>,
    equals_number: isize,
    minimum_value: Anything,
    maximum_value: Anything,
    none_of: Vec<AnonymousTypeExpression>,
    exactly_one_of: Vec<AnonymousTypeExpression>,
    any_of: Vec<AnonymousTypeExpression>,
    all_of: Vec<AnonymousTypeExpression>,
    name: String,
    id_prefixes: Vec<ncname>,
    id_prefixes_are_closed: bool,
    definition_uri: uriorcurie,
    local_names: Vec<LocalName>,
    conforms_to: String,
    implements: Vec<uriorcurie>,
    instantiates: Vec<uriorcurie>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct SubsetDefinition {
    name: String,
    id_prefixes: Vec<ncname>,
    id_prefixes_are_closed: bool,
    definition_uri: uriorcurie,
    local_names: Vec<LocalName>,
    conforms_to: String,
    implements: Vec<uriorcurie>,
    instantiates: Vec<uriorcurie>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct Definition {
    is_a: Definition,
    abstract_: bool,
    mixin: bool,
    mixins: Vec<Definition>,
    apply_to: Vec<Definition>,
    values_from: Vec<uriorcurie>,
    string_serialization: String,
    name: String,
    id_prefixes: Vec<ncname>,
    id_prefixes_are_closed: bool,
    definition_uri: uriorcurie,
    local_names: Vec<LocalName>,
    conforms_to: String,
    implements: Vec<uriorcurie>,
    instantiates: Vec<uriorcurie>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct AnonymousEnumExpression {
    code_set: uriorcurie,
    code_set_tag: String,
    code_set_version: String,
    pv_formula: String,
    permissible_values: Vec<PermissibleValue>,
    include: Vec<AnonymousEnumExpression>,
    minus: Vec<AnonymousEnumExpression>,
    inherits: Vec<EnumDefinition>,
    reachable_from: ReachabilityQuery,
    matches: MatchQuery,
    concepts: Vec<uriorcurie>
}


struct EnumDefinition {
    enum_uri: uriorcurie,
    code_set: uriorcurie,
    code_set_tag: String,
    code_set_version: String,
    pv_formula: String,
    permissible_values: Vec<PermissibleValue>,
    include: Vec<AnonymousEnumExpression>,
    minus: Vec<AnonymousEnumExpression>,
    inherits: Vec<EnumDefinition>,
    reachable_from: ReachabilityQuery,
    matches: MatchQuery,
    concepts: Vec<uriorcurie>,
    is_a: Definition,
    abstract_: bool,
    mixin: bool,
    mixins: Vec<Definition>,
    apply_to: Vec<Definition>,
    values_from: Vec<uriorcurie>,
    string_serialization: String,
    name: String,
    id_prefixes: Vec<ncname>,
    id_prefixes_are_closed: bool,
    definition_uri: uriorcurie,
    local_names: Vec<LocalName>,
    conforms_to: String,
    implements: Vec<uriorcurie>,
    instantiates: Vec<uriorcurie>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct EnumBinding {
    range: EnumDefinition,
    obligation_level: String,
    binds_value_of: String,
    pv_formula: String,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct MatchQuery {
    identifier_pattern: String,
    source_ontology: uriorcurie
}


struct ReachabilityQuery {
    source_ontology: uriorcurie,
    source_nodes: Vec<uriorcurie>,
    relationship_types: Vec<uriorcurie>,
    is_direct: bool,
    include_self: bool,
    traverse_up: bool
}


struct StructuredAlias {
    literal_form: String,
    alias_predicate: String,
    categories: Vec<uriorcurie>,
    alias_contexts: Vec<uri>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    keywords: Vec<String>
}


struct Expression {
}


struct TypeExpression {
    pattern: String,
    structured_pattern: PatternExpression,
    unit: UnitOfMeasure,
    implicit_prefix: String,
    equals_string: String,
    equals_string_in: Vec<String>,
    equals_number: isize,
    minimum_value: Anything,
    maximum_value: Anything,
    none_of: Vec<AnonymousTypeExpression>,
    exactly_one_of: Vec<AnonymousTypeExpression>,
    any_of: Vec<AnonymousTypeExpression>,
    all_of: Vec<AnonymousTypeExpression>
}


struct EnumExpression {
    code_set: uriorcurie,
    code_set_tag: String,
    code_set_version: String,
    pv_formula: String,
    permissible_values: Vec<PermissibleValue>,
    include: Vec<AnonymousEnumExpression>,
    minus: Vec<AnonymousEnumExpression>,
    inherits: Vec<EnumDefinition>,
    reachable_from: ReachabilityQuery,
    matches: MatchQuery,
    concepts: Vec<uriorcurie>
}


struct AnonymousExpression {
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct PathExpression {
    followed_by: PathExpression,
    none_of: Vec<PathExpression>,
    any_of: Vec<PathExpression>,
    all_of: Vec<PathExpression>,
    exactly_one_of: Vec<PathExpression>,
    reversed: bool,
    traverse: SlotDefinition,
    range_expression: AnonymousClassExpression,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct SlotExpression {
    range: Element,
    range_expression: AnonymousClassExpression,
    enum_range: EnumExpression,
    bindings: Vec<EnumBinding>,
    required: bool,
    recommended: bool,
    multivalued: bool,
    inlined: bool,
    inlined_as_list: bool,
    minimum_value: Anything,
    maximum_value: Anything,
    pattern: String,
    structured_pattern: PatternExpression,
    unit: UnitOfMeasure,
    implicit_prefix: String,
    value_presence: String,
    equals_string: String,
    equals_string_in: Vec<String>,
    equals_number: isize,
    equals_expression: String,
    exact_cardinality: isize,
    minimum_cardinality: isize,
    maximum_cardinality: isize,
    has_member: AnonymousSlotExpression,
    all_members: AnonymousSlotExpression,
    none_of: Vec<AnonymousSlotExpression>,
    exactly_one_of: Vec<AnonymousSlotExpression>,
    any_of: Vec<AnonymousSlotExpression>,
    all_of: Vec<AnonymousSlotExpression>
}


struct AnonymousSlotExpression {
    range: Element,
    range_expression: AnonymousClassExpression,
    enum_range: EnumExpression,
    bindings: Vec<EnumBinding>,
    required: bool,
    recommended: bool,
    multivalued: bool,
    inlined: bool,
    inlined_as_list: bool,
    minimum_value: Anything,
    maximum_value: Anything,
    pattern: String,
    structured_pattern: PatternExpression,
    unit: UnitOfMeasure,
    implicit_prefix: String,
    value_presence: String,
    equals_string: String,
    equals_string_in: Vec<String>,
    equals_number: isize,
    equals_expression: String,
    exact_cardinality: isize,
    minimum_cardinality: isize,
    maximum_cardinality: isize,
    has_member: AnonymousSlotExpression,
    all_members: AnonymousSlotExpression,
    none_of: Vec<AnonymousSlotExpression>,
    exactly_one_of: Vec<AnonymousSlotExpression>,
    any_of: Vec<AnonymousSlotExpression>,
    all_of: Vec<AnonymousSlotExpression>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct SlotDefinition {
    singular_name: String,
    domain: ClassDefinition,
    slot_uri: uriorcurie,
    array: ArrayExpression,
    inherited: bool,
    readonly: String,
    ifabsent: String,
    list_elements_unique: bool,
    list_elements_ordered: bool,
    shared: bool,
    key: bool,
    identifier: bool,
    designates_type: bool,
    alias: String,
    owner: Definition,
    domain_of: Vec<ClassDefinition>,
    subproperty_of: SlotDefinition,
    symmetric: bool,
    reflexive: bool,
    locally_reflexive: bool,
    irreflexive: bool,
    asymmetric: bool,
    transitive: bool,
    inverse: SlotDefinition,
    is_class_field: bool,
    transitive_form_of: SlotDefinition,
    reflexive_transitive_form_of: SlotDefinition,
    role: String,
    is_usage_slot: bool,
    usage_slot_name: String,
    relational_role: String,
    slot_group: SlotDefinition,
    is_grouping_slot: bool,
    path_rule: PathExpression,
    disjoint_with: Vec<SlotDefinition>,
    children_are_mutually_disjoint: bool,
    union_of: Vec<SlotDefinition>,
    type_mappings: Vec<TypeMapping>,
    range: Element,
    range_expression: AnonymousClassExpression,
    enum_range: EnumExpression,
    bindings: Vec<EnumBinding>,
    required: bool,
    recommended: bool,
    multivalued: bool,
    inlined: bool,
    inlined_as_list: bool,
    minimum_value: Anything,
    maximum_value: Anything,
    pattern: String,
    structured_pattern: PatternExpression,
    unit: UnitOfMeasure,
    implicit_prefix: String,
    value_presence: String,
    equals_string: String,
    equals_string_in: Vec<String>,
    equals_number: isize,
    equals_expression: String,
    exact_cardinality: isize,
    minimum_cardinality: isize,
    maximum_cardinality: isize,
    has_member: AnonymousSlotExpression,
    all_members: AnonymousSlotExpression,
    none_of: Vec<AnonymousSlotExpression>,
    exactly_one_of: Vec<AnonymousSlotExpression>,
    any_of: Vec<AnonymousSlotExpression>,
    all_of: Vec<AnonymousSlotExpression>,
    is_a: SlotDefinition,
    abstract_: bool,
    mixin: bool,
    mixins: Vec<SlotDefinition>,
    apply_to: Vec<SlotDefinition>,
    values_from: Vec<uriorcurie>,
    string_serialization: String,
    name: String,
    id_prefixes: Vec<ncname>,
    id_prefixes_are_closed: bool,
    definition_uri: uriorcurie,
    local_names: Vec<LocalName>,
    conforms_to: String,
    implements: Vec<uriorcurie>,
    instantiates: Vec<uriorcurie>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct ClassExpression {
    any_of: Vec<AnonymousClassExpression>,
    exactly_one_of: Vec<AnonymousClassExpression>,
    none_of: Vec<AnonymousClassExpression>,
    all_of: Vec<AnonymousClassExpression>,
    slot_conditions: Vec<SlotDefinition>
}


struct AnonymousClassExpression {
    is_a: Definition,
    any_of: Vec<AnonymousClassExpression>,
    exactly_one_of: Vec<AnonymousClassExpression>,
    none_of: Vec<AnonymousClassExpression>,
    all_of: Vec<AnonymousClassExpression>,
    slot_conditions: Vec<SlotDefinition>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct ClassDefinition {
    slots: Vec<SlotDefinition>,
    slot_usage: Vec<SlotDefinition>,
    attributes: Vec<SlotDefinition>,
    class_uri: uriorcurie,
    subclass_of: uriorcurie,
    union_of: Vec<ClassDefinition>,
    defining_slots: Vec<SlotDefinition>,
    tree_root: bool,
    unique_keys: Vec<UniqueKey>,
    rules: Vec<ClassRule>,
    classification_rules: Vec<AnonymousClassExpression>,
    slot_names_unique: bool,
    represents_relationship: bool,
    disjoint_with: Vec<ClassDefinition>,
    children_are_mutually_disjoint: bool,
    any_of: Vec<AnonymousClassExpression>,
    exactly_one_of: Vec<AnonymousClassExpression>,
    none_of: Vec<AnonymousClassExpression>,
    all_of: Vec<AnonymousClassExpression>,
    slot_conditions: Vec<SlotDefinition>,
    is_a: ClassDefinition,
    abstract_: bool,
    mixin: bool,
    mixins: Vec<ClassDefinition>,
    apply_to: Vec<ClassDefinition>,
    values_from: Vec<uriorcurie>,
    string_serialization: String,
    name: String,
    id_prefixes: Vec<ncname>,
    id_prefixes_are_closed: bool,
    definition_uri: uriorcurie,
    local_names: Vec<LocalName>,
    conforms_to: String,
    implements: Vec<uriorcurie>,
    instantiates: Vec<uriorcurie>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct ClassLevelRule {
}


struct ClassRule {
    preconditions: AnonymousClassExpression,
    postconditions: AnonymousClassExpression,
    elseconditions: AnonymousClassExpression,
    bidirectional: bool,
    open_world: bool,
    rank: isize,
    deactivated: bool,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct ArrayExpression {
    exact_number_dimensions: isize,
    minimum_number_dimensions: isize,
    maximum_number_dimensions: Anything,
    dimensions: Vec<DimensionExpression>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct DimensionExpression {
    alias: String,
    maximum_cardinality: isize,
    minimum_cardinality: isize,
    exact_cardinality: isize,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct PatternExpression {
    syntax: String,
    interpolated: bool,
    partial_match: bool,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct ImportExpression {
    import_from: uriorcurie,
    import_as: ncname,
    import_map: Vec<Setting>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct Setting {
    setting_key: ncname,
    setting_value: String
}


struct Prefix {
    prefix_prefix: ncname,
    prefix_reference: uri
}


struct LocalName {
    local_name_source: ncname,
    local_name_value: String
}


struct Example {
    value: String,
    value_description: String,
    value_object: Anything
}


struct AltDescription {
    alt_description_source: String,
    alt_description_text: String
}


struct PermissibleValue {
    text: String,
    description: String,
    meaning: uriorcurie,
    unit: UnitOfMeasure,
    instantiates: Vec<uriorcurie>,
    implements: Vec<uriorcurie>,
    is_a: PermissibleValue,
    mixins: Vec<PermissibleValue>,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct UniqueKey {
    unique_key_name: String,
    unique_key_slots: Vec<SlotDefinition>,
    consider_nulls_inequal: bool,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}


struct TypeMapping {
    framework_key: String,
    mapped_type: TypeDefinition,
    string_serialization: String,
    extensions: Vec<Extension>,
    annotations: Vec<Annotation>,
    description: String,
    alt_descriptions: Vec<AltDescription>,
    title: String,
    deprecated: String,
    todos: Vec<String>,
    notes: Vec<String>,
    comments: Vec<String>,
    examples: Vec<Example>,
    in_subset: Vec<SubsetDefinition>,
    from_schema: uri,
    imported_from: String,
    source: uriorcurie,
    in_language: String,
    see_also: Vec<uriorcurie>,
    deprecated_element_has_exact_replacement: uriorcurie,
    deprecated_element_has_possible_replacement: uriorcurie,
    aliases: Vec<String>,
    structured_aliases: Vec<StructuredAlias>,
    mappings: Vec<uriorcurie>,
    exact_mappings: Vec<uriorcurie>,
    close_mappings: Vec<uriorcurie>,
    related_mappings: Vec<uriorcurie>,
    narrow_mappings: Vec<uriorcurie>,
    broad_mappings: Vec<uriorcurie>,
    created_by: uriorcurie,
    contributors: Vec<uriorcurie>,
    created_on: PyDateTime,
    last_updated_on: PyDateTime,
    modified_by: uriorcurie,
    status: uriorcurie,
    rank: isize,
    categories: Vec<uriorcurie>,
    keywords: Vec<String>
}




