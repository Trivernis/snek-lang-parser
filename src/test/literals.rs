use pest::consumes_to;
use pest::parses_to;

use crate::Rule;
use crate::SnekParser;

#[test]
fn it_parses_numbers() {
    parses_to!(parser: SnekParser, input: "0", rule: Rule::literal, tokens: [
        literal(0, 1, [
            integer(0, 1)
        ])
    ]);
    parses_to!(parser: SnekParser, input: "100", rule: Rule::literal, tokens: [
        literal(0, 3, [
            integer(0, 3)
        ])
    ]);
}

#[test]
fn it_parses_floats() {
    parses_to!(parser: SnekParser, input: "0.0", rule: Rule::literal, tokens: [
        literal(0, 3, [
            float(0, 3)
        ])
    ]);
    parses_to!(parser: SnekParser, input: ".0", rule: Rule::literal, tokens: [
        literal(0, 2, [
            float(0, 2)
        ])
    ]);
    parses_to!(parser: SnekParser, input: "0.", rule: Rule::literal, tokens: [
        literal(0, 2, [
            float(0, 2)
        ])
    ]);
    parses_to!(parser: SnekParser, input: "11.2", rule: Rule::literal, tokens: [
        literal(0, 4, [
            float(0, 4)
        ])
    ]);
}

#[test]
fn it_parses_bytes() {
    parses_to!(parser: SnekParser, input: "0xFF", rule: Rule::literal, tokens: [
        literal(0, 4, [
            byte(0, 4)
        ])
    ]);
    parses_to!(parser: SnekParser, input: "0b0010", rule: Rule::literal, tokens: [
        literal(0, 6, [
            byte(0, 6)
        ])
    ]);
}

#[test]
fn it_parses_strings() {
    parses_to!(parser: SnekParser, input: r#""Hello world""#, rule: Rule::literal, tokens: [
        literal(0, 13, [
            string(1, 12)
        ])
    ]);
    parses_to!(parser: SnekParser, input: r#""""#, rule: Rule::literal, tokens: [
        literal(0, 2, [
            string(1, 1)
        ])
    ]);
    parses_to!(parser: SnekParser, input: r#""\"""#, rule: Rule::literal, tokens: [
        literal(0, 3, [
            string(1, 2)
        ])
    ]);
}

#[test]
fn it_parses_chars() {
    parses_to!(parser: SnekParser, input: "'c'", rule: Rule::literal, tokens: [
        literal(0, 3, [
            char(0, 3)
        ])
    ]);
    parses_to!(parser: SnekParser, input: r"'\''", rule: Rule::literal, tokens: [
        literal(0, 3, [
            char(0, 3)
        ])
    ]);
}

#[test]
fn it_parses_booleans() {
    parses_to!(parser: SnekParser, input: "true", rule: Rule::literal, tokens: [
        literal(0, 4, [
            boolean(0, 4)
        ])
    ]);
    parses_to!(parser: SnekParser, input: "false", rule: Rule::literal, tokens: [
        literal(0, 5, [
            boolean(0, 5)
        ])
    ]);
}
