use std::{io::Read, usize};
use std::io::Result as IoResult;

const BUF_SIZE: usize = 512;
pub struct CharIterator<R: Read> {
    r: R,
    buf: [u8; BUF_SIZE],
    cursor: usize,
    cur_len: usize,
    remain: Remain
}
/// utf_8 bytes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum U8Unit {
    OneByte([u8; 1]),
    TwoByte([u8; 2]),
    ThreeByte([u8; 3]),
    FourByte([u8; 4]),
}

impl<'a> From<&'a U8Unit> for &'a [u8] {
    fn from(value: &'a U8Unit) -> Self {
        match value {
            U8Unit::OneByte(chr) => &chr[..],
            U8Unit::TwoByte(chr) => &chr[..],
            U8Unit::ThreeByte(chr) => &chr[..],
            U8Unit::FourByte(chr) => &chr[..]
        }
    }
}

impl PartialEq<u8> for U8Unit{
    fn eq(&self, other: &u8) -> bool {
        match self {
            U8Unit::OneByte(byte) => &byte[0] == other,
            _ => false
        }
    }
}

impl PartialEq<u8> for &U8Unit{
    fn eq(&self, other: &u8) -> bool {
       (*self).eq(other)
    }
}

// 缓冲区剩下的无法表示一个utf_8字符几个字节
struct Remain {
    //remain size
    len: usize,
    // need next round buf append byte size
    append_len: usize,
    //maybe first remain byte
    a: u8,
    //maybe second remain byte
    b: u8,
    //maybe third remain byte
    c: u8
}

impl Remain {
    fn default() -> Self {
        Self { 
            len: 0,
            append_len: 0,
            a: 0u8, 
            b: 0u8, 
            c: 0u8
         }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn clear(&mut self) {
        self.len = 0
    }
}

impl<R: Read> CharIterator<R> {
    
    pub fn new(r: R) -> Self {
        Self { 
            r, 
            buf: [0; BUF_SIZE],
            cursor: 0, 
            cur_len: usize::MAX, 
            remain: Remain::default()
        }
    }

    //buf is satisfied 
    fn cur_u8bytes(&mut self, chr_len: usize) -> U8Unit {

        let chr = match chr_len {
            1 => {
                U8Unit::OneByte([self.get_byte(self.cursor)])
            }
            2 => {
                U8Unit::TwoByte([
                    self.get_byte(self.cursor),
                    self.get_byte(self.cursor + 1)
                ])
            }
            3 => {
                U8Unit::ThreeByte([
                    self.get_byte(self.cursor),
                    self.get_byte(self.cursor + 1),
                    self.get_byte(self.cursor + 2)
                ])
            }
            4 => {
                U8Unit::FourByte([
                    self.get_byte(self.cursor),
                    self.get_byte(self.cursor + 1),
                    self.get_byte(self.cursor + 2),
                    self.get_byte(self.cursor + 3)
                ])
            }
            _ => unreachable!()
        };
        self.cursor += chr_len;
        chr
    }

    fn remain_u8bytes(&mut self) -> U8Unit {
        let chr = match self.remain.len {
            2 => U8Unit::TwoByte([
                self.remain.a,
                self.get_byte(self.cursor)
            ]),
            3 => {
                match self.remain.append_len {
                    1 => U8Unit::ThreeByte([
                        self.remain.a,
                        self.remain.b,
                        self.get_byte(self.cursor)
                    ]),
                    2 => U8Unit::ThreeByte([
                        self.remain.a,
                        self.get_byte(self.cursor),
                        self.get_byte(self.cursor + 1)
                    ]),
                    _ => unreachable!()
                }
            }
            4 => {
                match self.remain.append_len {
                    1 => U8Unit::FourByte([
                        self.remain.a,
                        self.remain.b,
                        self.remain.c,
                        self.get_byte(self.cursor)
                    ]),
                    2 => U8Unit::FourByte([
                        self.remain.a,
                        self.remain.b,
                        self.get_byte(self.cursor),
                        self.get_byte(self.cursor + 1)
                    ]),
                    3 => U8Unit::FourByte([
                        self.remain.a,
                        self.get_byte(self.cursor),
                        self.get_byte(self.cursor + 1),
                        self.get_byte(self.cursor + 2)
                    ]),
                    _=> unreachable!()
                }
            }
            _ => unreachable!()
        };
        self.cursor += self.remain.append_len;
        self.remain.clear();
        chr
    }

    fn set_remain(&mut self, len: usize, append_len: usize) {
        let remain_len = len - append_len;
        match remain_len {
            1 => self.remain.a = self.get_byte(self.cursor),
            2 => {
                self.remain.a = self.get_byte(self.cursor);
                self.remain.b = self.get_byte(self.cursor + 1);
            }
            3 => {
                self.remain.a = self.get_byte(self.cursor);
                self.remain.b = self.get_byte(self.cursor + 1);
                self.remain.c = self.get_byte(self.cursor + 2)
            }
            _ => unreachable!()
        }
        self.remain.len = len;
        self.remain.append_len = append_len;
    }

    fn get_byte(&mut self, index: usize) -> u8 {
        unsafe {
            *self.buf.get_unchecked(index)
        }
    }

}

impl <R: Read> Iterator for CharIterator<R> {
    type Item = IoResult<U8Unit>;

    fn next(&mut self) -> Option<Self::Item> {

        loop {

            if self.cur_len == usize::MAX {
                self.cur_len = match self.r.read(&mut self.buf) {
                    Ok(0) => return None,
                    Ok(n) => n,
                    Err(e) => return Some(Err(e))
                };
                
                return if !self.remain.is_empty() {
                    Some(Ok(self.remain_u8bytes()))
                } else {
                    let len = utf8_len(self.get_byte(self.cursor));
                    Some(Ok(self.cur_u8bytes(len)))
                }

            } else {
                if self.cursor == self.cur_len {
                    self.cur_len = usize::MAX;
                    self.cursor = 0;
                } else {
                    let len = utf8_len(self.get_byte(self.cursor));
                    if self.cursor + len > self.cur_len {
                        let append_len = self.cursor + len - self.cur_len;
                        self.set_remain(len, append_len);
                        self.cur_len = usize::MAX;
                        self.cursor = 0;
                    } else {
                        return Some(Ok(self.cur_u8bytes(len)));
                    }
                }
            }

        }
    }
}

/**
 *
 * 1. 如果首字节的最高位是0，那这个字符是单字节（0xxxxxxx）。
 * 2. 如果首字节的最高三位是110，则是双字节字符（首字节110xxxxx，后面一个字节10xxxxxx）。
 * 3. 如果首字节的前四位是1110，则是三字节字符（首字节1110xxxx，后面两个10xxxxxx的字节）。
 * 4. 如果首字节的前五位是11110，则是四字节字符（首字节11110xxx，后面三个10xxxxxx的字节）
 */
#[allow(clippy::unusual_byte_groupings)]
pub const fn utf8_len(first: u8) -> usize {
    if first & 0b1000_0000 == 0b0000_0000 {
        1
    } else if first & 0b1110_0000 == 0b1100_0000 {
        2
    } else if first & 0b1111_0000 == 0b1110_0000 {
        3
    } else if first & 0b1111_1000 == 0b1111_0000 {
        4
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use super::*;

    #[test]
    fn char_iterator_works() {
        let file = File::open("../123.txt").unwrap();
        let iter = CharIterator::new(file);

        for ch in iter {
            println!("{:?}", std::str::from_utf8((&ch.unwrap()).into()));
        }
    }
}