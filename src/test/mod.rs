mod infix_expressions;
mod literals;

use crate::parse;

#[test]
fn it_parses() {
    parse("hello world").unwrap();
}
