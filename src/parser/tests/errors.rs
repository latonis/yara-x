use crate::parser::Parser;
use pretty_assertions::assert_eq;

#[test]
fn syntax_errors() {
    let tests = vec![
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"rule test : tag1 tag1 { condition: true }"#,
            r#"error: duplicate tag `tag1`
   ╭─[line:1:18]
   │
 1 │ rule test : tag1 tag1 { condition: true }
   ·                  ──┬─  
   ·                    ╰─── duplicate tag
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"rule 1_foo { condition: true }"#,
            r#"error: syntax error
   ╭─[line:1:6]
   │
 1 │ rule 1_foo { condition: true }
   ·      │ 
   ·      ╰─ expected identifier
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test {\n\ttrue }",
            r#"error: syntax error
   ╭─[line:2:2]
   │
 2 │     true }
   ·     │ 
   ·     ╰─ expected `condition`, `meta`, or `strings`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test { condition: true",
            r#"error: syntax error
   ╭─[line:1:28]
   │
 1 │ rule test { condition: true
   ·                            │ 
   ·                            ╰─ expected closing brace `}` or operator
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test condition: true }",
            r#"error: syntax error
   ╭─[line:1:11]
   │
 1 │ rule test condition: true }
   ·           │ 
   ·           ╰─ expected colon `:` or opening brace `{`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test : condition: true }",
            r#"error: syntax error
   ╭─[line:1:13]
   │
 1 │ rule test : condition: true }
   ·             │ 
   ·             ╰─ expected identifier
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test {}",
            r#"error: syntax error
   ╭─[line:1:12]
   │
 1 │ rule test {}
   ·            │ 
   ·            ╰─ expected `condition`, `meta`, or `strings`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test { meta: condition: true }",
            r#"error: syntax error
   ╭─[line:1:19]
   │
 1 │ rule test { meta: condition: true }
   ·                   │ 
   ·                   ╰─ expected identifier
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test { meta: a = condition: true }",
            r#"error: syntax error
   ╭─[line:1:23]
   │
 1 │ rule test { meta: a = condition: true }
   ·                       │ 
   ·                       ╰─ expected `false`, `true`, number, or string literal
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test { strings: condition: true }",
            r#"error: syntax error
   ╭─[line:1:22]
   │
 1 │ rule test { strings: condition: true }
   ·                      │ 
   ·                      ╰─ expected string identifier
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test : rule { condition: true }",
            r#"error: syntax error
   ╭─[line:1:13]
   │
 1 │ rule test : rule { condition: true }
   ·             │ 
   ·             ╰─ expected identifier
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test { condition: }",
            r#"error: syntax error
   ╭─[line:1:24]
   │
 1 │ rule test { condition: }
   ·                        │ 
   ·                        ╰─ expected boolean expression
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "private foo rule test { condition: true }",
            r#"error: syntax error
   ╭─[line:1:9]
   │
 1 │ private foo rule test { condition: true }
   ·         │ 
   ·         ╰─ expected `global` or `rule`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "global foo rule test { condition: true }",
            r#"error: syntax error
   ╭─[line:1:8]
   │
 1 │ global foo rule test { condition: true }
   ·        │ 
   ·        ╰─ expected `private` or `rule`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "global global rule test { condition: true }",
            r#"error: syntax error
   ╭─[line:1:8]
   │
 1 │ global global rule test { condition: true }
   ·        │ 
   ·        ╰─ expected `private` or `rule`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test { condition: 4 + }",
            r#"error: syntax error
   ╭─[line:1:28]
   │
 1 │ rule test { condition: 4 + }
   ·                            │ 
   ·                            ╰─ expected expression
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "foo rule test { condition: true }",
            r#"error: syntax error
   ╭─[line:1:1]
   │
 1 │ foo rule test { condition: true }
   · │ 
   · ╰─ expected YARA rules
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            "rule test { strings: $a = {} condition: true }",
            r#"error: syntax error
   ╭─[line:1:28]
   │
 1 │ rule test { strings: $a = {} condition: true }
   ·                            │ 
   ·                            ╰─ unexpected closing brace `}`
───╯
"#,
        ),
        /////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
strings:
  $a = { 00 [0-1] }
condition: true
}"#,
            r#"error: syntax error
   ╭─[line:4:19]
   │
 4 │   $a = { 00 [0-1] }
   ·                   │ 
   ·                   ╰─ expected byte, opening bracket `[`, or opening parenthesis `(`
───╯
"#,
        ),
        /////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
strings:
  $a = { [0-1] 00 }
condition: true
}"#,
            r#"error: syntax error
   ╭─[line:4:10]
   │
 4 │   $a = { [0-1] 00 }
   ·          │ 
   ·          ╰─ expected bytes
───╯
"#,
        ),
        /////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
strings:
  $a = { 00 ( 00 }
condition: true
}"#,
            r#"error: syntax error
   ╭─[line:4:18]
   │
 4 │   $a = { 00 ( 00 }
   ·                  │ 
   ·                  ╰─ expected byte, closing parenthesis `)`, opening bracket `[`, opening parenthesis `(`, or pipe `|`
───╯
"#,
        ),
        /////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
strings:
  $a = { 00 ~?? 11 }
condition: true
}"#,
            r#"error: invalid hex string `$a`
   ╭─[line:4:13]
   │
 4 │   $a = { 00 ~?? 11 }
   ·             ─┬─  
   ·              ╰─── negation of `??` is not allowed
───╯
"#,
        ),
        /////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
strings:
  $a = { G0 }
condition: true
}"#,
            r#"error: syntax error
   ╭─[line:4:10]
   │
 4 │   $a = { G0 }
   ·          │ 
   ·          ╰─ expected bytes
───╯
"#,
        ),
        /////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
strings:
  $a = { 01 02 0 }
condition: 
  $a
}"#,
            r#"error: invalid hex string `$a`
   ╭─[line:4:16]
   │
 4 │   $a = { 01 02 0 }
   ·                ┬  
   ·                ╰── uneven number of nibbles
───╯
"#,
        ),
        /////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
condition:
  any of (a,b,c) in (0..100)
}"#,
            r#"error: syntax error
   ╭─[line:4:18]
   │
 4 │   any of (a,b,c) in (0..100)
   ·                  │ 
   ·                  ╰─ unexpected `in`
───╯
"#,
        ),
        /////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
condition:
  any of (a,b,c) at 0
}"#,
            r#"error: syntax error
   ╭─[line:4:18]
   │
 4 │   any of (a,b,c) at 0
   ·                  │ 
   ·                  ╰─ unexpected `at`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  condition: 1  1 == 1
}
"#,
            r#"error: syntax error
   ╭─[line:3:17]
   │
 3 │   condition: 1  1 == 1
   ·                 │ 
   ·                 ╰─ expected `of`, closing brace `}`, dot `.`, opening bracket `[`, opening parenthesis `(`, operator, or percent `%`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
   condition: true
}

rule test {
   condition: false
}
"#,
            r#"error: duplicate rule `test`
   ╭─[line:6:6]
   │
 2 │ rule test {
   ·      ──┬─  
   ·        ╰─── `test` declared here for the first time
   · 
 6 │ rule test {
   ·      ──┬─  
   ·        ╰─── duplicate declaration of `test`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
   strings:
     $a = "foo"
     $a = "bar"
   condition:
     all of them
}

"#,
            r#"error: duplicate string `$a`
   ╭─[line:5:6]
   │
 4 │      $a = "foo"
   ·      ─┬  
   ·       ╰── `$a` declared here for the first time
 5 │      $a = "bar"
   ·      ─┬  
   ·       ╰── duplicate declaration of `$a`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
   strings:
     $a = "foo"
     $b = "bar"
   condition:
     $a
}

"#,
            r#"error: unused string `$b`
   ╭─[line:5:6]
   │
 5 │      $b = "bar"
   ·      ─┬  
   ·       ╰── this was not used in the condition
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
   strings:
     $ = "foo"
   condition:
     true
}

"#,
            r#"error: unused string `$`
   ╭─[line:4:6]
   │
 4 │      $ = "foo"
   ·      ┬  
   ·      ╰── this was not used in the condition
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
   strings:
     $a = "foo"
     $  = "bar"
   condition:
     for all of ($a) : ($)
}

"#,
            r#"error: unused string `$`
   ╭─[line:5:6]
   │
 5 │      $  = "bar"
   ·      ┬  
   ·      ╰── this was not used in the condition
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
   strings:
     $ = "foo"
   condition:
     $
}

"#,
            r#"error: syntax error
   ╭─[line:6:6]
   │
 6 │      $
   ·      ┬  
   ·      ╰── this `$` is outside of the condition of a `for .. of` statement
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
   condition: 99999999999999999999
}
"#,
            r#"error: invalid integer
   ╭─[line:3:15]
   │
 3 │    condition: 99999999999999999999
   ·               ──────────┬─────────  
   ·                         ╰─────────── this number is out of the valid range: [-9223372036854775808, 9223372036854775807]
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  condition: -99999999999999999999
}
"#,
            r#"error: invalid integer
   ╭─[line:3:14]
   │
 3 │   condition: -99999999999999999999
   ·              ──────────┬──────────  
   ·                        ╰──────────── this number is out of the valid range: [-9223372036854775808, 9223372036854775807]
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings:
    $a = { 11 [0-65536] 22 }
  condition:
    $a
}
"#,
            r#"error: invalid integer
   ╭─[line:4:18]
   │
 4 │     $a = { 11 [0-65536] 22 }
   ·                  ──┬──  
   ·                    ╰──── this number is out of the valid range: [0, 65535]
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings:
    $a = { 11 [-1-65535] 22 }
  condition:
    $a
}
"#,
            r#"error: invalid integer
   ╭─[line:4:16]
   │
 4 │     $a = { 11 [-1-65535] 22 }
   ·                ─┬  
   ·                 ╰── this number is out of the valid range: [0, 65535]
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings:
    $a = { 11 [2-1] 22 }
  condition:
    $a
}
"#,
            r#"error: invalid hex string `$a`
   ╭─[line:4:15]
   │
 4 │     $a = { 11 [2-1] 22 }
   ·               ──┬──  
   ·                 ╰──── lower bound (2) is greater than upper bound (1)
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings:
    $a = { 11 [1-2][40-38] 22 }
  condition:
    $a
}
"#,
            r#"error: invalid hex string `$a`
   ╭─[line:4:15]
   │
 4 │     $a = { 11 [1-2][40-38] 22 }
   ·               ──────┬─────  
   ·                     ╰─────── lower bound (41) is greater than upper bound (40)
   · 
   · Note: consecutive jumps were coalesced into a single one
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  condition: #a in (100..0)
}
"#,
            r#"error: invalid range
   ╭─[line:3:21]
   │
 3 │   condition: #a in (100..0)
   ·                     ─┬─  
   ·                      ╰─── lower bound (100) is greater than upper bound (100)
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  condition: #a in (-100..100)
}
"#,
            r#"error: invalid range
   ╭─[line:3:21]
   │
 3 │   condition: #a in (-100..100)
   ·                     ──┬─  
   ·                       ╰─── range bound can not be negative
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  condition: "\g" == "\g"
}
"#,
            r#"error: invalid escape sequence
   ╭─[line:3:15]
   │
 3 │   condition: "\g" == "\g"
   ·               ─┬  
   ·                ╰── invalid escape sequence `\g`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  condition: "\x" == "\x"
}
"#,
            r#"error: invalid escape sequence
   ╭─[line:3:15]
   │
 3 │   condition: "\x" == "\x"
   ·               ─┬  
   ·                ╰── expecting two hex digits after `\x`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  condition: "\xZZ" == "\xZZ"
}
"#,
            r#"error: invalid escape sequence
   ╭─[line:3:17]
   │
 3 │   condition: "\xZZ" == "\xZZ"
   ·                 ─┬  
   ·                  ╰── invalid hex value `ZZ` after `\x`
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings: 
    $a = "foo" xor(256)
  condition:
    $a
}
"#,
            r#"error: invalid integer
   ╭─[line:4:20]
   │
 4 │     $a = "foo" xor(256)
   ·                    ─┬─  
   ·                     ╰─── this number is out of the valid range: [0, 255]
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings: 
    $a = "foo" xor(0) xor(1-2)
  condition:
    $a
}
"#,
            r#"error: duplicate string modifier
   ╭─[line:4:23]
   │
 4 │     $a = "foo" xor(0) xor(1-2)
   ·                       ─┬─  
   ·                        ╰─── duplicate modifier
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings: 
    $a = "foo" xor(2-1)
  condition:
    $a
}
"#,
            r#"error: invalid range
   ╭─[line:4:20]
   │
 4 │     $a = "foo" xor(2-1)
   ·                    ┬  
   ·                    ╰── lower bound (2) is greater than upper bound (1)
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings: 
    $a = "foo" xor nocase
  condition:
    $a
}
"#,
            r#"error: invalid string modifier combination: `xor` `nocase`
   ╭─[line:4:16]
   │
 4 │     $a = "foo" xor nocase
   ·                ─┬─ ───┬──  
   ·                 ╰────────── `xor` modifier used here
   ·                       │    
   ·                       ╰──── `nocase` modifier used here
   · 
   · Note: these two modifiers can't be used together
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings: 
    $a = "foo" nocase ascii base64wide
  condition:
    $a
}
"#,
            r#"error: invalid string modifier combination: `base64wide` `nocase`
   ╭─[line:4:29]
   │
 4 │     $a = "foo" nocase ascii base64wide
   ·                ───┬──       ─────┬────  
   ·                   ╰───────────────────── `nocase` modifier used here
   ·                                  │      
   ·                                  ╰────── `base64wide` modifier used here
   · 
   · Note: these two modifiers can't be used together
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        (
            line!(),
            r#"
rule test {
  strings: 
    $a = { 01 02 } nocase
  condition:
    $a
}
"#,
            r#"error: invalid string modifier
   ╭─[line:4:20]
   │
 4 │     $a = { 01 02 } nocase
   ·                    ───┬──  
   ·                       ╰──── this modifier can't be applied to a hex string
───╯
"#,
        ),
        ////////////////////////////////////////////////////////////
        /*(
                    line!(),
                    r#"
        rule test {
          condition: not 2+2
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: not 2+2
          |                  ^^^ expression should be `boolean`, but is `integer`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: true or "false"
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: true or "false"
          |                      ^^^^^^^ expression should be `boolean`, but is `string`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: 1 + "1"
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: 1 + "1"
          |                  ^^^ expression should be `integer` or `float`, but is `string`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: 1 + -"1"
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: 1 + -"1"
          |                   ^^^ expression should be `integer` or `float`, but is `string`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: 1 << 2.0 + 2
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: 1 << 2.0 + 2
          |                   ^^^^^^^ expression should be `integer`, but is `float`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: 1 << -1 == 0
        }
        "#,
                    r#"error: unexpected negative number
          |
        3 |   condition: 1 << -1 == 0
          |                   ^^ expression should be a non-negative integer
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: $a at "1"
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: $a at "1"
          |                    ^^^ expression should be `integer`, but is `string`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: @a["1"] == 0x100
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: @a["1"] == 0x100
          |                 ^^^ expression should be `integer`, but is `string`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: @a[1]
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: @a[1]
          |              ^^^^^ expression should be `boolean`, but is `integer`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: !a[1]
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: !a[1]
          |              ^^^^^ expression should be `boolean`, but is `integer`
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: !a[-1]
        }
        "#,
                    r#"error: unexpected negative number
          |
        3 |   condition: !a[-1]
          |                 ^^ expression should be a non-negative integer
          |"#,
                ),
                ////////////////////////////////////////////////////////////
                (
                    line!(),
                    r#"
        rule test {
          condition: #a in (0.."10")
        }
        "#,
                    r#"error: wrong expression type
          |
        3 |   condition: #a in (0.."10")
          |                        ^^^^ expression should be `integer`, but is `string`
          |"#,
                ),*/
    ];

    for t in tests {
        assert_eq!(
            Parser::new().build_ast(t.1, None)
                .expect_err(&*format!(
                    "rule at line {} parsed without errors, but error was expected.\n\n",
                    t.0,
                ))
                .to_string(),
            t.2,
            "test at line {}", t.0
        )
    }
}