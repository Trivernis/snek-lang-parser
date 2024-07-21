mod expressions;
mod literals;
mod statements;
mod types;

use crate::parse;

#[test]
fn it_parses() {
    parse("hello world").unwrap();
}
