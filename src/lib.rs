use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct SnekParser;

pub fn parse<'a>(src: &'a str) -> Result<Pairs<'a, Rule>, pest::error::Error<Rule>> {
    SnekParser::parse(Rule::file, src)
}

#[cfg(test)]
mod test {
    use crate::parse;

    #[test]
    fn it_parses() {
        parse("hello world").unwrap();
    }
}
