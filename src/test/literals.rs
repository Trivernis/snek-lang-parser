use crate::{test::parse_inner_token, Rule};

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
    assert_eq!(
        parse_inner_token(Rule::literal, r#""\"""#).as_rule(),
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
