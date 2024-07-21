use pest::{consumes_to, parses_to};

use crate::{Rule, SnekParser};

#[test]
fn it_parses_plain_assignments() {
    parses_to!(parser: SnekParser, input: "let a = 1", rule: Rule::statement, tokens: [
        statement(0, 9, [
            decl(0, 9, [
                ident(4, 5),
                expr(8, 9, [
                    literal(8, 9, [
                        integer(8, 9)
                    ]),
                ])
            ])
        ])
    ]);
}

#[test]
fn it_parses_assignments_with_args() {
    parses_to!(parser: SnekParser, input: "let add a = a + 1", rule: Rule::statement, tokens: [
        statement(0, 17, [
            decl(0, 17, [
                ident(4, 7),
                ident(8, 9),
                expr(12, 17, [
                    infix_expr(12, 17, [
                        ident(12, 13),
                        operator(14, 15),
                        expr(16, 17, [
                            literal(16, 17, [
                                integer(16, 17)
                            ])
                        ])
                    ])
                ])
            ])
        ])
    ]);
}
