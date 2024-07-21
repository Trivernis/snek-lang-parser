use pest::iterators::Pair;
mod literals;

use crate::{parse, parse_rule, Rule};

fn parse_token<'a>(rule: Rule, src: &'a str) -> Pair<'a, Rule> {
    let mut pairs = parse_rule(rule, src).unwrap();
    pairs.next().unwrap()
}

fn parse_inner_token<'a>(rule: Rule, src: &'a str) -> Pair<'a, Rule> {
    parse_token(rule, src).into_inner().next().unwrap()
}

#[test]
fn it_parses() {
    parse("hello world").unwrap();
}
