use super::reader::Reader;

pub struct Compiler {
    pub reader: Reader
}

impl Compiler {
    pub fn new(path: &str) -> Compiler {
        let mut reader = Reader::new(path);
        reader.tokenize();

        Compiler { reader }
    }
}