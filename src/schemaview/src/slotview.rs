use std::cell::{OnceCell, Ref, RefCell};
use std::collections::HashSet;


use curies::Converter;
use linkml_meta::{EnumDefinition, SchemaDefinition, SlotDefinition, SlotExpressionOrSubtype};
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
pub struct RangeInfo<'a>{
    pub e: SlotExpressionOrSubtype,
    pub slotview: &'a SlotView<'a>,
}



#[derive(Clone)]
pub struct SlotView<'a> {
    pub name: String,
    pub(crate) schema_uri: &'a str,
    pub definitions: Vec<&'a SlotDefinition>,
    pub(crate) sv: &'a SchemaView,
    pub(crate) schema_definition: &'a SchemaDefinition,
    cached_definition: OnceCell<SlotDefinition>,
}



impl<'a> SlotView<'a> {
    pub fn new(name: String, definitions: Vec<&'a SlotDefinition>, schema_uri: &'a str, schemaview: &'a SchemaView, sdefinition: &'a SchemaDefinition) -> Self {
        Self {
            name,
            schema_uri,
            definitions: definitions,
            sv: schemaview,
            schema_definition: sdefinition,
            cached_definition: OnceCell::new(),
        }
    }

    pub fn definition(&self) -> &SlotDefinition {
        self.cached_definition.get_or_init(|| {
            let mut b = self.definitions[0].clone();
            for d in self.definitions.iter().skip(1) {
                b.merge_with(*d);
            }
            return b;
        })
    }


    pub fn get_range_info(&'a self) -> Box<dyn Iterator<Item = RangeInfo<'a>> + 'a> {
        let def = self.definition();

        if !def.any_of.is_empty() {
            Box::new(
                def.any_of.iter().map(move |expr| RangeInfo {
                    e: SlotExpressionOrSubtype::from(expr.as_ref().clone()),
                    slotview: self,
                })
            )
        } else {
            Box::new(std::iter::once(RangeInfo {
                e: SlotExpressionOrSubtype::from(def.clone()),
                slotview: self,
            }))
        }
    }



    pub fn get_range_class(&self) -> Option<ClassView<'a>> {
        let conv = self.sv.converter_for_schema(self.schema_uri)?;
        self.definition()
            .range
            .as_ref()
            .and_then(|r| self.sv.get_class(&Identifier::new(r), &conv).ok().flatten())
    }

    pub fn get_range_enum(&self) -> Option<EnumDefinition> {
        self.definition()
            .range
            .as_ref()
            .and_then(|r| self.sv.get_enum_definition(&Identifier::new(r)))
    }

    pub fn is_range_scalar(&self) -> bool {
        // its scalar if its not a class range
        let cr = self.get_range_class();
        if let Some(cr) = cr {
            if cr.name() == "Anything" || cr.name() == "AnyValue" {
                return true;
            }
            return false;
        }
        true
    }


    pub fn determine_slot_container_mode(&self) -> SlotContainerMode {
        let conv = self.sv.converter_for_schema(self.schema_uri).unwrap();
        let s = self.definition();
        let multivalued = s.multivalued.unwrap_or(false);
        let class_range = match &s.range {
            Some(r) => self.sv
                .get_class(&Identifier::new(r), &conv)
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
            .and_then(|r| self.sv.get_class(&Identifier::new(r), &conv).ok().flatten());
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

    pub fn determine_slot_inline_mode(&self) -> SlotInlineMode {
        let conv = self.sv.converter_for_schema(self.schema_uri).unwrap();
        let s = self.definition();
        let multivalued = s.multivalued.unwrap_or(false);
        let class_range = match &s.range {
            Some(r) => self.sv
                .get_class(&Identifier::new(r), &conv)
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
            .and_then(|r| self.sv.get_class(&Identifier::new(r), &conv).ok().flatten());
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

    pub fn can_contain_reference_to_class(
        &self,
        cls: &ClassView<'a>,
        sv: &'a SchemaView,
        conv: &Converter,
    ) -> bool {
        let mut classes_to_check = match self.definition().range.to_owned() {
            Some(r) => vec![r],
            None => return false,
        };
        let mut seen = HashSet::new();
        let target = &cls.class.name;

        while let Some(c) = classes_to_check.pop() {
            if !seen.insert(c.to_owned()) {
                continue;
            }
            if let Ok(Some(cv)) = sv.get_class(&Identifier::new(&c), conv) {
                if cv.class.name == *target {
                    return true;
                }
                for slot in cv.slots() {
                    if let Some(r) = &slot.definition().range {
                        if !seen.contains(r) {
                            classes_to_check.push(r.to_owned());
                        }
                    }
                }
            }
        }
        false
    }
}
