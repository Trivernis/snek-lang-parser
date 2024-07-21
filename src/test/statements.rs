use pest::{consumes_to, parses_to};

use crate::{Rule, SnekParser};

#[test]
fn it_parses_assignments() {
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
