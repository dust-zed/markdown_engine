use std::{io::Read, iter::Peekable};
use std::io::Result as IoResult;
use u8unit::{CharIterator, U8Unit};

pub enum Token {
    //just 1 ~ 6
    Header(u8),
    CR,
    LF,
    CRLF,
    Text(U8Unit),
    TextChunk(Vec<U8Unit>)
}


impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Header(arg0) => f.debug_tuple("Header").field(arg0).finish(),
            Self::CR => write!(f, "CR"),
            Self::LF => write!(f, "LF"),
            Self::CRLF => write!(f, "CRLF"),
            Self::Text(arg0) => write!(f, "{:?}", arg0),
            Self::TextChunk(arg0) => write!(f, "{:?}", arg0)
        }
    }
}

pub struct Tokenizer<R: Read> {
    iter: Peekable<CharIterator<R>>,
    //用于存储‘#’等可能的标识符方便解析
    cache: Vec<U8Unit>
}

impl <R: Read> Tokenizer<R> {
    
    pub fn new(r: R) -> Self {
        Self {
            iter: CharIterator::new(r).peekable(),
            cache: Vec::new()
        }
    }
}

impl<R: Read> Iterator for Tokenizer<R> {
    type Item = IoResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        
        'outer: loop {

            if !self.cache.is_empty() {
                let res = Some(Ok(Token::TextChunk(self.cache.clone())));
                self.cache.clear();
                return res;
            }

            let unit = self.iter.next()?;
            match unit {
                Ok(chr) => {
                    return Some(Ok(match chr {
                        U8Unit::OneByte(byte) => {
                            match byte[0] {
                                b'\n' => Token::LF,
                                b'\r' => match self.iter.peek() {
                                    Some(e) => match e {
                                        Ok(u) => {
                                            if u == b'\n' {
                                                self.iter.next();
                                                Token::CRLF
                                            } else {
                                                Token::CR
                                            }
                                        }
                                        Err(_) => continue
                                    }
                                    None => Token::CR
                                }
                                b'#' => {
                                    let mut head_level = 1;
                                    self.cache.push(chr);
                                    while let Some(res) = self.iter.peek() {
                                        match res {
                                            Ok(res) => {
                                                if res == b'#' {
                                                    head_level += 1;
                                                    self.cache.push(*res);
                                                    self.iter.next();
                                                    // "#" more than 7
                                                    if head_level > 6 {
                                                        continue 'outer;
                                                    }
                                                } else if res == b' ' {
                                                    // #### 解析为header标识
                                                    self.iter.next();
                                                    self.cache.clear();
                                                    break;
                                                } else {
                                                    continue 'outer;
                                                }
                                            }
                                            Err(_) => {

                                            }
                                        }
                                    }
                                    Token::Header(head_level)
                                }
                                _ => {
                                    Token::Text(chr)
                                }
                            }
                        }
                        _ => {
                            Token::Text(chr)
                        }
                    }))
                }
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use std::fs::File;

    use super::*;

    #[test]
    fn tokenizer_iter_works() {
        let tokenizer = Tokenizer::new(File::open("./123.txt").unwrap());

        for token in tokenizer {
            println!("{:?}", token.unwrap())
        }
    }
}