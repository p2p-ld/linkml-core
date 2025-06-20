use std::collections::HashSet;

use curies::Converter;
use linkml_meta::SlotDefinition;

use crate::classview::ClassView;
use crate::identifier::Identifier;
use crate::schemaview::SchemaView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotContainerMode {
    SingleValue,
    Mapping,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotInlineMode {
    Inline,
    Primitive,
    Reference,
}

#[derive(Clone)]
pub struct SlotView<'a> {
    pub name: String,
    pub definitions: Vec<&'a SlotDefinition>,
}

impl<'a> SlotView<'a> {
    pub fn new(name: String, slot: &'a SlotDefinition) -> Self {
        Self {
            name,
            definitions: vec![slot],
        }
    }

    pub fn merged_definition(&self) -> SlotDefinition {
        let mut base = self.definitions[0].clone();
        for d in self.definitions.iter().skip(1) {
            if let Some(v) = &d.range {
                base.range = Some(v.clone());
            }
            if let Some(v) = d.multivalued {
                base.multivalued = Some(v);
            }
            if let Some(v) = d.inlined {
                base.inlined = Some(v);
            }
            if let Some(v) = d.inlined_as_list {
                base.inlined_as_list = Some(v);
            }
            if let Some(v) = d.key {
                base.key = Some(v);
            }
            if let Some(v) = d.identifier {
                base.identifier = Some(v);
            }
        }
        base
    }

    pub fn determine_slot_container_mode(
        &self,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> SlotContainerMode {
        let s = self.merged_definition();
        let multivalued = s.multivalued.unwrap_or(false);
        let class_range = match &s.range {
            Some(r) => sv
                .get_class(&Identifier::new(r), conv)
                .ok()
                .flatten()
                .is_some(),
            None => false,
        };

        if !class_range {
            return if multivalued {
                SlotContainerMode::List
            } else {
                SlotContainerMode::SingleValue
            };
        }

        if multivalued && s.inlined_as_list.unwrap_or(false) {
            return SlotContainerMode::List;
        }

        let range_cv = s
            .range
            .as_ref()
            .and_then(|r| sv.get_class(&Identifier::new(r), conv).ok().flatten());
        let key_slot = range_cv.as_ref().and_then(|cv| cv.key_or_identifier_slot());
        let identifier_slot = range_cv.as_ref().and_then(|cv| cv.identifier_slot());

        let mut inlined = s.inlined.unwrap_or(false);
        if identifier_slot.is_none() {
            inlined = true;
        }

        if !multivalued {
            return SlotContainerMode::SingleValue;
        }

        if !inlined {
            return SlotContainerMode::List;
        }

        if key_slot.is_some() {
            SlotContainerMode::Mapping
        } else {
            SlotContainerMode::List
        }
    }

    pub fn determine_slot_inline_mode(
        &self,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> SlotInlineMode {
        let s = self.merged_definition();
        let multivalued = s.multivalued.unwrap_or(false);
        let class_range = match &s.range {
            Some(r) => sv
                .get_class(&Identifier::new(r), conv)
                .ok()
                .flatten()
                .is_some(),
            None => false,
        };

        if !class_range {
            return SlotInlineMode::Primitive;
        }

        if multivalued && s.inlined_as_list.unwrap_or(false) {
            return SlotInlineMode::Inline;
        }

        let range_cv = s
            .range
            .as_ref()
            .and_then(|r| sv.get_class(&Identifier::new(r), conv).ok().flatten());
        let identifier_slot = range_cv.as_ref().and_then(|cv| cv.identifier_slot());

        let mut inlined = s.inlined.unwrap_or(false);
        if identifier_slot.is_none() {
            inlined = true;
        }

        if !multivalued {
            return if inlined {
                SlotInlineMode::Inline
            } else {
                SlotInlineMode::Reference
            };
        }

        if !inlined {
            SlotInlineMode::Reference
        } else {
            SlotInlineMode::Inline
        }
    }

    pub fn determine_slot_mode(
        &self,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> (SlotContainerMode, SlotInlineMode) {
        (
            self.determine_slot_container_mode(sv, conv),
            self.determine_slot_inline_mode(sv, conv),
        )
    }

    pub fn can_contain_reference_to_class(
        &self,
        cls: &ClassView<'a>,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> bool {
        let mut classes_to_check = match self.merged_definition().range {
            Some(r) => vec![r],
            None => return false,
        };
        let mut seen = HashSet::new();
        let target = &cls.class.name;

        while let Some(c) = classes_to_check.pop() {
            if !seen.insert(c.clone()) {
                continue;
            }
            if let Ok(Some(cv)) = sv.get_class(&Identifier::new(&c), conv) {
                if cv.class.name == *target {
                    return true;
                }
                for slot in cv.slots() {
                    if let Some(r) = &slot.merged_definition().range {
                        if !seen.contains(r) {
                            classes_to_check.push(r.clone());
                        }
                    }
                }
            }
        }
        false
    }
}

