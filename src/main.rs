mod meaning;

use meaning::compiler::Compiler;

fn main() {
    Compiler::new("main.m");
    // println!("{}", 'a' <= 'v' && 'v' <= 'z')
}