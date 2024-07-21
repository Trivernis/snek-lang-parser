use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[cfg(test)]
mod test;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct SnekParser;

pub fn parse<'a>(src: &'a str) -> Result<Pairs<'a, Rule>, pest::error::Error<Rule>> {
    SnekParser::parse(Rule::file, src)
}

pub fn parse_rule<'a>(
    rule: Rule,
    src: &'a str,
) -> Result<Pairs<'a, Rule>, pest::error::Error<Rule>> {
    SnekParser::parse(rule, src)
}
