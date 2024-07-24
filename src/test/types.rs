use pest::{consumes_to, parses_to};
use pest_test::PestTester;

use crate::{Rule, SnekParser};
use lazy_static::lazy_static;

lazy_static! {
    static ref TESTER: PestTester<Rule, SnekParser> = super::s_tester("types");
}

#[test]
fn it_parses_aliases() {
    if let Err(e) = (*TESTER).evaluate_strict("aliases") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_tuples() {
    if let Err(e) = (*TESTER).evaluate_strict("tuples") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_recs() {
    if let Err(e) = (*TESTER).evaluate_strict("recs") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_enums() {
    if let Err(e) = (*TESTER).evaluate_strict("enums") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_generics() {
    if let Err(e) = (*TESTER).evaluate_strict("generics") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_functions() {
    if let Err(e) = (*TESTER).evaluate_strict("functions") {
        panic!("{e}")
    }
}
