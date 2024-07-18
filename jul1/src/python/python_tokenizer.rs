use crate::{choice, Choice2, eat_char, eat_char_range, EatU8, Eps, repeat0, repeat1, Repeat1, seq, Seq2};

// Define character ranges and specific characters for the Python tokenizer

pub fn xid_start() -> Choice2<EatU8, Choice2<EatU8, EatU8>> {
    choice!(eat_char('_'), eat_char_range(b'a', b'z'), eat_char_range(b'A', b'Z'))
}

pub fn xid_continue() -> Choice2<Choice2<EatU8, Choice2<EatU8, EatU8>>, EatU8> {
    choice!(xid_start(), eat_char_range(b'0', b'9'))
}

pub fn NAME() -> Seq2<Choice2<EatU8, Choice2<EatU8, EatU8>>, Choice2<Repeat1<Choice2<Choice2<EatU8, Choice2<EatU8, EatU8>>, EatU8>>, Eps>> {
    seq!(xid_start(), repeat0(xid_continue()))
}

pub fn TYPE_COMMENT() -> Seq2<EatU8, Choice2<Repeat1<EatU8>, Eps>> {
    seq!(eat_char('#'), repeat0(eat_char(' ')))
}

pub fn FSTRING_START() -> EatU8 {
    eat_char('f')
}

pub fn FSTRING_MIDDLE() -> Repeat1<EatU8> {
    repeat1(eat_char_range(b'0', b'9'))
}

pub fn FSTRING_END() -> EatU8 {
    eat_char('"')
}

pub fn SOFT_KEYWORD() -> Choice2<Seq2<EatU8, EatU8>, Seq2<EatU8, EatU8>> {
    choice!(seq!(eat_char('i'), eat_char('f')), seq!(eat_char('e'), eat_char('l')))
}

pub fn NUMBER() -> Choice2<Repeat1<EatU8>, Seq2<Repeat1<EatU8>, Seq2<EatU8, Repeat1<EatU8>>>> {
    choice!(repeat1(eat_char_range(b'0', b'9')), seq!(repeat1(eat_char_range(b'0', b'9')), seq!(eat_char('.'), repeat1(eat_char_range(b'0', b'9')))))
}

pub fn STRING() -> Choice2<Seq2<EatU8, Seq2<Choice2<Repeat1<EatU8>, Eps>, EatU8>>, Seq2<EatU8, Seq2<Choice2<Repeat1<EatU8>, Eps>, EatU8>>> {
    choice!(seq!(eat_char('"'), repeat0(eat_char('\\')), eat_char('"')), seq!(eat_char('\''), repeat0(eat_char('\\')), eat_char('\'')))
}
