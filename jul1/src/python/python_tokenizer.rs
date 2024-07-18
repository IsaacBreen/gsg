use unicode_general_category::GeneralCategory;

use crate::{choice, Choice2, CombinatorTrait, dedent, dent, DynCombinator, eat_bytestring_choice, eat_char, eat_char_choice, eat_char_negation, eat_byte_range, eat_string, EatString, EatU8, eps, Eps, indent, mutate_right_data, MutateRightData, opt, repeat0, repeat1, Repeat1, RightData, seq, Seq2, symbol, Symbol, eat_char_negation_choice};
use crate::unicode::{get_unicode_general_category_bytestrings, get_unicode_general_category_combinator};

pub fn breaking_space() -> EatU8 {
    eat_char_choice("\n\r")
}

pub fn non_breaking_space() -> EatU8 {
    eat_char_choice(" \t")
}

pub fn whitespace() -> Repeat1<Choice2<Seq2<MutateRightData, EatU8>, Choice2<Seq2<EatString, EatU8>, EatU8>>> {
    repeat1(choice!(
        // If right_data.num_scopes > 0 then we can match a newline as a whitespace. Otherwise, we can't.
        seq!(
            mutate_right_data(|right_data| right_data.scope_count > 0),
            breaking_space()
        ),
        // But we can match an escaped newline.
        seq!(eat_string("\\"), breaking_space()),
        non_breaking_space()
    ))
}

pub fn python_symbol<A: CombinatorTrait>(a: A) -> Symbol<Box<DynCombinator>> {
    symbol(seq!(opt(whitespace()), a).into_boxed())
}

pub fn python_literal(s: &str) -> Symbol<Box<DynCombinator>> {
    let increment_scope_count = |right_data: &mut RightData| { right_data.scope_count += 1; true };
    let decrement_scope_count = |right_data: &mut RightData| { right_data.scope_count -= 1; true };

    match s {
        "(" | "[" | "{" => python_symbol(seq!(eat_string(s), mutate_right_data(increment_scope_count))),
        ")" | "]" | "}" => python_symbol(seq!(eat_string(s), mutate_right_data(decrement_scope_count))),
        _ => python_symbol(eat_string(s)),
    }
}

// https://docs.python.org/3/reference/lexical_analysis.html#identifiers
// identifier   ::=  xid_start xid_continue*
// id_start     ::=  <all characters in general categories Lu, Ll, Lt, Lm, Lo, Nl, the underscore, and characters with the Other_ID_Start property>
// id_continue  ::=  <all characters in id_start, plus characters in the categories Mn, Mc, Nd, Pc and others with the Other_ID_Continue property>
// xid_start    ::=  <all characters in id_start whose NFKC normalization is in "id_start xid_continue*">
// xid_continue ::=  <all characters in id_continue whose NFKC normalization is in "id_continue*">
//
// The Unicode category codes mentioned above stand for:
//
// Lu - uppercase letters
// Ll - lowercase letters
// Lt - titlecase letters
// Lm - modifier letters
// Lo - other letters
// Nl - letter numbers
// Mn - nonspacing marks
// Mc - spacing combining marks
// Nd - decimal numbers
// Pc - connector punctuations
// Other_ID_Start - explicit list of characters in PropList.txt to support backwards compatibility
// Other_ID_Continue - likewise
// pub fn id_start() -> Box<DynCombinator> {
//     // all characters in general categories Lu, Ll, Lt, Lm, Lo, Nl, the underscore, and characters with the Other_ID_Start property
//     let categories = [
//         GeneralCategory::UppercaseLetter,
//         GeneralCategory::LowercaseLetter,
//         GeneralCategory::TitlecaseLetter,
//         GeneralCategory::ModifierLetter,
//         GeneralCategory::OtherLetter,
//         GeneralCategory::LetterNumber,
//         // We ignore Other_ID_Start - it's just for backwards compatibility.
//     ];
//
//     let categories_combinator = choice_from_vec(categories.iter().map(|category| get_unicode_general_category_combinator(*category)).collect());
//     let other_combinator = eat_char_choice("_");
//
//     choice!(categories_combinator, other_combinator).into_boxed()
// }
//
// pub fn id_continue() -> Box<DynCombinator> {
//     // all characters in id_start, plus characters in the categories Mn, Mc, Nd, Pc and others with the Other_ID_Continue property
//     let new_categories = [
//         GeneralCategory::NonspacingMark,
//         // todo: where is SpacingCombiningMark?
//         // GeneralCategory::SpacingCombiningMark,
//         GeneralCategory::DecimalNumber,
//         GeneralCategory::ConnectorPunctuation,
//     ];
//
//     let new_categories_combinator = choice_from_vec(new_categories.iter().map(|category| get_unicode_general_category_combinator(*category)).collect());
//     let other_combinator = eat_byte_range(b'0', b'9');
//
//     choice!(id_start(), new_categories_combinator, other_combinator).into_boxed()
// }

pub fn id_start_bytestrings() -> Vec<Vec<u8>> {
    // all characters in general categories Lu, Ll, Lt, Lm, Lo, Nl, the underscore, and characters with the Other_ID_Start property
    let categories = [
        GeneralCategory::LowercaseLetter,
        GeneralCategory::UppercaseLetter,
        GeneralCategory::TitlecaseLetter,
        GeneralCategory::ModifierLetter,
        GeneralCategory::OtherLetter,
        GeneralCategory::LetterNumber,
        // We ignore Other_ID_Start - it's just for backwards compatibility.
    ];

    let category_bytestrings: Vec<Vec<u8>> = categories.iter().map(|category| get_unicode_general_category_bytestrings(*category)).flatten().collect();
    let other_bytestrings: Vec<Vec<u8>> = vec![vec![b'_']];

    category_bytestrings.into_iter().chain(other_bytestrings.into_iter()).collect()
}

pub fn id_continue_bytestrings() -> Vec<Vec<u8>> {
    // all characters in id_start, plus characters in the categories Mn, Mc, Nd, Pc and others with the Other_ID_Continue property
    let new_categories = [
        GeneralCategory::NonspacingMark,
        // todo: where is SpacingCombiningMark?
        // GeneralCategory::SpacingCombiningMark,
        GeneralCategory::DecimalNumber,
        GeneralCategory::ConnectorPunctuation,
    ];

    let new_category_bytestrings: Vec<Vec<u8>> = new_categories.iter().flat_map(|category| get_unicode_general_category_bytestrings(*category)).collect();

    let mut bytestrings = Vec::new();
    bytestrings.extend(id_start_bytestrings());
    bytestrings.extend(new_category_bytestrings);
    bytestrings
}

pub fn id_start() -> Box<DynCombinator> {
    eat_bytestring_choice(id_start_bytestrings()).into_boxed()
}

pub fn id_continue() -> Box<DynCombinator> {
    eat_bytestring_choice(id_continue_bytestrings()).into_boxed()
}

pub fn xid_start() -> Box<DynCombinator> {
    // all characters in id_start whose NFKC normalization is in "id_start xid_continue*"
    // Honestly, I don't know what this means.
    id_start()
}

pub fn xid_continue() -> Box<DynCombinator> {
    // all characters in id_continue whose NFKC normalization is in "id_continue*"
    // Honestly, I don't know what this means.
    id_continue()
}

pub fn NAME() -> Symbol<Box<DynCombinator>> {
    python_symbol(seq!(xid_start(), repeat0(xid_continue())))
}

pub fn TYPE_COMMENT() -> Symbol<Box<DynCombinator>> {
    python_symbol(seq!(eat_char('#'), repeat0(eat_char(' '))))
}

pub fn FSTRING_START() -> Symbol<Box<DynCombinator>> {
    python_symbol(eat_char('f'))
}

pub fn FSTRING_MIDDLE() -> Symbol<Box<DynCombinator>> {
    python_symbol(repeat1(eat_byte_range(b'0', b'9')))
}

pub fn FSTRING_END() -> Symbol<Box<DynCombinator>> {
    python_symbol(eat_char('"'))
}

pub fn SOFT_KEYWORD() -> Symbol<Box<DynCombinator>> {
    python_symbol(choice!(seq!(eat_char('i'), eat_char('f')), seq!(eat_char('e'), eat_char('l'))))
}

pub fn NUMBER() -> Symbol<Box<DynCombinator>> {
    python_symbol(choice!(repeat1(eat_byte_range(b'0', b'9')), seq!(repeat1(eat_byte_range(b'0', b'9')), seq!(eat_char('.'), repeat1(eat_byte_range(b'0', b'9'))))))
}

// https://docs.python.org/3/reference/lexical_analysis.html#strings
// String literals are described by the following lexical definitions:
//
// ```
// stringliteral   ::=  [stringprefix](shortstring | longstring)
// stringprefix    ::=  "r" | "u" | "R" | "U" | "f" | "F"
//                      | "fr" | "Fr" | "fR" | "FR" | "rf" | "rF" | "Rf" | "RF"
// shortstring     ::=  "'" shortstringitem* "'" | '"' shortstringitem* '"'
// longstring      ::=  "'''" longstringitem* "'''" | '"""' longstringitem* '"""'
// shortstringitem ::=  shortstringchar | stringescapeseq
// longstringitem  ::=  longstringchar | stringescapeseq
// shortstringchar ::=  <any source character except "\" or newline or the quote>
// longstringchar  ::=  <any source character except "\">
// stringescapeseq ::=  "\" <any source character>
// ```
//
// ```
// bytesliteral   ::=  bytesprefix(shortbytes | longbytes)
// bytesprefix    ::=  "b" | "B" | "br" | "Br" | "bR" | "BR" | "rb" | "rB" | "Rb" | "RB"
// shortbytes     ::=  "'" shortbytesitem* "'" | '"' shortbytesitem* '"'
// longbytes      ::=  "'''" longbytesitem* "'''" | '"""' longbytesitem* '"""'
// shortbytesitem ::=  shortbyteschar | bytesescapeseq
// longbytesitem  ::=  longbyteschar | bytesescapeseq
// shortbyteschar ::=  <any ASCII character except "\" or newline or the quote>
// longbyteschar  ::=  <any ASCII character except "\">
// bytesescapeseq ::=  "\" <any ASCII character>
// ```
//
// One syntactic restriction not indicated by these productions is that whitespace is not allowed between the stringprefix or bytesprefix and the rest of the literal. The source character set is defined by the encoding declaration; it is UTF-8 if no encoding declaration is given in the source file; see section Encoding declarations.
//
// In plain English: Both types of literals can be enclosed in matching single quotes (') or double quotes ("). They can also be enclosed in matching groups of three single or double quotes (these are generally referred to as triple-quoted strings). The backslash (\) character is used to give special meaning to otherwise ordinary characters like n, which means ‘newline’ when escaped (\n). It can also be used to escape characters that otherwise have a special meaning, such as newline, backslash itself, or the quote character. See escape sequences below for examples.
//
// Bytes literals are always prefixed with 'b' or 'B'; they produce an instance of the bytes type instead of the str type. They may only contain ASCII characters; bytes with a numeric value of 128 or greater must be expressed with escapes.
//
// Both string and bytes literals may optionally be prefixed with a letter 'r' or 'R'; such strings are called raw strings and treat backslashes as literal characters. As a result, in string literals, '\U' and '\u' escapes in raw strings are not treated specially. Given that Python 2.x’s raw unicode literals behave differently than Python 3.x’s the 'ur' syntax is not supported.
//
// Added in version 3.3: The 'rb' prefix of raw bytes literals has been added as a synonym of 'br'.
//
// Support for the unicode legacy literal (u'value') was reintroduced to simplify the maintenance of dual Python 2.x and 3.x codebases. See PEP 414 for more information.
//
// A string literal with 'f' or 'F' in its prefix is a formatted string literal; see f-strings. The 'f' may be combined with 'r', but not with 'b' or 'u', therefore raw formatted strings are possible, but formatted bytes literals are not.
//
// In triple-quoted literals, unescaped newlines and quotes are allowed (and are retained), except that three unescaped quotes in a row terminate the literal. (A “quote” is the character used to open the literal, i.e. either ' or ".)
pub fn STRING() -> Symbol<Box<DynCombinator>> {
    todo!()
}

pub fn NEWLINE() -> Symbol<Box<DynCombinator>> {
    let comment = seq!(eat_char('#'), repeat0(eat_char_negation_choice("\n\r")));
    let blank_line = seq!(repeat0(non_breaking_space()), opt(comment), breaking_space());
    python_symbol(seq!(repeat1(blank_line), dent()))
}

pub fn INDENT() -> Symbol<Box<DynCombinator>> {
    python_symbol(indent())
}

pub fn DEDENT() -> Symbol<Box<DynCombinator>> {
    python_symbol(dedent())
}

pub fn ENDMARKER() -> Symbol<Box<DynCombinator>> {
    python_symbol(eps())
}