pub enum Token<'a> {
    //
    Header {
        typ:  HeaderType,
        //
        value: &'a str
    }
}

pub enum HeaderType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

pub trait Tokennizer {
    fn next(&mut self) -> Token;
}

pub struct TokenizerImpl {

}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test() {
        let mut f = File::open("123.txt").unwrap();
        let mut buf = [0; 512];

        loop {
            let len = f.read(&mut buf).unwrap();
            println!("{:?}", len);
            break;
        }
    }
}