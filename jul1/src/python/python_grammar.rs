use std::rc::Rc;
use crate::{cache_context, cached, symbol, Symbol, Choice, deferred, Combinator, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, Repeat1, Seq, tag, lookahead, negative_lookahead};
use super::python_tokenizer::python_literal;
use crate::seq;
use crate::{opt_greedy as opt, choice_greedy as choice, seprep0_greedy as seprep0, seprep1_greedy as seprep1, repeat0_greedy as repeat0, repeat1_greedy as repeat1};

enum Forbidden {
    WS,
    NAME,
    TYPE_COMMENT,
    FSTRING_START,
    FSTRING_MIDDLE,
    FSTRING_END,
    NUMBER,
    STRING,
    NEWLINE,
    INDENT,
    DEDENT,
    ENDMARKER,
}

use super::python_tokenizer as token;
fn WS() -> Combinator { cached(tag("WS", crate::profile("WS", seq!(forbid_follows_check_not(Forbidden::WS as usize), token::WS().compile(), forbid_follows(&[Forbidden::WS as usize, Forbidden::DEDENT as usize, Forbidden::INDENT as usize, Forbidden::NEWLINE as usize]))))).into() }
fn NAME() -> Combinator { cached(seq!(tag("NAME", crate::profile("NAME", seq!(forbid_follows_check_not(Forbidden::NAME as usize), token::NAME().compile(), forbid_follows(&[Forbidden::NAME as usize, Forbidden::NUMBER as usize])))), opt(&WS))).into() }
fn TYPE_COMMENT() -> Combinator { cached(seq!(tag("TYPE_COMMENT", crate::profile("TYPE_COMMENT", seq!(forbid_follows_clear(), token::TYPE_COMMENT().compile()))), opt(&WS))).into() }
fn FSTRING_START() -> Combinator { cached(tag("FSTRING_START", crate::profile("FSTRING_START", seq!(token::FSTRING_START().compile(), forbid_follows(&[Forbidden::WS as usize, Forbidden::NEWLINE as usize]))))).into() }
fn FSTRING_MIDDLE() -> Combinator { cached(tag("FSTRING_MIDDLE", crate::profile("FSTRING_MIDDLE", seq!(forbid_follows_check_not(Forbidden::FSTRING_MIDDLE as usize), token::FSTRING_MIDDLE().compile(), forbid_follows(&[Forbidden::WS as usize, Forbidden::FSTRING_MIDDLE as usize]))))).into() }
fn FSTRING_END() -> Combinator { cached(seq!(tag("FSTRING_END", crate::profile("FSTRING_END", seq!(forbid_follows_clear(), token::FSTRING_END().compile()))), opt(&WS))).into() }
fn NUMBER() -> Combinator { cached(seq!(tag("NUMBER", crate::profile("NUMBER", seq!(forbid_follows_check_not(Forbidden::NUMBER as usize), token::NUMBER().compile(), forbid_follows(&[Forbidden::NUMBER as usize])))), opt(&WS))).into() }
fn STRING() -> Combinator { cached(seq!(tag("STRING", crate::profile("STRING", seq!(forbid_follows_clear(), token::STRING().compile()))), opt(&WS))).into() }
fn NEWLINE() -> Combinator { cached(tag("NEWLINE", crate::profile("NEWLINE", seq!(forbid_follows_check_not(Forbidden::NEWLINE as usize), token::NEWLINE().compile(), forbid_follows(&[Forbidden::WS as usize]))))).into() }
fn INDENT() -> Combinator { cached(tag("INDENT", crate::profile("INDENT", seq!(forbid_follows_check_not(Forbidden::INDENT as usize), token::INDENT().compile(), forbid_follows(&[Forbidden::WS as usize]))))).into() }
fn DEDENT() -> Combinator { cached(tag("DEDENT", crate::profile("DEDENT", seq!(forbid_follows_check_not(Forbidden::DEDENT as usize), token::DEDENT().compile(), forbid_follows(&[Forbidden::WS as usize]))))).into() }
fn ENDMARKER() -> Combinator { cached(seq!(tag("ENDMARKER", crate::profile("ENDMARKER", seq!(forbid_follows_clear(), token::ENDMARKER().compile()))), opt(&WS))).into() }

fn invalid_type_params() -> Combinator {
    tag("invalid_type_params", seq!(python_literal("["), python_literal("]"))).into()
}

fn invalid_factor() -> Combinator {
    tag("invalid_factor", seq!(choice!(python_literal("+"), python_literal("-"), python_literal("~")), python_literal("not"), choice!(seq!(python_literal("+"), &factor), seq!(python_literal("-"), &factor), seq!(python_literal("~"), &factor), &power))).into()
}

fn invalid_arithmetic() -> Combinator {
    tag("invalid_arithmetic", seq!(seq!(&term, opt(repeat1(choice!(seq!(python_literal("+"), &term), seq!(python_literal("-"), &term))))), choice!(python_literal("+"), python_literal("-"), python_literal("*"), python_literal("/"), python_literal("%"), python_literal("//"), python_literal("@")), python_literal("not"), choice!(seq!(python_literal("not"), &inversion), &comparison))).into()
}

fn invalid_conversion_character() -> Combinator {
    tag("invalid_conversion_character", choice!(
        seq!(python_literal("!"), lookahead(choice!(python_literal(":"), python_literal("}")))),
        seq!(python_literal("!"), negative_lookahead(&NAME))
    )).into()
}

fn invalid_replacement_field() -> Combinator {
    tag("invalid_replacement_field", choice!(
        seq!(python_literal("{"), python_literal("=")),
        seq!(python_literal("{"), python_literal("!")),
        seq!(python_literal("{"), python_literal(":")),
        seq!(python_literal("{"), python_literal("}")),
        seq!(python_literal("{"), negative_lookahead(&annotated_rhs)),
        seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), negative_lookahead(choice!(python_literal("="), python_literal("!"), python_literal(":"), python_literal("}")))),
        seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), python_literal("="), negative_lookahead(choice!(python_literal("!"), python_literal(":"), python_literal("}")))),
        seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), opt(python_literal("=")), &invalid_conversion_character),
        seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), opt(python_literal("=")), opt(seq!(python_literal("!"), &NAME)), negative_lookahead(choice!(python_literal(":"), python_literal("}")))),
        seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), opt(python_literal("=")), opt(seq!(python_literal("!"), &NAME)), python_literal(":"), opt(repeat1(choice!(&FSTRING_MIDDLE, choice!(seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), opt(python_literal("=")), opt(&fstring_conversion), opt(&fstring_full_format_spec), python_literal("}")), &invalid_replacement_field)))), negative_lookahead(python_literal("}"))),
        seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), opt(python_literal("=")), opt(seq!(python_literal("!"), &NAME)), negative_lookahead(python_literal("}")))
    )).into()
}

fn invalid_starred_expression() -> Combinator {
    tag("invalid_starred_expression", python_literal("*")).into()
}

fn invalid_starred_expression_unpacking() -> Combinator {
    tag("invalid_starred_expression_unpacking", seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal("="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))).into()
}

fn invalid_kvpair() -> Combinator {
    tag("invalid_kvpair", choice!(
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":"))),
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(":"), python_literal("*"), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor))))),
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(":"), lookahead(choice!(python_literal("}"), python_literal(","))))
    )).into()
}

fn invalid_double_starred_kvpairs() -> Combinator {
    tag("invalid_double_starred_kvpairs", choice!(
        seq!(seq!(choice!(seq!(python_literal("**"), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor))))), &kvpair), opt(repeat1(seq!(python_literal(","), choice!(seq!(python_literal("**"), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor))))), &kvpair))))), python_literal(","), &invalid_kvpair),
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(":"), python_literal("*"), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor))))),
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(":"), lookahead(choice!(python_literal("}"), python_literal(","))))
    )).into()
}

fn invalid_class_def_raw() -> Combinator {
    tag("invalid_class_def_raw", choice!(
        seq!(python_literal("class"), &NAME, opt(choice!(&invalid_type_params, seq!(python_literal("["), &type_param_seq, python_literal("]")))), opt(seq!(python_literal("("), opt(choice!(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))), &invalid_arguments)), python_literal(")"))), &NEWLINE),
        seq!(python_literal("class"), &NAME, opt(choice!(&invalid_type_params, seq!(python_literal("["), &type_param_seq, python_literal("]")))), opt(seq!(python_literal("("), opt(choice!(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))), &invalid_arguments)), python_literal(")"))), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_def_raw() -> Combinator {
    tag("invalid_def_raw", choice!(
        seq!(opt(python_literal("async")), python_literal("def"), &NAME, opt(choice!(&invalid_type_params, seq!(python_literal("["), &type_param_seq, python_literal("]")))), python_literal("("), opt(choice!(&invalid_parameters, &parameters)), python_literal(")"), opt(seq!(python_literal("->"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT)),
        seq!(opt(python_literal("async")), python_literal("def"), &NAME, opt(choice!(&invalid_type_params, seq!(python_literal("["), &type_param_seq, python_literal("]")))), python_literal("("), opt(choice!(&invalid_parameters, &parameters)), python_literal(")"), opt(seq!(python_literal("->"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))), python_literal(":"), opt(choice!(seq!(&NEWLINE, &TYPE_COMMENT, lookahead(seq!(&NEWLINE, &INDENT))), &invalid_double_type_comments, &TYPE_COMMENT)), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))
    )).into()
}

fn invalid_for_stmt() -> Combinator {
    tag("invalid_for_stmt", choice!(
        seq!(opt(python_literal("async")), python_literal("for"), choice!(seq!(&star_target, negative_lookahead(python_literal(","))), seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))), python_literal("in"), choice!(seq!(&star_expression, repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), seq!(&star_expression, python_literal(",")), &star_expression), &NEWLINE),
        seq!(opt(python_literal("async")), python_literal("for"), choice!(seq!(&star_target, negative_lookahead(python_literal(","))), seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))), python_literal("in"), choice!(seq!(&star_expression, repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), seq!(&star_expression, python_literal(",")), &star_expression), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_while_stmt() -> Combinator {
    tag("invalid_while_stmt", choice!(
        seq!(python_literal("while"), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), &NEWLINE),
        seq!(python_literal("while"), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_else_stmt() -> Combinator {
    tag("invalid_else_stmt", seq!(python_literal("else"), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))).into()
}

fn invalid_elif_stmt() -> Combinator {
    tag("invalid_elif_stmt", choice!(
        seq!(python_literal("elif"), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), &NEWLINE),
        seq!(python_literal("elif"), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_if_stmt() -> Combinator {
    tag("invalid_if_stmt", choice!(
        seq!(python_literal("if"), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), &NEWLINE),
        seq!(python_literal("if"), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_class_argument_pattern() -> Combinator {
    tag("invalid_class_argument_pattern", seq!(opt(seq!(seq!(choice!(&as_pattern, &or_pattern), opt(repeat1(seq!(python_literal(","), choice!(&as_pattern, &or_pattern))))), python_literal(","))), seq!(&keyword_pattern, opt(repeat1(seq!(python_literal(","), &keyword_pattern)))), python_literal(","), seq!(choice!(&as_pattern, &or_pattern), opt(repeat1(seq!(python_literal(","), choice!(&as_pattern, &or_pattern))))))).into()
}

fn invalid_class_pattern() -> Combinator {
    tag("invalid_class_pattern", seq!(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME)))), python_literal("("), &invalid_class_argument_pattern)).into()
}

fn invalid_as_pattern() -> Combinator {
    tag("invalid_as_pattern", choice!(
        seq!(seq!(&closed_pattern, opt(repeat1(seq!(python_literal("|"), &closed_pattern)))), python_literal("as"), python_literal("_")),
        seq!(seq!(&closed_pattern, opt(repeat1(seq!(python_literal("|"), &closed_pattern)))), python_literal("as"), negative_lookahead(&NAME), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))
    )).into()
}

fn invalid_case_block() -> Combinator {
    tag("invalid_case_block", choice!(
        seq!(python_literal("case"), choice!(&open_sequence_pattern, &pattern), opt(seq!(python_literal("if"), &named_expression)), &NEWLINE),
        seq!(python_literal("case"), choice!(&open_sequence_pattern, &pattern), opt(seq!(python_literal("if"), &named_expression)), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_match_stmt() -> Combinator {
    tag("invalid_match_stmt", choice!(
        seq!(python_literal("match"), choice!(seq!(&star_named_expression, python_literal(","), opt(&star_named_expressions)), &named_expression), &NEWLINE),
        seq!(python_literal("match"), choice!(seq!(&star_named_expression, python_literal(","), opt(&star_named_expressions)), &named_expression), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_except_star_stmt_indent() -> Combinator {
    tag("invalid_except_star_stmt_indent", seq!(
        python_literal("except"),
         python_literal("*"),
         choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef),
         opt(seq!(python_literal("as"), &NAME)),
         python_literal(":"),
         &NEWLINE,
         negative_lookahead(&INDENT)
    )).into()
}

fn invalid_except_stmt_indent() -> Combinator {
    tag("invalid_except_stmt_indent", choice!(
        seq!(python_literal("except"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(seq!(python_literal("as"), &NAME)), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT)),
        seq!(python_literal("except"), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_finally_stmt() -> Combinator {
    tag("invalid_finally_stmt", seq!(python_literal("finally"), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))).into()
}

fn invalid_except_stmt() -> Combinator {
    tag("invalid_except_stmt", choice!(
        seq!(python_literal("except"), opt(python_literal("*")), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(","), choice!(seq!(&expression, repeat1(seq!(python_literal(","), &expression)), opt(python_literal(","))), seq!(&expression, python_literal(",")), &expression), opt(seq!(python_literal("as"), &NAME)), python_literal(":")),
        seq!(python_literal("except"), opt(python_literal("*")), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(seq!(python_literal("as"), &NAME)), &NEWLINE),
        seq!(python_literal("except"), &NEWLINE),
        seq!(python_literal("except"), python_literal("*"), choice!(&NEWLINE, python_literal(":")))
    )).into()
}

fn invalid_try_stmt() -> Combinator {
    tag("invalid_try_stmt", choice!(
        seq!(python_literal("try"), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT)),
        seq!(python_literal("try"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), negative_lookahead(choice!(python_literal("except"), python_literal("finally")))),
        seq!(python_literal("try"), python_literal(":"), opt(repeat1(choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))), repeat1(choice!(&invalid_except_stmt_indent, seq!(python_literal("except"), &expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)), seq!(python_literal("except"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)), &invalid_except_stmt)), python_literal("except"), python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(seq!(python_literal("as"), &NAME)), python_literal(":")),
        seq!(python_literal("try"), python_literal(":"), opt(repeat1(choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))), repeat1(choice!(&invalid_except_star_stmt_indent, seq!(python_literal("except"), python_literal("*"), &expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)), &invalid_except_stmt)), python_literal("except"), opt(seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(seq!(python_literal("as"), &NAME)))), python_literal(":"))
    )).into()
}

fn invalid_with_stmt_indent() -> Combinator {
    tag("invalid_with_stmt_indent", choice!(
        seq!(opt(python_literal("async")), python_literal("with"), seprep1(seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(seq!(python_literal("as"), choice!(seq!(python_literal("*"), seq!(negative_lookahead(python_literal("*")), &star_target)), &target_with_star_atom)))), python_literal(",")), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT)),
        seq!(opt(python_literal("async")), python_literal("with"), python_literal("("), seprep1(seq!(choice!(seq!(&expression, repeat1(seq!(python_literal(","), &expression)), opt(python_literal(","))), seq!(&expression, python_literal(",")), &expression), opt(seq!(python_literal("as"), choice!(seq!(python_literal("*"), seq!(negative_lookahead(python_literal("*")), &star_target)), &target_with_star_atom)))), python_literal(",")), opt(python_literal(",")), python_literal(")"), python_literal(":"), &NEWLINE, negative_lookahead(&INDENT))
    )).into()
}

fn invalid_with_stmt() -> Combinator {
    tag("invalid_with_stmt", choice!(
        seq!(opt(python_literal("async")), python_literal("with"), seprep1(seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(seq!(python_literal("as"), choice!(seq!(python_literal("*"), seq!(negative_lookahead(python_literal("*")), &star_target)), &target_with_star_atom)))), python_literal(",")), &NEWLINE),
        seq!(opt(python_literal("async")), python_literal("with"), python_literal("("), seprep1(seq!(choice!(seq!(&expression, repeat1(seq!(python_literal(","), &expression)), opt(python_literal(","))), seq!(&expression, python_literal(",")), &expression), opt(seq!(python_literal("as"), choice!(seq!(python_literal("*"), seq!(negative_lookahead(python_literal("*")), &star_target)), &target_with_star_atom)))), python_literal(",")), opt(python_literal(",")), python_literal(")"), &NEWLINE)
    )).into()
}

fn invalid_import_from_targets() -> Combinator {
    tag("invalid_import_from_targets", choice!(
        seq!(seq!(&import_from_as_name, opt(repeat1(seq!(python_literal(","), &import_from_as_name)))), python_literal(","), &NEWLINE),
        &NEWLINE
    )).into()
}

fn invalid_import() -> Combinator {
    tag("invalid_import", choice!(
        seq!(python_literal("import"), seprep1(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME)))), python_literal(",")), python_literal("from"), seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME))))),
        seq!(python_literal("import"), &NEWLINE)
    )).into()
}

fn invalid_group() -> Combinator {
    tag("invalid_group", choice!(
        seq!(python_literal("("), choice!(&invalid_starred_expression_unpacking, seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_starred_expression), python_literal(")")),
        seq!(python_literal("("), python_literal("**"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(")"))
    )).into()
}

fn invalid_for_target() -> Combinator {
    tag("invalid_for_target", seq!(opt(python_literal("async")), python_literal("for"), choice!(seq!(&star_expression, repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), seq!(&star_expression, python_literal(",")), &star_expression))).into()
}

fn invalid_for_if_clause() -> Combinator {
    tag("invalid_for_if_clause", seq!(opt(python_literal("async")), python_literal("for"), seq!(seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor)))), opt(repeat1(seq!(python_literal(","), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor))))))), opt(python_literal(","))), negative_lookahead(python_literal("in")))).into()
}

fn invalid_with_item() -> Combinator {
    tag("invalid_with_item", seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal("as"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), lookahead(choice!(python_literal(","), python_literal(")"), python_literal(":"))))).into()
}

fn invalid_double_type_comments() -> Combinator {
    tag("invalid_double_type_comments", seq!(
        &TYPE_COMMENT,
         &NEWLINE,
         &TYPE_COMMENT,
         &NEWLINE,
         &INDENT
    )).into()
}

fn invalid_lambda_kwds() -> Combinator {
    tag("invalid_lambda_kwds", choice!(
        seq!(python_literal("**"), &NAME, python_literal("=")),
        seq!(python_literal("**"), &NAME, python_literal(","), &NAME),
        seq!(python_literal("**"), &NAME, python_literal(","), choice!(python_literal("*"), python_literal("**"), python_literal("/")))
    )).into()
}

fn invalid_lambda_star_etc() -> Combinator {
    tag("invalid_lambda_star_etc", choice!(
        seq!(python_literal("*"), choice!(python_literal(":"), seq!(python_literal(","), choice!(python_literal(":"), python_literal("**"))))),
        seq!(python_literal("*"), &NAME, python_literal("=")),
        seq!(python_literal("*"), choice!(choice!(seq!(&lambda_param, python_literal(",")), seq!(&lambda_param, lookahead(python_literal(":")))), python_literal(",")), opt(repeat1(choice!(seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), python_literal(",")), seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), lookahead(python_literal(":")))))), python_literal("*"), choice!(choice!(seq!(&lambda_param, python_literal(",")), seq!(&lambda_param, lookahead(python_literal(":")))), python_literal(",")))
    )).into()
}

fn invalid_lambda_parameters_helper() -> Combinator {
    tag("invalid_lambda_parameters_helper", choice!(
        choice!(seq!(opt(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default)))), repeat1(&lambda_param_with_default), python_literal("/"), python_literal(",")), seq!(opt(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default)))), repeat1(&lambda_param_with_default), python_literal("/"), lookahead(python_literal(":")))),
        seq!(choice!(seq!(&lambda_param, choice!(seq!(python_literal("="), &expression), &invalid_default), python_literal(",")), seq!(&lambda_param, choice!(seq!(python_literal("="), &expression), &invalid_default), lookahead(python_literal(":")))), opt(repeat1(choice!(seq!(&lambda_param, choice!(seq!(python_literal("="), &expression), &invalid_default), python_literal(",")), seq!(&lambda_param, choice!(seq!(python_literal("="), &expression), &invalid_default), lookahead(python_literal(":")))))))
    )).into()
}

fn invalid_lambda_parameters() -> Combinator {
    tag("invalid_lambda_parameters", choice!(
        seq!(python_literal("/"), python_literal(",")),
        seq!(choice!(choice!(seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), python_literal("/"), python_literal(",")), seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), python_literal("/"), lookahead(python_literal(":")))), choice!(seq!(opt(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default)))), repeat1(&lambda_param_with_default), python_literal("/"), python_literal(",")), seq!(opt(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default)))), repeat1(&lambda_param_with_default), python_literal("/"), lookahead(python_literal(":"))))), opt(repeat1(choice!(seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), python_literal(",")), seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), lookahead(python_literal(":")))))), python_literal("/")),
        seq!(opt(choice!(seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), python_literal("/"), python_literal(",")), seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), python_literal("/"), lookahead(python_literal(":"))))), opt(repeat1(choice!(seq!(&lambda_param, python_literal(",")), seq!(&lambda_param, lookahead(python_literal(":")))))), &invalid_lambda_parameters_helper, choice!(seq!(&lambda_param, python_literal(",")), seq!(&lambda_param, lookahead(python_literal(":"))))),
        seq!(opt(seq!(choice!(seq!(&lambda_param, python_literal(",")), seq!(&lambda_param, lookahead(python_literal(":")))), opt(repeat1(choice!(seq!(&lambda_param, python_literal(",")), seq!(&lambda_param, lookahead(python_literal(":")))))))), python_literal("("), seprep1(&NAME, python_literal(",")), opt(python_literal(",")), python_literal(")")),
        seq!(opt(choice!(choice!(seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), python_literal("/"), python_literal(",")), seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), python_literal("/"), lookahead(python_literal(":")))), choice!(seq!(opt(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default)))), repeat1(&lambda_param_with_default), python_literal("/"), python_literal(",")), seq!(opt(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default)))), repeat1(&lambda_param_with_default), python_literal("/"), lookahead(python_literal(":")))))), opt(repeat1(choice!(seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), python_literal(",")), seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), lookahead(python_literal(":")))))), python_literal("*"), choice!(python_literal(","), choice!(seq!(&lambda_param, python_literal(",")), seq!(&lambda_param, lookahead(python_literal(":"))))), opt(repeat1(choice!(seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), python_literal(",")), seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), lookahead(python_literal(":")))))), python_literal("/")),
        seq!(seq!(choice!(seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), python_literal(",")), seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), lookahead(python_literal(":")))), opt(repeat1(choice!(seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), python_literal(",")), seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), lookahead(python_literal(":"))))))), python_literal("/"), python_literal("*"))
    )).into()
}

fn invalid_parameters_helper() -> Combinator {
    tag("invalid_parameters_helper", choice!(
        choice!(seq!(opt(seq!(&param_no_default, opt(repeat1(&param_no_default)))), repeat1(&param_with_default), python_literal("/"), python_literal(",")), seq!(opt(seq!(&param_no_default, opt(repeat1(&param_no_default)))), repeat1(&param_with_default), python_literal("/"), lookahead(python_literal(")")))),
        seq!(choice!(seq!(&param, &default, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, &default, opt(&TYPE_COMMENT), lookahead(python_literal(")")))), opt(repeat1(choice!(seq!(&param, &default, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, &default, opt(&TYPE_COMMENT), lookahead(python_literal(")")))))))
    )).into()
}

fn invalid_kwds() -> Combinator {
    tag("invalid_kwds", choice!(
        seq!(python_literal("**"), seq!(&NAME, opt(&annotation)), python_literal("=")),
        seq!(python_literal("**"), seq!(&NAME, opt(&annotation)), python_literal(","), seq!(&NAME, opt(&annotation))),
        seq!(python_literal("**"), seq!(&NAME, opt(&annotation)), python_literal(","), choice!(python_literal("*"), python_literal("**"), python_literal("/")))
    )).into()
}

fn invalid_star_etc() -> Combinator {
    tag("invalid_star_etc", choice!(
        seq!(python_literal("*"), choice!(python_literal(")"), seq!(python_literal(","), choice!(python_literal(")"), python_literal("**"))))),
        seq!(python_literal("*"), python_literal(","), &TYPE_COMMENT),
        seq!(python_literal("*"), seq!(&NAME, opt(&annotation)), python_literal("=")),
        seq!(python_literal("*"), choice!(choice!(seq!(&param, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")")))), python_literal(",")), opt(repeat1(choice!(seq!(&param, opt(&default), python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&default), opt(&TYPE_COMMENT), lookahead(python_literal(")")))))), python_literal("*"), choice!(choice!(seq!(&param, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")")))), python_literal(",")))
    )).into()
}

fn invalid_default() -> Combinator {
    tag("invalid_default", seq!(python_literal("="), lookahead(choice!(python_literal(")"), python_literal(","))))).into()
}

fn invalid_parameters() -> Combinator {
    tag("invalid_parameters", choice!(
        seq!(python_literal("/"), python_literal(",")),
        seq!(choice!(choice!(seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), python_literal("/"), python_literal(",")), seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), python_literal("/"), lookahead(python_literal(")")))), choice!(seq!(opt(seq!(&param_no_default, opt(repeat1(&param_no_default)))), repeat1(&param_with_default), python_literal("/"), python_literal(",")), seq!(opt(seq!(&param_no_default, opt(repeat1(&param_no_default)))), repeat1(&param_with_default), python_literal("/"), lookahead(python_literal(")"))))), opt(repeat1(choice!(seq!(&param, opt(&default), python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&default), opt(&TYPE_COMMENT), lookahead(python_literal(")")))))), python_literal("/")),
        seq!(opt(choice!(seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), python_literal("/"), python_literal(",")), seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), python_literal("/"), lookahead(python_literal(")"))))), opt(repeat1(choice!(seq!(&param, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")")))))), &invalid_parameters_helper, choice!(seq!(&param, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")"))))),
        seq!(opt(seq!(choice!(seq!(&param, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")")))), opt(repeat1(choice!(seq!(&param, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")")))))))), python_literal("("), repeat1(choice!(seq!(&param, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")"))))), opt(python_literal(",")), python_literal(")")),
        seq!(opt(choice!(choice!(seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), python_literal("/"), python_literal(",")), seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), python_literal("/"), lookahead(python_literal(")")))), choice!(seq!(opt(seq!(&param_no_default, opt(repeat1(&param_no_default)))), repeat1(&param_with_default), python_literal("/"), python_literal(",")), seq!(opt(seq!(&param_no_default, opt(repeat1(&param_no_default)))), repeat1(&param_with_default), python_literal("/"), lookahead(python_literal(")")))))), opt(repeat1(choice!(seq!(&param, opt(&default), python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&default), opt(&TYPE_COMMENT), lookahead(python_literal(")")))))), python_literal("*"), choice!(python_literal(","), choice!(seq!(&param, python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")"))))), opt(repeat1(choice!(seq!(&param, opt(&default), python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&default), opt(&TYPE_COMMENT), lookahead(python_literal(")")))))), python_literal("/")),
        seq!(seq!(choice!(seq!(&param, opt(&default), python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&default), opt(&TYPE_COMMENT), lookahead(python_literal(")")))), opt(repeat1(choice!(seq!(&param, opt(&default), python_literal(","), opt(&TYPE_COMMENT)), seq!(&param, opt(&default), opt(&TYPE_COMMENT), lookahead(python_literal(")"))))))), python_literal("/"), python_literal("*"))
    )).into()
}

fn invalid_dict_comprehension() -> Combinator {
    tag("invalid_dict_comprehension", seq!(
        python_literal("{"),
         python_literal("**"),
         seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor)))),
         seq!(&for_if_clause, opt(repeat1(&for_if_clause))),
         python_literal("}")
    )).into()
}

fn invalid_comprehension() -> Combinator {
    tag("invalid_comprehension", choice!(
        seq!(choice!(python_literal("["), python_literal("("), python_literal("{")), choice!(&invalid_starred_expression_unpacking, seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_starred_expression), seq!(&for_if_clause, opt(repeat1(&for_if_clause)))),
        seq!(choice!(python_literal("["), python_literal("{")), choice!(seq!(python_literal("*"), &bitwise_or), &named_expression), python_literal(","), seq!(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression)))), opt(python_literal(","))), seq!(&for_if_clause, opt(repeat1(&for_if_clause)))),
        seq!(choice!(python_literal("["), python_literal("{")), choice!(seq!(python_literal("*"), &bitwise_or), &named_expression), python_literal(","), seq!(&for_if_clause, opt(repeat1(&for_if_clause))))
    )).into()
}

fn invalid_block() -> Combinator {
    tag("invalid_block", seq!(&NEWLINE, negative_lookahead(&INDENT))).into()
}

fn invalid_del_stmt() -> Combinator {
    tag("invalid_del_stmt", seq!(python_literal("del"), choice!(seq!(&star_expression, repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), seq!(&star_expression, python_literal(",")), &star_expression))).into()
}

fn invalid_ann_assign_target() -> Combinator {
    tag("invalid_ann_assign_target", choice!(
        seq!(python_literal("["), opt(seq!(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression)))), opt(python_literal(",")))), python_literal("]")),
        seq!(python_literal("("), opt(seq!(choice!(seq!(python_literal("*"), &bitwise_or), &named_expression), python_literal(","), opt(seq!(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression)))), opt(python_literal(",")))))), python_literal(")")),
        seq!(python_literal("("), &invalid_ann_assign_target, python_literal(")"))
    )).into()
}

fn invalid_assignment() -> Combinator {
    tag("invalid_assignment", choice!(
        seq!(&invalid_ann_assign_target, python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(choice!(seq!(python_literal("*"), &bitwise_or), &named_expression), python_literal(","), opt(repeat1(seq!(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression)))), opt(python_literal(","))))), python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(opt(seq!(seq!(choice!(seq!(&star_target, negative_lookahead(python_literal(","))), seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))), python_literal("=")), opt(repeat1(seq!(choice!(seq!(&star_target, negative_lookahead(python_literal(","))), seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))), python_literal("=")))))), choice!(seq!(&star_expression, repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), seq!(&star_expression, python_literal(",")), &star_expression), python_literal("=")),
        seq!(opt(seq!(seq!(choice!(seq!(&star_target, negative_lookahead(python_literal(","))), seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))), python_literal("=")), opt(repeat1(seq!(choice!(seq!(&star_target, negative_lookahead(python_literal(","))), seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))), python_literal("=")))))), choice!(seq!(python_literal("yield"), python_literal("from"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(python_literal("yield"), opt(&star_expressions))), python_literal("=")),
        seq!(choice!(seq!(&star_expression, repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), seq!(&star_expression, python_literal(",")), &star_expression), choice!(python_literal("+="), python_literal("-="), python_literal("*="), python_literal("@="), python_literal("/="), python_literal("%="), python_literal("&="), python_literal("|="), python_literal("^="), python_literal("<<="), python_literal(">>="), python_literal("**="), python_literal("//=")), choice!(&yield_expr, &star_expressions))
    )).into()
}

fn invalid_named_expression() -> Combinator {
    cached(tag("invalid_named_expression", choice!(
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(&NAME, python_literal("="), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor)))), negative_lookahead(choice!(python_literal("="), python_literal(":=")))),
        seq!(negative_lookahead(choice!(&list, &tuple, &genexp, python_literal("True"), python_literal("None"), python_literal("False"))), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor)))), python_literal("="), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor)))), negative_lookahead(choice!(python_literal("="), python_literal(":="))))
    ))).into()
}

fn invalid_expression() -> Combinator {
    tag("invalid_expression", choice!(
        seq!(negative_lookahead(choice!(seq!(&NAME, &STRING), &SOFT_KEYWORD)), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), choice!(seq!(choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), python_literal("if"), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), python_literal("else"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)))),
        seq!(choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), python_literal("if"), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), negative_lookahead(choice!(python_literal("else"), python_literal(":")))),
        seq!(python_literal("lambda"), opt(choice!(&invalid_lambda_parameters, &lambda_parameters)), python_literal(":"), lookahead(&FSTRING_MIDDLE))
    )).into()
}

fn invalid_type_param() -> Combinator {
    tag("invalid_type_param", choice!(
        seq!(python_literal("*"), &NAME, python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(python_literal("**"), &NAME, python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))
    )).into()
}

fn invalid_legacy_expression() -> Combinator {
    tag("invalid_legacy_expression", seq!(&NAME, negative_lookahead(python_literal("(")), choice!(seq!(&star_expression, repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), seq!(&star_expression, python_literal(",")), &star_expression))).into()
}

fn expression_without_invalid() -> Combinator {
    tag("expression_without_invalid", choice!(
        seq!(choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), python_literal("if"), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), python_literal("else"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction),
        seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))
    )).into()
}

fn invalid_kwarg() -> Combinator {
    tag("invalid_kwarg", choice!(
        seq!(choice!(python_literal("True"), python_literal("False"), python_literal("None")), python_literal("=")),
        seq!(&NAME, python_literal("="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), seq!(&for_if_clause, opt(repeat1(&for_if_clause)))),
        seq!(negative_lookahead(seq!(&NAME, python_literal("="))), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal("=")),
        seq!(python_literal("**"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal("="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))
    )).into()
}

fn invalid_arguments() -> Combinator {
    tag("invalid_arguments", choice!(
        seq!(choice!(seq!(seq!(choice!(choice!(&invalid_starred_expression_unpacking, seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_starred_expression), seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(python_literal(","), choice!(choice!(&invalid_starred_expression_unpacking, seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_starred_expression), seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))))))), python_literal(","), choice!(seq!(seq!(&kwarg_or_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_starred)))), python_literal(","), seprep1(&kwarg_or_double_starred, python_literal(","))), seq!(&kwarg_or_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_starred)))), seq!(&kwarg_or_double_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_double_starred)))))), choice!(seq!(seq!(&kwarg_or_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_starred)))), python_literal(","), seprep1(&kwarg_or_double_starred, python_literal(","))), seq!(&kwarg_or_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_starred)))), seq!(&kwarg_or_double_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_double_starred)))))), python_literal(","), seprep1(seq!(choice!(&invalid_starred_expression_unpacking, seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_starred_expression), negative_lookahead(python_literal("="))), python_literal(","))),
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal(","), opt(choice!(choice!(seq!(seq!(choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))))))), opt(seq!(python_literal(","), &kwargs))), &kwargs), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), seq!(&for_if_clause, opt(repeat1(&for_if_clause))))))),
        seq!(&NAME, python_literal("="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), seq!(&for_if_clause, opt(repeat1(&for_if_clause)))),
        seq!(opt(seq!(choice!(seq!(seq!(choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))))))), opt(seq!(python_literal(","), &kwargs))), &kwargs), python_literal(","))), &NAME, python_literal("="), lookahead(choice!(python_literal(","), python_literal(")")))),
        seq!(choice!(seq!(seq!(choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))))))), opt(seq!(python_literal(","), &kwargs))), &kwargs), seq!(&for_if_clause, opt(repeat1(&for_if_clause)))),
        seq!(choice!(seq!(seq!(choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))))))), opt(seq!(python_literal(","), &kwargs))), &kwargs), python_literal(","), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), seq!(&for_if_clause, opt(repeat1(&for_if_clause)))),
        seq!(choice!(seq!(seq!(choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))))))), opt(seq!(python_literal(","), &kwargs))), &kwargs), python_literal(","), choice!(seq!(seq!(choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))))))), opt(seq!(python_literal(","), &kwargs))), &kwargs))
    )).into()
}

fn func_type_comment() -> Combinator {
    tag("func_type_comment", choice!(
        seq!(&NEWLINE, &TYPE_COMMENT, lookahead(seq!(&NEWLINE, &INDENT))),
        &invalid_double_type_comments,
        &TYPE_COMMENT
    )).into()
}

fn type_expressions() -> Combinator {
    tag("type_expressions", choice!(
        seq!(seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(repeat1(seq!(python_literal(","), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))))), python_literal(","), python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(","), python_literal("**"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(repeat1(seq!(python_literal(","), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))))), python_literal(","), python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(repeat1(seq!(python_literal(","), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))))), python_literal(","), python_literal("**"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(","), python_literal("**"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(python_literal("**"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), opt(repeat1(seq!(python_literal(","), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)))))
    )).into()
}

fn del_t_atom() -> Combinator {
    tag("del_t_atom", choice!(
        &NAME,
        seq!(python_literal("("), choice!(seq!(seq!(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead)), opt(repeat1(choice!(seq!(python_literal("."), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), lookahead(&t_lookahead)), seq!(choice!(seq!(python_literal("("), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal(")")), &invalid_comprehension), lookahead(&t_lookahead)), seq!(python_literal("("), opt(choice!(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))), &invalid_arguments)), python_literal(")"), lookahead(&t_lookahead)))))), python_literal("."), &NAME, negative_lookahead(&t_lookahead)), seq!(seq!(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead)), opt(repeat1(choice!(seq!(python_literal("."), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), lookahead(&t_lookahead)), seq!(choice!(seq!(python_literal("("), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal(")")), &invalid_comprehension), lookahead(&t_lookahead)), seq!(python_literal("("), opt(choice!(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))), &invalid_arguments)), python_literal(")"), lookahead(&t_lookahead)))))), python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead)), &del_t_atom), python_literal(")")),
        seq!(python_literal("("), opt(seq!(seq!(&del_target, opt(repeat1(seq!(python_literal(","), &del_target)))), opt(python_literal(",")))), python_literal(")")),
        seq!(python_literal("["), opt(seq!(seq!(&del_target, opt(repeat1(seq!(python_literal(","), &del_target)))), opt(python_literal(",")))), python_literal("]"))
    )).into()
}

fn del_target() -> Combinator {
    cached(tag("del_target", choice!(
        seq!(seq!(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead)), opt(repeat1(choice!(seq!(python_literal("."), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), lookahead(&t_lookahead)), seq!(choice!(seq!(python_literal("("), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal(")")), &invalid_comprehension), lookahead(&t_lookahead)), seq!(python_literal("("), opt(choice!(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))), &invalid_arguments)), python_literal(")"), lookahead(&t_lookahead)))))), python_literal("."), &NAME, negative_lookahead(&t_lookahead)),
        seq!(seq!(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead)), opt(repeat1(choice!(seq!(python_literal("."), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), lookahead(&t_lookahead)), seq!(choice!(seq!(python_literal("("), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal(")")), &invalid_comprehension), lookahead(&t_lookahead)), seq!(python_literal("("), opt(choice!(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))), &invalid_arguments)), python_literal(")"), lookahead(&t_lookahead)))))), python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead)),
        &del_t_atom
    ))).into()
}

fn del_targets() -> Combinator {
    tag("del_targets", seq!(seq!(&del_target, opt(repeat1(seq!(python_literal(","), &del_target)))), opt(python_literal(",")))).into()
}

fn t_lookahead() -> Combinator {
    tag("t_lookahead", choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    )).into()
}

fn t_primary() -> Combinator {
    tag("t_primary", seq!(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead)), opt(repeat1(choice!(seq!(python_literal("."), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), lookahead(&t_lookahead)), seq!(choice!(seq!(python_literal("("), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal(")")), &invalid_comprehension), lookahead(&t_lookahead)), seq!(python_literal("("), opt(choice!(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))), &invalid_arguments)), python_literal(")"), lookahead(&t_lookahead))))))).into()
}

fn single_subscript_attribute_target() -> Combinator {
    tag("single_subscript_attribute_target", choice!(
        seq!(&t_primary, python_literal("."), &NAME, negative_lookahead(&t_lookahead)),
        seq!(&t_primary, python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead))
    )).into()
}

fn single_target() -> Combinator {
    tag("single_target", choice!(
        &single_subscript_attribute_target,
        &NAME,
        seq!(python_literal("("), &single_target, python_literal(")"))
    )).into()
}

fn star_atom() -> Combinator {
    tag("star_atom", choice!(
        &NAME,
        seq!(python_literal("("), choice!(seq!(&t_primary, python_literal("."), &NAME, negative_lookahead(&t_lookahead)), seq!(&t_primary, python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead)), &star_atom), python_literal(")")),
        seq!(python_literal("("), opt(choice!(seq!(&star_target, repeat1(seq!(python_literal(","), &star_target)), opt(python_literal(","))), seq!(&star_target, python_literal(",")))), python_literal(")")),
        seq!(python_literal("["), opt(seq!(seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target)))), opt(python_literal(",")))), python_literal("]"))
    )).into()
}

fn target_with_star_atom() -> Combinator {
    cached(tag("target_with_star_atom", choice!(
        seq!(&t_primary, python_literal("."), &NAME, negative_lookahead(&t_lookahead)),
        seq!(&t_primary, python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead)),
        &star_atom
    ))).into()
}

fn star_target() -> Combinator {
    cached(tag("star_target", choice!(
        seq!(python_literal("*"), seq!(negative_lookahead(python_literal("*")), &star_target)),
        &target_with_star_atom
    ))).into()
}

fn star_targets_tuple_seq() -> Combinator {
    tag("star_targets_tuple_seq", choice!(
        seq!(&star_target, repeat1(seq!(python_literal(","), &star_target)), opt(python_literal(","))),
        seq!(&star_target, python_literal(","))
    )).into()
}

fn star_targets_list_seq() -> Combinator {
    tag("star_targets_list_seq", seq!(seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target)))), opt(python_literal(",")))).into()
}

fn star_targets() -> Combinator {
    tag("star_targets", choice!(
        seq!(&star_target, negative_lookahead(python_literal(","))),
        seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))
    )).into()
}

fn kwarg_or_double_starred() -> Combinator {
    tag("kwarg_or_double_starred", choice!(
        &invalid_kwarg,
        seq!(&NAME, python_literal("="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(python_literal("**"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))
    )).into()
}

fn kwarg_or_starred() -> Combinator {
    tag("kwarg_or_starred", choice!(
        &invalid_kwarg,
        seq!(&NAME, python_literal("="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        choice!(&invalid_starred_expression_unpacking, seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_starred_expression)
    )).into()
}

fn starred_expression() -> Combinator {
    tag("starred_expression", choice!(
        &invalid_starred_expression_unpacking,
        seq!(python_literal("*"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        &invalid_starred_expression
    )).into()
}

fn kwargs() -> Combinator {
    tag("kwargs", choice!(
        seq!(seq!(&kwarg_or_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_starred)))), python_literal(","), seprep1(&kwarg_or_double_starred, python_literal(","))),
        seq!(&kwarg_or_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_starred)))),
        seq!(&kwarg_or_double_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_double_starred))))
    )).into()
}

fn args() -> Combinator {
    tag("args", choice!(
        seq!(seq!(choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))))))), opt(seq!(python_literal(","), &kwargs))),
        &kwargs
    )).into()
}

fn arguments() -> Combinator {
    cached(tag("arguments", choice!(
        seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))),
        &invalid_arguments
    ))).into()
}

fn dictcomp() -> Combinator {
    tag("dictcomp", choice!(
        seq!(python_literal("{"), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal("}")),
        &invalid_dict_comprehension
    )).into()
}

fn genexp() -> Combinator {
    tag("genexp", choice!(
        seq!(python_literal("("), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal(")")),
        &invalid_comprehension
    )).into()
}

fn setcomp() -> Combinator {
    tag("setcomp", choice!(
        seq!(python_literal("{"), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal("}")),
        &invalid_comprehension
    )).into()
}

fn listcomp() -> Combinator {
    tag("listcomp", choice!(
        seq!(python_literal("["), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))), seq!(&for_if_clause, opt(repeat1(&for_if_clause))), python_literal("]")),
        &invalid_comprehension
    )).into()
}

fn for_if_clause() -> Combinator {
    tag("for_if_clause", choice!(
        seq!(python_literal("async"), python_literal("for"), &star_targets, python_literal("in"), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), opt(repeat1(seq!(python_literal("if"), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction))))),
        seq!(python_literal("for"), &star_targets, python_literal("in"), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction), opt(repeat1(seq!(python_literal("if"), choice!(seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))), &conjunction))))),
        &invalid_for_if_clause,
        &invalid_for_target
    )).into()
}

fn for_if_clauses() -> Combinator {
    tag("for_if_clauses", seq!(&for_if_clause, opt(repeat1(&for_if_clause)))).into()
}

fn kvpair() -> Combinator {
    tag("kvpair", seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))).into()
}

fn double_starred_kvpair() -> Combinator {
    tag("double_starred_kvpair", choice!(
        seq!(python_literal("**"), seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor))))),
        &kvpair
    )).into()
}

fn double_starred_kvpairs() -> Combinator {
    tag("double_starred_kvpairs", seq!(seq!(&double_starred_kvpair, opt(repeat1(seq!(python_literal(","), &double_starred_kvpair)))), opt(python_literal(",")))).into()
}

fn dict() -> Combinator {
    tag("dict", choice!(
        seq!(python_literal("{"), opt(&double_starred_kvpairs), python_literal("}")),
        seq!(python_literal("{"), &invalid_double_starred_kvpairs, python_literal("}"))
    )).into()
}

fn set() -> Combinator {
    tag("set", seq!(python_literal("{"), seq!(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression)))), opt(python_literal(","))), python_literal("}"))).into()
}

fn tuple() -> Combinator {
    tag("tuple", seq!(python_literal("("), opt(seq!(choice!(seq!(python_literal("*"), &bitwise_or), &named_expression), python_literal(","), opt(seq!(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression)))), opt(python_literal(",")))))), python_literal(")"))).into()
}

fn list() -> Combinator {
    tag("list", seq!(python_literal("["), opt(seq!(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression)))), opt(python_literal(",")))), python_literal("]"))).into()
}

fn strings() -> Combinator {
    cached(tag("strings", seq!(choice!(seq!(&FSTRING_START, opt(repeat1(choice!(&fstring_replacement_field, &FSTRING_MIDDLE))), &FSTRING_END), &STRING), opt(repeat1(choice!(seq!(&FSTRING_START, opt(repeat1(choice!(&fstring_replacement_field, &FSTRING_MIDDLE))), &FSTRING_END), &STRING)))))).into()
}

fn string() -> Combinator {
    tag("string", &STRING).into()
}

fn fstring() -> Combinator {
    tag("fstring", seq!(&FSTRING_START, opt(repeat1(choice!(&fstring_replacement_field, &FSTRING_MIDDLE))), &FSTRING_END)).into()
}

fn fstring_format_spec() -> Combinator {
    tag("fstring_format_spec", choice!(
        &FSTRING_MIDDLE,
        choice!(seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), opt(python_literal("=")), opt(&fstring_conversion), opt(&fstring_full_format_spec), python_literal("}")), &invalid_replacement_field)
    )).into()
}

fn fstring_full_format_spec() -> Combinator {
    tag("fstring_full_format_spec", seq!(python_literal(":"), opt(repeat1(&fstring_format_spec)))).into()
}

fn fstring_conversion() -> Combinator {
    tag("fstring_conversion", seq!(python_literal("!"), &NAME)).into()
}

fn fstring_replacement_field() -> Combinator {
    tag("fstring_replacement_field", choice!(
        seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), opt(python_literal("=")), opt(&fstring_conversion), opt(&fstring_full_format_spec), python_literal("}")),
        &invalid_replacement_field
    )).into()
}

fn fstring_middle() -> Combinator {
    tag("fstring_middle", choice!(
        &fstring_replacement_field,
        &FSTRING_MIDDLE
    )).into()
}

fn lambda_param() -> Combinator {
    tag("lambda_param", &NAME).into()
}

fn lambda_param_maybe_default() -> Combinator {
    tag("lambda_param_maybe_default", choice!(
        seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), python_literal(",")),
        seq!(&lambda_param, opt(choice!(seq!(python_literal("="), &expression), &invalid_default)), lookahead(python_literal(":")))
    )).into()
}

fn lambda_param_with_default() -> Combinator {
    tag("lambda_param_with_default", choice!(
        seq!(&lambda_param, choice!(seq!(python_literal("="), &expression), &invalid_default), python_literal(",")),
        seq!(&lambda_param, choice!(seq!(python_literal("="), &expression), &invalid_default), lookahead(python_literal(":")))
    )).into()
}

fn lambda_param_no_default() -> Combinator {
    tag("lambda_param_no_default", choice!(
        seq!(&lambda_param, python_literal(",")),
        seq!(&lambda_param, lookahead(python_literal(":")))
    )).into()
}

fn lambda_kwds() -> Combinator {
    tag("lambda_kwds", choice!(
        &invalid_lambda_kwds,
        seq!(python_literal("**"), &lambda_param_no_default)
    )).into()
}

fn lambda_star_etc() -> Combinator {
    tag("lambda_star_etc", choice!(
        &invalid_lambda_star_etc,
        seq!(python_literal("*"), &lambda_param_no_default, opt(repeat1(&lambda_param_maybe_default)), opt(&lambda_kwds)),
        seq!(python_literal("*"), python_literal(","), repeat1(&lambda_param_maybe_default), opt(&lambda_kwds)),
        &lambda_kwds
    )).into()
}

fn lambda_slash_with_default() -> Combinator {
    tag("lambda_slash_with_default", choice!(
        seq!(opt(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default)))), repeat1(&lambda_param_with_default), python_literal("/"), python_literal(",")),
        seq!(opt(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default)))), repeat1(&lambda_param_with_default), python_literal("/"), lookahead(python_literal(":")))
    )).into()
}

fn lambda_slash_no_default() -> Combinator {
    tag("lambda_slash_no_default", choice!(
        seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), python_literal("/"), python_literal(",")),
        seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), python_literal("/"), lookahead(python_literal(":")))
    )).into()
}

fn lambda_parameters() -> Combinator {
    tag("lambda_parameters", choice!(
        seq!(&lambda_slash_no_default, opt(repeat1(&lambda_param_no_default)), opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(&lambda_slash_with_default, opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_no_default))), opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(seq!(&lambda_param_with_default, opt(repeat1(&lambda_param_with_default))), opt(&lambda_star_etc)),
        &lambda_star_etc
    )).into()
}

fn lambda_params() -> Combinator {
    tag("lambda_params", choice!(
        &invalid_lambda_parameters,
        &lambda_parameters
    )).into()
}

fn lambdef() -> Combinator {
    tag("lambdef", seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))).into()
}

fn group() -> Combinator {
    tag("group", choice!(
        seq!(python_literal("("), choice!(choice!(seq!(python_literal("yield"), python_literal("from"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), seq!(python_literal("yield"), opt(&star_expressions))), choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":="))))), python_literal(")")),
        &invalid_group
    )).into()
}

fn atom() -> Combinator {
    tag("atom", choice!(
        &NAME,
        python_literal("True"),
        python_literal("False"),
        python_literal("None"),
        seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings),
        &NUMBER,
        seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)),
        seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)),
        seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)),
        python_literal("...")
    )).into()
}

fn slice() -> Combinator {
    tag("slice", choice!(
        seq!(opt(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), python_literal(":"), opt(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), opt(seq!(python_literal(":"), opt(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))))),
        choice!(seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)), &invalid_named_expression, seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":="))))
    )).into()
}

fn slices() -> Combinator {
    tag("slices", choice!(
        seq!(&slice, negative_lookahead(python_literal(","))),
        seq!(seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression))))), opt(python_literal(",")))
    )).into()
}

fn primary() -> Combinator {
    tag("primary", seq!(&atom, opt(repeat1(choice!(seq!(python_literal("."), &NAME), &genexp, seq!(python_literal("("), opt(&arguments), python_literal(")")), seq!(python_literal("["), &slices, python_literal("]"))))))).into()
}

fn await_primary() -> Combinator {
    cached(tag("await_primary", choice!(
        seq!(python_literal("await"), &primary),
        &primary
    ))).into()
}

fn power() -> Combinator {
    tag("power", choice!(
        seq!(&await_primary, python_literal("**"), choice!(seq!(python_literal("+"), &factor), seq!(python_literal("-"), &factor), seq!(python_literal("~"), &factor), &power)),
        &await_primary
    )).into()
}

fn factor() -> Combinator {
    cached(tag("factor", choice!(
        seq!(python_literal("+"), &factor),
        seq!(python_literal("-"), &factor),
        seq!(python_literal("~"), &factor),
        &power
    ))).into()
}

fn term() -> Combinator {
    tag("term", seq!(choice!(&invalid_factor, &factor), opt(repeat1(choice!(seq!(python_literal("*"), &factor), seq!(python_literal("/"), &factor), seq!(python_literal("//"), &factor), seq!(python_literal("%"), &factor), seq!(python_literal("@"), &factor)))))).into()
}

fn sum() -> Combinator {
    tag("sum", seq!(&term, opt(repeat1(choice!(seq!(python_literal("+"), &term), seq!(python_literal("-"), &term)))))).into()
}

fn shift_expr() -> Combinator {
    tag("shift_expr", seq!(choice!(&invalid_arithmetic, &sum), opt(repeat1(choice!(seq!(python_literal("<<"), &sum), seq!(python_literal(">>"), &sum)))))).into()
}

fn bitwise_and() -> Combinator {
    tag("bitwise_and", seq!(&shift_expr, opt(repeat1(seq!(python_literal("&"), &shift_expr))))).into()
}

fn bitwise_xor() -> Combinator {
    tag("bitwise_xor", seq!(&bitwise_and, opt(repeat1(seq!(python_literal("^"), &bitwise_and))))).into()
}

fn bitwise_or() -> Combinator {
    tag("bitwise_or", seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor))))).into()
}

fn is_bitwise_or() -> Combinator {
    tag("is_bitwise_or", seq!(python_literal("is"), &bitwise_or)).into()
}

fn isnot_bitwise_or() -> Combinator {
    tag("isnot_bitwise_or", seq!(python_literal("is"), python_literal("not"), &bitwise_or)).into()
}

fn in_bitwise_or() -> Combinator {
    tag("in_bitwise_or", seq!(python_literal("in"), &bitwise_or)).into()
}

fn notin_bitwise_or() -> Combinator {
    tag("notin_bitwise_or", seq!(python_literal("not"), python_literal("in"), &bitwise_or)).into()
}

fn gt_bitwise_or() -> Combinator {
    tag("gt_bitwise_or", seq!(python_literal(">"), &bitwise_or)).into()
}

fn gte_bitwise_or() -> Combinator {
    tag("gte_bitwise_or", seq!(python_literal(">="), &bitwise_or)).into()
}

fn lt_bitwise_or() -> Combinator {
    tag("lt_bitwise_or", seq!(python_literal("<"), &bitwise_or)).into()
}

fn lte_bitwise_or() -> Combinator {
    tag("lte_bitwise_or", seq!(python_literal("<="), &bitwise_or)).into()
}

fn noteq_bitwise_or() -> Combinator {
    tag("noteq_bitwise_or", seq!(python_literal("!="), &bitwise_or)).into()
}

fn eq_bitwise_or() -> Combinator {
    tag("eq_bitwise_or", seq!(python_literal("=="), &bitwise_or)).into()
}

fn compare_op_bitwise_or_pair() -> Combinator {
    tag("compare_op_bitwise_or_pair", choice!(
        &eq_bitwise_or,
        &noteq_bitwise_or,
        &lte_bitwise_or,
        &lt_bitwise_or,
        &gte_bitwise_or,
        &gt_bitwise_or,
        &notin_bitwise_or,
        &in_bitwise_or,
        &isnot_bitwise_or,
        &is_bitwise_or
    )).into()
}

fn comparison() -> Combinator {
    tag("comparison", choice!(
        seq!(&bitwise_or, repeat1(&compare_op_bitwise_or_pair)),
        &bitwise_or
    )).into()
}

fn inversion() -> Combinator {
    cached(tag("inversion", choice!(
        seq!(python_literal("not"), &inversion),
        &comparison
    ))).into()
}

fn conjunction() -> Combinator {
    cached(tag("conjunction", choice!(
        seq!(&inversion, repeat1(seq!(python_literal("and"), &inversion))),
        &inversion
    ))).into()
}

fn disjunction() -> Combinator {
    cached(tag("disjunction", choice!(
        seq!(&conjunction, repeat1(seq!(python_literal("or"), &conjunction))),
        &conjunction
    ))).into()
}

fn named_expression() -> Combinator {
    tag("named_expression", choice!(
        seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        &invalid_named_expression,
        seq!(choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef), negative_lookahead(python_literal(":=")))
    )).into()
}

fn assignment_expression() -> Combinator {
    tag("assignment_expression", seq!(&NAME, python_literal(":="), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef))).into()
}

fn star_named_expression() -> Combinator {
    tag("star_named_expression", choice!(
        seq!(python_literal("*"), &bitwise_or),
        &named_expression
    )).into()
}

fn star_named_expressions() -> Combinator {
    tag("star_named_expressions", seq!(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression)))), opt(python_literal(",")))).into()
}

fn star_expression() -> Combinator {
    cached(tag("star_expression", choice!(
        seq!(python_literal("*"), &bitwise_or),
        choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)
    ))).into()
}

fn star_expressions() -> Combinator {
    tag("star_expressions", choice!(
        seq!(&star_expression, repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))),
        seq!(&star_expression, python_literal(",")),
        &star_expression
    )).into()
}

fn yield_expr() -> Combinator {
    tag("yield_expr", choice!(
        seq!(python_literal("yield"), python_literal("from"), choice!(&invalid_expression, &invalid_legacy_expression, seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression), &disjunction, &lambdef)),
        seq!(python_literal("yield"), opt(&star_expressions))
    )).into()
}

fn expression() -> Combinator {
    cached(tag("expression", choice!(
        &invalid_expression,
        &invalid_legacy_expression,
        seq!(&disjunction, python_literal("if"), &disjunction, python_literal("else"), &expression),
        &disjunction,
        &lambdef
    ))).into()
}

fn expressions() -> Combinator {
    tag("expressions", choice!(
        seq!(&expression, repeat1(seq!(python_literal(","), &expression)), opt(python_literal(","))),
        seq!(&expression, python_literal(",")),
        &expression
    )).into()
}

fn type_param_starred_default() -> Combinator {
    tag("type_param_starred_default", seq!(python_literal("="), &star_expression)).into()
}

fn type_param_default() -> Combinator {
    tag("type_param_default", seq!(python_literal("="), &expression)).into()
}

fn type_param_bound() -> Combinator {
    tag("type_param_bound", seq!(python_literal(":"), &expression)).into()
}

fn type_param() -> Combinator {
    cached(tag("type_param", choice!(
        seq!(&NAME, opt(&type_param_bound), opt(&type_param_default)),
        &invalid_type_param,
        seq!(python_literal("*"), &NAME, opt(&type_param_starred_default)),
        seq!(python_literal("**"), &NAME, opt(&type_param_default))
    ))).into()
}

fn type_param_seq() -> Combinator {
    tag("type_param_seq", seq!(seq!(&type_param, opt(repeat1(seq!(python_literal(","), &type_param)))), opt(python_literal(",")))).into()
}

fn type_params() -> Combinator {
    tag("type_params", choice!(
        &invalid_type_params,
        seq!(python_literal("["), &type_param_seq, python_literal("]"))
    )).into()
}

fn type_alias() -> Combinator {
    tag("type_alias", seq!(
        python_literal("type"),
         &NAME,
         opt(&type_params),
         python_literal("="),
         &expression
    )).into()
}

fn keyword_pattern() -> Combinator {
    tag("keyword_pattern", seq!(&NAME, python_literal("="), choice!(&as_pattern, &or_pattern))).into()
}

fn keyword_patterns() -> Combinator {
    tag("keyword_patterns", seq!(&keyword_pattern, opt(repeat1(seq!(python_literal(","), &keyword_pattern))))).into()
}

fn positional_patterns() -> Combinator {
    tag("positional_patterns", seq!(choice!(&as_pattern, &or_pattern), opt(repeat1(seq!(python_literal(","), choice!(&as_pattern, &or_pattern)))))).into()
}

fn class_pattern() -> Combinator {
    tag("class_pattern", choice!(
        seq!(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME)))), python_literal("("), python_literal(")")),
        seq!(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME)))), python_literal("("), &positional_patterns, opt(python_literal(",")), python_literal(")")),
        seq!(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME)))), python_literal("("), &keyword_patterns, opt(python_literal(",")), python_literal(")")),
        seq!(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME)))), python_literal("("), &positional_patterns, python_literal(","), &keyword_patterns, opt(python_literal(",")), python_literal(")")),
        &invalid_class_pattern
    )).into()
}

fn double_star_pattern() -> Combinator {
    tag("double_star_pattern", seq!(python_literal("**"), seq!(negative_lookahead(python_literal("_")), &NAME, negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("=")))))).into()
}

fn key_value_pattern() -> Combinator {
    tag("key_value_pattern", seq!(choice!(choice!(seq!(&signed_number, negative_lookahead(choice!(python_literal("+"), python_literal("-")))), &complex_number, &strings, python_literal("None"), python_literal("True"), python_literal("False")), seq!(&name_or_attr, python_literal("."), &NAME)), python_literal(":"), choice!(&as_pattern, &or_pattern))).into()
}

fn items_pattern() -> Combinator {
    tag("items_pattern", seq!(&key_value_pattern, opt(repeat1(seq!(python_literal(","), &key_value_pattern))))).into()
}

fn mapping_pattern() -> Combinator {
    tag("mapping_pattern", choice!(
        seq!(python_literal("{"), python_literal("}")),
        seq!(python_literal("{"), &double_star_pattern, opt(python_literal(",")), python_literal("}")),
        seq!(python_literal("{"), &items_pattern, python_literal(","), &double_star_pattern, opt(python_literal(",")), python_literal("}")),
        seq!(python_literal("{"), &items_pattern, opt(python_literal(",")), python_literal("}"))
    )).into()
}

fn star_pattern() -> Combinator {
    cached(tag("star_pattern", choice!(
        seq!(python_literal("*"), seq!(negative_lookahead(python_literal("_")), &NAME, negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("="))))),
        seq!(python_literal("*"), python_literal("_"))
    ))).into()
}

fn maybe_star_pattern() -> Combinator {
    tag("maybe_star_pattern", choice!(
        &star_pattern,
        choice!(&as_pattern, &or_pattern)
    )).into()
}

fn maybe_sequence_pattern() -> Combinator {
    tag("maybe_sequence_pattern", seq!(seq!(&maybe_star_pattern, opt(repeat1(seq!(python_literal(","), &maybe_star_pattern)))), opt(python_literal(",")))).into()
}

fn open_sequence_pattern() -> Combinator {
    tag("open_sequence_pattern", seq!(&maybe_star_pattern, python_literal(","), opt(&maybe_sequence_pattern))).into()
}

fn sequence_pattern() -> Combinator {
    tag("sequence_pattern", choice!(
        seq!(python_literal("["), opt(&maybe_sequence_pattern), python_literal("]")),
        seq!(python_literal("("), opt(&open_sequence_pattern), python_literal(")"))
    )).into()
}

fn group_pattern() -> Combinator {
    tag("group_pattern", seq!(python_literal("("), choice!(&as_pattern, &or_pattern), python_literal(")"))).into()
}

fn name_or_attr() -> Combinator {
    tag("name_or_attr", seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME))))).into()
}

fn attr() -> Combinator {
    tag("attr", seq!(&name_or_attr, python_literal("."), &NAME)).into()
}

fn value_pattern() -> Combinator {
    tag("value_pattern", seq!(&attr, negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("="))))).into()
}

fn wildcard_pattern() -> Combinator {
    tag("wildcard_pattern", python_literal("_")).into()
}

fn pattern_capture_target() -> Combinator {
    tag("pattern_capture_target", seq!(negative_lookahead(python_literal("_")), &NAME, negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("="))))).into()
}

fn capture_pattern() -> Combinator {
    tag("capture_pattern", &pattern_capture_target).into()
}

fn imaginary_number() -> Combinator {
    tag("imaginary_number", &NUMBER).into()
}

fn real_number() -> Combinator {
    tag("real_number", &NUMBER).into()
}

fn signed_real_number() -> Combinator {
    tag("signed_real_number", choice!(
        &real_number,
        seq!(python_literal("-"), &real_number)
    )).into()
}

fn signed_number() -> Combinator {
    tag("signed_number", choice!(
        &NUMBER,
        seq!(python_literal("-"), &NUMBER)
    )).into()
}

fn complex_number() -> Combinator {
    tag("complex_number", choice!(
        seq!(&signed_real_number, python_literal("+"), &imaginary_number),
        seq!(&signed_real_number, python_literal("-"), &imaginary_number)
    )).into()
}

fn literal_expr() -> Combinator {
    tag("literal_expr", choice!(
        seq!(&signed_number, negative_lookahead(choice!(python_literal("+"), python_literal("-")))),
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).into()
}

fn literal_pattern() -> Combinator {
    tag("literal_pattern", choice!(
        seq!(&signed_number, negative_lookahead(choice!(python_literal("+"), python_literal("-")))),
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).into()
}

fn closed_pattern() -> Combinator {
    cached(tag("closed_pattern", choice!(
        &literal_pattern,
        &capture_pattern,
        &wildcard_pattern,
        &value_pattern,
        &group_pattern,
        &sequence_pattern,
        &mapping_pattern,
        &class_pattern
    ))).into()
}

fn or_pattern() -> Combinator {
    tag("or_pattern", seq!(&closed_pattern, opt(repeat1(seq!(python_literal("|"), &closed_pattern))))).into()
}

fn as_pattern() -> Combinator {
    tag("as_pattern", choice!(
        seq!(&or_pattern, python_literal("as"), &pattern_capture_target),
        &invalid_as_pattern
    )).into()
}

fn pattern() -> Combinator {
    tag("pattern", choice!(
        &as_pattern,
        &or_pattern
    )).into()
}

fn patterns() -> Combinator {
    tag("patterns", choice!(
        &open_sequence_pattern,
        &pattern
    )).into()
}

fn guard() -> Combinator {
    tag("guard", seq!(python_literal("if"), &named_expression)).into()
}

fn case_block() -> Combinator {
    tag("case_block", choice!(
        &invalid_case_block,
        seq!(python_literal("case"), &patterns, opt(&guard), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))
    )).into()
}

fn subject_expr() -> Combinator {
    tag("subject_expr", choice!(
        seq!(&star_named_expression, python_literal(","), opt(&star_named_expressions)),
        &named_expression
    )).into()
}

fn match_stmt() -> Combinator {
    tag("match_stmt", choice!(
        seq!(python_literal("match"), &subject_expr, python_literal(":"), &NEWLINE, &INDENT, repeat1(&case_block), &DEDENT),
        &invalid_match_stmt
    )).into()
}

fn finally_block() -> Combinator {
    tag("finally_block", choice!(
        &invalid_finally_stmt,
        seq!(python_literal("finally"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))
    )).into()
}

fn except_star_block() -> Combinator {
    tag("except_star_block", choice!(
        &invalid_except_star_stmt_indent,
        seq!(python_literal("except"), python_literal("*"), &expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)),
        &invalid_except_stmt
    )).into()
}

fn except_block() -> Combinator {
    tag("except_block", choice!(
        &invalid_except_stmt_indent,
        seq!(python_literal("except"), &expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)),
        seq!(python_literal("except"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)),
        &invalid_except_stmt
    )).into()
}

fn try_stmt() -> Combinator {
    tag("try_stmt", choice!(
        &invalid_try_stmt,
        seq!(python_literal("try"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), &finally_block),
        seq!(python_literal("try"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), repeat1(&except_block), opt(choice!(&invalid_else_stmt, seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)))), opt(&finally_block)),
        seq!(python_literal("try"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), repeat1(&except_star_block), opt(choice!(&invalid_else_stmt, seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)))), opt(&finally_block))
    )).into()
}

fn with_item() -> Combinator {
    tag("with_item", choice!(
        seq!(&expression, python_literal("as"), &star_target, lookahead(choice!(python_literal(","), python_literal(")"), python_literal(":")))),
        &invalid_with_item,
        &expression
    )).into()
}

fn with_stmt() -> Combinator {
    tag("with_stmt", choice!(
        &invalid_with_stmt_indent,
        seq!(python_literal("with"), python_literal("("), seprep1(&with_item, python_literal(",")), opt(python_literal(",")), python_literal(")"), python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)),
        seq!(python_literal("with"), seprep1(&with_item, python_literal(",")), python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)),
        seq!(python_literal("async"), python_literal("with"), python_literal("("), seprep1(&with_item, python_literal(",")), opt(python_literal(",")), python_literal(")"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)),
        seq!(python_literal("async"), python_literal("with"), seprep1(&with_item, python_literal(",")), python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)),
        &invalid_with_stmt
    )).into()
}

fn for_stmt() -> Combinator {
    tag("for_stmt", choice!(
        &invalid_for_stmt,
        seq!(python_literal("for"), &star_targets, python_literal("in"), &star_expressions, python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), opt(choice!(&invalid_else_stmt, seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))))),
        seq!(python_literal("async"), python_literal("for"), &star_targets, python_literal("in"), &star_expressions, python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), opt(choice!(&invalid_else_stmt, seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))))),
        &invalid_for_target
    )).into()
}

fn while_stmt() -> Combinator {
    tag("while_stmt", choice!(
        &invalid_while_stmt,
        seq!(python_literal("while"), &named_expression, python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), opt(choice!(&invalid_else_stmt, seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)))))
    )).into()
}

fn else_block() -> Combinator {
    tag("else_block", choice!(
        &invalid_else_stmt,
        seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))
    )).into()
}

fn elif_stmt() -> Combinator {
    tag("elif_stmt", choice!(
        &invalid_elif_stmt,
        seq!(python_literal("elif"), &named_expression, python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), &elif_stmt),
        seq!(python_literal("elif"), &named_expression, python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), opt(&else_block))
    )).into()
}

fn if_stmt() -> Combinator {
    tag("if_stmt", choice!(
        &invalid_if_stmt,
        seq!(python_literal("if"), &named_expression, python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), &elif_stmt),
        seq!(python_literal("if"), &named_expression, python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block), opt(&else_block))
    )).into()
}

fn default() -> Combinator {
    tag("default", choice!(
        seq!(python_literal("="), &expression),
        &invalid_default
    )).into()
}

fn star_annotation() -> Combinator {
    tag("star_annotation", seq!(python_literal(":"), &star_expression)).into()
}

fn annotation() -> Combinator {
    tag("annotation", seq!(python_literal(":"), &expression)).into()
}

fn param_star_annotation() -> Combinator {
    tag("param_star_annotation", seq!(&NAME, &star_annotation)).into()
}

fn param() -> Combinator {
    tag("param", seq!(&NAME, opt(&annotation))).into()
}

fn param_maybe_default() -> Combinator {
    tag("param_maybe_default", choice!(
        seq!(&param, opt(&default), python_literal(","), opt(&TYPE_COMMENT)),
        seq!(&param, opt(&default), opt(&TYPE_COMMENT), lookahead(python_literal(")")))
    )).into()
}

fn param_with_default() -> Combinator {
    tag("param_with_default", choice!(
        seq!(&param, &default, python_literal(","), opt(&TYPE_COMMENT)),
        seq!(&param, &default, opt(&TYPE_COMMENT), lookahead(python_literal(")")))
    )).into()
}

fn param_no_default_star_annotation() -> Combinator {
    tag("param_no_default_star_annotation", choice!(
        seq!(&param_star_annotation, python_literal(","), opt(&TYPE_COMMENT)),
        seq!(&param_star_annotation, opt(&TYPE_COMMENT), lookahead(python_literal(")")))
    )).into()
}

fn param_no_default() -> Combinator {
    tag("param_no_default", choice!(
        seq!(&param, python_literal(","), opt(&TYPE_COMMENT)),
        seq!(&param, opt(&TYPE_COMMENT), lookahead(python_literal(")")))
    )).into()
}

fn kwds() -> Combinator {
    tag("kwds", choice!(
        &invalid_kwds,
        seq!(python_literal("**"), &param_no_default)
    )).into()
}

fn star_etc() -> Combinator {
    tag("star_etc", choice!(
        &invalid_star_etc,
        seq!(python_literal("*"), &param_no_default, opt(repeat1(&param_maybe_default)), opt(&kwds)),
        seq!(python_literal("*"), &param_no_default_star_annotation, opt(repeat1(&param_maybe_default)), opt(&kwds)),
        seq!(python_literal("*"), python_literal(","), repeat1(&param_maybe_default), opt(&kwds)),
        &kwds
    )).into()
}

fn slash_with_default() -> Combinator {
    tag("slash_with_default", choice!(
        seq!(opt(seq!(&param_no_default, opt(repeat1(&param_no_default)))), repeat1(&param_with_default), python_literal("/"), python_literal(",")),
        seq!(opt(seq!(&param_no_default, opt(repeat1(&param_no_default)))), repeat1(&param_with_default), python_literal("/"), lookahead(python_literal(")")))
    )).into()
}

fn slash_no_default() -> Combinator {
    tag("slash_no_default", choice!(
        seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), python_literal("/"), python_literal(",")),
        seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), python_literal("/"), lookahead(python_literal(")")))
    )).into()
}

fn parameters() -> Combinator {
    tag("parameters", choice!(
        seq!(&slash_no_default, opt(repeat1(&param_no_default)), opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(&slash_with_default, opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(seq!(&param_no_default, opt(repeat1(&param_no_default))), opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(seq!(&param_with_default, opt(repeat1(&param_with_default))), opt(&star_etc)),
        &star_etc
    )).into()
}

fn params() -> Combinator {
    tag("params", choice!(
        &invalid_parameters,
        &parameters
    )).into()
}

fn function_def_raw() -> Combinator {
    tag("function_def_raw", choice!(
        &invalid_def_raw,
        seq!(python_literal("def"), &NAME, opt(&type_params), python_literal("("), opt(&params), python_literal(")"), opt(seq!(python_literal("->"), &expression)), python_literal(":"), opt(&func_type_comment), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block)),
        seq!(python_literal("async"), python_literal("def"), &NAME, opt(&type_params), python_literal("("), opt(&params), python_literal(")"), opt(seq!(python_literal("->"), &expression)), python_literal(":"), opt(&func_type_comment), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))
    )).into()
}

fn function_def() -> Combinator {
    tag("function_def", choice!(
        seq!(seq!(seq!(python_literal("@"), &named_expression, &NEWLINE), opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE)))), &function_def_raw),
        &function_def_raw
    )).into()
}

fn class_def_raw() -> Combinator {
    tag("class_def_raw", choice!(
        &invalid_class_def_raw,
        seq!(python_literal("class"), &NAME, opt(&type_params), opt(seq!(python_literal("("), opt(&arguments), python_literal(")"))), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)), &invalid_block))
    )).into()
}

fn class_def() -> Combinator {
    tag("class_def", choice!(
        seq!(seq!(seq!(python_literal("@"), &named_expression, &NEWLINE), opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE)))), &class_def_raw),
        &class_def_raw
    )).into()
}

fn decorators() -> Combinator {
    tag("decorators", seq!(seq!(python_literal("@"), &named_expression, &NEWLINE), opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE))))).into()
}

fn block() -> Combinator {
    cached(tag("block", choice!(
        seq!(&NEWLINE, &INDENT, seq!(&statement, opt(repeat1(&statement))), &DEDENT),
        choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)),
        &invalid_block
    ))).into()
}

fn dotted_name() -> Combinator {
    tag("dotted_name", seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME))))).into()
}

fn dotted_as_name() -> Combinator {
    tag("dotted_as_name", seq!(&dotted_name, opt(seq!(python_literal("as"), &NAME)))).into()
}

fn dotted_as_names() -> Combinator {
    tag("dotted_as_names", seq!(&dotted_as_name, opt(repeat1(seq!(python_literal(","), &dotted_as_name))))).into()
}

fn import_from_as_name() -> Combinator {
    tag("import_from_as_name", seq!(&NAME, opt(seq!(python_literal("as"), &NAME)))).into()
}

fn import_from_as_names() -> Combinator {
    tag("import_from_as_names", seq!(&import_from_as_name, opt(repeat1(seq!(python_literal(","), &import_from_as_name))))).into()
}

fn import_from_targets() -> Combinator {
    tag("import_from_targets", choice!(
        seq!(python_literal("("), &import_from_as_names, opt(python_literal(",")), python_literal(")")),
        seq!(&import_from_as_names, negative_lookahead(python_literal(","))),
        python_literal("*"),
        &invalid_import_from_targets
    )).into()
}

fn import_from() -> Combinator {
    tag("import_from", choice!(
        seq!(python_literal("from"), opt(repeat1(choice!(python_literal("."), python_literal("...")))), &dotted_name, python_literal("import"), &import_from_targets),
        seq!(python_literal("from"), repeat1(choice!(python_literal("."), python_literal("..."))), python_literal("import"), &import_from_targets)
    )).into()
}

fn import_name() -> Combinator {
    tag("import_name", seq!(python_literal("import"), &dotted_as_names)).into()
}

fn import_stmt() -> Combinator {
    tag("import_stmt", choice!(
        &invalid_import,
        &import_name,
        &import_from
    )).into()
}

fn assert_stmt() -> Combinator {
    tag("assert_stmt", seq!(python_literal("assert"), &expression, opt(seq!(python_literal(","), &expression)))).into()
}

fn yield_stmt() -> Combinator {
    tag("yield_stmt", &yield_expr).into()
}

fn del_stmt() -> Combinator {
    tag("del_stmt", choice!(
        seq!(python_literal("del"), &del_targets, lookahead(choice!(python_literal(";"), &NEWLINE))),
        &invalid_del_stmt
    )).into()
}

fn nonlocal_stmt() -> Combinator {
    tag("nonlocal_stmt", seq!(python_literal("nonlocal"), seprep1(&NAME, python_literal(",")))).into()
}

fn global_stmt() -> Combinator {
    tag("global_stmt", seq!(python_literal("global"), seprep1(&NAME, python_literal(",")))).into()
}

fn raise_stmt() -> Combinator {
    tag("raise_stmt", choice!(
        seq!(python_literal("raise"), &expression, opt(seq!(python_literal("from"), &expression))),
        python_literal("raise")
    )).into()
}

fn return_stmt() -> Combinator {
    tag("return_stmt", seq!(python_literal("return"), opt(&star_expressions))).into()
}

fn augassign() -> Combinator {
    tag("augassign", choice!(
        python_literal("+="),
        python_literal("-="),
        python_literal("*="),
        python_literal("@="),
        python_literal("/="),
        python_literal("%="),
        python_literal("&="),
        python_literal("|="),
        python_literal("^="),
        python_literal("<<="),
        python_literal(">>="),
        python_literal("**="),
        python_literal("//=")
    )).into()
}

fn annotated_rhs() -> Combinator {
    tag("annotated_rhs", choice!(
        &yield_expr,
        &star_expressions
    )).into()
}

fn assignment() -> Combinator {
    tag("assignment", choice!(
        seq!(&NAME, python_literal(":"), &expression, opt(seq!(python_literal("="), &annotated_rhs))),
        seq!(choice!(seq!(python_literal("("), &single_target, python_literal(")")), &single_subscript_attribute_target), python_literal(":"), &expression, opt(seq!(python_literal("="), &annotated_rhs))),
        seq!(seq!(seq!(&star_targets, python_literal("=")), opt(repeat1(seq!(&star_targets, python_literal("="))))), choice!(&yield_expr, &star_expressions), negative_lookahead(python_literal("=")), opt(&TYPE_COMMENT)),
        seq!(&single_target, &augassign, choice!(&yield_expr, &star_expressions)),
        &invalid_assignment
    )).into()
}

fn compound_stmt() -> Combinator {
    tag("compound_stmt", choice!(
        seq!(lookahead(choice!(python_literal("def"), python_literal("@"), python_literal("async"))), &function_def),
        seq!(lookahead(python_literal("if")), &if_stmt),
        seq!(lookahead(choice!(python_literal("class"), python_literal("@"))), &class_def),
        seq!(lookahead(choice!(python_literal("with"), python_literal("async"))), &with_stmt),
        seq!(lookahead(choice!(python_literal("for"), python_literal("async"))), &for_stmt),
        seq!(lookahead(python_literal("try")), &try_stmt),
        seq!(lookahead(python_literal("while")), &while_stmt),
        &match_stmt
    )).into()
}

fn simple_stmt() -> Combinator {
    cached(tag("simple_stmt", choice!(
        &assignment,
        seq!(lookahead(python_literal("type")), &type_alias),
        &star_expressions,
        seq!(lookahead(python_literal("return")), &return_stmt),
        seq!(lookahead(choice!(python_literal("import"), python_literal("from"))), &import_stmt),
        seq!(lookahead(python_literal("raise")), &raise_stmt),
        python_literal("pass"),
        seq!(lookahead(python_literal("del")), &del_stmt),
        seq!(lookahead(python_literal("yield")), &yield_stmt),
        seq!(lookahead(python_literal("assert")), &assert_stmt),
        python_literal("break"),
        python_literal("continue"),
        seq!(lookahead(python_literal("global")), &global_stmt),
        seq!(lookahead(python_literal("nonlocal")), &nonlocal_stmt)
    ))).into()
}

fn simple_stmts() -> Combinator {
    tag("simple_stmts", choice!(
        seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE),
        seq!(seq!(&simple_stmt, opt(repeat1(seq!(python_literal(";"), &simple_stmt)))), opt(python_literal(";")), &NEWLINE)
    )).into()
}

fn statement_newline() -> Combinator {
    tag("statement_newline", choice!(
        seq!(&compound_stmt, &NEWLINE),
        &simple_stmts,
        &NEWLINE,
        &ENDMARKER
    )).into()
}

fn statement() -> Combinator {
    tag("statement", choice!(
        &compound_stmt,
        &simple_stmts
    )).into()
}

fn statements() -> Combinator {
    tag("statements", seq!(&statement, opt(repeat1(&statement)))).into()
}

fn func_type() -> Combinator {
    tag("func_type", seq!(
        python_literal("("),
         opt(&type_expressions),
         python_literal(")"),
         python_literal("->"),
         &expression,
         opt(repeat1(&NEWLINE)),
         &ENDMARKER
    )).into()
}

fn eval() -> Combinator {
    tag("eval", seq!(&expressions, opt(repeat1(&NEWLINE)), &ENDMARKER)).into()
}

fn interactive() -> Combinator {
    tag("interactive", &statement_newline).into()
}

fn file() -> Combinator {
    tag("file", seq!(opt(&statements), &ENDMARKER)).into()
}


pub fn python_file() -> Combinator {

    cache_context(tag("main", seq!(opt(&NEWLINE), &file))).compile()
}
