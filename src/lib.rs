use std::borrow::Cow;

pub mod tokenizer;
pub mod parser;
pub mod writer;
pub mod schema;
pub mod error_handler;
pub mod mapper;
pub mod engine;

pub type CowStr = Cow<'static, str>;