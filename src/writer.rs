use std::io::{BufWriter, Result as IoResult, Write};
pub trait Writer {
    fn flush(&mut self) -> IoResult<()>;
    fn write(&mut self, data: &[u8]) -> IoResult<usize>;
}

pub struct WriterImpl<W: Write> {
    buf_writer: BufWriter<W>,
}

impl<W: Write> WriterImpl<W> {
    pub fn new(w: W) -> Self {
        Self { 
            buf_writer: BufWriter::new(w)
        }
    }
}

impl<W: Write> Writer for WriterImpl<W> {
    fn write(&mut self, data: &[u8]) -> IoResult<usize> {
        self.buf_writer.write(data)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.buf_writer.flush()
    }
}