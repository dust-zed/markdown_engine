use crate::parser::ParserError;

pub trait ErrorHandler {
    fn handle_error(&mut self, e: ParserError);
}

pub struct ErrorHandlerImpl {

}

impl ErrorHandlerImpl {
    
    pub fn new() -> Self {
        Self {  

        }
    }
}

impl ErrorHandler for ErrorHandlerImpl {
    fn handle_error(&mut self, e: ParserError) {
        println!("{:?}", e);
        panic!()
    }
}