use std::fs::File;

use mrakdown_engine::{
    engine::MrakdownEngine,
    error_handler::ErrorHandlerImpl,
    parser::ParserImpl,
    schema::HtmlSchema,
    writer::WriterImpl,
};

fn main() {
    println!("Hello, world!");
    let mut engine = MrakdownEngine::new();
    let error_handler = ErrorHandlerImpl::new();
    let writer = WriterImpl::new(File::create("123.html").unwrap());
    let file = File::open("123.txt").unwrap();
    let parser = ParserImpl::new(file);
    engine.start(parser, writer, error_handler, HtmlSchema::new());
}
