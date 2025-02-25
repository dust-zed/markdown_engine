use crate::{error_handler::ErrorHandler, mapper::Mapper, parser::Parser, schema::Schema, writer::Writer};

pub struct MrakdownEngine {

}

impl MrakdownEngine {
    
    pub fn new() -> Self {
        Self {  
            
        }
    }

    pub fn start<P: Parser, S: Schema, W: Writer, H: ErrorHandler>(
        &mut self,
        mut p: P,
        w: W,
        mut h: H,
        s: S
    ) {
        let mapper  = Mapper::new(s, w);
        let parser_res = p.parse_and_write(mapper);

        match parser_res {
            Ok(_) => {
                println!("It's Ok!")
            }

            Err(e) => h.handle_error(e),
        }
    }
}