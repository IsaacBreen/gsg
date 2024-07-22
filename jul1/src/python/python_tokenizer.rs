use std::rc::Rc;
use unicode_general_category::GeneralCategory;

use crate::{choice, Choice2, CombinatorTrait, dedent, dent, DynCombinator, eat_bytestring_choice, eat_char, eat_char_choice, eat_char_negation, eat_byte_range, eat_string, EatString, EatU8, eps, Eps, indent, mutate_right_data, MutateRightData, opt, repeat0, repeat1, Repeat1, RightData, seq, Seq2, symbol, Symbol, eat_char_negation_choice, IndentCombinator, assert_no_dedents, tag, fail, IntoCombinator, seprep0, seprep1, prevent_consecutive_matches_clear, prevent_consecutive_matches, PreventConsecutiveMatches, PreventConsecutiveMatchesClear};
use crate::unicode::{get_unicode_general_category_bytestrings, get_unicode_general_category_combinator};

pub fn breaking_space() -> EatU8 {
    eat_char_choice("\n\r")
}

pub fn not_breaking_space() -> EatU8 {
    eat_char_negation_choice("\n\r")
}

pub fn non_breaking_space() -> EatU8 {
    eat_char_choice(" \t")
}

// .. _blank-lines:
//
// Blank lines
// -----------
//
// .. index:: single: blank line
//
// A logical line that contains only spaces, tabs, formfeeds and possibly a
// comment, is ignored (i.e., no NEWLINE token is generated).  During interactive
// input of statements, handling of a blank line may differ depending on the
// implementation of the read-eval-print loop.  In the standard interactive
// interpreter, an entirely blank logical line (i.e. one containing not even
// whitespace or a comment) terminates a multi-line statement.
//
// ...
//
// .. _whitespace:
//
// Whitespace between tokens
// -------------------------
//
// Except at the beginning of a logical line or in string literals, the whitespace
// characters space, tab and formfeed can be used interchangeably to separate
// tokens.  Whitespace is needed between two tokens only if their concatenation
// could otherwise be interpreted as a different token (e.g., ab is one token, but
// a b is two tokens).
pub fn whitespace() -> Seq2<PreventConsecutiveMatches, Repeat1<Choice2<Seq2<MutateRightData, EatU8>, Choice2<Seq2<EatString, EatU8>, EatU8>>>> {
    seq!(
        prevent_consecutive_matches("whitespace"),
        repeat1(choice!(
            // If right_data.num_scopes > 0 then we can match a newline as a whitespace. Otherwise, we can't.
            seq!(
                mutate_right_data(|right_data| right_data.scope_count > 0),
                breaking_space()
            ),
            // But we can match an escaped newline.
            seq!(eat_string("\\"), breaking_space()),
            non_breaking_space()
        )),
    )
}

pub fn python_symbol<A: CombinatorTrait>(a: A) -> Symbol<Box<DynCombinator>> {
    symbol(seq!(tag("assert_no_dedents()", assert_no_dedents()), tag("opt(whitespace()))", opt(whitespace())), a).into_box_dyn())
}

pub fn python_literal(s: &str) -> Symbol<Box<DynCombinator>> {
    let increment_scope_count = |right_data: &mut RightData| { right_data.scope_count += 1; true };
    let decrement_scope_count = |right_data: &mut RightData| { right_data.scope_count -= 1; true };

    match s {
        "(" | "[" | "{" => python_symbol(seq!(eat_string(s), mutate_right_data(increment_scope_count), prevent_consecutive_matches_clear())),
        ")" | "]" | "}" => python_symbol(seq!(eat_string(s), mutate_right_data(decrement_scope_count), prevent_consecutive_matches_clear())),
        _ => python_symbol(seq!(eat_string(s), prevent_consecutive_matches_clear())),
    }
}

// https://docs.python.org/3/reference/lexical_analysis.html#identifiers
// Identifiers and keywords
// ========================
//
// .. index:: identifier, name
//
// Identifiers (also referred to as *names*) are described by the following lexical
// definitions.
//
// The syntax of identifiers in Python is based on the Unicode standard annex
// UAX-31, with elaboration and changes as defined below; see also :pep:`3131` for
// further details.
//
// Within the ASCII range (U+0001..U+007F), the valid characters for identifiers
// are the same as in Python 2.x: the uppercase and lowercase letters ``A`` through
// ``Z``, the underscore ``_`` and, except for the first character, the digits
// ``0`` through ``9``.
//
// Python 3.0 introduces additional characters from outside the ASCII range (see
// :pep:`3131`).  For these characters, the classification uses the version of the
// Unicode Character Database as included in the :mod:`unicodedata` module.
//
// Identifiers are unlimited in length.  Case is significant.
//
// .. productionlist:: python-grammar
//    identifier: `xid_start` `xid_continue`*
//    id_start: <all characters in general categories Lu, Ll, Lt, Lm, Lo, Nl, the underscore, and characters with the Other_ID_Start property>
//    id_continue: <all characters in `id_start`, plus characters in the categories Mn, Mc, Nd, Pc and others with the Other_ID_Continue property>
//    xid_start: <all characters in `id_start` whose NFKC normalization is in "id_start xid_continue*">
//    xid_continue: <all characters in `id_continue` whose NFKC normalization is in "id_continue*">
//
// The Unicode category codes mentioned above stand for:
//
// * *Lu* - uppercase letters
// * *Ll* - lowercase letters
// * *Lt* - titlecase letters
// * *Lm* - modifier letters
// * *Lo* - other letters
// * *Nl* - letter numbers
// * *Mn* - nonspacing marks
// * *Mc* - spacing combining marks
// * *Nd* - decimal numbers
// * *Pc* - connector punctuations
// * *Other_ID_Start* - explicit list of characters in `PropList.txt
//   <https://www.unicode.org/Public/15.1.0/ucd/PropList.txt>`_ to support backwards
//   compatibility
// * *Other_ID_Continue* - likewise
//
// All identifiers are converted into the normal form NFKC while parsing; comparison
// of identifiers is based on NFKC.
//
// A non-normative HTML file listing all valid identifier characters for Unicode
// 15.1.0 can be found at
// https://www.unicode.org/Public/15.1.0/ucd/DerivedCoreProperties.txt
//
//
// .. _keywords:
//
// Keywords
// --------
//
// .. index::
//    single: keyword
//    single: reserved word
//
// The following identifiers are used as reserved words, or *keywords* of the
// language, and cannot be used as ordinary identifiers.  They must be spelled
// exactly as written here:
//
// .. sourcecode:: text
//
//    False      await      else       import     pass
//    None       break      except     in         raise
//    True       class      finally    is         return
//    and        continue   for        lambda     try
//    as         def        from       nonlocal   while
//    assert     del        global     not        with
//    async      elif       if         or         yield
//
//
// .. _soft-keywords:
//
// Soft Keywords
// -------------
//
// .. index:: soft keyword, keyword
//
// .. versionadded:: 3.10
//
// Some identifiers are only reserved under specific contexts. These are known as
// *soft keywords*.  The identifiers ``match``, ``case``, ``type`` and ``_`` can
// syntactically act as keywords in certain contexts,
// but this distinction is done at the parser level, not when tokenizing.
//
// As soft keywords, their use in the grammar is possible while still
// preserving compatibility with existing code that uses these names as
// identifier names.
//
// ``match``, ``case``, and ``_`` are used in the :keyword:`match` statement.
// ``type`` is used in the :keyword:`type` statement.
//
// .. versionchanged:: 3.12
//    ``type`` is now a soft keyword.
//
// .. index::
//    single: _, identifiers
//    single: __, identifiers
// .. _id-classes:
//
// Reserved classes of identifiers
// -------------------------------
//
// Certain classes of identifiers (besides keywords) have special meanings.  These
// classes are identified by the patterns of leading and trailing underscore
// characters:
//
// ``_*``
//    Not imported by ``from module import *``.
//
// ``_``
//    In a ``case`` pattern within a :keyword:`match` statement, ``_`` is a
//    :ref:`soft keyword <soft-keywords>` that denotes a
//    :ref:`wildcard <wildcard-patterns>`.
//
//    Separately, the interactive interpreter makes the result of the last evaluation
//    available in the variable ``_``.
//    (It is stored in the :mod:`builtins` module, alongside built-in
//    functions like ``print``.)
//
//    Elsewhere, ``_`` is a regular identifier. It is often used to name
//    "special" items, but it is not special to Python itself.
//
//    .. note::
//
//       The name ``_`` is often used in conjunction with internationalization;
//       refer to the documentation for the :mod:`gettext` module for more
//       information on this convention.
//
//       It is also commonly used for unused variables.
//
// ``__*__``
//    System-defined names, informally known as "dunder" names. These names are
//    defined by the interpreter and its implementation (including the standard library).
//    Current system names are discussed in the :ref:`specialnames` section and elsewhere.
//    More will likely be defined in future versions of Python.  *Any* use of ``__*__`` names,
//    in any context, that does not follow explicitly documented use, is subject to
//    breakage without warning.
//
// ``__*``
//    Class-private names.  Names in this category, when used within the context of a
//    class definition, are re-written to use a mangled form to help avoid name
//    clashes between "private" attributes of base and derived classes. See section
//    :ref:`atom-identifiers`.
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
    eat_bytestring_choice(id_start_bytestrings()).into_box_dyn()
}

pub fn id_continue() -> Box<DynCombinator> {
    eat_bytestring_choice(id_continue_bytestrings()).into_box_dyn()
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
    python_symbol(seq!(prevent_consecutive_matches("NAME"), xid_start(), repeat0(xid_continue())))
}

// .. _literals:
//
// Literals
// ========
//
// .. index:: literal, constant
//
// Literals are notations for constant values of some built-in types.
//
//
// .. index:: string literal, bytes literal, ASCII
//    single: ' (single quote); string literal
//    single: " (double quote); string literal
//    single: u'; string literal
//    single: u"; string literal
// .. _strings:
//
// String and Bytes literals
// -------------------------
//
// String literals are described by the following lexical definitions:
//
// .. productionlist:: python-grammar
//    stringliteral: [`stringprefix`](`shortstring` | `longstring`)
//    stringprefix: "r" | "u" | "R" | "U" | "f" | "F"
//                : | "fr" | "Fr" | "fR" | "FR" | "rf" | "rF" | "Rf" | "RF"
//    shortstring: "'" `shortstringitem`* "'" | '"' `shortstringitem`* '"'
//    longstring: "'''" `longstringitem`* "'''" | '"""' `longstringitem`* '"""'
//    shortstringitem: `shortstringchar` | `stringescapeseq`
//    longstringitem: `longstringchar` | `stringescapeseq`
//    shortstringchar: <any source character except "\" or newline or the quote>
//    longstringchar: <any source character except "\">
//    stringescapeseq: "\" <any source character>
//
// .. productionlist:: python-grammar
//    bytesliteral: `bytesprefix`(`shortbytes` | `longbytes`)
//    bytesprefix: "b" | "B" | "br" | "Br" | "bR" | "BR" | "rb" | "rB" | "Rb" | "RB"
//    shortbytes: "'" `shortbytesitem`* "'" | '"' `shortbytesitem`* '"'
//    longbytes: "'''" `longbytesitem`* "'''" | '"""' `longbytesitem`* '"""'
//    shortbytesitem: `shortbyteschar` | `bytesescapeseq`
//    longbytesitem: `longbyteschar` | `bytesescapeseq`
//    shortbyteschar: <any ASCII character except "\" or newline or the quote>
//    longbyteschar: <any ASCII character except "\">
//    bytesescapeseq: "\" <any ASCII character>
//
// One syntactic restriction not indicated by these productions is that whitespace
// is not allowed between the :token:`~python-grammar:stringprefix` or
// :token:`~python-grammar:bytesprefix` and the rest of the literal. The source
// character set is defined by the encoding declaration; it is UTF-8 if no encoding
// declaration is given in the source file; see section :ref:`encodings`.
//
// .. index:: triple-quoted string, Unicode Consortium, raw string
//    single: """; string literal
//    single: '''; string literal
//
// In plain English: Both types of literals can be enclosed in matching single quotes
// (``'``) or double quotes (``"``).  They can also be enclosed in matching groups
// of three single or double quotes (these are generally referred to as
// *triple-quoted strings*). The backslash (``\``) character is used to give special
// meaning to otherwise ordinary characters like ``n``, which means 'newline' when
// escaped (``\n``). It can also be used to escape characters that otherwise have a
// special meaning, such as newline, backslash itself, or the quote character.
// See :ref:`escape sequences <escape-sequences>` below for examples.
//
// .. index::
//    single: b'; bytes literal
//    single: b"; bytes literal
//
// Bytes literals are always prefixed with ``'b'`` or ``'B'``; they produce an
// instance of the :class:`bytes` type instead of the :class:`str` type.  They
// may only contain ASCII characters; bytes with a numeric value of 128 or greater
// must be expressed with escapes.
//
// .. index::
//    single: r'; raw string literal
//    single: r"; raw string literal
//
// Both string and bytes literals may optionally be prefixed with a letter ``'r'``
// or ``'R'``; such strings are called :dfn:`raw strings` and treat backslashes as
// literal characters.  As a result, in string literals, ``'\U'`` and ``'\u'``
// escapes in raw strings are not treated specially. Given that Python 2.x's raw
// unicode literals behave differently than Python 3.x's the ``'ur'`` syntax
// is not supported.
//
// .. versionadded:: 3.3
//    The ``'rb'`` prefix of raw bytes literals has been added as a synonym
//    of ``'br'``.
//
//    Support for the unicode legacy literal (``u'value'``) was reintroduced
//    to simplify the maintenance of dual Python 2.x and 3.x codebases.
//    See :pep:`414` for more information.
//
// .. index::
//    single: f'; formatted string literal
//    single: f"; formatted string literal
//
// A string literal with ``'f'`` or ``'F'`` in its prefix is a
// :dfn:`formatted string literal`; see :ref:`f-strings`.  The ``'f'`` may be
// combined with ``'r'``, but not with ``'b'`` or ``'u'``, therefore raw
// formatted strings are possible, but formatted bytes literals are not.
//
// In triple-quoted literals, unescaped newlines and quotes are allowed (and are
// retained), except that three unescaped quotes in a row terminate the literal.  (A
// "quote" is the character used to open the literal, i.e. either ``'`` or ``"``.)
//
// .. index:: physical line, escape sequence, Standard C, C
//    single: \ (backslash); escape sequence
//    single: \\; escape sequence
//    single: \a; escape sequence
//    single: \b; escape sequence
//    single: \f; escape sequence
//    single: \n; escape sequence
//    single: \r; escape sequence
//    single: \t; escape sequence
//    single: \v; escape sequence
//    single: \x; escape sequence
//    single: \N; escape sequence
//    single: \u; escape sequence
//    single: \U; escape sequence
pub fn STRING() -> Symbol<Box<DynCombinator>> {
    let stringprefix = choice!(
        eat_char_choice("ruRUfF"),
        choice!(
            seq!(eat_char_choice("fF"), eat_char_choice("rR")),
            seq!(eat_char_choice("rR"), eat_char_choice("fF"))
        )
    );

    let shortstring = choice!(
        seq!(eat_char('\''), repeat0(choice!(eat_char_negation_choice("\\\'\n"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))), eat_char('\'')),
        seq!(eat_char('"'), repeat0(choice!(eat_char_negation_choice("\\\"\n"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))), eat_char('"'))
    );

    let longstring = choice!(
        seq!(eat_string("'''"), repeat0(choice!(eat_char_negation('\\'), seq!(eat_char('\\'), eat_char_negation_choice("\0")))), eat_string("'''")),
        seq!(eat_string("\"\"\""), repeat0(choice!(eat_char_negation('\\'), seq!(eat_char('\\'), eat_char_negation_choice("\0")))), eat_string("\"\"\""))
    );

    python_symbol(seq!(opt(stringprefix), choice!(shortstring, longstring), prevent_consecutive_matches_clear()))
}

pub fn FSTRING_START() -> Symbol<Box<DynCombinator>> {
    let fstringprefix = choice!(
        eat_char_choice("fF"),
        seq!(eat_char_choice("fF"), eat_char_choice("rR")),
        seq!(eat_char_choice("rR"), eat_char_choice("fF"))
    );

    let fstring_start = choice!(
        seq!(eat_char('\''), repeat0(choice!(eat_char_negation_choice("\\\'{"), seq!(eat_char('\\'), eat_char_negation_choice("\0"))))),
        seq!(eat_char('"'), repeat0(choice!(eat_char_negation_choice("\\\"{"), seq!(eat_char('\\'), eat_char_negation_choice("\0"))))),
        seq!(eat_string("'''"), repeat0(choice!(eat_char_negation_choice("\\{"), seq!(eat_char('\\'), eat_char_negation_choice("\0"))))),
        seq!(eat_string("\"\"\""), repeat0(choice!(eat_char_negation_choice("\\{"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))))
    );

    python_symbol(seq!(fstringprefix, fstring_start, prevent_consecutive_matches_clear()))
}

pub fn FSTRING_MIDDLE() -> Symbol<Box<DynCombinator>> {
    let fstring_middle = choice!(
        repeat0(choice!(eat_char_negation_choice("\\\'{"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))),
        repeat0(choice!(eat_char_negation_choice("\\\"{"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))),
        repeat0(choice!(eat_char_negation_choice("\\{"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))),
        repeat0(choice!(eat_char_negation_choice("\\{"), seq!(eat_char('\\'), eat_char_negation_choice("\0"))))
    );

    python_symbol(seq!(fstring_middle, prevent_consecutive_matches_clear()))
}

pub fn FSTRING_END() -> Symbol<Box<DynCombinator>> {
    let fstring_end = choice!(
        seq!(repeat0(choice!(eat_char_negation_choice("\\\'{"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))), eat_char('\'')),
        seq!(repeat0(choice!(eat_char_negation_choice("\\\"{"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))), eat_char('"')),
        seq!(repeat0(choice!(eat_char_negation_choice("\\{"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))), eat_string("'''")),
        seq!(repeat0(choice!(eat_char_negation_choice("\\{"), seq!(eat_char('\\'), eat_char_negation_choice("\0")))), eat_string("\"\"\""))
    );

    python_symbol(seq!(fstring_end, prevent_consecutive_matches_clear()))
}

// .. _numbers:
//
// Numeric literals
// ----------------
//
// .. index:: number, numeric literal, integer literal
//    floating point literal, hexadecimal literal
//    octal literal, binary literal, decimal literal, imaginary literal, complex literal
//
// There are three types of numeric literals: integers, floating point numbers, and
// imaginary numbers.  There are no complex literals (complex numbers can be formed
// by adding a real number and an imaginary number).
//
// Note that numeric literals do not include a sign; a phrase like ``-1`` is
// actually an expression composed of the unary operator '``-``' and the literal
// ``1``.
//
//
// .. index::
//    single: 0b; integer literal
//    single: 0o; integer literal
//    single: 0x; integer literal
//    single: _ (underscore); in numeric literal
//
// .. _integers:
//
// Integer literals
// ----------------
//
// Integer literals are described by the following lexical definitions:
//
// .. productionlist:: python-grammar
//    integer: `decinteger` | `bininteger` | `octinteger` | `hexinteger`
//    decinteger: `nonzerodigit` (["_"] `digit`)* | "0"+ (["_"] "0")*
//    bininteger: "0" ("b" | "B") (["_"] `bindigit`)+
//    octinteger: "0" ("o" | "O") (["_"] `octdigit`)+
//    hexinteger: "0" ("x" | "X") (["_"] `hexdigit`)+
//    nonzerodigit: "1"..."9"
//    digit: "0"..."9"
//    bindigit: "0" | "1"
//    octdigit: "0"..."7"
//    hexdigit: `digit` | "a"..."f" | "A"..."F"
//
// There is no limit for the length of integer literals apart from what can be
// stored in available memory.
//
// Underscores are ignored for determining the numeric value of the literal.  They
// can be used to group digits for enhanced readability.  One underscore can occur
// between digits, and after base specifiers like ``0x``.
//
// Note that leading zeros in a non-zero decimal number are not allowed. This is
// for disambiguation with C-style octal literals, which Python used before version
// 3.0.
//
// Some examples of integer literals::
//
//    7     2147483647                        0o177    0b100110111
//    3     79228162514264337593543950336     0o377    0xdeadbeef
//          100_000_000_000                   0b_1110_0101
//
// .. versionchanged:: 3.6
//    Underscores are now allowed for grouping purposes in literals.
//
//
// .. index::
//    single: . (dot); in numeric literal
//    single: e; in numeric literal
//    single: _ (underscore); in numeric literal
// .. _floating:
//
// Floating point literals
// -----------------------
//
// Floating point literals are described by the following lexical definitions:
//
// .. productionlist:: python-grammar
//    floatnumber: `pointfloat` | `exponentfloat`
//    pointfloat: [`digitpart`] `fraction` | `digitpart` "."
//    exponentfloat: (`digitpart` | `pointfloat`) `exponent`
//    digitpart: `digit` (["_"] `digit`)*
//    fraction: "." `digitpart`
//    exponent: ("e" | "E") ["+" | "-"] `digitpart`
//
// Note that the integer and exponent parts are always interpreted using radix 10.
// For example, ``077e010`` is legal, and denotes the same number as ``77e10``. The
// allowed range of floating point literals is implementation-dependent.  As in
// integer literals, underscores are supported for digit grouping.
//
// Some examples of floating point literals::
//
//    3.14    10.    .001    1e100    3.14e-10    0e0    3.14_15_93
//
// .. versionchanged:: 3.6
//    Underscores are now allowed for grouping purposes in literals.
//
//
// .. index::
//    single: j; in numeric literal
// .. _imaginary:
//
// Imaginary literals
// ------------------
//
// Imaginary literals are described by the following lexical definitions:
//
// .. productionlist:: python-grammar
//    imagnumber: (`floatnumber` | `digitpart`) ("j" | "J")
//
// An imaginary literal yields a complex number with a real part of 0.0.  Complex
// numbers are represented as a pair of floating point numbers and have the same
// restrictions on their range.  To create a complex number with a nonzero real
// part, add a floating point number to it, e.g., ``(3+4j)``.  Some examples of
// imaginary literals::
//
//    3.14j   10.j    10j     .001j   1e100j   3.14e-10j   3.14_15_93j
pub fn NUMBER() -> Symbol<Box<DynCombinator>> {
    let digit = eat_byte_range(b'0', b'9');
    let nonzerodigit = eat_byte_range(b'1', b'9');
    let bindigit = eat_byte_range(b'0', b'1');
    let octdigit = eat_byte_range(b'0', b'7');
    let hexdigit = choice!(digit, eat_byte_range(b'a', b'f'), eat_byte_range(b'A', b'F'));

    let decinteger = choice!(
        seq!(nonzerodigit, repeat0(seq!(opt(eat_char('_')), digit))),
        seq!(repeat1(eat_char('0')), repeat0(seq!(opt(eat_char('_')), eat_char('0'))))
    );
    let bininteger = seq!(eat_char('0'), eat_char_choice("bB"), repeat1(seq!(opt(eat_char('_')), bindigit)));
    let octinteger = seq!(eat_char('0'), eat_char_choice("oO"), repeat1(seq!(opt(eat_char('_')), octdigit)));
    let hexinteger = seq!(eat_char('0'), eat_char_choice("xX"), repeat1(seq!(opt(eat_char('_')), hexdigit)));

    let integer = choice!(decinteger, bininteger, octinteger, hexinteger);

    let digitpart = Rc::new(seq!(digit, repeat0(seq!(opt(eat_char('_')), digit))));
    let fraction = seq!(eat_char('.'), digitpart.clone());
    let exponent = seq!(eat_char_choice("eE"), opt(eat_char_choice("+-")), digitpart.clone());

    let pointfloat = Rc::new(choice!(
        seq!(opt(digitpart.clone()), fraction),
        seq!(digitpart.clone(), eat_char('.'))
    ));
    let exponentfloat = seq!(choice!(digitpart.clone(), pointfloat.clone()), exponent);

    let floatnumber = Rc::new(choice!(pointfloat, exponentfloat));

    let imagnumber = seq!(choice!(floatnumber.clone(), digitpart), eat_char_choice("jJ"));

    python_symbol(seq!(choice!(integer, floatnumber, imagnumber), prevent_consecutive_matches("NUMBER")))
}

// .. _comments:
//
// Comments
// --------
//
// .. index:: comment, hash character
//    single: # (hash); comment
//
// A comment starts with a hash character (``#``) that is not part of a string
// literal, and ends at the end of the physical line.  A comment signifies the end
// of the logical line unless the implicit line joining rules are invoked. Comments
// are ignored by the syntax.
pub fn comment() -> Seq2<EatU8, Seq2<Choice2<Repeat1<EatU8>, Eps>, PreventConsecutiveMatchesClear>> {
    seq!(eat_char('#'), repeat0(not_breaking_space()), prevent_consecutive_matches_clear())
}

// .. _line-structure:
//
// Line structure
// ==============
//
// .. index:: line structure
//
// A Python program is divided into a number of *logical lines*.
//
//
// .. _logical-lines:
//
// Logical lines
// -------------
//
// .. index:: logical line, physical line, line joining, NEWLINE token
//
// The end of a logical line is represented by the token NEWLINE.  Statements
// cannot cross logical line boundaries except where NEWLINE is allowed by the
// syntax (e.g., between statements in compound statements). A logical line is
// constructed from one or more *physical lines* by following the explicit or
// implicit *line joining* rules.
//
//
// .. _physical-lines:
//
// Physical lines
// --------------
//
// A physical line is a sequence of characters terminated by an end-of-line
// sequence.  In source files and strings, any of the standard platform line
// termination sequences can be used - the Unix form using ASCII LF (linefeed),
// the Windows form using the ASCII sequence CR LF (return followed by linefeed),
// or the old Macintosh form using the ASCII CR (return) character.  All of these
// forms can be used equally, regardless of platform. The end of input also serves
// as an implicit terminator for the final physical line.
//
// When embedding Python, source code strings should be passed to Python APIs using
// the standard C conventions for newline characters (the ``\n`` character,
// representing ASCII LF, is the line terminator).
pub fn NEWLINE() -> Symbol<Rc<DynCombinator>> {
    let blank_line = seq!(repeat0(non_breaking_space()), opt(comment()), breaking_space());
    symbol(seq!(repeat1(blank_line), dent(), prevent_consecutive_matches_clear()).into_rc_dyn())
}

// .. _indentation:
//
// Indentation
// -----------
//
// .. index:: indentation, leading whitespace, space, tab, grouping, statement grouping
//
// Leading whitespace (spaces and tabs) at the beginning of a logical line is used
// to compute the indentation level of the line, which in turn is used to determine
// the grouping of statements.
//
// Tabs are replaced (from left to right) by one to eight spaces such that the
// total number of characters up to and including the replacement is a multiple of
// eight (this is intended to be the same rule as used by Unix).  The total number
// of spaces preceding the first non-blank character then determines the line's
// indentation.  Indentation cannot be split over multiple physical lines using
// backslashes; the whitespace up to the first backslash determines the
// indentation.
//
// Indentation is rejected as inconsistent if a source file mixes tabs and spaces
// in a way that makes the meaning dependent on the worth of a tab in spaces; a
// :exc:`TabError` is raised in that case.
//
// **Cross-platform compatibility note:** because of the nature of text editors on
// non-UNIX platforms, it is unwise to use a mixture of spaces and tabs for the
// indentation in a single source file.  It should also be noted that different
// platforms may explicitly limit the maximum indentation level.
//
// A formfeed character may be present at the start of the line; it will be ignored
// for the indentation calculations above.  Formfeed characters occurring elsewhere
// in the leading whitespace have an undefined effect (for instance, they may reset
// the space count to zero).
//
// .. index:: INDENT token, DEDENT token
//
// The indentation levels of consecutive lines are used to generate INDENT and
// DEDENT tokens, using a stack, as follows.
//
// Before the first line of the file is read, a single zero is pushed on the stack;
// this will never be popped off again.  The numbers pushed on the stack will
// always be strictly increasing from bottom to top.  At the beginning of each
// logical line, the line's indentation level is compared to the top of the stack.
// If it is equal, nothing happens. If it is larger, it is pushed on the stack, and
// one INDENT token is generated.  If it is smaller, it *must* be one of the
// numbers occurring on the stack; all numbers on the stack that are larger are
// popped off, and for each number popped off a DEDENT token is generated.  At the
// end of the file, a DEDENT token is generated for each number remaining on the
// stack that is larger than zero.
//
// Here is an example of a correctly (though confusingly) indented piece of Python
// code::
//
//    def perm(l):
//            # Compute the list of all permutations of l
//        if len(l) <= 1:
//                     return [l]
//        r = []
//        for i in range(len(l)):
//                s = l[:i] + l[i+1:]
//                p = perm(s)
//                for x in p:
//                 r.append(l[i:i+1] + x)
//        return r
//
// The following example shows various indentation errors::
//
//     def perm(l):                       # error: first line indented
//    for i in range(len(l)):             # error: not indented
//        s = l[:i] + l[i+1:]
//            p = perm(l[:i] + l[i+1:])   # error: unexpected indent
//            for x in p:
//                    r.append(l[i:i+1] + x)
//                return r                # error: inconsistent dedent
//
// (Actually, the first three errors are detected by the parser; only the last
// error is found by the lexical analyzer --- the indentation of ``return r`` does
// not match a level popped off the stack.)
pub fn INDENT() -> Symbol<Seq2<PreventConsecutiveMatches, IndentCombinator>> {
    symbol(seq!(prevent_consecutive_matches("whitespace"), indent()))
}

pub fn DEDENT() -> Symbol<Seq2<PreventConsecutiveMatchesClear, IndentCombinator>> {
    symbol(seq!(prevent_consecutive_matches_clear(), dedent()))
}

pub fn ENDMARKER() -> Symbol<Seq2<PreventConsecutiveMatches, Eps>> {
    symbol(seq!(prevent_consecutive_matches("ENDMARKER"), eps()))
}

pub fn TYPE_COMMENT() -> Symbol<Box<DynCombinator>> {
    // python_symbol(seq!(eat_string("#"), opt(whitespace()), eat_string("type:"), opt(whitespace()), repeat0(eat_char_negation_choice("\n\r"))))
    symbol(fail().into_box_dyn())
}