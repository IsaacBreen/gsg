use crate::{choice, Choice2, CombinatorTrait, dedent, dent, DynCombinator, eat_char, eat_char_choice, eat_char_range, eat_string, EatString, EatU8, Eps, eps, indent, IndentCombinator, mutate_right_data, MutateRightData, IntoCombinator, newline, opt, repeat0, repeat1, Repeat1, RightData, seq, Seq2, symbol, Symbol};


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

pub fn xid_start() -> Symbol<Box<DynCombinator>> {
    python_symbol(choice!(eat_char('_'), eat_char_range(b'a', b'z'), eat_char_range(b'A', b'Z')))
}

pub fn xid_continue() -> Symbol<Box<DynCombinator>> {
    python_symbol(choice!(xid_start(), eat_char_range(b'0', b'9')))
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