use crate::parser::Parser;
use pretty_assertions::assert_eq;

#[test]
fn newlines_and_spaces() {
    let cst = Parser::new()
        .build_cst("rule\n\rtest\r\n{ condition:\ntrue\n }", None)
        .unwrap()
        .whitespaces(true)
        .ascii_tree_string();

    assert_eq!(
        cst,
        r#" source_file
 └─ rule_decl
    ├─ k_RULE "rule"
    ├─ WHITESPACE ""
    ├─ WHITESPACE ""
    ├─ ident "test"
    ├─ WHITESPACE ""
    ├─ WHITESPACE ""
    ├─ LBRACE "{"
    ├─ WHITESPACE ""
    ├─ k_CONDITION "condition"
    ├─ COLON ":"
    ├─ WHITESPACE ""
    ├─ boolean_expr
    │  ├─ boolean_term
    │  │  └─ k_TRUE "true"
    │  ├─ WHITESPACE ""
    │  └─ WHITESPACE ""
    └─ RBRACE "}"
"#
    );
}

#[test]
fn identifiers() {
    // The following identifiers are ok, even if they are prefixed by a
    // keyword.
    assert!(Parser::new()
        .build_cst("rule true_ { condition: true }", None)
        .is_ok());
    assert!(Parser::new()
        .build_cst("rule false_ { condition: false }", None)
        .is_ok());
    assert!(Parser::new()
        .build_cst("rule rules { condition: true }", None)
        .is_ok());
    assert!(Parser::new()
        .build_cst("rule _true { condition: true }", None)
        .is_ok());
}

mod ast;
mod cst;
mod errors;
mod warnings;