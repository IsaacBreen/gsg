use unicode_general_category::GeneralCategory;

use crate::{choice, Choice2, CombinatorTrait, dedent, dent, DynCombinator, eat_bytestring_choice, eat_char, eat_char_choice, eat_char_range, eat_string, EatString, EatU8, eps, indent, mutate_right_data, MutateRightData, newline, opt, repeat0, repeat1, Repeat1, RightData, seq, Seq2, symbol, Symbol};
use crate::unicode::{get_unicode_general_category_bytestrings, get_unicode_general_category_combinator};

pub fn whitespace() -> Repeat1<Choice2<Seq2<MutateRightData, EatU8>, Choice2<EatString, EatU8>>> {
    repeat1(choice!(
        // If right_data.num_scopes > 0 then we can match a newline as a whitespace. Otherwise, we can't.
        seq!(
            mutate_right_data(|right_data| right_data.scope_count > 0),
            eat_char('\n')
        ),
        // But we can match an escaped newline.
        eat_string("\\\n"),
        eat_char_choice(" \t")
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
//     let other_combinator = eat_char_range(b'0', b'9');
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
    python_symbol(repeat1(eat_char_range(b'0', b'9')))
}

pub fn FSTRING_END() -> Symbol<Box<DynCombinator>> {
    python_symbol(eat_char('"'))
}

pub fn SOFT_KEYWORD() -> Symbol<Box<DynCombinator>> {
    python_symbol(choice!(seq!(eat_char('i'), eat_char('f')), seq!(eat_char('e'), eat_char('l'))))
}

pub fn NUMBER() -> Symbol<Box<DynCombinator>> {
    python_symbol(choice!(repeat1(eat_char_range(b'0', b'9')), seq!(repeat1(eat_char_range(b'0', b'9')), seq!(eat_char('.'), repeat1(eat_char_range(b'0', b'9'))))))
}

pub fn STRING() -> Symbol<Box<DynCombinator>> {
    python_symbol(choice!(seq!(eat_char('"'), repeat0(eat_char('\\')), eat_char('"')), seq!(eat_char('\''), repeat0(eat_char('\\')), eat_char('\''))))
}

pub fn NEWLINE() -> Symbol<Box<DynCombinator>> {
    python_symbol(seq!(repeat1(newline()), dent()))
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