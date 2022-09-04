use std::path::Path;
use std::env::current_dir;

use super::file::{File, FileType, Directory};

pub struct Reader {
    pub(crate) main: File,
    pub(crate) files: Vec<File>,
    pub(crate) directorys: Vec<Directory>,

    pub(crate) token_buf: Vec<String>
}

impl Reader {
    pub fn _new(path: &str, main_file: Option<File>) -> Reader {
        let dir = match current_dir() {
            Ok(path) => Some(path),
            Err(e) => { println!("{:#?}", e); None }
        };

        let full_path = if let Some(mut p) = dir {
            p.push(Path::new(path).file_name().unwrap());
            
            p.into_os_string().into_string().unwrap()
        } else { String::new() };

        let r#type = if full_path.ends_with(".m") { FileType::SourceCode } else { FileType::ByteCode };
        let file = File::new(full_path, r#type);

        Reader {
            main: if let Some(file) = main_file { file } else { file },
            files: vec![],
            directorys: vec![],
            token_buf: vec![],
        }
    }

    pub fn new(path: &str) -> Reader {
        Reader::_new(path, None)
    }
}

impl Reader {
    pub fn tokenize(&mut self) -> () {
       println!("{:#?}", self.main.tokenize());
    }
}

/*
> main.m
class Something : OtherClass
or
use full.path.OtherClass

If file refeneced other classes, 
Reader reads full/path/OtherClass.m or full/path/OtherClass.mbc if file is compiled

Notice that Root level of file only contains class declaration

> full/path/OtherClass.m
...

Reader read classes inside the file and store them in memory
Then, return target class back to the process of original class

We can support Root level function declaration by compiling functions into a class at compile

> Example
var main() {...}
> Will be compiled into
class Companion {
    var main() {...}
}
*/