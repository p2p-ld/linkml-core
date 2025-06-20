use curies::Converter;
use linkml_meta::SchemaDefinition;

use linkml_schemaview::schemaview::{ClassView, SchemaView};
use serde_json::Value as JsonValue;
use std::io::{Result as IoResult, Write};

use rio_api::formatter::TriplesFormatter;
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term, Triple};
use rio_turtle::TurtleFormatter;

use crate::LinkMLValue;

pub struct TurtleOptions {
    pub skolem: bool,
}

enum Node {
    Named(String),
    Blank(String),
}

impl Node {
    fn as_subject(&self) -> Subject<'_> {
        match self {
            Node::Named(iri) => Subject::NamedNode(NamedNode { iri }),
            Node::Blank(id) => Subject::BlankNode(BlankNode { id }),
        }
    }

    fn as_term(&self) -> Term<'_> {
        match self {
            Node::Named(iri) => Term::NamedNode(NamedNode { iri }),
            Node::Blank(id) => Term::BlankNode(BlankNode { id }),
        }
    }
}

struct State {
    counter: usize,
    base: String,
    skolem: bool,
    default_prefix: String,
}

impl State {
    fn next_subject(&mut self) -> Node {
        self.counter += 1;
        if self.skolem {
            Node::Named(format!("{}gen{}", self.base, self.counter))
        } else {
            Node::Blank(format!("b{}", self.counter))
        }
    }
}

fn literal_value(v: &JsonValue) -> String {
    match v {
        JsonValue::String(s) => s.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::Bool(b) => b.to_string(),
        _ => v.to_string(),
    }
}

fn serialize_map<W: Write>(
    subject: &Node,
    map: &std::collections::HashMap<String, LinkMLValue>,
    class: Option<&ClassView>,
    formatter: &mut TurtleFormatter<W>,
    sv: &SchemaView,
    conv: &Converter,
    state: &mut State,
) -> IoResult<()> {
    if let Some(cv) = class {
        if let Ok(id) = cv.get_uri(conv, false, true) {
            let id_string = id.to_string();
            let triple = Triple {
                subject: subject.as_subject(),
                predicate: NamedNode {
                    iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
                },
                object: Term::NamedNode(NamedNode { iri: &id_string }),
            };
            formatter.format(&triple)?;
        }
    }
    for (k, v) in map {
        let pred_iri = format!("{}:{}", state.default_prefix, k);
        let predicate = NamedNode { iri: &pred_iri };
        match v {
            LinkMLValue::Scalar { value, .. } => {
                let lit = literal_value(value);
                let object = Term::Literal(Literal::Simple { value: &lit });
                let triple = Triple {
                    subject: subject.as_subject(),
                    predicate,
                    object,
                };
                formatter.format(&triple)?;
            }
            LinkMLValue::Map { values, class, .. } => {
                let class = *class;
                let obj = state.next_subject();
                let triple = Triple {
                    subject: subject.as_subject(),
                    predicate,
                    object: obj.as_term(),
                };
                formatter.format(&triple)?;
                serialize_map(&obj, values, class, formatter, sv, conv, state)?;
            }
            LinkMLValue::List { values, .. } => {
                for item in values {
                    match item {
                        LinkMLValue::Scalar { value, .. } => {
                            let lit = literal_value(value);
                            let object = Term::Literal(Literal::Simple { value: &lit });
                            let triple = Triple {
                                subject: subject.as_subject(),
                                predicate,
                                object,
                            };
                            formatter.format(&triple)?;
                        }
                        LinkMLValue::Map {
                            values: mv, class, ..
                        } => {
                            let class = *class;
                            let obj = state.next_subject();
                            let triple = Triple {
                                subject: subject.as_subject(),
                                predicate,
                                object: obj.as_term(),
                            };
                            formatter.format(&triple)?;
                            serialize_map(&obj, mv, class, formatter, sv, conv, state)?;
                        }
                        LinkMLValue::List { .. } => {}
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn write_turtle<W: Write>(
    value: &LinkMLValue,
    sv: &SchemaView,
    schema: &SchemaDefinition,
    conv: &Converter,
    w: &mut W,
    options: TurtleOptions,
) -> IoResult<()> {
    for (pfx, pref) in &schema.prefixes {
        writeln!(w, "@prefix {}: <{}> .", pfx, pref.prefix_reference)?;
    }
    writeln!(
        w,
        "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> ."
    )?;
    writeln!(w)?;
    let base = schema.id.trim_end_matches('#').to_string();
    let mut state = State {
        counter: 0,
        base: if base.ends_with('/') {
            base
        } else {
            format!("{}/", base)
        },
        skolem: options.skolem,
        default_prefix: schema
            .default_prefix
            .as_deref()
            .unwrap_or(&schema.name)
            .to_string(),
    };
    let mut formatter = TurtleFormatter::new(w);
    match value {
        LinkMLValue::Map { values, class, .. } => {
            let class = *class;
            let subj = Node::Named(format!("{}root", state.base));
            serialize_map(&subj, values, class, &mut formatter, sv, conv, &mut state)?;
        }
        LinkMLValue::List { values, .. } => {
            for item in values {
                let subj = state.next_subject();
                match item {
                    LinkMLValue::Map {
                        values: mv, class, ..
                    } => {
                        let class = *class;
                        serialize_map(&subj, mv, class, &mut formatter, sv, conv, &mut state)?;
                    }
                    LinkMLValue::Scalar { value, .. } => {
                        let lit = literal_value(value);
                        let triple = Triple {
                            subject: subj.as_subject(),
                            predicate: NamedNode {
                                iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#value",
                            },
                            object: Term::Literal(Literal::Simple { value: &lit }),
                        };
                        formatter.format(&triple)?;
                    }
                    LinkMLValue::List { .. } => {}
                }
            }
        }
        LinkMLValue::Scalar { .. } => {}
    }
    formatter.finish()?;
    Ok(())
}

pub fn turtle_to_string(
    value: &LinkMLValue,
    sv: &SchemaView,
    schema: &SchemaDefinition,
    conv: &Converter,
    options: TurtleOptions,
) -> String {
    let mut buf = Vec::new();
    write_turtle(value, sv, schema, conv, &mut buf, options).unwrap();
    String::from_utf8(buf).unwrap()
}
