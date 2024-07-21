mod expressions;
mod literals;
mod statements;

use crate::parse;

#[test]
fn it_parses() {
    parse("hello world").unwrap();
}
