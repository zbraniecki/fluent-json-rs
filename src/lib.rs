#![feature(proc_macro)]

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate fluent;

use fluent::syntax::ast;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource(pub Vec<Entry>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Entry {
    Message(Message),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub value: Option<Pattern>,
    pub traits: Option<Vec<Member>>,
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
pub struct Member {
    pub key: String,
    pub value: Pattern,
    pub default: bool,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    MessageReference(Identifier),
}

impl From<ast::Resource> for Resource {
    fn from(r: ast::Resource) -> Resource {
        Resource(r.body
                     .into_iter()
                     .map(|e| Entry::from(e))
                     .collect::<Vec<_>>())
    }
}

impl From<ast::Entry> for Entry {
    fn from(e: ast::Entry) -> Entry {
        match e {
            ast::Entry::Message { id, value, .. } => {
                Entry::Message(Message {
                                   id: id.name,
                                   value: Some(Pattern::from(value.unwrap())),
                                   traits: None,
                               })
            }
            _ => unimplemented!(),
        }
    }
}

impl From<ast::Identifier> for Identifier {
    fn from(i: ast::Identifier) -> Identifier {
        Identifier(String::from("key2"))
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
            ast::Expression::MessageReference { id } => {
                Expression::MessageReference(Identifier(id))
            }
            _ => {
                unimplemented!();
            }
        }
    }
}

pub fn serialize_json(res: &Resource) -> String {
    serde_json::to_string_pretty(res).unwrap()
}
