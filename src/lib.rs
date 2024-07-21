use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

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

#[cfg(test)]
mod test {

    use pest::iterators::Pair;

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

    #[test]
    fn it_parses_numbers() {
        assert_eq!(
            parse_inner_token(Rule::literal, "0").as_rule(),
            Rule::integer
        );
        assert_eq!(
            parse_inner_token(Rule::literal, "100").as_rule(),
            Rule::integer
        );
    }

    #[test]
    fn it_parses_floats() {
        assert_eq!(
            parse_inner_token(Rule::literal, "0.").as_rule(),
            Rule::float
        );
        assert_eq!(
            parse_inner_token(Rule::literal, ".5").as_rule(),
            Rule::float
        );
        assert_eq!(
            parse_inner_token(Rule::literal, "11.15").as_rule(),
            Rule::float
        );
    }

    #[test]
    fn it_parses_bytes() {
        assert_eq!(
            parse_inner_token(Rule::literal, "0xFF").as_rule(),
            Rule::byte
        );
        assert_eq!(
            parse_inner_token(Rule::literal, "0b0010").as_rule(),
            Rule::byte
        );
    }

    #[test]
    fn it_parses_strings() {
        assert_eq!(
            parse_inner_token(Rule::literal, r#""Hello world""#).as_rule(),
            Rule::string
        );
        assert_eq!(
            parse_inner_token(Rule::literal, r#""""#).as_rule(),
            Rule::string
        );
    }

    #[test]
    fn it_parses_booleans() {
        assert_eq!(
            parse_inner_token(Rule::literal, "true").as_rule(),
            Rule::boolean
        );
        assert_eq!(parse_inner_token(Rule::literal, "true").as_str(), "true");
        assert_eq!(
            parse_inner_token(Rule::literal, "false").as_rule(),
            Rule::boolean
        );
        assert_eq!(parse_inner_token(Rule::literal, "false").as_str(), "false");
    }
}
