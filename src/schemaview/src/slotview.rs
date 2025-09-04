use std::sync::{Arc, OnceLock};

use crate::classview::ClassView;
use crate::identifier::Identifier;
use crate::schemaview::{EnumView, SchemaView};
use linkml_meta::poly::SlotExpression;
use linkml_meta::{SlotDefinition, SlotExpressionOrSubtype};

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
pub struct RangeInfo {
    pub e: SlotExpressionOrSubtype,
    pub slotview: SlotView,
    pub range_class: Option<ClassView>,
    pub range_enum: Option<EnumView>,
    pub is_range_scalar: bool,
    pub slot_container_mode: SlotContainerMode,
    pub slot_inline_mode: SlotInlineMode,
}

impl RangeInfo {
    pub fn new(e: SlotExpressionOrSubtype, slotview: SlotView) -> Self {
        let range_class = Self::determine_range_class(&e, &slotview);
        let range_enum = Self::determine_range_enum(&e, &slotview);
        let is_range_scalar = Self::determine_range_scalar(&range_class);
        let slot_container_mode = Self::determine_slot_container_mode(&range_class, &e);
        let slot_inline_mode = Self::determine_slot_inline_mode(&range_class, &e);
        Self {
            e,
            slotview,
            range_class,
            range_enum,
            is_range_scalar,
            slot_container_mode,
            slot_inline_mode,
        }
    }

    fn determine_range_class(
        e: &SlotExpressionOrSubtype,
        slotview: &SlotView,
    ) -> Option<ClassView> {
        let conv = slotview.sv.converter_for_schema(&slotview.schema_uri)?;
        e.range().and_then(|r| {
            slotview
                .sv
                .get_class(&Identifier::new(r), &conv)
                .ok()
                .flatten()
        })
    }

    fn determine_range_enum(e: &SlotExpressionOrSubtype, slotview: &SlotView) -> Option<EnumView> {
        let conv = slotview.sv.converter_for_schema(&slotview.schema_uri)?;
        e.range().and_then(|r| {
            slotview
                .sv
                .get_enum(&Identifier::new(r), conv)
                .ok()
                .flatten()
        })
    }

    fn determine_range_scalar(range_class: &Option<ClassView>) -> bool {
        // its scalar if its not a class range
        if let Some(cr) = range_class {
            if cr.name() == "Anything" || cr.name() == "AnyValue" {
                return true;
            }
            return false;
        }
        true
    }

    fn determine_slot_container_mode(
        range_class: &Option<ClassView>,
        e: &SlotExpressionOrSubtype,
    ) -> SlotContainerMode {
        let multivalued = e.multivalued().unwrap_or(false);
        if range_class.is_none() {
            return if multivalued {
                SlotContainerMode::List
            } else {
                SlotContainerMode::SingleValue
            };
        }
        if multivalued && e.inlined_as_list().unwrap_or(false) {
            return SlotContainerMode::List;
        }
        let key_slot = range_class
            .as_ref()
            .and_then(|cv| cv.key_or_identifier_slot());
        let identifier_slot = range_class.as_ref().and_then(|cv| cv.identifier_slot());
        let mut inlined = e.inlined().unwrap_or(false);
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

    fn determine_slot_inline_mode(
        range_class: &Option<ClassView>,
        e: &SlotExpressionOrSubtype,
    ) -> SlotInlineMode {
        let multivalued = e.multivalued().unwrap_or(false);

        if range_class.is_none() {
            return SlotInlineMode::Primitive;
        }

        if multivalued && e.inlined_as_list().unwrap_or(false) {
            return SlotInlineMode::Inline;
        }

        let identifier_slot = range_class.as_ref().and_then(|cv| cv.identifier_slot());

        let mut inlined = e.inlined().unwrap_or(false);
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

pub struct SlotViewData {
    pub definitions: Vec<SlotDefinition>,
    cached_definition: OnceLock<SlotDefinition>,
    cached_range_info: OnceLock<Vec<RangeInfo>>,
}

#[derive(Clone)]
pub struct SlotView {
    pub name: String,
    pub(crate) schema_uri: String,
    pub sv: SchemaView,
    data: Arc<SlotViewData>,
}

impl SlotView {
    pub fn new(
        name: String,
        definitions: Vec<SlotDefinition>,
        schema_uri: &str,
        schemaview: &SchemaView,
    ) -> Self {
        Self {
            name,
            schema_uri: schema_uri.to_owned(),
            sv: schemaview.clone(),
            data: Arc::new(SlotViewData {
                definitions: definitions,
                cached_definition: OnceLock::new(),
                cached_range_info: OnceLock::new(),
            }),
        }
    }

    pub fn definition(&self) -> &SlotDefinition {
        self.data.cached_definition.get_or_init(|| {
            let mut b = self.data.definitions[0].clone();
            for d in self.data.definitions.iter().skip(1) {
                b.merge_with(d);
            }
            return b;
        })
    }

    pub fn definitions(&self) -> &Vec<SlotDefinition> {
        &self.data.definitions
    }

    pub fn get_range_info(&self) -> &Vec<RangeInfo> {
        self.data.cached_range_info.get_or_init(|| {
            let def = self.definition();
            if let Some(any_of) = def.any_of.clone() {
                if !any_of.is_empty() {
                    let sv = self.clone();
                    let iter = any_of.clone().into_iter().map(move |expr| -> RangeInfo {
                        RangeInfo::new(
                            SlotExpressionOrSubtype::from(expr.as_ref().clone()),
                            sv.clone(),
                        )
                    });
                    return iter.collect();
                }
            }
            return std::iter::once(RangeInfo::new(
                SlotExpressionOrSubtype::from(def.clone()),
                self.clone(),
            ))
            .collect();
        })
    }

    pub fn get_range_class(&self) -> Option<ClassView> {
        return self
            .get_range_info()
            .first()
            .and_then(|ri| ri.range_class.clone());
    }

    pub fn get_range_enum(&self) -> Option<EnumView> {
        return self
            .get_range_info()
            .first()
            .and_then(|ri| ri.range_enum.clone());
    }

    pub fn is_range_scalar(&self) -> bool {
        return self
            .get_range_info()
            .first()
            .map_or(true, |ri| ri.is_range_scalar);
    }

    pub fn determine_slot_container_mode(&self) -> SlotContainerMode {
        return self
            .get_range_info()
            .first()
            .map_or(SlotContainerMode::SingleValue, |ri| ri.slot_container_mode);
    }

    pub fn determine_slot_inline_mode(&self) -> SlotInlineMode {
        return self
            .get_range_info()
            .first()
            .map_or(SlotInlineMode::Primitive, |ri| ri.slot_inline_mode);
    }
}
