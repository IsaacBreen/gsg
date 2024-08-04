use std::rc::Rc;

use crate::{assert_no_dedents, check_right_data, Choice, Combinator, CombinatorTrait, Compile, dedent, dent, eat_any_byte, eat_byte_range, eat_bytestring_choice, eat_char, eat_char_choice, eat_char_negation, eat_char_negation_choice, eat_string, eat_string_choice, EatString, EatU8, eps, Eps, fail, forbid_follows, forbid_follows_check_not, forbid_follows_clear, ForbidFollows, ForbidFollowsClear, indent, IndentCombinator, mutate_right_data, MutateRightData, negative_lookahead, exclude_strings, Repeat1, RightData, seq, Seq, Symbol, tag, choice_greedy};
use crate::{opt_greedy as opt, repeat0_greedy as repeat0, repeat1_greedy as repeat1, repeatn_greedy as repeatn, seprep0_greedy as seprep0, seprep1_greedy as seprep1, choice_greedy as choice};
use crate::unicode::{get_unicode_general_category_bytestrings, get_unicode_general_category_combinator};
use crate::unicode_categories::GeneralCategory;

pub fn breaking_space() -> Combinator {
    eat_char_choice("\n\r").into()
}

pub fn not_breaking_space() -> Combinator {
    eat_char_negation_choice("\n\r").into()
}

pub fn non_breaking_space() -> Combinator {
    eat_char_choice(" \t").into()
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
pub fn whitespace() -> Combinator {
    repeat1(choice!(
            // If right_data.num_scopes > 0 then we can match a newline as a whitespace. Otherwise, we can't.
            seq!(
                check_right_data(|right_data| right_data.scope_count > 0),
                breaking_space()
            ),
            // But we can match an escaped newline.
            seq!(eat_string("\\"), breaking_space()),
            non_breaking_space()
        )).into()
}

pub fn WS() -> Combinator {
    whitespace()
}

pub fn python_literal(s: &str) -> Combinator {
    let increment_scope_count = |right_data: &mut RightData| { right_data.scope_count += 1; true };
    let decrement_scope_count = |right_data: &mut RightData| { right_data.scope_count -= 1; true };

    match s {
        "(" | "[" | "{" => seq!(eat_string(s), mutate_right_data(increment_scope_count), forbid_follows_clear()),
        ")" | "]" | "}" => seq!(eat_string(s), mutate_right_data(decrement_scope_count), forbid_follows_clear()),
        _ => seq!(eat_string(s), forbid_follows_clear()),
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
        GeneralCategory::Lu,
        GeneralCategory::Ll,
        GeneralCategory::Lt,
        GeneralCategory::Lm,
        GeneralCategory::Lo,
        GeneralCategory::Nl,
        // We ignore Other_ID_Start - it's just for backwards compatibility.
    ];

    let category_bytestrings: Vec<Vec<u8>> = categories.iter().map(|category| get_unicode_general_category_bytestrings(*category)).flatten().collect();
    let other_bytestrings: Vec<Vec<u8>> = vec![vec![b'_']];

    category_bytestrings.into_iter().chain(other_bytestrings.into_iter()).collect()
}

pub fn id_continue_bytestrings() -> Vec<Vec<u8>> {
    // all characters in id_start, plus characters in the categories Mn, Mc, Nd, Pc and others with the Other_ID_Continue property
    let new_categories = [
        GeneralCategory::Mn,
        GeneralCategory::Mc,
        GeneralCategory::Nd,
        GeneralCategory::Pc,
    ];

    let new_category_bytestrings: Vec<Vec<u8>> = new_categories.iter().flat_map(|category| get_unicode_general_category_bytestrings(*category)).collect();

    let mut bytestrings = Vec::new();
    bytestrings.extend(id_start_bytestrings());
    bytestrings.extend(new_category_bytestrings);
    bytestrings
}

pub fn id_start() -> Combinator {
    eat_bytestring_choice(id_start_bytestrings())
}

pub fn id_continue() -> Combinator {
    eat_bytestring_choice(id_continue_bytestrings())
}

pub fn xid_start() -> Combinator {
    // all characters in id_start whose NFKC normalization is in "id_start xid_continue*"
    // Honestly, I don't know what this means.
    id_start()
}

pub fn xid_continue() -> Combinator {
    // all characters in id_continue whose NFKC normalization is in "id_continue*"
    // Honestly, I don't know what this means.
    id_continue()
}

// https://github.com/python/cpython/blob/3.12/Lib/keyword.py
//
// kwlist = [
//     'False',
//     'None',
//     'True',
//     'and',
//     'as',
//     'assert',
//     'async',
//     'await',
//     'break',
//     'class',
//     'continue',
//     'def',
//     'del',
//     'elif',
//     'else',
//     'except',
//     'finally',
//     'for',
//     'from',
//     'global',
//     'if',
//     'import',
//     'in',
//     'is',
//     'lambda',
//     'nonlocal',
//     'not',
//     'or',
//     'pass',
//     'raise',
//     'return',
//     'try',
//     'while',
//     'with',
//     'yield'
// ]
pub fn reserved_keywords() -> Vec<&'static str> {
    vec![
        "False",
        "None",
        "True",
        "and",
        "as",
        "assert",
        "async",
        "await",
        "break",
        "class",
        "continue",
        "def",
        "del",
        "elif",
        "else",
        "except",
        "finally",
        "for",
        "from",
        "global",
        "if",
        "import",
        "in",
        "is",
        "lambda",
        "nonlocal",
        "not",
        "or",
        "pass",
        "raise",
        "return",
        "try",
        "while",
        "with",
        "yield",
    ]
}

pub fn NAME() -> Combinator {
    exclude_strings(seq!(xid_start(), repeat0(xid_continue()), negative_lookahead(eat_char_choice("\'\""))), reserved_keywords())
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
pub fn eat_char_hex_digit() -> EatU8 {
    eat_char_choice("0123456789abcdefABCDEF")
}

pub fn eat_char_digit() -> EatU8 {
    eat_char_choice("0123456789")
}

pub fn eat_until_terminator(terminator: char) -> Repeat1 {
    repeat1(eat_char_negation(terminator))
}

pub fn STRING() -> Combinator {
    let stringprefix = opt(choice!(
        eat_char_choice("ruRUfF"),
        choice!(
            seq!(eat_char_choice("fF"), eat_char_choice("rR")),
            seq!(eat_char_choice("rR"), eat_char_choice("fF"))
        )
    ));

    let stringescapeseq = choice!(
        seq!(eat_char('\\'), eat_char_choice("\\'\"abfnrtv")),
        seq!(eat_char('\\'), eat_char('x'), repeatn(2, eat_char_hex_digit())),
        seq!(eat_char('\\'), eat_char('u'), repeatn(4, eat_char_hex_digit())),
        seq!(eat_char('\\'), eat_char('U'), repeatn(8, eat_char_hex_digit())),
        seq!(eat_char('\\'), eat_char('N'), eat_until_terminator(';')),
        seq!(eat_char('\\'), eat_char_digit(), opt(eat_char_digit()), opt(eat_char_digit())),
    );

    let shortstringitem = choice!(
        eat_char_negation_choice("\\'\"\n"),
        stringescapeseq.clone()
    );

    let longstringitem = choice!(
        eat_char_negation('\\'),
        stringescapeseq
    );

    let shortstring = choice!(
        seq!(eat_char('\''), repeat0(choice!(shortstringitem.clone(), eat_char('"'), eat_string(r#"\'"#))), eat_char('\'')),
        seq!(eat_char('"'), repeat0(choice!(shortstringitem, eat_char('\''), eat_string(r#"\""#))), eat_char('"'))
    );

    let longstring = choice!(
        seq!(eat_string("'''"), repeat0(longstringitem.clone()), eat_string("'''")),
        seq!(eat_string("\"\"\""), repeat0(longstringitem), eat_string("\"\"\""))
    );

    seq!(stringprefix, choice!(shortstring, longstring))
}

// From https://peps.python.org/pep-0701/
// Specification
// =============
//
// The formal proposed PEG grammar specification for f-strings is (see :pep:`617`
// for details on the syntax):
//
// .. code-block:: peg
//
//     fstring
//         | FSTRING_START fstring_middle* FSTRING_END
//     fstring_middle
//         | fstring_replacement_field
//         | FSTRING_MIDDLE
//     fstring_replacement_field
//         | '{' (yield_expr | star_expressions) "="? [ "!" NAME ] [ ':' fstring_format_spec* ] '}'
//     fstring_format_spec:
//         | FSTRING_MIDDLE
//         | fstring_replacement_field
//
// ...
//
// New tokens
// ----------
//
// Three new tokens are introduced: ``FSTRING_START``, ``FSTRING_MIDDLE`` and
// ``FSTRING_END``. Different lexers may have different implementations that may be
// more efficient than the ones proposed here given the context of the particular
// implementation. However, the following definitions will be used as part of the
// public APIs of CPython (such as the ``tokenize`` module) and are also provided
// as a reference so that the reader can have a better understanding of the
// proposed grammar changes and how the tokens are used:
//
// * ``FSTRING_START``: This token includes the f-string prefix (``f``/``F``/``fr``) and the opening quote(s).
// * ``FSTRING_MIDDLE``: This token includes a portion of text inside the string that's not part of the
//   expression part and isn't an opening or closing brace. This can include the text between the opening quote
//   and the first expression brace (``{``), the text between two expression braces (``}`` and ``{``) and the text
//   between the last expression brace (``}``) and the closing quote.
// * ``FSTRING_END``: This token includes the closing quote.
//
// These tokens are always string parts and they are semantically equivalent to the
// ``STRING`` token with the restrictions specified. These tokens must be produced by the lexer
// when lexing f-strings.  This means that **the tokenizer cannot produce a single token for f-strings anymore**.
// How the lexer emits this token is **not specified** as this will heavily depend on every
// implementation (even the Python version of the lexer in the standard library is implemented
// differently to the one used by the PEG parser).
//
// As an example::
//
//     f'some words {a+b:.3f} more words {c+d=} final words'
//
// will be tokenized as::
//
//     FSTRING_START - "f'"
//     FSTRING_MIDDLE - 'some words '
//     LBRACE - '{'
//     NAME - 'a'
//     PLUS - '+'
//     NAME - 'b'
//     OP - ':'
//     FSTRING_MIDDLE - '.3f'
//     RBRACE - '}'
//     FSTRING_MIDDLE - ' more words '
//     LBRACE - '{'
//     NAME - 'c'
//     PLUS - '+'
//     NAME - 'd'
//     OP - '='
//     RBRACE - '}'
//     FSTRING_MIDDLE - ' final words'
//     FSTRING_END - "'"
//
// while ``f"""some words"""`` will be tokenized simply as::
//
//     FSTRING_START - 'f"""'
//     FSTRING_MIDDLE - 'some words'
//     FSTRING_END - '"""'
//
// Changes to the tokenize module
// ------------------------------
//
// The :mod:`tokenize` module will be adapted to emit these tokens as described in the previous section
// when parsing f-strings so tools can take advantage of this new tokenization schema and avoid having
// to implement their own f-string tokenizer and parser.
//
// How to produce these new tokens
// -------------------------------
//
// One way existing lexers can be adapted to emit these tokens is to incorporate a
// stack of "lexer modes" or to use a stack of different lexers. This is because
// the lexer needs to switch from "regular Python lexing" to "f-string lexing" when
// it encounters an f-string start token and as f-strings can be nested, the
// context needs to be preserved until the f-string closes. Also, the "lexer mode"
// inside an f-string expression part needs to behave as a "super-set" of the
// regular Python lexer (as it needs to be able to switch back to f-string lexing
// when it encounters the ``}`` terminator for the expression part as well as
// handling f-string formatting and debug expressions). For reference, here is a
// draft of the algorithm to modify a CPython-like tokenizer to emit these new
// tokens:
//
// 1. If the lexer detects that an f-string is starting (by detecting the letter
//    'f/F' and one of the possible quotes) keep advancing until a valid quote is
//    detected (one of ``"``, ``"""``, ``'`` or ``'''``) and emit a
//    ``FSTRING_START`` token with the contents captured (the 'f/F' and the
//    starting quote). Push a new tokenizer mode to the tokenizer mode stack for
//    "F-string tokenization". Go to step 2.
// 2. Keep consuming tokens until a one of the following is encountered:
//
//    * A closing quote equal to the opening quote.
//    * If in "format specifier mode" (see step 3), an opening brace (``{``), a
//      closing brace (``}``), or a newline token (``\n``).
//    * If not in "format specifier mode" (see step 3), an opening brace (``{``) or
//      a closing brace (``}``) that is not immediately followed by another opening/closing
//      brace.
//
//    In all cases, if the character buffer is not empty, emit a ``FSTRING_MIDDLE``
//    token with the contents captured so far but transform any double
//    opening/closing braces into single opening/closing braces.  Now, proceed as
//    follows depending on the character encountered:
//
//    * If a closing quote matching the opening quite is encountered go to step 4.
//    * If an opening bracket (not immediately followed by another opening bracket)
//      is encountered, go to step 3.
//    * If a closing bracket (not immediately followed by another closing bracket)
//      is encountered, emit a token for the closing bracket and go to step 2.
// 3. Push a new tokenizer mode to the tokenizer mode stack for "Regular Python
//    tokenization within f-string" and proceed to tokenize with it. This mode
//    tokenizes as the "Regular Python tokenization" until a ``:`` or a ``}``
//    character is encountered with the same level of nesting as the opening
//    bracket token that was pushed when we enter the f-string part. Using this mode,
//    emit tokens until one of the stop points are reached. When this happens, emit
//    the corresponding token for the stopping character encountered and, pop the
//    current tokenizer mode from the tokenizer mode stack and go to step 2. If the
//    stopping point is a ``:`` character, enter step 2 in "format specifier" mode.
// 4. Emit a ``FSTRING_END`` token with the contents captured and pop the current
//    tokenizer mode (corresponding to "F-string tokenization") and go back to
//    "Regular Python mode".
//
// Of course, as mentioned before, it is not possible to provide a precise
// specification of how this should be done for an arbitrary tokenizer as it will
// depend on the specific implementation and nature of the lexer to be changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PythonQuoteType {
    OneSingle,
    OneDouble,
    ThreeSingle,
    ThreeDouble,
}

pub fn FSTRING_START() -> Combinator {
    let prefix = choice!(
        eat_char_choice("fF"),
        seq!(eat_char_choice("fF"), eat_char_choice("rR")),
        seq!(eat_char_choice("rR"), eat_char_choice("fF"))
    );

    let quote = choice!(
        seq!(eat_char('\''), mutate_right_data(|right_data| { right_data.fstring_start_stack.push(PythonQuoteType::OneSingle); true })),
        seq!(eat_char('"'), mutate_right_data(|right_data| { right_data.fstring_start_stack.push(PythonQuoteType::OneDouble); true })),
        seq!(eat_string("'''"), mutate_right_data(|right_data| { right_data.fstring_start_stack.push(PythonQuoteType::ThreeSingle); true })),
        seq!(eat_string("\"\"\""), mutate_right_data(|right_data| { right_data.fstring_start_stack.push(PythonQuoteType::ThreeDouble); true }))
    );

    seq!(
        prefix, quote,
    )
}

pub fn FSTRING_MIDDLE() -> Combinator {
    let stringescapeseq = choice!(
        seq!(eat_char('\\'), eat_char_choice("\\'\"abfnrtv")),
        seq!(eat_char('\\'), eat_char('x'), repeatn(2, eat_char_hex_digit())),
        seq!(eat_char('\\'), eat_char('u'), repeatn(4, eat_char_hex_digit())),
        seq!(eat_char('\\'), eat_char('U'), repeatn(8, eat_char_hex_digit())),
        seq!(eat_char('\\'), eat_char('N'), eat_until_terminator(';')),
        seq!(eat_char('\\'), eat_char_digit(), opt(eat_char_digit()), opt(eat_char_digit())),
    );

    let regular_char = eat_char_negation_choice("{}\\\n\r\'\"");

    let quote = choice!(
        seq!(eat_char('\''), mutate_right_data(|right_data| { *right_data.fstring_start_stack.last().unwrap() != PythonQuoteType::OneSingle })),
        seq!(eat_char('"'), mutate_right_data(|right_data| { *right_data.fstring_start_stack.last().unwrap() != PythonQuoteType::OneDouble })),
    );

    repeat1(choice!(
            regular_char,
            stringescapeseq,
            quote,
            seq!(eat_char('{'), eat_char('{')),
            seq!(eat_char('}'), eat_char('}'))
        )
    ).into()
}

pub fn FSTRING_END() -> Combinator {
    let quote = choice!(
        seq!(eat_char('\''), mutate_right_data(|right_data| { right_data.fstring_start_stack.pop().unwrap() == PythonQuoteType::OneSingle })),
        seq!(eat_char('"'), mutate_right_data(|right_data| { right_data.fstring_start_stack.pop().unwrap() == PythonQuoteType::OneDouble })),
        seq!(eat_string("'''"), mutate_right_data(|right_data| { right_data.fstring_start_stack.pop().unwrap() == PythonQuoteType::ThreeSingle })),
        seq!(eat_string("\"\"\""), mutate_right_data(|right_data| { right_data.fstring_start_stack.pop().unwrap() == PythonQuoteType::ThreeDouble })),
    );

    quote.into()
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
pub fn NUMBER() -> Combinator {
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

    let digitpart = seq!(digit, repeat0(seq!(opt(eat_char('_')), digit)));
    let fraction = seq!(eat_char('.'), digitpart.clone());
    let exponent = seq!(eat_char_choice("eE"), opt(eat_char_choice("+-")), digitpart.clone());

    let pointfloat = choice!(
        seq!(opt(digitpart.clone()), fraction),
        seq!(digitpart.clone(), eat_char('.'))
    );
    let exponentfloat = seq!(choice!(digitpart.clone(), pointfloat.clone()), exponent);

    let floatnumber = choice!(pointfloat, exponentfloat);

    let imagnumber = seq!(choice!(floatnumber.clone(), digitpart), eat_char_choice("jJ"));

    choice!(integer, floatnumber, imagnumber).into()
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
pub fn comment() -> Combinator {
    seq!(eat_char('#'), repeat0(not_breaking_space()))
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
pub fn NEWLINE() -> Combinator {
    let blank_line = seq!(repeat0(non_breaking_space()), opt(comment()), breaking_space());
    seq!(repeat1(blank_line), tag("dent()", dent()))
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
pub fn INDENT() -> Combinator {
    indent().into()
}

pub fn DEDENT() -> Combinator {
    dedent().into()
}

pub fn ENDMARKER() -> Combinator {
    eps().into()
}

pub fn TYPE_COMMENT() -> Combinator {
    // seq!(eat_string("#"), opt(whitespace()), eat_string("type:"), opt(whitespace()), repeat0(eat_char_negation_choice("\n\r")))
    fail().into()
}