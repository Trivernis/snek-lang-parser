use pest::{consumes_to, parses_to};
use pest_test::PestTester;

use crate::{Rule, SnekParser};
use lazy_static::lazy_static;

lazy_static! {
    static ref TESTER: PestTester<Rule, SnekParser> = super::s_tester("types");
}

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
    if let Err(e) = (*TESTER).evaluate_strict("recs") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_enums() {
    if let Err(e) = (*TESTER).evaluate_strict("enums") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_generics() {
    if let Err(e) = (*TESTER).evaluate_strict("generics") {
        panic!("{e}")
    }
}
