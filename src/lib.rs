use anyhow::Result;

mod parser;

pub mod types;
pub use types::*;

pub fn from_str(s: &str) -> Result<UI> {
    return todo!()
}

pub fn from_file(path: &str) -> Result<UI> {
    return todo!()
}

pub fn to_str(element: &UI) -> String {
    return todo!()
}

pub fn to_file(element: &UI, path: &str) -> Result<()> {
    return todo!()
}