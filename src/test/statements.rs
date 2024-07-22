use pest::{consumes_to, parses_to};
use pest_test::PestTester;

use crate::{Rule, SnekParser};
use lazy_static::lazy_static;

lazy_static! {
    static ref TESTER: PestTester<Rule, SnekParser> = super::s_tester("statements");
}

#[test]
fn it_parses_simple_decl() {
    if let Err(e) = (*TESTER).evaluate_strict("simple-decl") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_decl_with_args() {
    if let Err(e) = (*TESTER).evaluate_strict("fn-decl") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_decl_with_type() {
    if let Err(e) = (*TESTER).evaluate_strict("decl-with-type") {
        panic!("{e}")
    }
}

#[test]
fn it_parses_if_stmt() {
    if let Err(e) = (*TESTER).evaluate_strict("if-stmt") {
        panic!("{e}")
    }
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
                               literal(14, 15, [
                                   integer(14, 15)
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
                                       literal(22, 23, [
                                           integer(22, 23)
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
