use std::cell::{OnceCell};
use std::collections::HashSet;


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
pub struct RangeInfo<'a>{
    pub e: SlotExpressionOrSubtype,
    pub slotview: &'a SlotView<'a>,
}

impl<'a> RangeInfo<'a> {
    pub fn get_range_class(&self) -> Option<ClassView<'a>> {
        let conv = self.slotview.sv.converter_for_schema(self.slotview.schema_uri)?;
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
        if let Some(any_of) = &def.any_of {
            if !any_of.is_empty() {
                return Box::new(
                    any_of.iter().map(move |expr| RangeInfo {
                        e: SlotExpressionOrSubtype::from(expr.as_ref().clone()),
                        slotview: self,
                    })
                );
            }
        }
        return Box::new(std::iter::once(RangeInfo {
            e: SlotExpressionOrSubtype::from(def.clone()),
            slotview: self,
        }));

    }



    pub fn get_range_class(&'a self) -> Option<ClassView<'a>> {
        return self.get_range_info().next().and_then(|ri| ri.get_range_class());
    }

    pub fn get_range_enum(&self) -> Option<EnumDefinition> {
        return self.get_range_info().next().and_then(|ri| ri.get_range_enum());
    }

    pub fn is_range_scalar(&self) -> bool {
        return self.get_range_info().next().map_or(true, |ri| ri.is_range_scalar());
    }


    pub fn determine_slot_container_mode(&self) -> SlotContainerMode {
        return self.get_range_info().next().map_or(SlotContainerMode::SingleValue, |ri| ri.determine_slot_container_mode());
    }

    pub fn determine_slot_inline_mode(&self) -> SlotInlineMode {
        return self.get_range_info().next().map_or(SlotInlineMode::Primitive, |ri| ri.determine_slot_inline_mode());
    }

}
