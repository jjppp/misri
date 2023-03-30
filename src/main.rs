mod env;
mod exec;
mod instr;
mod lexer;
mod parser;
mod value;

use clap::{arg, Command};
use parser::Parser;
use std::fs;

use crate::exec::exec;

fn main() {
    let matches = Command::new("misri")
        .version("0.1.0")
        .author("jjppp <jpwang@smail.nju.edu.cn>")
        .about("Yet another interpreter for NJU irsim")
        .arg(arg!(-f --file <FILE> "ir file"))
        .get_matches();

    let file = match matches.get_one::<String>("file") {
        Some(file) => file,
        None => panic!("arg error"),
    };

    let cont = fs::read_to_string(file).expect("file error");
    let mut parser = Parser::from(cont.as_str());
    let mut program = parser.parse();
    program.init();
    exec(&program);
}
