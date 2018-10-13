extern crate failure;
#[macro_use]
extern crate lalrpop_util;
extern crate regex;

mod ast;
mod parser;

use std::env;
use std::fs::File;
use std::io::Read;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut args = env::args();
    if args.len() < 2 {
        panic!("Usage: {} <file>", args.nth(0).unwrap());
    }

    let mut file = File::open(args.nth(1).unwrap())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let ast = parser::convert(&contents);
    println!("{:#?}", ast);

    Ok(())
}
