use pest::{consumes_to, parses_to};

use crate::{Rule, SnekParser};

#[test]
fn it_parses_addition() {
    parses_to!(parser: SnekParser, input: "1 + 1", rule: Rule::expr, tokens: [
        expr(0, 5, [
            infix_expr(0, 5, [
                literal(0, 1, [
                    integer(0, 1)
                ]),
                operator(2, 3),
                expr(4, 5, [
                    literal(4, 5, [
                        integer(4, 5)
                    ])
                ])
            ])
        ])
    ]);
}

#[test]
fn it_parses_addition_with_variables() {
    parses_to!(parser: SnekParser, input: "first + second", rule: Rule::expr, tokens: [
        expr(0, 14, [
            infix_expr(0, 14, [
                ident(0, 5),
                operator(6, 7),
                expr(8, 14, [
                    ident(8, 14)
                ])
            ])
        ])
    ]);
}

#[test]
fn it_parses_subtraction() {
    parses_to!(parser: SnekParser, input: "1.0 - 1.1", rule: Rule::expr, tokens: [
        expr(0, 9, [
            infix_expr(0, 9, [
                literal(0, 3, [
                    float(0, 3)
                ]),
                operator(4, 5),
                expr(6, 9, [
                    literal(6, 9, [
                        float(6, 9)
                    ])
                ])
            ])
        ])
    ]);
}

#[test]
fn it_parses_multiple_operations() {
    parses_to!(parser: SnekParser, input: "1 + 2 + 3", rule: Rule::expr, tokens: [
        expr(0, 9, [
            infix_expr(0, 9, [
                literal(0, 1, [
                    integer(0, 1)
                ]),
                operator(2, 3),
                expr(4, 9, [
                    infix_expr(4, 9, [
                        literal(4, 5, [
                            integer(4, 5)
                        ]),
                        operator(6, 7),
                        expr(8, 9, [
                            literal(8, 9, [
                                integer(8, 9)
                            ])
                        ])
                    ])
                ])
            ])
        ])
    ]);
}
