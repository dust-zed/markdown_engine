use std::io::{Error, Read};

use crate::{mapper::Mapper, schema::Schema, tokenizer::{Token, Tokenizer}, writer::Writer, CowStr};

pub trait Parser {
    fn parse_and_write<S: Schema, W: Writer>(&mut self, mapper: Mapper<S, W>) -> Result<(), ParserError>;
}
#[derive(Debug)]
pub enum ParserError {
    SyntaxError(SyntaxError),
    IoError(Error)
}

#[derive(Debug)]
pub struct SyntaxError {
    msg: CowStr,
    line_num: u32,
    column_num: u32,
}

#[derive(Clone, Copy)]
pub enum State {
    LineStart,
    HeaderMark(u8),
    Paragraph,
    EOF
}

impl State  {
    pub fn is_mark_state(self) -> bool {
        matches!(self, Self::HeaderMark(_))
    }

    pub fn is_eof(self) -> bool {
        matches!(self, Self::EOF)
    }
}

struct Record {
    line_num: u32,
    col_num: u32,
}

pub struct ParserImpl<R: Read> {
    tokenizer: Tokenizer<R>,
    state: State
}

impl <R: Read> Parser for ParserImpl<R> {
    fn parse_and_write<S: Schema, W: Writer>(
        &mut self, 
        mut mapper: Mapper<S, W>
    ) -> Result<(), ParserError> {
        if self.state.is_eof() {
            return Ok(());
        }
        loop {
            match self.tokenizer.next() {
                Some(res) => match res {
                    Ok(t) => {
                        match self.state {
                            State::LineStart => {
                                match t {
                                    Token::Header(level) => {
                                        self.state = State::HeaderMark(level);
                                        mapper.write_html_header_start(level).map_err(ParserError::IoError)?;
                                    }

                                    Token::Text(text) => {
                                        self.state = State::Paragraph;
                                        mapper.write_html_content((&text).into()).map_err(ParserError::IoError)?;
                                    }
                                    Token::TextChunk(chunk) => {
                                        self.state = State::Paragraph;
                                        mapper.write_html_chunk(&chunk).map_err(ParserError::IoError)?;
                                    }

                                    Token::LF | Token::CRLF | Token::CR => {
                                        mapper.write_html_new_line().map_err(ParserError::IoError)?;
                                    }
                                }
                            }
                            State::HeaderMark(level) => {
                                match t {
                                    Token::Text(text) => {
                                        mapper.write_html_content((&text).into()).map_err(ParserError::IoError)?;
                                    }

                                    Token::TextChunk(chunk) => {
                                        mapper.write_html_chunk(&chunk).map_err(ParserError::IoError)?;
                                    }

                                    Token::LF | Token::CRLF | Token::CR => {
                                        self.state = State::LineStart;
                                        mapper.write_html_header_end(level).map_err(ParserError::IoError)?;
                                        mapper.write_html_new_line().map_err(ParserError::IoError)?;
                                    }
                                    Token::Header(level) => {
                                        mapper.write_html_fake_header(level).map_err(ParserError::IoError)?;
                                    }
                                }
                            }
                            State::Paragraph => {
                                match t {
                                    Token::Text(text) => {
                                        mapper.write_html_content((&text).into()).map_err(ParserError::IoError)?;
                                    }

                                    Token::TextChunk(chunk) => {
                                        mapper.write_html_chunk(&chunk).map_err(ParserError::IoError)?;
                                    }

                                    Token::LF | Token::CRLF | Token::CR => {
                                        self.state = State::LineStart;
                                        mapper.write_html_paragraph_end().map_err(ParserError::IoError)?;
                                    }
                                    Token::Header(level) => {
                                        mapper.write_html_fake_header(level).map_err(ParserError::IoError)?;
                                    }
                                }
                            }
                            State::EOF => {
                                mapper.flush().map_err(ParserError::IoError)?;
                                return Ok(());
                            }
                        }
                    }
                    Err(_) => todo!()
                }
                None =>  {
                    self.state = State::EOF;
                    mapper.flush().map_err(ParserError::IoError)?;
                    return Ok(());
                }
            }
        }
    }
}

impl<R: Read> ParserImpl<R> {
    pub fn new(reader: R) -> Self {
        Self { 
            tokenizer: Tokenizer::new(reader), 
            state: State::LineStart
        }
    }
}