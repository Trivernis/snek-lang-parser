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
            ]),
            EOI(9, 9)
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
            ]),
            EOI(17, 17)
        ]),
    ]);
}

#[test]
fn it_parses_blocks() {
    parses_to!(parser: SnekParser, input: "let a = { 1 + 1 }", rule: Rule::statement, tokens: [
        statement(0, 17, [
            decl(0, 17, [
                ident(4, 5),
                expr(8, 17, [
                    block(8, 17, [
                       expr(10, 15, [
                           infix_expr(10, 15, [
                               literal(10, 11, [
                                   integer(10, 11)
                               ]),
                               operator(12, 13),
                               expr(14, 15, [
                                   literal(14, 15, [
                                       integer(14, 15)
                                   ])
                               ])
                           ])
                       ])
                    ])
                ])
            ]),
            EOI(17, 17)
        ])
    ]);
}

#[test]
fn it_parses_multiline_blocks() {
    parses_to!(parser: SnekParser, input: "let a = {\nlet b = 1 + 1\nb\n}", rule: Rule::statement, tokens: [
        statement(0, 27, [
            decl(0, 27, [
                ident(4, 5),
                expr(8, 27, [
                    block(8, 27, [
                        statement(10, 24, [
                           decl(10, 23, [
                               ident(14, 15),
                               expr(18, 23, [
                                   infix_expr(18, 23, [
                                       literal(18, 19, [
                                           integer(18, 19)
                                       ]),
                                       operator(20, 21),
                                       expr(22, 23, [
                                           literal(22, 23, [
                                               integer(22, 23)
                                           ])
                                       ])
                                   ])
                               ])
                           ]),
                        ]),
                        statement(24, 26, [
                           expr(24, 25, [
                               ident(24, 25)
                           ])
                        ])
                    ])
                ])
            ]),
            EOI(27, 27)
        ])
    ]);
}
