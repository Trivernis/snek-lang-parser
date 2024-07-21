use pest::{consumes_to, parses_to};

use crate::{Rule, SnekParser};

#[test]
fn it_parses_aliases() {
    parses_to!(parser: SnekParser, input: "type MyNum = Num", rule: Rule::statement, tokens: [
        statement(0, 16, [
            type_decl(0, 16, [
                type_ident(5, 10, [
                    ident(5, 10)
                ]),
                type_expr(13, 16, [
                    type_ident(13, 16, [
                        ident(13, 16)
                    ])
                ])
            ]),
            EOI(16, 16)
        ])
    ]);
}

#[test]
fn it_parses_tuples() {
    parses_to!(parser: SnekParser, input: "type MyTuple = #(Str Int)", rule: Rule::statement, tokens: [
        statement(0, 25, [
            type_decl(0, 25, [
                type_ident(5, 12, [
                    ident(5, 12)
                ]),
                type_expr(15, 25, [
                    tuple(15, 25, [
                        type_expr(17, 20, [
                            type_ident(17, 20, [
                                ident(17, 20)
                            ])
                        ]),
                        type_expr(21, 24, [
                            type_ident(21, 24, [
                                ident(21, 24)
                            ])
                        ]),
                    ])
                ])
            ]),
            EOI(25, 25)
        ])
    ]);
}

#[test]
fn it_parses_recs() {
    parses_to!(parser: SnekParser, input: "type MyRec = #{\nfield1: Num\n field2: Num2 }", rule: Rule::statement, tokens: [
        statement(0, 43, [
            type_decl(0, 43, [
                type_ident(5, 10, [
                    ident(5, 10)
                ]),
                type_expr(13, 43, [
                    rec(13, 43, [
                        rec_field(16, 27, [
                            ident(16, 22),
                            type_expr(24, 27, [
                                type_ident(24, 27, [
                                    ident(24, 27)
                                ])
                            ]),
                        ]),
                        rec_field(29, 41, [
                            ident(29, 35),
                            type_expr(37, 41, [
                                type_ident(37, 41, [
                                    ident(37, 41)
                                ])
                            ]),
                        ])
                    ])
                ])
            ]),
            EOI(43, 43)
        ])
    ]);
}

#[test]
fn it_parses_generics() {
    parses_to!(parser: SnekParser, input: "type MyNum<a> = Num<a>", rule: Rule::statement, tokens: [
        statement(0, 22, [
            type_decl(0, 22, [
                type_ident(5, 13, [
                    ident(5, 10),
                    type_args(10, 13, [
                        type_ident(11, 12, [
                            ident(11, 12)
                        ])
                    ])
                ]),
                type_expr(16, 22, [
                    type_ident(16, 22, [
                        ident(16, 19),
                        type_args(19, 22, [
                            type_ident(20, 21, [
                                ident(20, 21)
                            ])
                        ])
                    ])
                ])
            ]),
            EOI(22, 22)
        ])
    ]);
}
