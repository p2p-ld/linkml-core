use std::collections::btree_map::Range;
use std::collections::HashSet;
use std::sync::OnceLock;


use curies::Converter;
use linkml_meta::{EnumDefinition, SchemaDefinition, SlotDefinition, SlotExpressionOrSubtype};
use linkml_meta::poly::SlotExpression;
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
pub struct RangeInfo{
    pub e: SlotExpressionOrSubtype,
    pub slotview: SlotView,
}

impl RangeInfo {
    pub fn get_range_class(&self) -> Option<ClassView> {
        let conv = self.slotview.sv.converter_for_schema(&self.slotview.schema_uri)?;
        self.e
            .range()
            .and_then(|r| self.slotview.sv.get_class(&Identifier::new(r), &conv).ok().flatten())
    }

    pub fn get_range_enum(&self) -> Option<EnumDefinition> {
        self.e
            .range()
            .and_then(|r| self.slotview.sv.get_enum_definition(&Identifier::new(r)))
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
        let range_class = self.get_range_class();
        let multivalued = self.e.multivalued().unwrap_or(false);
        if range_class.is_none() {
            return if multivalued {
                SlotContainerMode::List
            } else {
                SlotContainerMode::SingleValue
            };
        }
        if multivalued && self.e.inlined_as_list().unwrap_or(false) {
            return SlotContainerMode::List;
        }
        let key_slot = range_class.as_ref().and_then(|cv| cv.key_or_identifier_slot());
        let identifier_slot = range_class.as_ref().and_then(|cv| cv.identifier_slot());
        let mut inlined = self.e.inlined().unwrap_or(false);
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
        let multivalued = self.e.multivalued().unwrap_or(false);
        let range_class = self.get_range_class();

        if range_class.is_none() {
            return SlotInlineMode::Primitive;
        }

        if multivalued && self.e.inlined_as_list().unwrap_or(false) {
            return SlotInlineMode::Inline;
        }

        let identifier_slot = range_class.as_ref().and_then(|cv| cv.identifier_slot());

        let mut inlined = self.e.inlined().unwrap_or(false);
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

    
}


#[derive(Clone)]
pub struct SlotView {
    pub name: String,
    pub(crate) schema_uri: String,
    pub definitions: Vec<SlotDefinition>,
    pub(crate) sv: SchemaView,
    cached_definition: OnceLock<SlotDefinition>,
    cached_range_info: OnceLock<Vec<RangeInfo>>,
}



impl SlotView {
    pub fn new(name: String, definitions: Vec<SlotDefinition>, schema_uri: &str, schemaview: &SchemaView) -> Self {
        Self {
            name,
            schema_uri: schema_uri.to_owned(),
            definitions: definitions,
            sv: schemaview.clone(),
            cached_definition: OnceLock::new(),
            cached_range_info: OnceLock::new(),
        }
    }

    pub fn definition(&self) -> &SlotDefinition {
        self.cached_definition.get_or_init(|| {
            let mut b = self.definitions[0].clone();
            for d in self.definitions.iter().skip(1) {
                b.merge_with(d);
            }
            return b;
        })
    }


    pub fn get_range_info(&self) -> &Vec<RangeInfo> {
        self.cached_range_info.get_or_init(|| {
            let def = self.definition();
            if let Some(any_of) = def.any_of.clone() {
                if !any_of.is_empty() {
                    let sv = self.clone();
                    let iter = any_of.clone().into_iter().map(move |expr| -> RangeInfo {RangeInfo {
                            e: SlotExpressionOrSubtype::from(expr.as_ref().clone()),
                            slotview: sv.clone(),
                        }});
                    return iter.collect();
                }
            }
            return std::iter::once(RangeInfo {
                e: SlotExpressionOrSubtype::from(def.clone()),
                slotview: self.clone(),
            }).collect();

        })

    }



    pub fn get_range_class(&self) -> Option<ClassView> {
        return self.get_range_info().first().and_then(|ri| ri.get_range_class());
    }

    pub fn get_range_enum(&self) -> Option<EnumDefinition> {
        return self.get_range_info().first().and_then(|ri| ri.get_range_enum());
    }

    pub fn is_range_scalar(&self) -> bool {
        return self.get_range_info().first().map_or(true, |ri| ri.is_range_scalar());
    }


    pub fn determine_slot_container_mode(&self) -> SlotContainerMode {
        return self.get_range_info().first().map_or(SlotContainerMode::SingleValue, |ri| ri.determine_slot_container_mode());
    }

    pub fn determine_slot_inline_mode(&self) -> SlotInlineMode {
        return self.get_range_info().first().map_or(SlotInlineMode::Primitive, |ri| ri.determine_slot_inline_mode());
    }

}
