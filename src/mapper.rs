use u8unit::U8Unit;

use crate::{schema::Schema, writer::Writer};
use std::io::Result as IoResult;
pub struct Mapper<S: Schema, W: Writer> {
    schema: S,
    writer: W,
}

impl<S: Schema, W: Writer> Mapper<S, W> {

    pub fn new(s: S, w: W) -> Self {
        Self { 
            schema: s,
            writer: w,
        }
    }



    pub fn write_html_content(&mut self, data: &[u8]) -> IoResult<()> {
        self.writer.write(data)?;
        Ok(())
    }

    pub fn write_html_chunk(&mut self, data: &[U8Unit]) -> IoResult<()> {
        for text in data {
            self.writer.write(text.into())?;
        }
        Ok(())
    }

    pub fn write_html_new_line(&mut self) -> IoResult<()> {
        self.writer.write(b"<br>")?;
        Ok(())
    }

    pub fn write_html_header_start(&mut self, level: u8) -> IoResult<()> {
        match level {
            1 => {
                self.writer.write(S::h1_start().as_bytes())?;
            }
            2 => {
                self.writer.write(S::h2_start().as_bytes())?;
            }
            3 => {
                self.writer.write(S::h3_start().as_bytes())?;
            }
            4 => {
                self.writer.write(S::h4_start().as_bytes())?;
            }
            5 => {
                self.writer.write(S::h5_start().as_bytes())?;
            }
            6 => {
                self.writer.write(S::h6_start().as_bytes())?;
            }
            _ => {

            }
        }
        Ok(())
    }

    pub fn write_html_header_end(&mut self, level: u8) -> IoResult<()> {
        match level {
            1 => {
                self.writer.write(S::h1_end().as_bytes())?;
            }
            2 => {
                self.writer.write(S::h2_end().as_bytes())?;
            }
            3 => {
                self.writer.write(S::h3_end().as_bytes())?;
            }
            4 => {
                self.writer.write(S::h4_end().as_bytes())?;
            }
            5 => {
                self.writer.write(S::h5_end().as_bytes())?;
            }
            6 => {
                self.writer.write(S::h6_end().as_bytes())?;
            }
            _ => {

            }
        }
        Ok(())
    }

    pub fn write_html_paragraph_start(&mut self) -> IoResult<()> {
        self.writer.write(b"<p>")?;
        Ok(())
    }

    pub fn write_html_paragraph_end(&mut self) -> IoResult<()> {
        self.writer.write(b"</p>")?;
        Ok(())
    }

    pub fn write_html_fake_header(&mut self, level: u8) -> IoResult<()> {
        for _ in 0..level {
            self.writer.write(b"#")?;
        }
        self.writer.write(b" ")?;
        Ok(())
    }

    pub fn flush(&mut self) -> IoResult<()> {
        self.writer.flush()
    }
}