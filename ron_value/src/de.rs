use std::error::Error;

use crate::Map;
use crate::Value;
use nom::Parser;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while1;
use nom::character::complete::line_ending;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::take_while,
    bytes::complete::{escaped_transform, is_not, tag},
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, value},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, terminated},
};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum DeserializeError<'a> {
    Parse(nom::Err<nom::error::Error<&'a str>>),
    TrailingInput(String),
}

impl fmt::Display for DeserializeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError::Parse(s) => write!(f, "parse error: {}", s),
            DeserializeError::TrailingInput(s) => write!(f, "trailing input is not empty: {}", s),
        }
    }
}

impl std::error::Error for DeserializeError<'_> {}

pub fn from_str<'a>(input: &'a str) -> Result<Value, DeserializeError<'a>> {
    match parse_value(input) {
        Ok((rest, v)) => {
            if rest.trim().is_empty() {
                Ok(v)
            } else {
                Err(DeserializeError::TrailingInput(rest.to_string()))
            }
        }
        Err(e) => Err(DeserializeError::Parse(e)),
    }
}

fn ws(input: &str) -> IResult<&str, ()> {
    value((), many0(alt((ws_single, comment)))).parse(input)
}

fn ws_single(input: &str) -> IResult<&str, ()> {
    value(
        (),
        take_while1(|c| {
            matches!(
                c,
                '\n' | '\t'
                    | '\r'
                    | ' '
                    | '\u{000B}'
                    | '\u{000C}'
                    | '\u{0085}'
                    | '\u{200E}'
                    | '\u{200F}'
                    | '\u{2028}'
                    | '\u{2029}'
            )
        }),
    )
    .parse(input)
}

fn comment(input: &str) -> IResult<&str, ()> {
    alt((line_comment, block_comment)).parse(input)
}

fn line_comment(input: &str) -> IResult<&str, ()> {
    value(
        (),
        terminated(preceded(tag("//"), take_until("\n")), line_ending),
    )
    .parse(input)
}

fn block_comment(input: &str) -> IResult<&str, ()> {
    value((), delimited(tag("/*"), nested_block_comment, tag("*/"))).parse(input)
}

fn nested_block_comment(input: &str) -> IResult<&str, ()> {
    let mut i = input;

    loop {
        // consume anything until we see /* or */
        let (i2, _) = take_until::<_, _, nom::error::Error<_>>("/*")(i).or_else(|_| Ok(("", i)))?;
        i = i2;

        if let Ok((after_open, _)) = tag::<_, _, nom::error::Error<_>>("/*")(i) {
            // recurse into nested comment
            let (after_nested, _) = nested_block_comment(after_open)?;
            let (after_close, _) = tag("*/")(after_nested)?;
            i = after_close;
            continue;
        }

        break;
    }

    Ok((i, ()))
}

fn comma(input: &str) -> IResult<&str, ()> {
    value((), delimited(ws, tag(","), ws)).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    map(digit1, |s: &str| Value::from(s.parse::<i64>().unwrap())).parse(input)
}

fn parse_string(input: &str) -> IResult<&str, Value> {
    let inner = escaped_transform(
        is_not("\\\""),
        '\\',
        alt((
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\n", tag("n")),
        )),
    );

    map(delimited(char('"'), inner, char('"')), Value::String).parse(input)
}

fn parse_bool(input: &str) -> IResult<&str, Value> {
    alt((
        value(Value::Bool(true), tag("true")),
        value(Value::Bool(false), tag("false")),
    ))
    .parse(input)
}

fn parse_char(input: &str) -> IResult<&str, Value> {
    let inner = escaped_transform(
        is_not("\\'"),
        '\\',
        alt((
            value("\\", tag("\\")),
            value("'", tag("'")),
            value("\n", tag("n")),
            value("\r", tag("r")),
            value("\t", tag("t")),
        )),
    );

    map(delimited(char('\''), inner, char('\'')), |s: String| {
        let c = s.chars().next().unwrap();
        Value::Char(c)
    })
    .parse(input)
}

fn parse_option(input: &str) -> IResult<&str, Value> {
    alt((
        map(tag("None"), |_| Value::Option(None)),
        parse_option_some,
    )).parse(input)
}

fn parse_option_some(input: &str) -> IResult<&str, Value> {
    map(
        tuple((
            tag("Some"),
            ws,
            tag("("),
            ws,
            value,
            ws,
            tag(")"),
        )),
        |(_, _, _, _, v, _, _)| Value::Option(Some(Box::new(v))),
    ).parse(input)
}




fn parse_list(input: &str) -> IResult<&str, Value> {
    map(
        delimited(
            ws(char('[')),
            terminated(
                separated_list0(ws(char(',')), parse_value),
                opt(ws(char(','))),
            ),
            ws(char(']')),
        ),
        Value::List,
    )
    .parse(input)
}

fn parse_tuple(input: &str) -> IResult<&str, Value> {
    // First attempt: parse an anonymous struct-like list inside parentheses: `(k: v, ...)`
    let struct_like = map(
        delimited(
            ws(char('(')),
            terminated(
                separated_list0(
                    ws(char(',')),
                    separated_pair(ws(parse_ident), ws(char(':')), ws(parse_value)),
                ),
                opt(ws(char(','))),
            ),
            ws(char(')')),
        ),
        |entries: Vec<(String, Value)>| {
            let map: crate::Map<std::borrow::Cow<'static, str>> = entries
                .into_iter()
                .map(|(k, v)| (std::borrow::Cow::Owned(k), v))
                .collect();

            Value::Tuple(vec![Value::Struct(None, map)])
        },
    );

    let normal = map(
        delimited(
            ws(char('(')),
            terminated(
                separated_list0(ws(char(',')), parse_value),
                opt(ws(char(','))),
            ),
            ws(char(')')),
        ),
        Value::Tuple,
    );

    alt((struct_like, normal)).parse(input)
}

fn parse_ident(input: &str) -> IResult<&str, String> {
    use nom::character::complete::alphanumeric1;
    use nom::character::complete::char as ch;
    use nom::combinator::recognize;
    use nom::multi::many0;
    // identifier: alphanumeric and underscores, starting with alpha
    let (rest, s): (&str, &str) = recognize((
        nom::character::complete::alpha1,
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ))
    .parse(input)?;
    Ok((rest, s.to_string()))
}

fn parse_map(input: &str) -> IResult<&str, Value> {
    let (rest, entries) = delimited(
        ws(char('{')),
        terminated(
            separated_list0(
                ws(char(',')),
                separated_pair(ws(parse_string), ws(char(':')), ws(parse_value)),
            ),
            opt(ws(char(','))),
        ),
        ws(char('}')),
    )
    .parse(input)?;

    let map: Map<Value> = entries.into_iter().map(|(k, v)| (k, v)).collect();

    Ok((rest, Value::Map(map)))
}

fn parse_unit_struct_or_enum(input: &str) -> IResult<&str, Value> {
    let (rest, name) = parse_ident(input)?;
    Ok((rest, Value::UnitStructOrEnum(Cow::Owned(name))))
}

fn parse_enum_tuple(input: &str) -> IResult<&str, Value> {
    let (rest, (name, vec)) = (
        ws(parse_ident),
        delimited(
            ws(char('(')),
            separated_list0(ws(char(',')), parse_value),
            ws(char(')')),
        ),
    )
        .parse(input)?;
    Ok((rest, Value::EnumTuple(Cow::Owned(name), vec)))
}

fn parse_struct_or_enum_named(input: &str) -> IResult<&str, Value> {
    let (rest, (name, entries)) = (
        ws(parse_ident),
        delimited(
            ws(char('{')),
            terminated(
                separated_list0(
                    ws(char(',')),
                    separated_pair(ws(parse_ident), ws(char(':')), ws(parse_value)),
                ),
                opt(ws(char(','))),
            ),
            ws(char('}')),
        ),
    )
        .parse(input)?;

    let map: crate::Map<std::borrow::Cow<'static, str>> = entries
        .into_iter()
        .map(|(k, v)| (std::borrow::Cow::Owned(k), v))
        .collect();

    Ok((
        rest,
        Value::Struct(Some(std::borrow::Cow::Owned(name)), map),
    ))
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        parse_bool,
        parse_option,
        parse_number,
        parse_char,
        parse_string,
        parse_list,
        parse_tuple,
        parse_map,
        parse_struct_or_enum_named,
        parse_enum_tuple,
        parse_unit_struct_or_enum,
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Number;

    #[test]
    fn parse_unit_ok() {
        assert_eq!(parse_unit("()"), Ok(("", Value::Unit)));
    }

    #[test]
    fn parse_bools_ok() {
        assert_eq!(parse_bool("true"), Ok(("", Value::Bool(true))));
        assert_eq!(parse_bool("false"), Ok(("", Value::Bool(false))));
    }

    #[test]
    fn parse_number_ok() {
        assert_eq!(
            parse_number("  42 "),
            Ok(("", Value::Number(Number::I64(42))))
        );
    }

    #[test]
    fn parse_string_ok() {
        assert_eq!(
            parse_string("\"hello\""),
            Ok(("", Value::String(String::from("hello"))))
        );
    }

    #[test]
    fn parse_list_ok() {
        let expected = Value::List(vec![
            Value::Number(Number::I64(1)),
            Value::Number(Number::I64(2)),
            Value::Number(Number::I64(3)),
        ]);
        assert_eq!(parse_list("[1, 2,3]"), Ok(("", expected)));
    }

    #[test]
    fn parse_tuple_ok() {
        let expected = Value::Tuple(vec![
            Value::Number(Number::I64(1)),
            Value::String(String::from("a")),
        ]);
        assert_eq!(parse_tuple("(1, \"a\")"), Ok(("", expected)));
    }

    #[test]
    fn parse_option_ok() {
        assert_eq!(parse_option("None"), Ok(("", Value::Option(None))));
        assert_eq!(
            parse_option("Some(5)"),
            Ok((
                "",
                Value::Option(Some(Box::new(Value::Number(Number::I64(5)))))
            ))
        );
    }

    #[test]
    fn parse_value_dispatch() {
        assert_eq!(parse_value("true"), Ok(("", Value::Bool(true))));
        assert_eq!(
            parse_value("123"),
            Ok(("", Value::Number(Number::I64(123))))
        );
    }
}
