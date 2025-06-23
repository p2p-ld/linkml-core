use std::collections::{HashMap, HashSet};

use curies::Converter;
use linkml_meta::{ClassDefinition, SlotDefinition};

use crate::identifier::{Identifier, IdentifierError};
use crate::schemaview::SchemaView;
use crate::slotview::SlotView;

#[derive(Clone)]
pub struct ClassView<'a> {
    pub class: &'a ClassDefinition,
    pub(crate) slots: Vec<SlotView<'a>>,
    pub(crate) schema_uri: &'a str,
    pub(crate) sv: &'a SchemaView,
}

impl<'a> ClassView<'a> {
    pub fn new(
        class: &'a ClassDefinition,
        sv: &'a SchemaView,
        schema_uri: &'a str,
        conv: &Converter,
    ) -> Result<Self, IdentifierError> {
        fn gather<'b>(
            class_def: &'b ClassDefinition,
            sv: &'b SchemaView,
            conv: &Converter,
            visited: &mut HashSet<String>,
            acc: &mut HashMap<String, SlotView<'b>>,
        ) -> Result<(), IdentifierError> {
            if !visited.insert(class_def.name.clone()) {
                return Ok(());
            }

            if let Some(parent) = &class_def.is_a {
                if let Some(cv) = sv.get_class(&Identifier::new(parent), conv)? {
                    gather(cv.class, sv, conv, visited, acc)?;
                }
            }
            for mixin in &class_def.mixins {
                if let Some(cv) = sv.get_class(&Identifier::new(mixin), conv)? {
                    gather(cv.class, sv, conv, visited, acc)?;
                }
            }

            for slot_ref in &class_def.slots {
                let mut defs: Vec<&'b SlotDefinition> = Vec::new();
                if let Some(base) = sv.get_slot(&Identifier::new(slot_ref), conv)? {
                    defs.extend(base.definitions);
                }
                if let Some(usage) = class_def.slot_usage.get(slot_ref) {
                    defs.push(usage);
                }
                acc.insert(
                    slot_ref.clone(),
                    SlotView {
                        name: slot_ref.clone(),
                        definitions: defs,
                    },
                );
            }

            for (attr_name, attr_def) in &class_def.attributes {
                let mut defs = vec![attr_def.as_ref()];
                if let Some(usage) = class_def.slot_usage.get(attr_name) {
                    defs.push(usage);
                }
                acc.insert(
                    attr_name.clone(),
                    SlotView {
                        name: attr_name.clone(),
                        definitions: defs,
                    },
                );
            }

            for (usage_name, usage_def) in &class_def.slot_usage {
                if !class_def.slots.contains(usage_name)
                    && !class_def.attributes.contains_key(usage_name)
                {
                    if let Some(existing) = acc.get_mut(usage_name) {
                        existing.definitions.push(usage_def);
                    } else if let Some(base) = sv.get_slot(&Identifier::new(usage_name), conv)? {
                        let mut defs: Vec<&'b SlotDefinition> = base.definitions.to_vec();
                        defs.push(usage_def);
                        acc.insert(
                            usage_name.clone(),
                            SlotView {
                                name: usage_name.clone(),
                                definitions: defs,
                            },
                        );
                    } else {
                        acc.insert(
                            usage_name.clone(),
                            SlotView {
                                name: usage_name.clone(),
                                definitions: vec![usage_def],
                            },
                        );
                    }
                }
            }
            Ok(())
        }

        let mut visited = HashSet::new();
        let mut acc: HashMap<String, SlotView<'a>> = HashMap::new();
        gather(class, sv, conv, &mut visited, &mut acc)?;
        Ok(Self {
            class,
            slots: acc.into_values().collect(),
            schema_uri,
            sv,
        })
    }

    pub fn slots(&self) -> &[SlotView<'a>] {
        &self.slots
    }

    pub fn get_uri(
        &self,
        conv: &Converter,
        native: bool,
        expand: bool,
    ) -> Result<Identifier, IdentifierError> {
        let schema = self
            .sv
            .schema_definitions
            .get(self.schema_uri)
            .ok_or_else(|| IdentifierError::NameNotResolvable)?;
        let default_prefix = schema.default_prefix.as_deref().unwrap_or(&schema.name);
        let base = if native || self.class.class_uri.is_none() {
            format!("{}:{}", default_prefix, self.class.name)
        } else {
            self.class.class_uri.as_ref().unwrap().clone()
        };

        if expand {
            Ok(Identifier::Uri(Identifier::new(&base).to_uri(conv)?))
        } else {
            match Identifier::new(&base) {
                Identifier::Curie(c) => Ok(Identifier::Curie(c)),
                Identifier::Uri(_) => Ok(Identifier::Curie(Identifier::new(&base).to_curie(conv)?)),
                Identifier::Name(_) => Ok(Identifier::Curie(Identifier::new(&base).to_curie(conv)?)),
            }
        }
    }

    pub fn get_type_designator_value(
        &self,
        type_slot: &SlotDefinition,
        conv: &Converter,
    ) -> Result<Identifier, IdentifierError> {
        if let Some(range) = &type_slot.range {
            let slot_types = self.sv.type_ancestors(&Identifier::new(range), conv)?;
            if slot_types.iter().any(|t| t.to_string() == "uri") {
                return self.get_uri(conv, false, true);
            } else if slot_types.iter().any(|t| t.to_string() == "uriorcurie") {
                return self.get_uri(conv, false, false);
            } else if slot_types.iter().any(|t| t.to_string() == "string") {
                return Ok(Identifier::Name(self.class.name.clone()));
            }
        }
        self.get_uri(conv, false, false)
    }

    pub fn get_accepted_type_designator_values(
        &self,
        type_slot: &SlotDefinition,
        conv: &Converter,
    ) -> Result<Vec<Identifier>, IdentifierError> {
        let mut vals = vec![
            self.get_uri(conv, true, true)?,
            self.get_uri(conv, false, true)?,
            self.get_uri(conv, true, false)?,
            self.get_uri(conv, false, false)?,
        ];
        let mut seen = std::collections::HashSet::new();
        vals.retain(|v| seen.insert(v.to_string()));

        if let Some(range) = &type_slot.range {
            let slot_types = self.sv.type_ancestors(&Identifier::new(range), conv)?;
            if slot_types
                .iter()
                .any(|t| t.to_string() == "uri" || t.to_string() == "uriorcurie")
            {
                return Ok(vals);
            } else if range == "string" {
                return Ok(vec![Identifier::Name(self.class.name.clone())]);
            }
        }
        Ok(vals)
    }

    pub fn get_descendants(
        &self,
        conv: &Converter,
        recurse: bool,
    ) -> Result<Vec<ClassView<'a>>, IdentifierError> {
        let mut out = Vec::new();
        for schema in self.sv.schema_definitions.values() {
            for (cls_name, cls_def) in &schema.classes {
                if let Some(parent) = &cls_def.is_a {
                    if let Some(parent_cv) =
                        self.sv.get_class(&Identifier::new(parent), conv)?
                    {
                        if parent_cv.class.name == self.class.name
                            && parent_cv.schema_uri == self.schema_uri
                        {
                            if let Some(child_cv) =
                                self.sv.get_class(&Identifier::new(cls_name), conv)?
                            {
                                if recurse {
                                    out.extend(child_cv.get_descendants(conv, true)?);
                                }
                                out.push(child_cv);
                            }
                        }
                    }
                }
            }
        }
        Ok(out)
    }

    pub fn key_or_identifier_slot(&'a self) -> Option<&'a SlotView<'a>> {
        self.slots.iter().find(|s| {
            let d = s.merged_definition();
            d.identifier.unwrap_or(false) || d.key.unwrap_or(false)
        })
    }

    pub fn identifier_slot(&'a self) -> Option<&'a SlotView<'a>> {
        self.slots
            .iter()
            .find(|s| s.merged_definition().identifier.unwrap_or(false))
    }
}

