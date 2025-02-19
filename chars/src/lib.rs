use std::{
    fs::File,
    io::{self, Read},
    marker::PhantomData,
    path::Path,
};

//使用&str的版本
enum U8Unit {
    OneByte([u8; 1]),
    TwoByte([u8; 2]),
    ThreeByte([u8; 3]),
    FourByte([u8; 4]),
}

const BUF_SIZE: usize = 4;

struct Chars<'a> {
    //实现一个迭代器，读取byte，返回一次返回一个char
    buf: [u8; BUF_SIZE],
    cursor: usize,
    cur_len: usize,
    file: File,
    pre_buf: [u8; 4],
    pre_len: usize,
    _marker: PhantomData<&'a str>,
}

impl<'a> Chars<'a> {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self {
            buf: [0; BUF_SIZE],
            cursor: 0,
            cur_len: usize::MAX,
            file: File::open(path)?,
            pre_buf: [0; 4],
            pre_len: 0,
            _marker: PhantomData,
        })
    }

    fn cur_char(&mut self) -> &'a str {
        if self.pre_len != 0 {
            //前一轮读取buf中有字节解析剩余
            let char_len = utf8_len(self.pre_buf[0]);
            let offset = char_len - self.pre_len;
            unsafe {
                std::ptr::copy(
                    self.buf.as_ptr(),
                    self.pre_buf.as_mut_ptr().add(self.pre_len),
                    offset,
                );
            }
            self.cursor += offset;
            let chr = unsafe { std::str::from_utf8_unchecked(&(*(self as *const Self)).pre_buf[..char_len]) };
            self.pre_len = 0;
            chr
        } else {
            let char_len = utf8_len(self.buf[self.cursor]);
            let chr = unsafe {
                std::str::from_utf8_unchecked(&(*(self as *const Self)).buf[self.cursor..self.cursor + char_len])
            };
            self.cursor += char_len;
            chr
        }
    }
}

impl<'a> Iterator for Chars<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cur_len == usize::MAX {
                //buf中无有效字符，重新从file中读取
                println!("reach new round");
                self.cur_len = match self.file.read(&mut self.buf) {
                    Ok(0) => return None,
                    Ok(n) => n,
                    Err(_) => return None,
                };

                let chr = self.cur_char();
                return Some(chr);
            } else {
                if self.cursor == self.cur_len {
                    //解析完一轮buf且刚好是完整的utf_8的byte，进行下一轮解析
                    println!("reach prefect bounded");
                    self.cur_len = usize::MAX;
                    self.cursor = 0;
                } else if self.cursor + utf8_len(self.buf[self.cursor]) > self.cur_len {
                    //解析完一轮，剩余utf_8的一部份
                    println!("reach not perfect bounded");
                    unsafe {
                        std::ptr::copy(
                            self.buf[self.cursor..].as_ptr(),
                            self.pre_buf.as_mut_ptr(),
                            self.cur_len - self.cursor
                        );
                    }
                    self.pre_len = self.cur_len - self.cursor;
                    self.cur_len = usize::MAX;
                    self.cursor = 0;
                } else {
                    let chr = self.cur_char();
                    return Some(chr);
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
mod tests {
    use super::*;

    #[test]
    fn utf8_len_works() {
        assert_eq!(1, utf8_len(b'a'));
        assert_eq!(2, utf8_len('¢'.to_string().as_bytes()[0]));
        assert_eq!(3, utf8_len('中'.to_string().as_bytes()[0]));
        assert_eq!(4, utf8_len('😊'.to_string().as_bytes()[0]));
    }

    #[test]
    fn utf8_char_works() {
        let chars = Chars::new("123.txt").unwrap();
        for c in chars {
            println!("{:?}", c)
        }
    }

}