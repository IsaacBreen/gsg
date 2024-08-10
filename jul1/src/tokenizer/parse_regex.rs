use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, opt, value},
    IResult,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
};
use nom::character::complete::none_of;
use nom::multi::many1;
use nom::sequence::tuple;

pub const special_chars: &str = "\\|[](){}?+*.^";

#[derive(Debug, PartialEq, Clone)]
pub enum ParsedRegex {
    Literal(char),
    AnyChar,
    CharClass(Vec<CharClassItem>),
    NegatedCharClass(Vec<CharClassItem>),
    Sequence(Vec<ParsedRegex>),
    Choice(Vec<ParsedRegex>),
    Group(Box<ParsedRegex>),
    ZeroOrMore(Box<ParsedRegex>),
    OneOrMore(Box<ParsedRegex>),
    Optional(Box<ParsedRegex>),
    PredefinedClass(CharClass),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CharClassItem {
    Single(char),
    Range(char, char),
    PredefinedClass(CharClass),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CharClass {
    Digit,
    Word,
    Space,
    NotDigit,
    NotWord,
    NotSpace,
}

pub fn parse_regex(input: &str) -> Result<ParsedRegex, nom::Err<nom::error::Error<&str>>> {
    regex_parser(input).map(|(_, r)| r)
}

pub fn regex_parser(input: &str) -> IResult<&str, ParsedRegex> {
    let (input, regex) = regex_expr(input)?;
    if input.is_empty() {
        Ok((input, regex))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )))
    }
}

fn regex_expr(input: &str) -> IResult<&str, ParsedRegex> {
    regex_choice(input)
}

fn regex_sequence(input: &str) -> IResult<&str, ParsedRegex> {
    let (input, seq) = many0(alt((
        regex_zero_or_more,
        regex_one_or_more,
        regex_optional,
        regex_group,
        regex_predefined_class,
        regex_char_class,
        regex_literal,
    )))(input)?;

    if seq.len() == 1 {
        Ok((input, seq[0].clone()))
    } else {
        Ok((input, ParsedRegex::Sequence(seq)))
    }
}

fn regex_zero_or_more(input: &str) -> IResult<&str, ParsedRegex> {
    let (input, expr) = terminated(regex_base, tag("*"))(input)?;
    Ok((input, ParsedRegex::ZeroOrMore(Box::new(expr))))
}

fn regex_one_or_more(input: &str) -> IResult<&str, ParsedRegex> {
    let (input, expr) = terminated(regex_base, tag("+"))(input)?;
    Ok((input, ParsedRegex::OneOrMore(Box::new(expr))))
}

fn regex_optional(input: &str) -> IResult<&str, ParsedRegex> {
    let (input, expr) = terminated(regex_base, tag("?"))(input)?;
    Ok((input, ParsedRegex::Optional(Box::new(expr))))
}

fn regex_predefined_class(input: &str) -> IResult<&str, ParsedRegex> {
    alt((
        value(ParsedRegex::PredefinedClass(CharClass::Digit), tag("\\d")),
        value(ParsedRegex::PredefinedClass(CharClass::Word), tag("\\w")),
        value(ParsedRegex::PredefinedClass(CharClass::Space), tag("\\s")),
        value(ParsedRegex::PredefinedClass(CharClass::NotDigit), tag("\\D")),
        value(ParsedRegex::PredefinedClass(CharClass::NotWord), tag("\\W")),
        value(ParsedRegex::PredefinedClass(CharClass::NotSpace), tag("\\S")),
    ))(input)
}

fn regex_base(input: &str) -> IResult<&str, ParsedRegex> {
    alt((
        regex_group,
        regex_predefined_class,
        regex_char_class,
        regex_literal,
    ))(input)
}

fn regex_literal(input: &str) -> IResult<&str, ParsedRegex> {
    alt((
        value(ParsedRegex::AnyChar, char('.')),
        map(
            preceded(
                char('\\'),
                alt((
                    // Valid escape sequences
                    one_of("nrt\\"),
                    // Escaped special characters
                    one_of(special_chars),
                )),
            ),
            |c| {
                match c {
                    'n' => ParsedRegex::Literal('\n'),
                    'r' => ParsedRegex::Literal('\r'),
                    't' => ParsedRegex::Literal('\t'),
                    _ => ParsedRegex::Literal(c),
                }
            }
        ),
        // Literal character (not backslash)
        map(none_of(special_chars), ParsedRegex::Literal),
    ))(input)
}

fn regex_char_class(input: &str) -> IResult<&str, ParsedRegex> {
    let parse_range = map(
        tuple((
            none_of("-[]\\"),                             // Disallow \x and other special chars
            char('-'),
            none_of("-[]\\"),                             // Disallow \x and other special chars
        )),
        |(start, _, end)| CharClassItem::Range(start, end),
    );

    let parse_single = map(
        alt((preceded(
            char('\\'),
            alt((
                map(one_of("ntr\\"), |c| match c {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    _ => unreachable!(),
                }),
            )),
        ),
             none_of("-]\\"))),
        CharClassItem::Single,
    );

    // \s \d \w \S \D \W
    let parse_char_class = map(
        alt((
            value(CharClass::Digit, tag("\\d")),
            value(CharClass::Word, tag("\\w")),
            value(CharClass::Space, tag("\\s")),
            value(CharClass::NotDigit, tag("\\D")),
            value(CharClass::NotWord, tag("\\W")),
            value(CharClass::NotSpace, tag("\\S")),
        )),
        CharClassItem::PredefinedClass
    );


    let parse_class_items = map(pair(opt(char('^')), many1(alt((parse_char_class, parse_range, parse_single)))), |(negated, items)| {
        if negated.is_some() {
            ParsedRegex::NegatedCharClass(items)
        } else {
            ParsedRegex::CharClass(items)
        }
    });

    delimited(
        char('['),
        parse_class_items,
        char(']'),
    )(input)
}

fn regex_choice(input: &str) -> IResult<&str, ParsedRegex> {
    let (input, choices) = separated_list1(tag("|"), regex_sequence)(input)?;
    if choices.len() == 1 {
        Ok((input, choices[0].clone()))
    } else {
        Ok((input, ParsedRegex::Choice(choices)))
    }
}

fn regex_group(input: &str) -> IResult<&str, ParsedRegex> {
    delimited(
        char('('),
        map(regex_expr, |r| ParsedRegex::Group(Box::new(r))),
        char(')'),
    )(input)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let result = regex_parser("a");
        assert_eq!(result, Ok(("", ParsedRegex::Literal('a'))));
    }

    #[test]
    fn test_any_char() {
        let result = regex_parser(".");
        assert_eq!(result, Ok(("", ParsedRegex::AnyChar)));
    }

    #[test]
    fn test_char_class() {
        let result = regex_parser("[a-z]");
        assert_eq!(result, Ok(("", ParsedRegex::CharClass(vec![CharClassItem::Range('a', 'z')]))));
    }

    #[test]
    fn test_negated_char_class() {
        let result = regex_parser("[^a-z]");
        assert_eq!(result, Ok(("", ParsedRegex::NegatedCharClass(vec![CharClassItem::Range('a', 'z')]))));
    }

    #[test]
    fn test_sequence() {
        let result = regex_parser("ab");
        assert_eq!(result, Ok(("", ParsedRegex::Sequence(vec![ParsedRegex::Literal('a'), ParsedRegex::Literal('b')]))));
    }

    #[test]
    fn test_choice() {
        let result = regex_parser("a|b");
        assert_eq!(result, Ok(("", ParsedRegex::Choice(vec![
            ParsedRegex::Literal('a'),
            ParsedRegex::Literal('b')
        ]))));
    }

    #[test]
    fn test_group() {
        let result = regex_parser("(a)");
        assert_eq!(result, Ok(("", ParsedRegex::Group(Box::new(ParsedRegex::Literal('a'))))));
    }

    #[test]
    fn test_zero_or_more() {
        let result = regex_parser("a*");
        assert_eq!(result, Ok(("", ParsedRegex::ZeroOrMore(Box::new(ParsedRegex::Literal('a'))))));
    }

    #[test]
    fn test_one_or_more() {
        let result = regex_parser("a+");
        assert_eq!(result, Ok(("", ParsedRegex::OneOrMore(Box::new(ParsedRegex::Literal('a'))))));
    }

    #[test]
    fn test_optional() {
        let result = regex_parser("a?");
        assert_eq!(result, Ok(("", ParsedRegex::Optional(Box::new(ParsedRegex::Literal('a'))))));
    }

    #[test]
    fn test_predefined_class() {
        let result = regex_parser("\\d");
        assert_eq!(result, Ok(("", ParsedRegex::PredefinedClass(CharClass::Digit))));
    }
}

#[cfg(test)]
mod more_complex_tests {
    use super::*;

    #[test]
    fn test_complex_sequence() {
        let result = regex_parser("abc[0-9]+(xyz)?");
        assert_eq!(result, Ok(("", ParsedRegex::Sequence(vec![
            ParsedRegex::Literal('a'),
            ParsedRegex::Literal('b'),
            ParsedRegex::Literal('c'),
            ParsedRegex::OneOrMore(Box::new(ParsedRegex::CharClass(vec![CharClassItem::Range('0', '9')]))),
            ParsedRegex::Optional(Box::new(ParsedRegex::Group(Box::new(ParsedRegex::Sequence(vec![
                ParsedRegex::Literal('x'),
                ParsedRegex::Literal('y'),
                ParsedRegex::Literal('z'),
            ])))))
        ]))));
    }

    #[test]
    fn test_complex_choice() {
        let result = regex_parser("(ab|cd)|efg");
        dbg!(&result);
        assert_eq!(result, Ok(("", ParsedRegex::Choice(vec![
            ParsedRegex::Group(Box::new(ParsedRegex::Choice(vec![
                ParsedRegex::Sequence(vec![ParsedRegex::Literal('a'), ParsedRegex::Literal('b')]),
                ParsedRegex::Sequence(vec![ParsedRegex::Literal('c'), ParsedRegex::Literal('d')]),
            ]))),
            ParsedRegex::Sequence(vec![ParsedRegex::Literal('e'), ParsedRegex::Literal('f'), ParsedRegex::Literal('g')]),
        ]))));
    }

    #[test]
    fn test_nested_groups() {
        let result = regex_parser("((a)b(c))");
        assert_eq!(result, Ok(("", ParsedRegex::Group(Box::new(ParsedRegex::Sequence(vec![
            ParsedRegex::Group(Box::new(ParsedRegex::Literal('a'))),
            ParsedRegex::Literal('b'),
            ParsedRegex::Group(Box::new(ParsedRegex::Literal('c'))),
        ]))))));
    }

    #[test]
    fn test_escaped_special_chars() {
        let result = regex_parser("\\(\\|\\[\\{");
        assert_eq!(result, Ok(("", ParsedRegex::Sequence(vec![
            ParsedRegex::Literal('('),
            ParsedRegex::Literal('|'),
            ParsedRegex::Literal('['),
            ParsedRegex::Literal('{'),
        ]))));
    }

    #[test]
    fn test_empty_regex() {
        let result = regex_parser("");
        assert_eq!(result, Ok(("", ParsedRegex::Sequence(vec![]))));
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_char_class() {
        let result = regex_parser("[]");
        dbg!(&result);
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_char_class() {
        let result = regex_parser("[a-z");
        dbg!(&result);
        assert!(result.is_err()); // Unclosed character classes are invalid
    }

    #[test]
    fn test_unmatched_parenthesis() {
        let result = regex_parser("(abc");
        assert!(result.is_err()); // Unmatched opening parenthesis
    }

    #[test]
    fn test_dangling_metacharacter() {
        let result = regex_parser("a*+");
        dbg!(&result);
        assert!(result.is_err()); // Invalid quantifier sequence
    }

    #[test]
    fn test_empty_group() {
        let result = regex_parser("()");
        dbg!(&result);
        assert_eq!(result, Ok(("", ParsedRegex::Group(Box::new(ParsedRegex::Sequence(vec![]))))));
    }

    #[test]
    fn test_invalid_escape_sequence() {
        let result = regex_parser("\\x");
        dbg!(&result);
        assert!(result.is_err()); // Invalid escape sequence (should be something like \d, \w, etc.)
    }

    #[test]
    fn test_leading_quantifier() {
        let result = regex_parser("*a");
        dbg!(&result);
        assert!(result.is_err()); // Quantifier cannot appear at the beginning
    }

    #[test]
    fn test_dot() {
        let result = regex_parser(".");
        dbg!(&result);
        assert_eq!(result, Ok(("", ParsedRegex::AnyChar)));
    }

    #[test]
    fn test_escaped_dot() {
        let result = regex_parser("\\.");
        dbg!(&result);
        assert_eq!(result, Ok(("", ParsedRegex::Literal('.'))));
    }

    #[test]
    fn test_indent() {
        let result = regex_parser(r"\n[^\S\n\r]*");
        dbg!(&result);
        assert_eq!(result, Ok(("", ParsedRegex::Sequence(vec![
            ParsedRegex::Literal('\n'),
            ParsedRegex::ZeroOrMore(Box::new(ParsedRegex::NegatedCharClass(vec![CharClassItem::Single('S'), CharClassItem::Single('\n'), CharClassItem::Single('\r')]))),
        ]))));
    }

    #[test]
    fn test_parse_the_beast() {
        let beast = vec![
            r"'.*'",
            r"'''[\s\S]*'''",
            r#"".*""#,
            r#""""[\s\S]*""""#,
        ].join("|");
        let result = regex_parser(&beast);
        dbg!(&result);
        assert_eq!(result, Ok(("", ParsedRegex::Choice(vec![
            ParsedRegex::Sequence(vec![
                ParsedRegex::Literal('\''),
                ParsedRegex::ZeroOrMore(Box::new(ParsedRegex::AnyChar)),
                ParsedRegex::Literal('\''),
            ]),
            ParsedRegex::Sequence(vec![
                ParsedRegex::Literal('\''),
                ParsedRegex::Literal('\''),
                ParsedRegex::Literal('\''),
                ParsedRegex::ZeroOrMore(Box::new(ParsedRegex::CharClass(vec![
                    CharClassItem::PredefinedClass(CharClass::Space),
                    CharClassItem::PredefinedClass(CharClass::NotSpace),
                ]))),
                ParsedRegex::Literal('\''),
                ParsedRegex::Literal('\''),
                ParsedRegex::Literal('\''),
            ]),
            ParsedRegex::Sequence(vec![
                ParsedRegex::Literal('"'),
                ParsedRegex::ZeroOrMore(Box::new(ParsedRegex::AnyChar)),
                ParsedRegex::Literal('"'),
            ]),
            ParsedRegex::Sequence(vec![
                ParsedRegex::Literal('"'),
                ParsedRegex::Literal('"'),
                ParsedRegex::Literal('"'),
                ParsedRegex::ZeroOrMore(Box::new(ParsedRegex::CharClass(vec![
                    CharClassItem::PredefinedClass(CharClass::Space),
                    CharClassItem::PredefinedClass(CharClass::NotSpace),
                ]))),
                ParsedRegex::Literal('"'),
                ParsedRegex::Literal('"'),
                ParsedRegex::Literal('"'),
            ]),
        ]))));
    }
}