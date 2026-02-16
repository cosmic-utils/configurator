use std::error::Error;

use crate::Map;
use crate::Value;
use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::take_while,
    bytes::complete::{escaped_transform, is_not, tag},
    character::complete::{char, digit1, multispace0},
    combinator::{map, value},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
};
use std::borrow::Cow;

fn deserialize(input: &str) -> Result<Value, Box<dyn Error>> {
    todo!()
}

fn ws<'a, P, O>(
    inner: P,
) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>> + 'a
where
    O: 'a,
    P: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>> + 'a,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_unit(input: &str) -> IResult<&str, Value> {
    value(Value::Unit, ws(tag("()"))).parse(input)
}

fn parse_bool(input: &str) -> IResult<&str, Value> {
    alt((
        value(Value::Bool(true), ws(tag("true"))),
        value(Value::Bool(false), ws(tag("false"))),
    ))
    .parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    map(ws(digit1), |s: &str| Value::from(s.parse::<i64>().unwrap())).parse(input)
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

fn parse_bytes(input: &str) -> IResult<&str, Value> {
    let inner = escaped_transform(
        is_not("\\\""),
        '\\',
        alt((
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\n", tag("n")),
            value("\r", tag("r")),
            value("\t", tag("t")),
        )),
    );

    map(
        preceded(ws(tag("b")), delimited(char('"'), inner, char('"'))),
        |s: String| Value::Bytes(s.into_bytes()),
    )
    .parse(input)
}

fn parse_list(input: &str) -> IResult<&str, Value> {
    map(
        delimited(
            ws(char('[')),
            separated_list0(ws(char(',')), parse_value),
            ws(char(']')),
        ),
        Value::List,
    )
    .parse(input)
}

fn parse_tuple(input: &str) -> IResult<&str, Value> {
    map(
        delimited(
            ws(char('(')),
            separated_list0(ws(char(',')), parse_value),
            ws(char(')')),
        ),
        Value::Tuple,
    )
    .parse(input)
}
fn parse_option(input: &str) -> IResult<&str, Value> {
    alt((
        value(Value::Option(None), ws(tag("None"))),
        map(
            preceded(
                ws(tag("Some")),
                delimited(char('('), parse_value, char(')')),
            ),
            |v| Value::Option(Some(Box::new(v))),
        ),
    ))
    .parse(input)
}

fn parse_ident(input: &str) -> IResult<&str, String> {
    use nom::character::complete::alphanumeric1;
    use nom::character::complete::char as ch;
    use nom::combinator::recognize;
    use nom::multi::many0;
    // identifier: alphanumeric and underscores, starting with alpha
    let (rest, s): (&str, &str) = recognize(tuple((
        nom::character::complete::alpha1,
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    )))
    .parse(input)?;
    Ok((rest, s.to_string()))
}

fn parse_map(input: &str) -> IResult<&str, Value> {
    let (rest, entries) = delimited(
        ws(char('{')),
        separated_list0(
            ws(char(',')),
            separated_pair(ws(parse_string), ws(char(':')), ws(parse_value)),
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
    let (rest, (name, vec)) = tuple((
        ws(parse_ident),
        delimited(
            ws(char('(')),
            separated_list0(ws(char(',')), parse_value),
            ws(char(')')),
        ),
    ))(input)?;
    Ok((rest, Value::EnumTuple(Cow::Owned(name), vec)))
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        parse_unit,
        parse_bool,
        parse_option,
        parse_number,
        parse_bytes,
        parse_char,
        parse_string,
        parse_list,
        parse_tuple,
        parse_map,
        parse_unit_struct_or_enum,
        parse_enum_tuple,
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
