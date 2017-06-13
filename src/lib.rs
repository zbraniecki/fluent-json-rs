#![feature(proc_macro)]

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate fluent;

use fluent::syntax::ast;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub body: Vec<Entry>,
    pub comment: Option<Comment>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Entry {
    Message {
        id: Identifier,
        value: Option<Pattern>,
        // attributes: Option<Vec<Attribute>>,
        // tags: Option<Vec<Tag>>,
        // comment: Option<Comment>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pattern {
    pub elements: Vec<PatternElement>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternElement {
    TextElement(String),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    MessageReference { id: String },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub content: String,
}

impl From<ast::Resource> for Resource {
    fn from(r: ast::Resource) -> Resource {
        Resource {
            body: r.body
                .into_iter()
                .map(|e| Entry::from(e))
                .collect::<Vec<_>>(),
            comment: None,
        }
    }
}

impl From<ast::Entry> for Entry {
    fn from(e: ast::Entry) -> Entry {
        match e {
            ast::Entry::Message { id, value, .. } => {
                Entry::Message {
                    id: Identifier::from(id),
                    value: Some(Pattern::from(value.unwrap())),
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl From<ast::Identifier> for Identifier {
    fn from(i: ast::Identifier) -> Identifier {
        Identifier { name: i.name }
    }
}

impl From<ast::Pattern> for Pattern {
    fn from(p: ast::Pattern) -> Pattern {
        Pattern {
            elements: p.elements
                .into_iter()
                .map(|e| PatternElement::from(e))
                .collect::<Vec<_>>(),
        }
    }
}

impl From<ast::PatternElement> for PatternElement {
    fn from(e: ast::PatternElement) -> PatternElement {
        match e {
            ast::PatternElement::TextElement(t) => PatternElement::TextElement(t),
            ast::PatternElement::Expression(p) => PatternElement::Expression(Expression::from(p)),
        }
    }
}

impl From<ast::Expression> for Expression {
    fn from(e: ast::Expression) -> Expression {
        match e {
            ast::Expression::MessageReference { id } => Expression::MessageReference { id: id },
            _ => {
                unimplemented!();
            }
        }
    }
}

pub fn serialize_json(res: &Resource) -> String {
    serde_json::to_string_pretty(res).unwrap()
}
