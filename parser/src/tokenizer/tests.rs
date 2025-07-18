use pretty_assertions::assert_eq;
use std::mem;

use super::Token;
use crate::Span;

#[test]
fn keywords() {
    let mut lexer = super::Tokenizer::new("global rule".as_bytes());

    mem::discriminant(&Token::L_BRACE(Span(0..0)));

    assert_eq!(lexer.next_token(), Some(Token::GLOBAL_KW(Span(0..6))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(6..7))));
    assert_eq!(lexer.next_token(), Some(Token::RULE_KW(Span(7..11))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new("globalrule".as_bytes());

    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(0..10))));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn identifiers() {
    let mut lexer = super::Tokenizer::new(
        "foo _bar baz0 qux_1 $ $_ $foo @foo #foo !foo _123".as_bytes(),
    );

    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(0..3))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(3..4))));
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(4..8))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(8..9))));
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(9..13))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(13..14))));
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(14..19))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(19..20))));
    assert_eq!(lexer.next_token(), Some(Token::PATTERN_IDENT(Span(20..21))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(21..22))));
    assert_eq!(lexer.next_token(), Some(Token::PATTERN_IDENT(Span(22..24))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(24..25))));
    assert_eq!(lexer.next_token(), Some(Token::PATTERN_IDENT(Span(25..29))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(29..30))));
    assert_eq!(lexer.next_token(), Some(Token::PATTERN_OFFSET(Span(30..34))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(34..35))));
    assert_eq!(lexer.next_token(), Some(Token::PATTERN_COUNT(Span(35..39))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(39..40))));
    assert_eq!(lexer.next_token(), Some(Token::PATTERN_LENGTH(Span(40..44))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(44..45))));
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(45..49))));

    assert_eq!(lexer.next_token(), None);
}

#[test]
fn integer_literals() {
    let mut lexer = super::Tokenizer::new(r#"1 10 999"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(0..1))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(1..2))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(2..4))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(4..5))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(5..8))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"0x10 0xAB 0xfc"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(0..4))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(4..5))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(5..9))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(9..10))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(10..14))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"0o10 0o07 0x77"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(0..4))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(4..5))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(5..9))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(9..10))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(10..14))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(
        r#"1_0 0x1_0 0o1_0 1__0 0x1__0 0o1__0"#.as_bytes(),
    );
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(0..3))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(3..4))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(4..9))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(9..10))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(10..15))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(15..16))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(16..20))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(20..21))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(21..27))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(27..28))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(28..34))));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn float_literals() {
    let mut lexer =
        super::Tokenizer::new(r#"3.14 10.0 1.0 1_0.0_1 1__0.0__1"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::FLOAT_LIT(Span(0..4))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(4..5))));
    assert_eq!(lexer.next_token(), Some(Token::FLOAT_LIT(Span(5..9))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(9..10))));
    assert_eq!(lexer.next_token(), Some(Token::FLOAT_LIT(Span(10..13))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(13..14))));
    assert_eq!(lexer.next_token(), Some(Token::FLOAT_LIT(Span(14..21))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(21..22))));
    assert_eq!(lexer.next_token(), Some(Token::FLOAT_LIT(Span(22..31))));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn string_literals() {
    let mut lexer = super::Tokenizer::new(r#""foo \"bar\" baz""#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::STRING_LIT(Span(0..17))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#""foo /*bar*/ baz""#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::STRING_LIT(Span(0..17))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#""foo \\" bar""#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::STRING_LIT(Span(0..8))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(8..9))));
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(9..12))));
    assert_eq!(lexer.next_token(), Some(Token::UNKNOWN(Span(12..13))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#""foo \x0 bar""#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::STRING_LIT(Span(0..13))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#""标识符""#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::STRING_LIT(Span(0..11))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(
        r#""""foo
    bar
    baz""""#
            .as_bytes(),
    );
    assert_eq!(lexer.next_token(), Some(Token::STRING_LIT(Span(0..25))));
    assert_eq!(lexer.next_token(), None);

    // String literals can contain invalid UTF-8 characters.
    let mut lexer = super::Tokenizer::new(b"\"foo \xFF\xFF\"");
    assert_eq!(lexer.next_token(), Some(Token::STRING_LIT(Span(0..8))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(
        b"\"\"\"foo
    bar
     \xFF\xFF\"\"\"",
    );
    assert_eq!(lexer.next_token(), Some(Token::STRING_LIT(Span(0..25))));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn comments() {
    let mut lexer = super::Tokenizer::new(r#"/* comment */"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..13))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(
        r#"/*
  comment * /
*/"#
        .as_bytes(),
    );
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..19))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"// comment "#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..11))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"/* Comment */s"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..13))));
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(13..14))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"/** Comment **/"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..15))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"/***/"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..5))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(b"/* \xFF\xFF Comment */s");
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..16))));
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(16..17))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(b"// \xFF\xFF Comment ");
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..14))));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn regexps() {
    let mut lexer = super::Tokenizer::new(r#"/foobar/ /.*/"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(0..8))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(8..9))));
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(9..13))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer =
        super::Tokenizer::new(r#"/foobar/i /(foo|bar)/s"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(0..9))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(9..10))));
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(10..22))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"/\x00/"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(0..6))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"///"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::COMMENT(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"/a/"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"/\\/ "#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(0..4))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(4..5))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(b"/\n/");
    assert_eq!(lexer.next_token(), Some(Token::UNKNOWN(Span(0..1))));
    assert_eq!(lexer.next_token(), Some(Token::NEWLINE(Span(1..2))));
    assert_eq!(lexer.next_token(), Some(Token::UNKNOWN(Span(2..3))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"/\/foo/"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(0..7))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"/foobar\\/"#.as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(0..10))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(b"/foobar\xFF\xFF/");
    assert_eq!(lexer.next_token(), Some(Token::REGEXP(Span(0..10))));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn hex_pattern() {
    let mut lexer = super::Tokenizer::new(r#"$a={a0}a0"#.as_bytes());

    assert_eq!(lexer.next_token(), Some(Token::PATTERN_IDENT(Span(0..2))));
    assert_eq!(lexer.next_token(), Some(Token::EQUAL(Span(2..3))));
    assert_eq!(lexer.next_token(), Some(Token::L_BRACE(Span(3..4))));
    lexer.enter_hex_pattern_mode();
    assert_eq!(lexer.next_token(), Some(Token::HEX_BYTE(Span(4..6))));
    assert_eq!(lexer.next_token(), Some(Token::R_BRACE(Span(6..7))));
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(7..9))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"AA0?BB?0[1-10]CC"#.as_bytes());

    lexer.enter_hex_pattern_mode();
    assert_eq!(lexer.next_token(), Some(Token::HEX_BYTE(Span(0..2))));
    assert_eq!(lexer.next_token(), Some(Token::HEX_BYTE(Span(2..4))));
    assert_eq!(lexer.next_token(), Some(Token::HEX_BYTE(Span(4..6))));
    assert_eq!(lexer.next_token(), Some(Token::HEX_BYTE(Span(6..8))));
    assert_eq!(lexer.next_token(), Some(Token::L_BRACKET(Span(8..9))));
    lexer.enter_hex_jump_mode();
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(9..10))));
    assert_eq!(lexer.next_token(), Some(Token::HYPHEN(Span(10..11))));
    assert_eq!(lexer.next_token(), Some(Token::INTEGER_LIT(Span(11..13))));
    assert_eq!(lexer.next_token(), Some(Token::R_BRACKET(Span(13..14))));
    assert_eq!(lexer.next_token(), Some(Token::HEX_BYTE(Span(14..16))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new(r#"11 [0] 22 [1-2] "#.as_bytes());

    lexer.enter_hex_pattern_mode();
    assert_eq!(lexer.next_token(), Some(Token::HEX_BYTE(Span(0..2))));
}

#[test]
fn whitespaces() {
    let mut lexer = super::Tokenizer::new(" \t".as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..2))));
    assert_eq!(lexer.next_token(), None);

    // No-Break Space (NBSP) (U+00A0).
    let mut lexer = super::Tokenizer::new(b"\xC2\xA0");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..2))));
    assert_eq!(lexer.next_token(), None);

    // "En Quad" character (U+2000).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x80");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Em Quad" character (U+2001).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x81");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "En Space" character (U+2002).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x82");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Em Space" character (U+2003).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x83");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Three-Per-Em" character (U+2004).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x84");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Four-Per-Em" character (U+2005).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x85");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Six-Per-Em" character (U+2006).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x86");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Figure Space" character (U+2007).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x87");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Punctuation Space" character (U+2008).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x88");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Thin Space" character (U+2009).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x89");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Hair Space" character (U+200A).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\x8A");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Narrow No-Break Space" character (U+202f).
    let mut lexer = super::Tokenizer::new(b"\xE2\x80\xAF");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);

    // "Medium Mathematical Space" character (U+205f).
    let mut lexer = super::Tokenizer::new(b"\xE2\x81\x9F");
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(0..3))));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn newline() {
    let mut lexer = super::Tokenizer::new("\n".as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::NEWLINE(Span(0..1))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new("\r".as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::NEWLINE(Span(0..1))));
    assert_eq!(lexer.next_token(), None);

    let mut lexer = super::Tokenizer::new("\r\n".as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::NEWLINE(Span(0..2))));
    assert_eq!(lexer.next_token(), None);
}

#[test]
fn errors() {
    let mut lexer = super::Tokenizer::new("标识符 标识符".as_bytes());
    assert_eq!(lexer.next_token(), Some(Token::UNKNOWN(Span(0..9))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(9..10))));
    assert_eq!(lexer.next_token(), Some(Token::UNKNOWN(Span(10..19))));

    let mut lexer = super::Tokenizer::new(b"\xC7\xA3\xFF\xFF");
    assert_eq!(lexer.next_token(), Some(Token::UNKNOWN(Span(0..2))));
    assert_eq!(lexer.next_token(), Some(Token::INVALID_UTF8(Span(2..3))));
    assert_eq!(lexer.next_token(), Some(Token::INVALID_UTF8(Span(3..4))));

    let mut lexer = super::Tokenizer::new(b"\xFF\xFF");
    assert_eq!(lexer.next_token(), Some(Token::INVALID_UTF8(Span(0..1))));

    let mut lexer = super::Tokenizer::new(b"foo \xFF\xFF");
    assert_eq!(lexer.next_token(), Some(Token::IDENT(Span(0..3))));
    assert_eq!(lexer.next_token(), Some(Token::WHITESPACE(Span(3..4))));
    assert_eq!(lexer.next_token(), Some(Token::INVALID_UTF8(Span(4..5))));
}
