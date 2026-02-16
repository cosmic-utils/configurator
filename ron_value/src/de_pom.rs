use core::num;
use std::num::NonZero;
use std::num::TryFromIntError;

use crate::Number;
use crate::Value;
use pom::Parser;
use pom::parser::*;

// #[derive(Debug, PartialEq, Eq)]
// pub enum DeserializeError {}

// pub fn from_str<'a>(input: &'a str) -> Result<Value, DeserializeError> {}

fn ws() -> Parser<char, ()> {
    (ws_single() | comment()).repeat(0..).discard()
}

fn ws_single() -> Parser<char, ()> {
    one_of("\n\t\r \u{000B}\u{000C}\u{0085}\u{200E}\u{200F}\u{2028}\u{2029}").discard()
}

fn comment() -> Parser<char, ()> {
    line_comment() | block_comment()
}

fn line_comment() -> Parser<char, ()> {
    (seq(&['/', '/']) * none_of("\n").repeat(0..) - sym('\n')).discard()
}

fn block_comment() -> Parser<char, ()> {
    (seq(&['/', '*']) * nested_block_comment() - seq(&['*', '/'])).discard()
}

fn nested_block_comment() -> Parser<char, ()> {
    Parser::new(|input, start| {
        let mut pos = start;

        while pos < input.len() {
            // stop if we see end delimiter
            if pos + 1 < input.len() {
                if input[pos] == '*' && input[pos + 1] == '/' {
                    break;
                }
            }

            // nested block
            if pos + 1 < input.len() && input[pos] == '/' && input[pos + 1] == '*' {
                let (_, next) = block_comment().parse_at(input, pos)?;
                pos = next;
                continue;
            }

            // consume one char
            pos += 1;
        }

        Ok(((), pos))
    })
}

fn comma() -> Parser<char, ()> {
    (ws() * sym(',') - ws()).discard()
}

fn digit() -> Parser<char, char> {
    one_of("0123456789")
}

fn digit_binary() -> Parser<char, char> {
    one_of("01")
}

fn digit_octal() -> Parser<char, char> {
    one_of("01234567")
}

fn digit_hexadecimal() -> Parser<char, char> {
    one_of("0123456789ABCDEFabcdef")
}

fn integer() -> Parser<char, Number> {
    let sign = (sym('-').map(|_| -1) | sym('+').map(|_| 1))
        .opt()
        .map(|s| s.unwrap_or(1));

    (sign + unsigned_decimal() + integer_suffix().opt()).convert(|((sign, digits), suffix)| {
        let default = if sign == 1 { "u64" } else { "i64" };

        let number = match suffix.unwrap_or(default) {
            "i8" => {
                let n: i8 = digits.try_into()?;
                Number::I8(n * sign)
            }
            "i16" => {
                let n: i16 = digits.try_into()?;
                Number::I16(n * sign as i16)
            }
            "i32" => {
                let n: i32 = digits.try_into()?;
                Number::I32(n * sign as i32)
            }
            "i64" => {
                let n: i64 = digits.try_into()?;
                Number::I64(n * sign as i64)
            }
            "i128" => {
                let n: i128 = digits.try_into()?;
                Number::I128(n * sign as i128)
            }
            "u8" => Number::U8(digits.try_into()?),
            "u16" => Number::U16(digits.try_into()?),
            "u32" => Number::U32(digits.try_into()?),
            "u64" => Number::U64(digits.try_into()?),
            "u128" => Number::U128(digits.try_into()?),
            _ => unreachable!(),
        };

        Ok::<Number, TryFromIntError>(number)
    })
}

fn integer_suffix() -> Parser<char, &'static str> {
    seq(&['i', '8']).map(|_| "i8")
        | seq(&['i', '1', '6']).map(|_| "i16")
        | seq(&['i', '3', '2']).map(|_| "i32")
        | seq(&['i', '6', '4']).map(|_| "i64")
        | seq(&['i', '1', '2', '8']).map(|_| "i128")
        | seq(&['u', '8']).map(|_| "u8")
        | seq(&['u', '1', '6']).map(|_| "u16")
        | seq(&['u', '3', '2']).map(|_| "u32")
        | seq(&['u', '6', '4']).map(|_| "u64")
        | seq(&['u', '1', '2', '8']).map(|_| "u128")
}

fn unsigned() -> Parser<char, u128> {
    unsigned_binary() | unsigned_octal() | unsigned_hexadecimal() | unsigned_decimal()
}

fn unsigned_binary() -> Parser<char, u128> {
    (seq(&['0', 'b']) * digit_binary() + (digit_binary() | sym('_')).repeat(0..)).map(|s| {
        let mut res = String::new();
        res.push(s.0);

        for c in s.1 {
            if c != '_' {
                res.push(c);
            }
        }
        u128::from_str_radix(&res, 2).unwrap()
    })
}

fn unsigned_octal() -> Parser<char, u128> {
    (seq(&['0', 'o']) * digit_octal() + (digit_octal() | sym('_')).repeat(0..)).map(|s| {
        let mut res = String::new();
        res.push(s.0);

        for c in s.1 {
            if c != '_' {
                res.push(c);
            }
        }
        u128::from_str_radix(&res, 8).unwrap()
    })
}
fn unsigned_hexadecimal() -> Parser<char, u128> {
    (seq(&['0', 'x']) * digit_hexadecimal() + (digit_hexadecimal() | sym('_')).repeat(0..)).map(
        |s| {
            let mut res = String::new();
            res.push(s.0);

            for c in s.1 {
                if c != '_' {
                    res.push(c);
                }
            }
            u128::from_str_radix(&res, 16).unwrap()
        },
    )
}
fn unsigned_decimal() -> Parser<char, u128> {
    (digit() + (digit() | sym('_')).repeat(0..)).map(|s| {
        let mut res = String::new();
        res.push(s.0);

        for c in s.1 {
            if c != '_' {
                res.push(c);
            }
        }
        u128::from_str_radix(&res, 10).unwrap()
    })
}

fn byte() -> Parser<char, Number> {}

fn byte_content() -> Parser<char, Number> {}

fn float() -> Parser<char, Number> {}

fn float_num() -> Parser<char, Number> {}

fn float_int() -> Parser<char, Number> {}

fn float_std() -> Parser<char, Number> {}

fn float_frac() -> Parser<char, Number> {}

fn float_exp() -> Parser<char, Number> {}

fn float_suffix() -> Parser<char, Number> {}

pub fn string() -> Parser<char, String> {
    string_std() | string_raw()
}

fn string_std() -> Parser<char, String> {
    sym('"')
        * no_double_quote_or_escape()
            .repeat(0..)
            .map(|s| s.into_iter().collect::<String>())
        - sym('"')
}

fn no_double_quote_or_escape() -> Parser<char, char> {
    // \\ is present to fail and test in string_escape
    none_of("\"\\") | string_escape()
}

fn string_escape() -> Parser<char, char> {
    sym('\\') * (escape_ascii() | escape_byte() | escape_unicode())
}

fn escape_ascii() -> Parser<char, char> {
    one_of("'\"\\nrt0").map(|c| match c {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '0' => '\0',
        other => other,
    })
}

fn escape_byte() -> Parser<char, char> {
    sym('x')
        * one_of("0123456789abcdefABCDEF")
            .repeat(2..3) // exactly 2 hex digits
            .collect::<String>()
            .map(|s| u8::from_str_radix(&s, 16).unwrap() as char)
}

/// Unicode escape: \uNNNN...
fn escape_unicode() -> Parser<char, char> {
    sym('u')
        * one_of("0123456789abcdefABCDEF")
            .repeat(4..7) // 4–6 hex digits
            .collect::<String>()
            .map(|s| std::char::from_u32(u32::from_str_radix(&s, 16).unwrap()).unwrap())
}

/// Raw string: r" ... " or r#" ... "# etc.
fn string_raw() -> Parser<char, String> {
    sym('r') * string_raw_content()
}

/// Raw string content
fn string_raw_content() -> Parser<char, String> {
    // Match nested #'s or standard "..."
    // Strategy: count leading #'s and match same number at end
    Parser::new(|input, start| {
        let mut pos = start;
        let mut hash_count = 0;

        // Count leading #
        while pos < input.len() && input[pos] == '#' {
            hash_count += 1;
            pos += 1;
        }

        // Next must be '"'
        if pos >= input.len() || input[pos] != '"' {
            return Err(pom::Error::Unexpected {
                found: if pos >= input.len() {
                    None
                } else {
                    Some(input[pos])
                },
            });
        }

        pos += 1; // skip opening "

        let mut content = String::new();

        'outer: while pos < input.len() {
            if input[pos] == '"' {
                // Check for matching trailing #'s
                let mut match_hash = true;
                for i in 0..hash_count {
                    if pos + 1 + i >= input.len() || input[pos + 1 + i] != '#' {
                        match_hash = false;
                        break;
                    }
                }

                if match_hash {
                    pos += 1 + hash_count; // skip closing quote + hashes
                    break 'outer;
                } else {
                    content.push('"');
                    pos += 1;
                }
            } else {
                content.push(input[pos]);
                pos += 1;
            }
        }

        Ok((content, pos))
    })
}
