mod expressions;
mod literals;
mod statements;
mod types;

use crate::parse;
use std::{collections::HashSet, path::PathBuf};

use lazy_static::lazy_static;
use pest_test::PestTester;

use crate::{Rule, SnekParser};

fn s_tester<D: Into<PathBuf>>(dir: D) -> PestTester<Rule, SnekParser> {
    PestTester::new(
        PathBuf::from("tests/corpus").join(dir.into()),
        "txt",
        Rule::file,
        HashSet::new(),
    )
}

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

lazy_static! {
    static ref TESTER: PestTester<Rule, SnekParser> = PestTester::new(
        PathBuf::from("test/corpus"),
        "txt",
        Rule::file,
        HashSet::new()
    );
}
