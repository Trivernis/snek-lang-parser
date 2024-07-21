mod expressions;
mod literals;
mod statements;
mod types;

use crate::parse;

#[test]
fn it_parse_types() {
    let result = parse(include_str!("../../corpus/types.sk"));

    if let Err(e) = result {
        panic!("{e}")
    }
}

#[test]
fn it_parse_declarations() {
    let result = parse(include_str!("../../corpus/declarations.sk"));

    if let Err(e) = result {
        panic!("{e}")
    }
}
