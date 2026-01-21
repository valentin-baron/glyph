use anyhow::Result;
use chumsky::prelude::*;

use crate::types::*;

// === AST Types ===
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Percentage(f64),
    Identifier(String),
    DString(String), // d"..." interpolated strings
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub kind: String,        // e.g., "Form", "Panel", "TextInput"
    pub name: String,        // e.g., "main_form", "left_panel"
    pub properties: Vec<Property>,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct Language {
    pub name: String,
    pub value: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub language: Language, // @language ratatui or @language my_lang("url")
    pub root: Element,
}

// === Parser ===
pub fn parser<'a>() -> impl Parser<'a, &'a str, Document, extra::Err<Rich<'a, char>>> {
    let ident = text::ident::<&'a str, extra::Err<Rich<'a, char>>>().padded();
    // Simple directive: @language ratatui
    let simple_directive = just('@')
        .ignore_then(ident)
        .then(ident)
        .map(|(name, value): (&str, &str)| Language {
            name: name.to_string(),
            value: value.to_string(),
            url: None,
        });

    // Directive with URL: @language my_language("https://...")
    let url_string = just('"')
        .ignore_then(none_of('"').repeated().collect::<String>())
        .then_ignore(just('"'));

    let directive_with_url = just('@')
        .ignore_then(ident)
        .then(ident)
        .then(just('(').ignore_then(url_string).then_ignore(just(')')))
        .map(|((name, value), url): ((&str, &str), String)| Language {
            name: name.to_string(),
            value: value.to_string(),
            url: Some(url),
        });

    let directive = directive_with_url
        .or(simple_directive)
        .padded();

    // String literals: "..."
    let string = just('"')
        .ignore_then(none_of('"').repeated().collect::<String>())
        .then_ignore(just('"'))
        .map(Value::String);

    // D-strings: d"..." (multiline with interpolation)
    let dstring = just("d\"")
        .ignore_then(none_of('"').repeated().collect::<String>())
        .then_ignore(just('"'))
        .map(Value::DString);

    // Numbers with optional percentage
    let frac = just('.').then(text::digits::<&str, extra::Err<Rich<'a, char>>>(10)).to_slice();
    let number = text::int::<&str, extra::Err<Rich<'a, char>>>(10)
        .then(frac.or_not())
        .to_slice()
        .then(just('%').or_not())
        .map(|(num_str, pct): (&str, Option<char>)| {
            let num_str = num_str.trim_end_matches('%');
            let num: f64 = num_str.parse().unwrap();
            if pct.is_some() {
                Value::Percentage(num)
            } else {
                Value::Number(num)
            }
        });

    // Identifier values (like: left-to-right, free-form, true)
    let ident_value = text::ident::<&str, extra::Err<Rich<'a, char>>>()
        .then(just('-').ignore_then(text::ident::<&str, extra::Err<Rich<'a, char>>>()).repeated().collect::<Vec<_>>())
        .map(|(first, rest): (&str, Vec<&str>)| {
            let mut s = first.to_string();
            for part in rest {
                s.push('-');
                s.push_str(part);
            }
            Value::Identifier(s)
        });

    let value = choice((dstring, string, number, ident_value)).padded();

    // Property: name = value
    let property = ident
        .then_ignore(just('=').padded())
        .then(value)
        .map(|(name, value): (&str, Value)| Property { name: name.to_string(), value });

    // Recursive element definition
    let element = recursive(|element| {
        let properties_and_children = property.clone()
            .map(Either::Left)
            .or(element.map(Either::Right))
            .padded()
            .repeated()
            .collect::<Vec<_>>();

        // @Kind name { ... } or @Kind name ( ... )
        let block = |open: char, close: char| {
            just('@')
                .ignore_then(ident)
                .then(ident)
                .then(
                    just(open)
                        .padded()
                        .ignore_then(properties_and_children.clone())
                        .then_ignore(just(close).padded())
                )
                .map(|((kind, name), items): ((&str, &str), Vec<Either<Property, Element>>)| {
                    let (props, children): (Vec<_>, Vec<_>) = items
                        .into_iter()
                        .partition(|e| matches!(e, Either::Left(_)));
                    Element {
                        kind: kind.to_string(),
                        name: name.to_string(),
                        properties: props.into_iter().filter_map(|e| e.left()).collect(),
                        children: children.into_iter().filter_map(|e| e.right()).collect(),
                    }
                })
        };

        block('{', '}').or(block('(', ')'))
    });

    // Parse directive first, then the root element
    directive
        .then(element)
        .map(|(language, root)| Document { language, root })
}

enum Either<L, R> { Left(L), Right(R) }
impl<L, R> Either<L, R> {
    fn left(self) -> Option<L> { match self { Either::Left(l) => Some(l), _ => None } }
    fn right(self) -> Option<R> { match self { Either::Right(r) => Some(r), _ => None } }
}