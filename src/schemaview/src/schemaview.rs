use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::identifier::{
    converter_from_schema, converter_from_schemas, Identifier, IdentifierError,
};
use curies::Converter;
use linkml_meta::SchemaDefinition;

use crate::curie::curie2uri;
// re-export views from submodules
pub use crate::classview::ClassView;
pub use crate::enumview::EnumView;
pub use crate::slotview::{SlotContainerMode, SlotInlineMode, SlotView};

#[derive(Debug)]
pub enum SchemaViewError {
    IdentifierError(IdentifierError),
    NoPrimarySchema(String),
    NotFound,
    NoConverterForSchema(String),
}

impl From<IdentifierError> for SchemaViewError {
    fn from(err: IdentifierError) -> Self {
        SchemaViewError::IdentifierError(err)
    }
}

#[derive(Clone)]
pub(crate) struct SchemaViewData {
    pub(crate) schema_definitions: HashMap<String, SchemaDefinition>,
    /**
     * A map of all resolved schema imports, having (schema ID where the import was, import name):  imported schema ID
     * So, if schema with ID X imported a schema using the uri ./y.yml that had schema ID Y, this would contain a tuple (X, ./y.yml): Y
     * we need to keep this because sometimes the ID of the imported schema is different from the URL the schema was refered to by the import
     */
    pub(crate) resolved_schema_imports: HashMap<(String, String), String>,
    pub(crate) primary_schema: Option<String>,
    pub(crate) converters: HashMap<String, Converter>,
}

pub(crate) struct SchemaViewCache {
    pub(crate) class_uri_index: HashMap<String, (String, String)>,
    pub(crate) class_name_index: HashMap<String, (String, String)>,
    pub(crate) slot_uri_index: HashMap<String, (String, String)>,
    pub(crate) slot_name_index: HashMap<String, (String, String)>,
    pub(crate) clas_view_cache: HashMap<(String, String), ClassView>,
    pub(crate) enum_uri_index: HashMap<String, (String, String)>,
    pub(crate) enum_name_index: HashMap<String, (String, String)>,
}

impl SchemaViewCache {
    pub fn new() -> Self {
        SchemaViewCache {
            class_uri_index: HashMap::new(),
            class_name_index: HashMap::new(),
            slot_uri_index: HashMap::new(),
            clas_view_cache: HashMap::new(),
            slot_name_index: HashMap::new(),
            enum_uri_index: HashMap::new(),
            enum_name_index: HashMap::new(),
        }
    }
}

impl SchemaViewData {
    pub fn new() -> Self {
        SchemaViewData {
            schema_definitions: HashMap::new(),
            resolved_schema_imports: HashMap::new(),
            primary_schema: None,
            converters: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct SchemaView {
    pub(crate) data: Arc<SchemaViewData>,
    pub(crate) cache: Arc<RwLock<SchemaViewCache>>,
}

impl Default for SchemaView {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaView {
    pub fn new() -> Self {
        SchemaView {
            data: Arc::new(SchemaViewData::new()),
            cache: Arc::new(RwLock::new(SchemaViewCache::new())),
        }
    }

    pub fn _get_resolved_schema_imports(&self) -> HashMap<(String, String), String> {
        // this is a private method to get the resolved schema imports
        // it is used in tests and should not be used in production code
        self.data.resolved_schema_imports.clone()
    }

    fn cache(&self) -> RwLockReadGuard<'_, SchemaViewCache> {
        self.cache.read().expect("SchemaView RwLock poisoned")
    }

    fn write_cache(&self) -> RwLockWriteGuard<'_, SchemaViewCache> {
        self.cache.write().expect("SchemaView RwLock poisoned")
    }

    pub fn get_tree_root_or(&self, class_name: Option<&str>) -> Option<ClassView> {
        let converter = self.converter_for_primary_schema()?;
        // if there is a class_name then its simple!
        if let Some(name) = class_name {
            if let Ok(Some(cv)) = self.get_class(&Identifier::new(name), converter) {
                return Some(cv);
            }
        } else {
            // find a class with tree_root set to true in the primary schema
            if let Some(primary) = &self.data.primary_schema {
                if let Some(schema) = self.data.schema_definitions.get(primary) {
                    if let Some(classes) = &schema.classes {
                        for (name, class_def) in classes {
                            if class_def.tree_root.is_some_and(|x| x) {
                                if let Ok(Some(cv)) =
                                    self.get_class(&Identifier::new(name), converter)
                                {
                                    return Some(cv);
                                }
                            }
                        }
                    }
                }
            }
        }
        None
        // only search in the primary schema for a class having an attribute `tree_root`
    }

    pub fn get_default_prefix_for_schema(&self, schema_id: &str, expand: bool) -> Option<String> {
        let not_expanded = self
            .data
            .schema_definitions
            .get(schema_id)
            .and_then(|s| s.default_prefix.clone());
        let result = if expand {
            let prefixes = self
                .data
                .schema_definitions
                .get(schema_id)
                .and_then(|s| s.prefixes.clone());
            match prefixes {
                Some(prefixes) => {
                    not_expanded.and_then(|n| prefixes.get(&n).map(|p| p.prefix_reference.clone()))
                }
                None => {
                    // if we don't have a converter, just return the not expanded prefix
                    not_expanded
                }
            }
        } else {
            not_expanded
        };
        result
    }

    pub fn get_uri(&self, schema_id: &str, class_name: &str) -> Identifier {
        let res = if class_name.contains(':') {
            Identifier::new(class_name)
        } else {
            let s = self
                .get_default_prefix_for_schema(schema_id, true)
                .unwrap_or(schema_id.to_string());

            let base = format!("{}{}", s, class_name);
            Identifier::new(&base)
        };
        res
    }

    pub fn add_schema(&mut self, schema: SchemaDefinition) -> Result<bool, String> {
        self.add_schema_with_import_ref(schema, None)
    }

    /**
     * Adds a schema to the view with an optional import reference.
     * Import reference is a tuple containing the:
     *    * schema_id of the schema that had the import statement
     *    * the URI of the schema that was imported
     */
    pub fn add_schema_with_import_ref(
        &mut self,
        schema: SchemaDefinition,
        import_reference: Option<(String, String)>,
    ) -> Result<bool, String> {
        let schema_uri = schema.id.clone();
        let conv = converter_from_schema(&schema);
        self.index_schema_classes(&schema_uri, &schema, &conv)
            .map_err(|e| format!("{:?}", e))?;
        self.index_schema_slots(&schema_uri, &schema, &conv)
            .map_err(|e| format!("{:?}", e))?;
        self.index_schema_enums(&schema_uri, &schema, &conv)
            .map_err(|e| format!("{:?}", e))?;
        let d = Arc::make_mut(&mut self.data); // &mut SchemaViewData
        d.converters.insert(schema_uri.to_string(), conv.clone());
        import_reference.map(|x| {
            d.resolved_schema_imports
                .insert((x.0, x.1), schema.id.clone())
        });
        if !d.schema_definitions.contains_key(&schema_uri) {
            d.schema_definitions.insert(schema_uri.to_string(), schema);
            if d.primary_schema.is_none() {
                d.primary_schema = Some(schema_uri.to_string());
            }
            self.write_cache().clas_view_cache.clear();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_schema(&self, id: &str) -> Option<&SchemaDefinition> {
        self.data.schema_definitions.get(id)
    }

    pub fn iter_schemas(&self) -> std::collections::hash_map::Iter<'_, String, SchemaDefinition> {
        self.data.schema_definitions.iter()
    }

    /// Returns a converter built from every schema loaded into this view.
    ///
    /// Note that prefix collisions across schemas will resolve to whichever
    /// expansion appears last; avoid this helper if you expect conflicting
    /// CURIE mappings and instead use `converter_for_schema` with a specific
    /// schema URI.
    pub fn converter(&self) -> Converter {
        converter_from_schemas(self.data.schema_definitions.values())
    }

    pub fn converter_for_schema(&self, schema_uri: &str) -> Option<&Converter> {
        self.data.converters.get(schema_uri)
    }

    pub fn converter_for_primary_schema(&self) -> Option<&Converter> {
        self.data
            .primary_schema
            .as_ref()
            .and_then(|uri| self.converter_for_schema(uri))
    }

    pub fn get_enum_definition(
        &self,
        _identifier: &Identifier,
    ) -> Option<linkml_meta::EnumDefinition> {
        match _identifier {
            Identifier::Name(name) => {
                // try by name (with simple alt names)
                let candidates = vec![name.clone()];
                for n in candidates {
                    if let Some((schema_uri, enum_name)) =
                        self.cache().enum_name_index.get(&n).cloned()
                    {
                        if let Some(schema) = self.data.schema_definitions.get(&schema_uri) {
                            if let Some(enums) = &schema.enums {
                                if let Some(e) = enums.get(&enum_name) {
                                    return Some(e.clone());
                                }
                            }
                        }
                    }
                }
                None
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let conv = self.converter_for_primary_schema()?;
                let target_uri = _identifier.to_uri(conv).ok()?;
                if let Some((schema_uri, enum_name)) =
                    self.cache().enum_uri_index.get(&target_uri.0).cloned()
                {
                    if let Some(schema) = self.data.schema_definitions.get(&schema_uri) {
                        if let Some(enums) = &schema.enums {
                            if let Some(e) = enums.get(&enum_name) {
                                return Some(e.clone());
                            }
                        }
                    }
                }
                None
            }
        }
    }

    fn index_schema_classes(
        &mut self,
        schema_uri: &str,
        schema: &SchemaDefinition,
        conv: &Converter,
    ) -> Result<(), IdentifierError> {
        let default_prefix = schema.default_prefix.as_deref().unwrap_or(&schema.name);
        if let Some(classes) = &schema.classes {
            for (class_name, class_def) in classes {
                let default_id = if class_name.contains(':') && class_def.class_uri.is_none() {
                    Identifier::new(class_name)
                } else {
                    Identifier::new(&format!("{}:{}", default_prefix, class_name))
                };
                let default_uri = default_id.to_uri(conv).map(|u| u.0).unwrap_or_else(|_| {
                    format!("{}/{}", schema.id.trim_end_matches('/'), class_name)
                });
                self.write_cache()
                    .class_name_index
                    .entry(class_name.clone())
                    .or_insert_with(|| (schema_uri.to_string(), class_name.clone()));

                if let Some(curi) = &class_def.class_uri {
                    let explicit_uri = Identifier::new(curi).to_uri(conv)?.0;
                    self.write_cache()
                        .class_uri_index
                        .entry(explicit_uri.clone())
                        .or_insert_with(|| (schema_uri.to_string(), class_name.clone()));
                    if explicit_uri != default_uri {
                        self.write_cache()
                            .class_uri_index
                            .entry(default_uri.clone())
                            .or_insert_with(|| (schema_uri.to_string(), class_name.clone()));
                    }
                } else {
                    self.write_cache()
                        .class_uri_index
                        .entry(default_uri)
                        .or_insert_with(|| (schema_uri.to_string(), class_name.clone()));
                }
            }
        }
        Ok(())
    }

    pub fn all_schema_definitions(&self) -> impl Iterator<Item = (&String, &SchemaDefinition)> {
        self.data.schema_definitions.iter()
    }

    fn index_schema_slots(
        &mut self,
        schema_uri: &str,
        schema: &SchemaDefinition,
        conv: &Converter,
    ) -> Result<(), IdentifierError> {
        let default_prefix = schema.default_prefix.as_deref().unwrap_or(&schema.name);
        if let Some(definitions) = &schema.slot_definitions {
            for (slot_name, slot_def) in definitions {
                let default_id = if slot_name.contains(':') && slot_def.slot_uri.is_none() {
                    Identifier::new(slot_name)
                } else {
                    Identifier::new(&format!("{}:{}", default_prefix, slot_name))
                };
                let default_uri = default_id.to_uri(conv).map(|u| u.0).unwrap_or_else(|_| {
                    format!("{}/{}", schema.id.trim_end_matches('/'), slot_name)
                });
                self.write_cache()
                    .slot_name_index
                    .entry(slot_name.clone())
                    .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                /*if let Some(s) = &slot_def.alias {
                    self.slot_name_index
                        .entry(s.clone())
                        .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                }*/

                if let Some(suri) = &slot_def.slot_uri {
                    let explicit_uri = Identifier::new(suri).to_uri(conv)?.0;
                    self.write_cache()
                        .slot_uri_index
                        .entry(explicit_uri.clone())
                        .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                    if explicit_uri != default_uri {
                        self.write_cache()
                            .slot_uri_index
                            .entry(default_uri.clone())
                            .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                    }
                } else {
                    self.write_cache()
                        .slot_uri_index
                        .entry(default_uri)
                        .or_insert_with(|| (schema_uri.to_string(), slot_name.clone()));
                }
            }
        }
        Ok(())
    }

    fn index_schema_enums(
        &mut self,
        schema_uri: &str,
        schema: &SchemaDefinition,
        conv: &Converter,
    ) -> Result<(), IdentifierError> {
        let default_prefix = schema.default_prefix.as_deref().unwrap_or(&schema.name);
        if let Some(definitions) = &schema.enums {
            for (enum_name, enum_def) in definitions {
                let default_id = if enum_name.contains(':') && enum_def.enum_uri.is_none() {
                    Identifier::new(enum_name)
                } else {
                    Identifier::new(&format!("{}:{}", default_prefix, enum_name))
                };
                let default_uri = default_id.to_uri(conv).map(|u| u.0).unwrap_or_else(|_| {
                    format!("{}/{}", schema.id.trim_end_matches('/'), enum_name)
                });
                self.write_cache()
                    .enum_name_index
                    .entry(enum_name.clone())
                    .or_insert_with(|| (schema_uri.to_string(), enum_name.clone()));

                if let Some(euri) = &enum_def.enum_uri {
                    let explicit_uri = Identifier::new(euri).to_uri(conv)?.0;
                    self.write_cache()
                        .enum_uri_index
                        .entry(explicit_uri.clone())
                        .or_insert_with(|| (schema_uri.to_string(), enum_name.clone()));
                    if explicit_uri != default_uri {
                        self.write_cache()
                            .enum_uri_index
                            .entry(default_uri.clone())
                            .or_insert_with(|| (schema_uri.to_string(), enum_name.clone()));
                    }
                } else {
                    self.write_cache()
                        .enum_uri_index
                        .entry(default_uri)
                        .or_insert_with(|| (schema_uri.to_string(), enum_name.clone()));
                }
            }
        }
        Ok(())
    }

    pub fn get_enum(
        &self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Option<EnumView>, IdentifierError> {
        match id {
            Identifier::Name(name) => {
                if let Some((schema_uri, enum_name)) =
                    self.cache().enum_name_index.get(name).cloned()
                {
                    if let Some(schema) = self.data.schema_definitions.get(&schema_uri) {
                        if let Some(enums) = &schema.enums {
                            if let Some(def) = enums.get(&enum_name) {
                                return Ok(Some(EnumView::new(def, self, &schema.id)));
                            }
                        }
                    }
                }
                Ok(None)
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv)?;
                if let Some((schema_uri, enum_name)) =
                    self.cache().enum_uri_index.get(&target_uri.0).cloned()
                {
                    if let Some(schema) = self.data.schema_definitions.get(&schema_uri) {
                        if let Some(enums) = &schema.enums {
                            if let Some(def) = enums.get(&enum_name) {
                                return Ok(Some(EnumView::new(def, self, &schema.id)));
                            }
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    pub fn get_resolution_uri_of_schema(&self, schema_id: &str) -> Option<String> {
        self.data
            .resolved_schema_imports
            .iter()
            .find_map(|((_, uri), resolved_id)| {
                if resolved_id == schema_id {
                    Some(uri.clone())
                } else {
                    None
                }
            })
    }

    pub fn get_unresolved_schemas(&self) -> Vec<(String, String)> {
        // every schemadefinition has imports. check if an import is not in our list
        let mut unresolved = Vec::new();
        for schema in self.data.schema_definitions.values() {
            if let Some(imports) = &schema.imports {
                for import in imports {
                    let import_uri = curie2uri(import, schema.prefixes.as_ref());
                    match import_uri {
                        Some(uri) => {
                            if !self.data.schema_definitions.contains_key(&uri)
                                && !self
                                    .data
                                    .resolved_schema_imports
                                    .contains_key(&(schema.id.clone(), uri.clone()))
                            {
                                unresolved.push((schema.id.clone(), uri));
                            }
                        }
                        None => {
                            // if the import cannot be expanded to a URI, treat it as a
                            // potential local file path and attempt to resolve later
                            let path = import.to_string();
                            if !self.data.schema_definitions.contains_key(&path)
                                && !self
                                    .data
                                    .resolved_schema_imports
                                    .contains_key(&(schema.id.clone(), path.clone()))
                            {
                                unresolved.push((schema.id.clone(), path));
                            }
                        }
                    }
                }
            }
        }
        unresolved
    }

    pub fn get_schema_definition(&self, id: &str) -> Option<&SchemaDefinition> {
        self.data.schema_definitions.get(id)
    }

    pub fn get_class_by_schema(
        &self,
        schema_uri: &str,
        class_name: &str,
    ) -> Result<Option<ClassView>, SchemaViewError> {
        if let Some(schema_def) = self.data.schema_definitions.get(schema_uri) {
            if let Some(classes) = &schema_def.classes {
                if let Some(class_def) = classes.get(class_name) {
                    // check classview cache and create if needed
                    if let Some(cv) = self
                        .cache()
                        .clas_view_cache
                        .get(&(schema_uri.to_string(), class_name.to_string()))
                    {
                        return Ok(Some(cv.clone()));
                    }
                    // create a new ClassView and cache it
                    let class_view = ClassView::new(
                        class_def,
                        self,
                        schema_uri,
                        schema_def,
                        self.converter_for_schema(schema_uri).ok_or_else(|| {
                            println!("No converter for schema {}", schema_uri);
                            SchemaViewError::NoConverterForSchema(schema_uri.to_string())
                        })?,
                    )?;
                    _ = class_view.get_descendants(true, false);
                    let cvc = class_view.clone();
                    self.write_cache()
                        .clas_view_cache
                        .insert((schema_uri.to_string(), class_name.to_string()), cvc);

                    return Ok(Some(class_view));
                }
            }
        }
        Ok(None)
    }

    pub fn identifier_equals(
        &self,
        id1: &Identifier,
        id2: &Identifier,
        conv: &Converter,
    ) -> Result<bool, IdentifierError> {
        let uri1 = id1.to_uri(conv)?;
        let uri2 = id2.to_uri(conv)?;
        Ok(uri1.0 == uri2.0)
    }

    pub fn exists_class(&self, id: &Identifier, conv: &Converter) -> Result<bool, SchemaViewError> {
        // avoid doing get_class here because this needs to be cheap
        match id {
            Identifier::Name(name) => {
                let lk = self.cache().class_name_index.get(name).cloned();
                if lk.is_some() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv).map_err(SchemaViewError::IdentifierError)?;
                let lk = self
                    .cache()
                    .class_uri_index
                    .get(target_uri.0.as_str())
                    .cloned();
                if lk.is_some() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    pub fn get_classdefinition(
        &self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Option<linkml_meta::ClassDefinition>, SchemaViewError> {
        // avoid using get_class
        match id {
            Identifier::Name(name) => {
                let lk = self.cache().class_name_index.get(name).cloned();
                if let Some((schema, class_name)) = lk {
                    if let Some(schema_def) = self.data.schema_definitions.get(&schema) {
                        if let Some(classes) = &schema_def.classes {
                            return Ok(classes.get(&class_name).cloned());
                        }
                    }
                }
                Ok(None)
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv).map_err(SchemaViewError::IdentifierError)?;
                let lk = self
                    .cache()
                    .class_uri_index
                    .get(target_uri.0.as_str())
                    .cloned();
                if let Some((schema_uri, class_name)) = lk {
                    if let Some(schema_def) = self.data.schema_definitions.get(&schema_uri) {
                        if let Some(classes) = &schema_def.classes {
                            return Ok(classes.get(&class_name).cloned());
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    pub fn get_class(
        &self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Option<ClassView>, SchemaViewError> {
        match id {
            Identifier::Name(name) => {
                let lk = self.cache().class_name_index.get(name).cloned();
                if let Some((schema, class_name)) = lk {
                    return self.get_class_by_schema(&schema, &class_name);
                }
                Ok(None)
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv).map_err(SchemaViewError::IdentifierError)?;
                let lk = self
                    .cache()
                    .class_uri_index
                    .get(target_uri.0.as_str())
                    .cloned();
                if let Some((schema_uri, class_name)) = lk {
                    return self.get_class_by_schema(&schema_uri, &class_name);
                }
                Ok(None)
            }
        }
    }

    pub fn get_class_ids(&self) -> Vec<String> {
        return self
            .cache()
            .class_uri_index
            .keys()
            .cloned()
            .collect::<Vec<String>>();
    }

    pub fn get_slot_ids(&self) -> Vec<String> {
        return self
            .cache()
            .slot_uri_index
            .keys()
            .cloned()
            .collect::<Vec<String>>();
    }

    pub fn get_slot(
        &self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Option<SlotView>, IdentifierError> {
        fn alt_names(name: &str) -> Vec<String> {
            let mut v = Vec::new();
            v.push(name.to_string());
            if name.contains('_') {
                v.push(name.replace('_', " "));
            }
            if name.contains(' ') {
                v.push(name.replace(' ', "_"));
            }
            v
        }
        match id {
            Identifier::Name(name) => {
                let names = alt_names(name);
                for name in names {
                    let index = self.cache().slot_name_index.get(&name).cloned();
                    if let Some((schema, slot_name)) = index {
                        if let Some(schema_def) = self.data.schema_definitions.get(&schema) {
                            if let Some(defs) = &schema_def.slot_definitions {
                                if let Some(slot) = defs.get(&slot_name) {
                                    return Ok(Some(SlotView::new(
                                        slot_name.clone(),
                                        vec![slot.clone()],
                                        &schema_def.id,
                                        self,
                                    )));
                                }
                            }
                        }
                    }
                }
                Ok(None)
            }
            Identifier::Curie(_) | Identifier::Uri(_) => {
                let target_uri = id.to_uri(conv)?;
                let index = self.cache().slot_uri_index.get(&target_uri.0).cloned();
                if let Some((schema_uri, slot_name)) = index {
                    if let Some(schema) = self.data.schema_definitions.get(&schema_uri) {
                        if let Some(defs) = &schema.slot_definitions {
                            if let Some(slot) = defs.get(&slot_name) {
                                return Ok(Some(SlotView::new(
                                    slot_name.clone(),
                                    vec![slot.clone()],
                                    &schema.id,
                                    self,
                                )));
                            }
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    pub fn type_ancestors(
        &self,
        id: &Identifier,
        conv: &Converter,
    ) -> Result<Vec<Identifier>, IdentifierError> {
        fn get_type<'b>(
            sv: &'b SchemaView,
            id: &Identifier,
            conv: &Converter,
        ) -> Result<Option<&'b linkml_meta::TypeDefinition>, IdentifierError> {
            match id {
                Identifier::Name(n) => {
                    for schema in sv.data.schema_definitions.values() {
                        if let Some(types) = &schema.types {
                            if let Some(t) = types.get(n) {
                                return Ok(Some(t));
                            }
                        }
                    }
                    Ok(None)
                }
                Identifier::Curie(_) | Identifier::Uri(_) => {
                    let target_uri = id.to_uri(conv)?;
                    for schema in sv.data.schema_definitions.values() {
                        if let Some(types) = &schema.types {
                            for t in types.values() {
                                if let Some(turi) = &t.type_uri {
                                    if Identifier::new(turi).to_uri(conv)?.0 == target_uri.0 {
                                        return Ok(Some(t));
                                    }
                                }
                            }
                        }
                    }
                    Ok(None)
                }
            }
        }

        let mut out = Vec::new();
        let mut cur = get_type(self, id, conv)?;
        while let Some(t) = cur {
            out.push(Identifier::Name(t.name.clone()));
            if let Some(parent) = &t.typeof_ {
                cur = get_type(self, &Identifier::new(parent), conv)?;
            } else {
                break;
            }
        }
        if out.is_empty() {
            out.push(id.clone());
        }
        Ok(out)
    }

    pub fn primary_schema(&self) -> Option<&SchemaDefinition> {
        match &self.data.primary_schema {
            Some(uri) => self.data.schema_definitions.get(uri),
            None => None,
        }
    }
}
