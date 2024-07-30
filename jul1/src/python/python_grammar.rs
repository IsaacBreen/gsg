use std::rc::Rc;
use crate::{cache_context, cached, symbol, Symbol, choice, Choice, deferred, Combinator, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, forward_decls, forward_ref, opt, Repeat1, seprep0, seprep1, Seq, tag, Compile};
use super::python_tokenizer::python_literal;
use crate::{seq, repeat0, repeat1};

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
fn WS() -> Combinator { cached(tag("WS", seq!(forbid_follows_check_not(Forbidden::WS as usize), token::WS().compile(), forbid_follows(&[Forbidden::DEDENT as usize, Forbidden::INDENT as usize, Forbidden::NEWLINE as usize])))).into() }
fn NAME() -> Combinator { cached(tag("NAME", seq!(forbid_follows_check_not(Forbidden::NAME as usize), token::NAME().compile(), forbid_follows(&[Forbidden::NAME as usize, Forbidden::NUMBER as usize])))).into() }
fn TYPE_COMMENT() -> Combinator { cached(tag("TYPE_COMMENT", seq!(token::TYPE_COMMENT().compile()))).into() }
fn FSTRING_START() -> Combinator { cached(tag("FSTRING_START", seq!(token::FSTRING_START().compile(), forbid_follows(&[Forbidden::WS as usize])))).into() }
fn FSTRING_MIDDLE() -> Combinator { cached(tag("FSTRING_MIDDLE", seq!(forbid_follows_check_not(Forbidden::FSTRING_MIDDLE as usize), token::FSTRING_MIDDLE().compile(), forbid_follows(&[Forbidden::FSTRING_MIDDLE as usize, Forbidden::WS as usize])))).into() }
fn FSTRING_END() -> Combinator { cached(tag("FSTRING_END", seq!(token::FSTRING_END().compile()))).into() }
fn NUMBER() -> Combinator { cached(tag("NUMBER", seq!(forbid_follows_check_not(Forbidden::NUMBER as usize), token::NUMBER().compile(), forbid_follows(&[Forbidden::NUMBER as usize])))).into() }
fn STRING() -> Combinator { cached(tag("STRING", seq!(token::STRING().compile()))).into() }
fn NEWLINE() -> Combinator { cached(tag("NEWLINE", seq!(forbid_follows_check_not(Forbidden::NEWLINE as usize), token::NEWLINE().compile(), forbid_follows(&[Forbidden::WS as usize])))).into() }
fn INDENT() -> Combinator { cached(tag("INDENT", seq!(forbid_follows_check_not(Forbidden::INDENT as usize), token::INDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))).into() }
fn DEDENT() -> Combinator { cached(tag("DEDENT", seq!(forbid_follows_check_not(Forbidden::DEDENT as usize), token::DEDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))).into() }
fn ENDMARKER() -> Combinator { cached(tag("ENDMARKER", seq!(token::ENDMARKER().compile()))).into() }

fn expression_without_invalid() -> Combinator {
    tag("expression_without_invalid", choice!(
        seq!(deferred(conjunction), opt(seq!(opt(deferred(WS)), python_literal("or"), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(conjunction), opt(repeat1(seq!(opt(deferred(WS)), python_literal("or"), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(conjunction)))))), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))),
        seq!(python_literal("lambda"), opt(seq!(opt(deferred(WS)), deferred(lambda_params))), opt(deferred(WS)), python_literal(":"), opt(deferred(WS)), deferred(expression))
    )).into()
}

fn func_type_comment() -> Combinator {
    tag("func_type_comment", choice!(
        seq!(deferred(NEWLINE), opt(deferred(WS)), deferred(TYPE_COMMENT)),
        deferred(TYPE_COMMENT)
    )).into()
}

fn type_expressions() -> Combinator {
    tag("type_expressions", choice!(
        seq!(choice!(seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))), deferred(lambdef)), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(expression), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(expression)))))), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), choice!(seq!(python_literal("*"), opt(deferred(WS)), deferred(expression), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), python_literal("**"), opt(deferred(WS)), deferred(expression)))), seq!(python_literal("**"), opt(deferred(WS)), deferred(expression)))))),
        seq!(python_literal("*"), opt(deferred(WS)), deferred(expression), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), python_literal("**"), opt(deferred(WS)), deferred(expression)))),
        seq!(python_literal("**"), opt(deferred(WS)), deferred(expression))
    )).into()
}

fn del_t_atom() -> Combinator {
    tag("del_t_atom", choice!(
        deferred(NAME),
        seq!(python_literal("("), opt(deferred(WS)), choice!(seq!(deferred(del_target), opt(deferred(WS)), python_literal(")")), seq!(opt(seq!(deferred(del_targets), opt(deferred(WS)))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(opt(deferred(WS)), deferred(del_targets))), opt(deferred(WS)), python_literal("]"))
    )).into()
}

fn del_target() -> Combinator {
    cached(tag("del_target", choice!(
        seq!(choice!(deferred(NAME), python_literal("True"), python_literal("False"), python_literal("None"), deferred(strings), deferred(NUMBER), deferred(tuple), deferred(group), deferred(genexp), deferred(list), deferred(listcomp), deferred(dict), deferred(set), deferred(dictcomp), deferred(setcomp), python_literal("...")), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(NAME)), seq!(python_literal("["), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(slices), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), python_literal("]")), deferred(genexp), seq!(python_literal("("), opt(seq!(opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(arguments))), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), python_literal(")"))), opt(repeat1(seq!(opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(NAME)), seq!(python_literal("["), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(slices), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), python_literal("]")), deferred(genexp), seq!(python_literal("("), opt(seq!(opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(arguments))), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), python_literal(")")))))))), opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), deferred(NAME)), seq!(python_literal("["), opt(deferred(WS)), deferred(slices), opt(deferred(WS)), python_literal("]")))),
        deferred(del_t_atom)
    ))).into()
}

fn del_targets() -> Combinator {
    tag("del_targets", seq!(deferred(del_target), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(del_target), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(del_target)))))), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn t_lookahead() -> Combinator {
    tag("t_lookahead", choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    )).into()
}

fn t_primary() -> Combinator {
    tag("t_primary", seq!(choice!(deferred(NAME), python_literal("True"), python_literal("False"), python_literal("None"), deferred(strings), deferred(NUMBER), deferred(tuple), deferred(group), deferred(genexp), deferred(list), deferred(listcomp), deferred(dict), deferred(set), deferred(dictcomp), deferred(setcomp), python_literal("...")), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), deferred(NAME)), seq!(python_literal("["), opt(deferred(WS)), deferred(slices), opt(deferred(WS)), python_literal("]")), deferred(genexp), seq!(python_literal("("), opt(seq!(opt(deferred(WS)), deferred(arguments))), opt(deferred(WS)), python_literal(")"))), opt(repeat1(seq!(opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), deferred(NAME)), seq!(python_literal("["), opt(deferred(WS)), deferred(slices), opt(deferred(WS)), python_literal("]")), deferred(genexp), seq!(python_literal("("), opt(seq!(opt(deferred(WS)), deferred(arguments))), opt(deferred(WS)), python_literal(")")))))))))).into()
}

fn single_subscript_attribute_target() -> Combinator {
    tag("single_subscript_attribute_target", seq!(deferred(t_primary), opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), deferred(NAME)), seq!(python_literal("["), opt(deferred(WS)), deferred(slices), opt(deferred(WS)), python_literal("]"))))).into()
}

fn single_target() -> Combinator {
    tag("single_target", choice!(
        deferred(single_subscript_attribute_target),
        deferred(NAME),
        seq!(python_literal("("), opt(deferred(WS)), deferred(single_target), opt(deferred(WS)), python_literal(")"))
    )).into()
}

fn star_atom() -> Combinator {
    tag("star_atom", choice!(
        deferred(NAME),
        seq!(python_literal("("), opt(deferred(WS)), choice!(seq!(deferred(target_with_star_atom), opt(deferred(WS)), python_literal(")")), seq!(opt(seq!(deferred(star_targets_tuple_seq), opt(deferred(WS)))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(opt(deferred(WS)), deferred(star_targets_list_seq))), opt(deferred(WS)), python_literal("]"))
    )).into()
}

fn target_with_star_atom() -> Combinator {
    cached(tag("target_with_star_atom", choice!(
        seq!(deferred(t_primary), opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), deferred(NAME)), seq!(python_literal("["), opt(deferred(WS)), deferred(slices), opt(deferred(WS)), python_literal("]")))),
        deferred(star_atom)
    ))).into()
}

fn star_target() -> Combinator {
    cached(tag("star_target", choice!(
        seq!(python_literal("*"), opt(deferred(WS)), deferred(star_target)),
        deferred(target_with_star_atom)
    ))).into()
}

fn star_targets_tuple_seq() -> Combinator {
    tag("star_targets_tuple_seq", seq!(deferred(star_target), opt(deferred(WS)), python_literal(","), opt(seq!(opt(deferred(WS)), deferred(star_target), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(star_target)))), opt(seq!(opt(deferred(WS)), python_literal(","))))))).into()
}

fn star_targets_list_seq() -> Combinator {
    tag("star_targets_list_seq", seq!(deferred(star_target), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(star_target), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(star_target)))))), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn star_targets() -> Combinator {
    tag("star_targets", seq!(deferred(star_target), opt(seq!(opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(star_target), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(star_target)))))), opt(seq!(opt(deferred(WS)), python_literal(","))))))).into()
}

fn kwarg_or_double_starred() -> Combinator {
    tag("kwarg_or_double_starred", choice!(
        seq!(deferred(NAME), opt(deferred(WS)), python_literal("="), opt(deferred(WS)), deferred(expression)),
        seq!(python_literal("**"), opt(deferred(WS)), deferred(expression))
    )).into()
}

fn kwarg_or_starred() -> Combinator {
    tag("kwarg_or_starred", choice!(
        seq!(deferred(NAME), opt(deferred(WS)), python_literal("="), opt(deferred(WS)), deferred(expression)),
        seq!(python_literal("*"), opt(deferred(WS)), deferred(expression))
    )).into()
}

fn starred_expression() -> Combinator {
    tag("starred_expression", seq!(python_literal("*"), opt(deferred(WS)), deferred(expression))).into()
}

fn kwargs() -> Combinator {
    tag("kwargs", choice!(
        seq!(deferred(kwarg_or_starred), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(kwarg_or_starred), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(kwarg_or_starred)))))), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(kwarg_or_double_starred), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(kwarg_or_double_starred), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(kwarg_or_double_starred))))))))),
        seq!(deferred(kwarg_or_double_starred), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(kwarg_or_double_starred), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(kwarg_or_double_starred)))))))
    )).into()
}

fn args() -> Combinator {
    tag("args", choice!(
        seq!(choice!(deferred(starred_expression), seq!(deferred(NAME), opt(deferred(WS)), python_literal(":="), opt(deferred(WS)), deferred(expression)), seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))), deferred(lambdef)), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), choice!(deferred(starred_expression), seq!(deferred(NAME), opt(deferred(WS)), python_literal(":="), opt(deferred(WS)), deferred(expression)), seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))), deferred(lambdef)), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), choice!(deferred(starred_expression), seq!(deferred(NAME), opt(deferred(WS)), python_literal(":="), opt(deferred(WS)), deferred(expression)), seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))), deferred(lambdef))))))), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(kwargs)))),
        deferred(kwargs)
    )).into()
}

fn arguments() -> Combinator {
    cached(tag("arguments", seq!(deferred(args), opt(seq!(opt(deferred(WS)), python_literal(",")))))).into()
}

fn dictcomp() -> Combinator {
    tag("dictcomp", seq!(
        python_literal("{"),
         opt(deferred(WS)),
         deferred(kvpair),
         opt(deferred(WS)),
         deferred(for_if_clauses),
         opt(deferred(WS)),
         python_literal("}")
    )).into()
}

fn genexp() -> Combinator {
    tag("genexp", seq!(
        python_literal("("),
         opt(deferred(WS)),
         choice!(deferred(assignment_expression), deferred(expression)),
         opt(deferred(WS)),
         deferred(for_if_clauses),
         opt(deferred(WS)),
         python_literal(")")
    )).into()
}

fn setcomp() -> Combinator {
    tag("setcomp", seq!(
        python_literal("{"),
         opt(deferred(WS)),
         deferred(named_expression),
         opt(deferred(WS)),
         deferred(for_if_clauses),
         opt(deferred(WS)),
         python_literal("}")
    )).into()
}

fn listcomp() -> Combinator {
    tag("listcomp", seq!(
        python_literal("["),
         opt(deferred(WS)),
         deferred(named_expression),
         opt(deferred(WS)),
         deferred(for_if_clauses),
         opt(deferred(WS)),
         python_literal("]")
    )).into()
}

fn for_if_clause() -> Combinator {
    tag("for_if_clause", choice!(
        seq!(python_literal("async"), opt(deferred(WS)), python_literal("for"), opt(deferred(WS)), deferred(star_targets), opt(deferred(WS)), python_literal("in"), opt(deferred(WS)), deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(repeat1(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction))))))),
        seq!(python_literal("for"), opt(deferred(WS)), deferred(star_targets), opt(deferred(WS)), python_literal("in"), opt(deferred(WS)), deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(repeat1(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction)))))))
    )).into()
}

fn for_if_clauses() -> Combinator {
    tag("for_if_clauses", seq!(deferred(for_if_clause), opt(repeat1(seq!(opt(deferred(WS)), deferred(for_if_clause)))))).into()
}

fn kvpair() -> Combinator {
    tag("kvpair", seq!(
        choice!(seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))), deferred(lambdef)),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(expression)
    )).into()
}

fn double_starred_kvpair() -> Combinator {
    tag("double_starred_kvpair", choice!(
        seq!(python_literal("**"), opt(deferred(WS)), deferred(bitwise_or)),
        deferred(kvpair)
    )).into()
}

fn double_starred_kvpairs() -> Combinator {
    tag("double_starred_kvpairs", seq!(deferred(double_starred_kvpair), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(double_starred_kvpair), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(double_starred_kvpair)))))), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn dict() -> Combinator {
    tag("dict", seq!(python_literal("{"), opt(seq!(opt(deferred(WS)), deferred(double_starred_kvpairs))), opt(deferred(WS)), python_literal("}"))).into()
}

fn set() -> Combinator {
    tag("set", seq!(
        python_literal("{"),
         opt(deferred(WS)),
         deferred(star_named_expressions),
         opt(deferred(WS)),
         python_literal("}")
    )).into()
}

fn tuple() -> Combinator {
    tag("tuple", seq!(python_literal("("), opt(seq!(opt(deferred(WS)), deferred(star_named_expression), opt(deferred(WS)), python_literal(","), opt(seq!(opt(deferred(WS)), deferred(star_named_expressions))))), opt(deferred(WS)), python_literal(")"))).into()
}

fn list() -> Combinator {
    tag("list", seq!(python_literal("["), opt(seq!(opt(deferred(WS)), deferred(star_named_expressions))), opt(deferred(WS)), python_literal("]"))).into()
}

fn strings() -> Combinator {
    cached(tag("strings", seq!(choice!(seq!(deferred(FSTRING_START), opt(seq!(opt(deferred(WS)), deferred(fstring_middle), opt(repeat1(seq!(opt(deferred(WS)), deferred(fstring_middle)))))), opt(deferred(WS)), deferred(FSTRING_END)), deferred(STRING)), opt(repeat1(seq!(opt(deferred(WS)), choice!(seq!(deferred(FSTRING_START), opt(seq!(opt(deferred(WS)), deferred(fstring_middle), opt(repeat1(seq!(opt(deferred(WS)), deferred(fstring_middle)))))), opt(deferred(WS)), deferred(FSTRING_END)), deferred(STRING)))))))).into()
}

fn string() -> Combinator {
    tag("string", deferred(STRING)).into()
}

fn fstring() -> Combinator {
    tag("fstring", seq!(deferred(FSTRING_START), opt(seq!(opt(deferred(WS)), deferred(fstring_middle), opt(repeat1(seq!(opt(deferred(WS)), deferred(fstring_middle)))))), opt(deferred(WS)), deferred(FSTRING_END))).into()
}

fn fstring_format_spec() -> Combinator {
    tag("fstring_format_spec", choice!(
        deferred(FSTRING_MIDDLE),
        seq!(python_literal("{"), opt(deferred(WS)), deferred(annotated_rhs), opt(seq!(opt(deferred(WS)), python_literal("="))), opt(seq!(opt(deferred(WS)), deferred(fstring_conversion))), opt(seq!(opt(deferred(WS)), deferred(fstring_full_format_spec))), opt(deferred(WS)), python_literal("}"))
    )).into()
}

fn fstring_full_format_spec() -> Combinator {
    tag("fstring_full_format_spec", seq!(python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(fstring_format_spec), opt(repeat1(seq!(opt(deferred(WS)), deferred(fstring_format_spec)))))))).into()
}

fn fstring_conversion() -> Combinator {
    tag("fstring_conversion", seq!(python_literal("!"), opt(deferred(WS)), deferred(NAME))).into()
}

fn fstring_replacement_field() -> Combinator {
    tag("fstring_replacement_field", seq!(
        python_literal("{"),
         opt(deferred(WS)),
         deferred(annotated_rhs),
         opt(seq!(opt(deferred(WS)), python_literal("="))),
         opt(seq!(opt(deferred(WS)), deferred(fstring_conversion))),
         opt(seq!(opt(deferred(WS)), deferred(fstring_full_format_spec))),
         opt(deferred(WS)),
         python_literal("}")
    )).into()
}

fn fstring_middle() -> Combinator {
    tag("fstring_middle", choice!(
        deferred(fstring_replacement_field),
        deferred(FSTRING_MIDDLE)
    )).into()
}

fn lambda_param() -> Combinator {
    tag("lambda_param", deferred(NAME)).into()
}

fn lambda_param_maybe_default() -> Combinator {
    tag("lambda_param_maybe_default", seq!(deferred(lambda_param), opt(seq!(opt(deferred(WS)), deferred(default))), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn lambda_param_with_default() -> Combinator {
    tag("lambda_param_with_default", seq!(deferred(lambda_param), opt(deferred(WS)), deferred(default), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn lambda_param_no_default() -> Combinator {
    tag("lambda_param_no_default", seq!(deferred(lambda_param), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn lambda_kwds() -> Combinator {
    tag("lambda_kwds", seq!(python_literal("**"), opt(deferred(WS)), deferred(lambda_param_no_default))).into()
}

fn lambda_star_etc() -> Combinator {
    tag("lambda_star_etc", choice!(
        seq!(python_literal("*"), opt(deferred(WS)), choice!(seq!(deferred(lambda_param_no_default), opt(seq!(opt(deferred(WS)), deferred(lambda_param_maybe_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_maybe_default)))))), opt(seq!(opt(deferred(WS)), deferred(lambda_kwds)))), seq!(python_literal(","), opt(deferred(WS)), deferred(lambda_param_maybe_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_maybe_default)))), opt(seq!(opt(deferred(WS)), deferred(lambda_kwds)))))),
        deferred(lambda_kwds)
    )).into()
}

fn lambda_slash_with_default() -> Combinator {
    tag("lambda_slash_with_default", seq!(
        opt(seq!(deferred(lambda_param_no_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_no_default)))), opt(deferred(WS)))),
         deferred(lambda_param_with_default),
         opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_with_default)))),
         opt(deferred(WS)),
         python_literal("/"),
         opt(seq!(opt(deferred(WS)), python_literal(",")))
    )).into()
}

fn lambda_slash_no_default() -> Combinator {
    tag("lambda_slash_no_default", seq!(
        deferred(lambda_param_no_default),
         opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_no_default)))),
         opt(deferred(WS)),
         python_literal("/"),
         opt(seq!(opt(deferred(WS)), python_literal(",")))
    )).into()
}

fn lambda_parameters() -> Combinator {
    tag("lambda_parameters", choice!(
        seq!(deferred(lambda_slash_no_default), opt(seq!(opt(deferred(WS)), deferred(lambda_param_no_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_no_default)))))), opt(seq!(opt(deferred(WS)), deferred(lambda_param_with_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_with_default)))))), opt(seq!(opt(deferred(WS)), deferred(lambda_star_etc)))),
        seq!(deferred(lambda_slash_with_default), opt(seq!(opt(deferred(WS)), deferred(lambda_param_with_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_with_default)))))), opt(seq!(opt(deferred(WS)), deferred(lambda_star_etc)))),
        seq!(deferred(lambda_param_no_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_no_default)))), opt(seq!(opt(deferred(WS)), deferred(lambda_param_with_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_with_default)))))), opt(seq!(opt(deferred(WS)), deferred(lambda_star_etc)))),
        seq!(deferred(lambda_param_with_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(lambda_param_with_default)))), opt(seq!(opt(deferred(WS)), deferred(lambda_star_etc)))),
        deferred(lambda_star_etc)
    )).into()
}

fn lambda_params() -> Combinator {
    tag("lambda_params", deferred(lambda_parameters)).into()
}

fn lambdef() -> Combinator {
    tag("lambdef", seq!(
        python_literal("lambda"),
         opt(seq!(opt(deferred(WS)), deferred(lambda_params))),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(expression)
    )).into()
}

fn group() -> Combinator {
    tag("group", seq!(
        python_literal("("),
         opt(deferred(WS)),
         choice!(deferred(yield_expr), deferred(named_expression)),
         opt(deferred(WS)),
         python_literal(")")
    )).into()
}

fn atom() -> Combinator {
    tag("atom", choice!(
        deferred(NAME),
        python_literal("True"),
        python_literal("False"),
        python_literal("None"),
        deferred(strings),
        deferred(NUMBER),
        deferred(tuple),
        deferred(group),
        deferred(genexp),
        deferred(list),
        deferred(listcomp),
        deferred(dict),
        deferred(set),
        deferred(dictcomp),
        deferred(setcomp),
        python_literal("...")
    )).into()
}

fn slice() -> Combinator {
    tag("slice", choice!(
        seq!(opt(choice!(seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression))), opt(deferred(WS))), seq!(deferred(lambdef), opt(deferred(WS))))), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(expression))), opt(seq!(opt(deferred(WS)), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(expression)))))),
        seq!(deferred(NAME), opt(deferred(WS)), python_literal(":="), opt(deferred(WS)), deferred(expression)),
        seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))),
        deferred(lambdef)
    )).into()
}

fn slices() -> Combinator {
    tag("slices", choice!(
        deferred(slice),
        seq!(choice!(deferred(slice), deferred(starred_expression)), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), choice!(deferred(slice), deferred(starred_expression)), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), choice!(deferred(slice), deferred(starred_expression))))))), opt(seq!(opt(deferred(WS)), python_literal(","))))
    )).into()
}

fn primary() -> Combinator {
    tag("primary", seq!(deferred(atom), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), deferred(NAME)), deferred(genexp), seq!(python_literal("("), opt(seq!(opt(deferred(WS)), deferred(arguments))), opt(deferred(WS)), python_literal(")")), seq!(python_literal("["), opt(deferred(WS)), deferred(slices), opt(deferred(WS)), python_literal("]"))), opt(repeat1(seq!(opt(deferred(WS)), choice!(seq!(python_literal("."), opt(deferred(WS)), deferred(NAME)), deferred(genexp), seq!(python_literal("("), opt(seq!(opt(deferred(WS)), deferred(arguments))), opt(deferred(WS)), python_literal(")")), seq!(python_literal("["), opt(deferred(WS)), deferred(slices), opt(deferred(WS)), python_literal("]")))))))))).into()
}

fn await_primary() -> Combinator {
    cached(tag("await_primary", choice!(
        seq!(python_literal("await"), opt(deferred(WS)), deferred(primary)),
        deferred(primary)
    ))).into()
}

fn power() -> Combinator {
    tag("power", seq!(deferred(await_primary), opt(seq!(opt(deferred(WS)), python_literal("**"), opt(deferred(WS)), deferred(factor))))).into()
}

fn factor() -> Combinator {
    cached(tag("factor", choice!(
        seq!(python_literal("+"), opt(deferred(WS)), deferred(factor)),
        seq!(python_literal("-"), opt(deferred(WS)), deferred(factor)),
        seq!(python_literal("~"), opt(deferred(WS)), deferred(factor)),
        deferred(power)
    ))).into()
}

fn term() -> Combinator {
    tag("term", seq!(deferred(factor), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal("*"), opt(deferred(WS)), deferred(factor)), seq!(python_literal("/"), opt(deferred(WS)), deferred(factor)), seq!(python_literal("//"), opt(deferred(WS)), deferred(factor)), seq!(python_literal("%"), opt(deferred(WS)), deferred(factor)), seq!(python_literal("@"), opt(deferred(WS)), deferred(factor))), opt(repeat1(seq!(opt(deferred(WS)), choice!(seq!(python_literal("*"), opt(deferred(WS)), deferred(factor)), seq!(python_literal("/"), opt(deferred(WS)), deferred(factor)), seq!(python_literal("//"), opt(deferred(WS)), deferred(factor)), seq!(python_literal("%"), opt(deferred(WS)), deferred(factor)), seq!(python_literal("@"), opt(deferred(WS)), deferred(factor)))))))))).into()
}

fn sum() -> Combinator {
    tag("sum", seq!(deferred(term), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal("+"), opt(deferred(WS)), deferred(term)), seq!(python_literal("-"), opt(deferred(WS)), deferred(term))), opt(repeat1(seq!(opt(deferred(WS)), choice!(seq!(python_literal("+"), opt(deferred(WS)), deferred(term)), seq!(python_literal("-"), opt(deferred(WS)), deferred(term)))))))))).into()
}

fn shift_expr() -> Combinator {
    tag("shift_expr", seq!(deferred(sum), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal("<<"), opt(deferred(WS)), deferred(sum)), seq!(python_literal(">>"), opt(deferred(WS)), deferred(sum))), opt(repeat1(seq!(opt(deferred(WS)), choice!(seq!(python_literal("<<"), opt(deferred(WS)), deferred(sum)), seq!(python_literal(">>"), opt(deferred(WS)), deferred(sum)))))))))).into()
}

fn bitwise_and() -> Combinator {
    tag("bitwise_and", seq!(deferred(shift_expr), opt(seq!(opt(deferred(WS)), python_literal("&"), opt(deferred(WS)), deferred(shift_expr), opt(repeat1(seq!(opt(deferred(WS)), python_literal("&"), opt(deferred(WS)), deferred(shift_expr)))))))).into()
}

fn bitwise_xor() -> Combinator {
    tag("bitwise_xor", seq!(deferred(bitwise_and), opt(seq!(opt(deferred(WS)), python_literal("^"), opt(deferred(WS)), deferred(bitwise_and), opt(repeat1(seq!(opt(deferred(WS)), python_literal("^"), opt(deferred(WS)), deferred(bitwise_and)))))))).into()
}

fn bitwise_or() -> Combinator {
    tag("bitwise_or", seq!(deferred(bitwise_xor), opt(seq!(opt(deferred(WS)), python_literal("|"), opt(deferred(WS)), deferred(bitwise_xor), opt(repeat1(seq!(opt(deferred(WS)), python_literal("|"), opt(deferred(WS)), deferred(bitwise_xor)))))))).into()
}

fn is_bitwise_or() -> Combinator {
    tag("is_bitwise_or", seq!(python_literal("is"), opt(deferred(WS)), deferred(bitwise_or))).into()
}

fn isnot_bitwise_or() -> Combinator {
    tag("isnot_bitwise_or", seq!(
        python_literal("is"),
         opt(deferred(WS)),
         python_literal("not"),
         opt(deferred(WS)),
         deferred(bitwise_or)
    )).into()
}

fn in_bitwise_or() -> Combinator {
    tag("in_bitwise_or", seq!(python_literal("in"), opt(deferred(WS)), deferred(bitwise_or))).into()
}

fn notin_bitwise_or() -> Combinator {
    tag("notin_bitwise_or", seq!(
        python_literal("not"),
         opt(deferred(WS)),
         python_literal("in"),
         opt(deferred(WS)),
         deferred(bitwise_or)
    )).into()
}

fn gt_bitwise_or() -> Combinator {
    tag("gt_bitwise_or", seq!(python_literal(">"), opt(deferred(WS)), deferred(bitwise_or))).into()
}

fn gte_bitwise_or() -> Combinator {
    tag("gte_bitwise_or", seq!(python_literal(">="), opt(deferred(WS)), deferred(bitwise_or))).into()
}

fn lt_bitwise_or() -> Combinator {
    tag("lt_bitwise_or", seq!(python_literal("<"), opt(deferred(WS)), deferred(bitwise_or))).into()
}

fn lte_bitwise_or() -> Combinator {
    tag("lte_bitwise_or", seq!(python_literal("<="), opt(deferred(WS)), deferred(bitwise_or))).into()
}

fn noteq_bitwise_or() -> Combinator {
    tag("noteq_bitwise_or", seq!(python_literal("!="), opt(deferred(WS)), deferred(bitwise_or))).into()
}

fn eq_bitwise_or() -> Combinator {
    tag("eq_bitwise_or", seq!(python_literal("=="), opt(deferred(WS)), deferred(bitwise_or))).into()
}

fn compare_op_bitwise_or_pair() -> Combinator {
    tag("compare_op_bitwise_or_pair", choice!(
        deferred(eq_bitwise_or),
        deferred(noteq_bitwise_or),
        deferred(lte_bitwise_or),
        deferred(lt_bitwise_or),
        deferred(gte_bitwise_or),
        deferred(gt_bitwise_or),
        deferred(notin_bitwise_or),
        deferred(in_bitwise_or),
        deferred(isnot_bitwise_or),
        deferred(is_bitwise_or)
    )).into()
}

fn comparison() -> Combinator {
    tag("comparison", seq!(deferred(bitwise_or), opt(seq!(opt(deferred(WS)), deferred(compare_op_bitwise_or_pair), opt(repeat1(seq!(opt(deferred(WS)), deferred(compare_op_bitwise_or_pair)))))))).into()
}

fn inversion() -> Combinator {
    cached(tag("inversion", choice!(
        seq!(python_literal("not"), opt(deferred(WS)), deferred(inversion)),
        deferred(comparison)
    ))).into()
}

fn conjunction() -> Combinator {
    cached(tag("conjunction", seq!(deferred(inversion), opt(seq!(opt(deferred(WS)), python_literal("and"), opt(deferred(WS)), deferred(inversion), opt(repeat1(seq!(opt(deferred(WS)), python_literal("and"), opt(deferred(WS)), deferred(inversion))))))))).into()
}

fn disjunction() -> Combinator {
    cached(tag("disjunction", seq!(deferred(conjunction), opt(seq!(opt(deferred(WS)), python_literal("or"), opt(deferred(WS)), deferred(conjunction), opt(repeat1(seq!(opt(deferred(WS)), python_literal("or"), opt(deferred(WS)), deferred(conjunction))))))))).into()
}

fn named_expression() -> Combinator {
    tag("named_expression", choice!(
        seq!(deferred(NAME), opt(deferred(WS)), python_literal(":="), opt(deferred(WS)), deferred(expression)),
        seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))),
        deferred(lambdef)
    )).into()
}

fn assignment_expression() -> Combinator {
    tag("assignment_expression", seq!(
        deferred(NAME),
         opt(deferred(WS)),
         python_literal(":="),
         opt(deferred(WS)),
         deferred(expression)
    )).into()
}

fn star_named_expression() -> Combinator {
    tag("star_named_expression", choice!(
        seq!(python_literal("*"), opt(deferred(WS)), deferred(bitwise_or)),
        deferred(named_expression)
    )).into()
}

fn star_named_expressions() -> Combinator {
    tag("star_named_expressions", seq!(deferred(star_named_expression), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(star_named_expression), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(star_named_expression)))))), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn star_expression() -> Combinator {
    cached(tag("star_expression", choice!(
        seq!(python_literal("*"), opt(deferred(WS)), deferred(bitwise_or)),
        seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))),
        deferred(lambdef)
    ))).into()
}

fn star_expressions() -> Combinator {
    tag("star_expressions", seq!(deferred(star_expression), opt(seq!(opt(deferred(WS)), python_literal(","), opt(seq!(opt(deferred(WS)), deferred(star_expression), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(star_expression)))), opt(seq!(opt(deferred(WS)), python_literal(","))))))))).into()
}

fn yield_expr() -> Combinator {
    tag("yield_expr", seq!(python_literal("yield"), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal("from"), opt(deferred(WS)), deferred(expression)), deferred(star_expressions)))))).into()
}

fn expression() -> Combinator {
    cached(tag("expression", choice!(
        seq!(deferred(disjunction), opt(seq!(opt(deferred(WS)), python_literal("if"), opt(deferred(WS)), deferred(disjunction), opt(deferred(WS)), python_literal("else"), opt(deferred(WS)), deferred(expression)))),
        deferred(lambdef)
    ))).into()
}

fn expressions() -> Combinator {
    tag("expressions", seq!(deferred(expression), opt(seq!(opt(deferred(WS)), python_literal(","), opt(seq!(opt(deferred(WS)), deferred(expression), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(expression)))), opt(seq!(opt(deferred(WS)), python_literal(","))))))))).into()
}

fn type_param_starred_default() -> Combinator {
    tag("type_param_starred_default", seq!(python_literal("="), opt(deferred(WS)), deferred(star_expression))).into()
}

fn type_param_default() -> Combinator {
    tag("type_param_default", seq!(python_literal("="), opt(deferred(WS)), deferred(expression))).into()
}

fn type_param_bound() -> Combinator {
    tag("type_param_bound", seq!(python_literal(":"), opt(deferred(WS)), deferred(expression))).into()
}

fn type_param() -> Combinator {
    cached(tag("type_param", choice!(
        seq!(deferred(NAME), opt(seq!(opt(deferred(WS)), deferred(type_param_bound))), opt(seq!(opt(deferred(WS)), deferred(type_param_default)))),
        seq!(python_literal("*"), opt(deferred(WS)), deferred(NAME), opt(seq!(opt(deferred(WS)), deferred(type_param_starred_default)))),
        seq!(python_literal("**"), opt(deferred(WS)), deferred(NAME), opt(seq!(opt(deferred(WS)), deferred(type_param_default))))
    ))).into()
}

fn type_param_seq() -> Combinator {
    tag("type_param_seq", seq!(deferred(type_param), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(type_param), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(type_param)))))), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn type_params() -> Combinator {
    tag("type_params", seq!(
        python_literal("["),
         opt(deferred(WS)),
         deferred(type_param_seq),
         opt(deferred(WS)),
         python_literal("]")
    )).into()
}

fn type_alias() -> Combinator {
    tag("type_alias", seq!(
        python_literal("type"),
         opt(deferred(WS)),
         deferred(NAME),
         opt(seq!(opt(deferred(WS)), deferred(type_params))),
         opt(deferred(WS)),
         python_literal("="),
         opt(deferred(WS)),
         deferred(expression)
    )).into()
}

fn keyword_pattern() -> Combinator {
    tag("keyword_pattern", seq!(
        deferred(NAME),
         opt(deferred(WS)),
         python_literal("="),
         opt(deferred(WS)),
         deferred(pattern)
    )).into()
}

fn keyword_patterns() -> Combinator {
    tag("keyword_patterns", seq!(deferred(keyword_pattern), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(keyword_pattern), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(keyword_pattern)))))))).into()
}

fn positional_patterns() -> Combinator {
    tag("positional_patterns", seq!(choice!(deferred(as_pattern), deferred(or_pattern)), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(pattern), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(pattern)))))))).into()
}

fn class_pattern() -> Combinator {
    tag("class_pattern", seq!(
        deferred(NAME),
         opt(seq!(opt(deferred(WS)), python_literal("."), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(NAME), opt(repeat1(seq!(opt(deferred(WS)), python_literal("."), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(NAME)))))),
         opt(deferred(WS)),
         python_literal("("),
         opt(deferred(WS)),
         choice!(python_literal(")"), seq!(deferred(positional_patterns), opt(deferred(WS)), choice!(seq!(opt(seq!(python_literal(","), opt(deferred(WS)))), python_literal(")")), seq!(python_literal(","), opt(deferred(WS)), deferred(keyword_patterns), opt(seq!(opt(deferred(WS)), python_literal(","))), opt(deferred(WS)), python_literal(")")))), seq!(deferred(keyword_patterns), opt(seq!(opt(deferred(WS)), python_literal(","))), opt(deferred(WS)), python_literal(")")))
    )).into()
}

fn double_star_pattern() -> Combinator {
    tag("double_star_pattern", seq!(python_literal("**"), opt(deferred(WS)), deferred(pattern_capture_target))).into()
}

fn key_value_pattern() -> Combinator {
    tag("key_value_pattern", seq!(
        choice!(deferred(signed_number), deferred(complex_number), deferred(strings), python_literal("None"), python_literal("True"), python_literal("False"), seq!(deferred(name_or_attr), opt(deferred(WS)), python_literal("."), opt(deferred(WS)), deferred(NAME))),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(pattern)
    )).into()
}

fn items_pattern() -> Combinator {
    tag("items_pattern", seq!(deferred(key_value_pattern), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(key_value_pattern), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(key_value_pattern)))))))).into()
}

fn mapping_pattern() -> Combinator {
    tag("mapping_pattern", seq!(python_literal("{"), opt(deferred(WS)), choice!(python_literal("}"), seq!(deferred(double_star_pattern), opt(seq!(opt(deferred(WS)), python_literal(","))), opt(deferred(WS)), python_literal("}")), seq!(deferred(items_pattern), opt(deferred(WS)), choice!(seq!(python_literal(","), opt(deferred(WS)), deferred(double_star_pattern), opt(seq!(opt(deferred(WS)), python_literal(","))), opt(deferred(WS)), python_literal("}")), seq!(opt(seq!(python_literal(","), opt(deferred(WS)))), python_literal("}"))))))).into()
}

fn star_pattern() -> Combinator {
    cached(tag("star_pattern", seq!(python_literal("*"), opt(deferred(WS)), choice!(deferred(pattern_capture_target), deferred(wildcard_pattern))))).into()
}

fn maybe_star_pattern() -> Combinator {
    tag("maybe_star_pattern", choice!(
        deferred(star_pattern),
        deferred(as_pattern),
        deferred(or_pattern)
    )).into()
}

fn maybe_sequence_pattern() -> Combinator {
    tag("maybe_sequence_pattern", seq!(deferred(maybe_star_pattern), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(maybe_star_pattern), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(maybe_star_pattern)))))), opt(seq!(opt(deferred(WS)), python_literal(","))))).into()
}

fn open_sequence_pattern() -> Combinator {
    tag("open_sequence_pattern", seq!(deferred(maybe_star_pattern), opt(deferred(WS)), python_literal(","), opt(seq!(opt(deferred(WS)), deferred(maybe_sequence_pattern))))).into()
}

fn sequence_pattern() -> Combinator {
    tag("sequence_pattern", choice!(
        seq!(python_literal("["), opt(seq!(opt(deferred(WS)), deferred(maybe_sequence_pattern))), opt(deferred(WS)), python_literal("]")),
        seq!(python_literal("("), opt(seq!(opt(deferred(WS)), deferred(open_sequence_pattern))), opt(deferred(WS)), python_literal(")"))
    )).into()
}

fn group_pattern() -> Combinator {
    tag("group_pattern", seq!(
        python_literal("("),
         opt(deferred(WS)),
         deferred(pattern),
         opt(deferred(WS)),
         python_literal(")")
    )).into()
}

fn name_or_attr() -> Combinator {
    tag("name_or_attr", seq!(deferred(NAME), opt(seq!(opt(deferred(WS)), python_literal("."), opt(deferred(WS)), deferred(NAME), opt(repeat1(seq!(opt(deferred(WS)), python_literal("."), opt(deferred(WS)), deferred(NAME)))))))).into()
}

fn attr() -> Combinator {
    tag("attr", seq!(
        deferred(name_or_attr),
         opt(deferred(WS)),
         python_literal("."),
         opt(deferred(WS)),
         deferred(NAME)
    )).into()
}

fn value_pattern() -> Combinator {
    tag("value_pattern", deferred(attr)).into()
}

fn wildcard_pattern() -> Combinator {
    tag("wildcard_pattern", python_literal("_")).into()
}

fn pattern_capture_target() -> Combinator {
    tag("pattern_capture_target", deferred(NAME)).into()
}

fn capture_pattern() -> Combinator {
    tag("capture_pattern", deferred(pattern_capture_target)).into()
}

fn imaginary_number() -> Combinator {
    tag("imaginary_number", deferred(NUMBER)).into()
}

fn real_number() -> Combinator {
    tag("real_number", deferred(NUMBER)).into()
}

fn signed_real_number() -> Combinator {
    tag("signed_real_number", choice!(
        deferred(real_number),
        seq!(python_literal("-"), opt(deferred(WS)), deferred(real_number))
    )).into()
}

fn signed_number() -> Combinator {
    tag("signed_number", choice!(
        deferred(NUMBER),
        seq!(python_literal("-"), opt(deferred(WS)), deferred(NUMBER))
    )).into()
}

fn complex_number() -> Combinator {
    tag("complex_number", seq!(deferred(signed_real_number), opt(deferred(WS)), choice!(seq!(python_literal("+"), opt(deferred(WS)), deferred(imaginary_number)), seq!(python_literal("-"), opt(deferred(WS)), deferred(imaginary_number))))).into()
}

fn literal_expr() -> Combinator {
    tag("literal_expr", choice!(
        deferred(signed_number),
        deferred(complex_number),
        deferred(strings),
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).into()
}

fn literal_pattern() -> Combinator {
    tag("literal_pattern", choice!(
        deferred(signed_number),
        deferred(complex_number),
        deferred(strings),
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).into()
}

fn closed_pattern() -> Combinator {
    cached(tag("closed_pattern", choice!(
        deferred(literal_pattern),
        deferred(capture_pattern),
        deferred(wildcard_pattern),
        deferred(value_pattern),
        deferred(group_pattern),
        deferred(sequence_pattern),
        deferred(mapping_pattern),
        deferred(class_pattern)
    ))).into()
}

fn or_pattern() -> Combinator {
    tag("or_pattern", seq!(deferred(closed_pattern), opt(seq!(opt(deferred(WS)), python_literal("|"), opt(deferred(WS)), deferred(closed_pattern), opt(repeat1(seq!(opt(deferred(WS)), python_literal("|"), opt(deferred(WS)), deferred(closed_pattern)))))))).into()
}

fn as_pattern() -> Combinator {
    tag("as_pattern", seq!(
        deferred(or_pattern),
         opt(deferred(WS)),
         python_literal("as"),
         opt(deferred(WS)),
         deferred(pattern_capture_target)
    )).into()
}

fn pattern() -> Combinator {
    tag("pattern", choice!(
        deferred(as_pattern),
        deferred(or_pattern)
    )).into()
}

fn patterns() -> Combinator {
    tag("patterns", choice!(
        deferred(open_sequence_pattern),
        deferred(pattern)
    )).into()
}

fn guard() -> Combinator {
    tag("guard", seq!(python_literal("if"), opt(deferred(WS)), deferred(named_expression))).into()
}

fn case_block() -> Combinator {
    tag("case_block", seq!(
        python_literal("case"),
         opt(deferred(WS)),
         deferred(patterns),
         opt(seq!(opt(deferred(WS)), deferred(guard))),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block)
    )).into()
}

fn subject_expr() -> Combinator {
    tag("subject_expr", choice!(
        seq!(deferred(star_named_expression), opt(deferred(WS)), python_literal(","), opt(seq!(opt(deferred(WS)), deferred(star_named_expressions)))),
        deferred(named_expression)
    )).into()
}

fn match_stmt() -> Combinator {
    tag("match_stmt", seq!(
        python_literal("match"),
         opt(deferred(WS)),
         deferred(subject_expr),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(NEWLINE),
         opt(deferred(WS)),
         deferred(INDENT),
         opt(deferred(WS)),
         deferred(case_block),
         opt(repeat1(seq!(opt(deferred(WS)), deferred(case_block)))),
         opt(deferred(WS)),
         deferred(DEDENT)
    )).into()
}

fn finally_block() -> Combinator {
    tag("finally_block", seq!(
        python_literal("finally"),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block)
    )).into()
}

fn except_star_block() -> Combinator {
    tag("except_star_block", seq!(
        python_literal("except"),
         opt(deferred(WS)),
         python_literal("*"),
         opt(deferred(WS)),
         deferred(expression),
         opt(seq!(opt(deferred(WS)), python_literal("as"), opt(deferred(WS)), deferred(NAME))),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block)
    )).into()
}

fn except_block() -> Combinator {
    tag("except_block", seq!(python_literal("except"), opt(deferred(WS)), choice!(seq!(deferred(expression), opt(seq!(opt(deferred(WS)), python_literal("as"), opt(deferred(WS)), deferred(NAME))), opt(deferred(WS)), python_literal(":"), opt(deferred(WS)), deferred(block)), seq!(python_literal(":"), opt(deferred(WS)), deferred(block))))).into()
}

fn try_stmt() -> Combinator {
    tag("try_stmt", seq!(
        python_literal("try"),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block),
         opt(deferred(WS)),
         choice!(deferred(finally_block), seq!(deferred(except_block), opt(repeat1(seq!(opt(deferred(WS)), deferred(except_block)))), opt(seq!(opt(deferred(WS)), deferred(else_block))), opt(seq!(opt(deferred(WS)), deferred(finally_block)))), seq!(deferred(except_star_block), opt(repeat1(seq!(opt(deferred(WS)), deferred(except_star_block)))), opt(seq!(opt(deferred(WS)), deferred(else_block))), opt(seq!(opt(deferred(WS)), deferred(finally_block)))))
    )).into()
}

fn with_item() -> Combinator {
    tag("with_item", seq!(deferred(expression), opt(seq!(opt(deferred(WS)), python_literal("as"), opt(deferred(WS)), deferred(star_target))))).into()
}

fn with_stmt() -> Combinator {
    tag("with_stmt", choice!(
        seq!(python_literal("with"), opt(deferred(WS)), choice!(seq!(python_literal("("), opt(deferred(WS)), deferred(with_item), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(with_item), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(with_item)))))), opt(seq!(opt(deferred(WS)), python_literal(","))), opt(deferred(WS)), python_literal(")"), opt(deferred(WS)), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT))), opt(deferred(WS)), deferred(block)), seq!(deferred(with_item), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(with_item), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(with_item)))))), opt(deferred(WS)), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT))), opt(deferred(WS)), deferred(block)))),
        seq!(python_literal("async"), opt(deferred(WS)), python_literal("with"), opt(deferred(WS)), choice!(seq!(python_literal("("), opt(deferred(WS)), deferred(with_item), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(with_item), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(with_item)))))), opt(seq!(opt(deferred(WS)), python_literal(","))), opt(deferred(WS)), python_literal(")"), opt(deferred(WS)), python_literal(":"), opt(deferred(WS)), deferred(block)), seq!(deferred(with_item), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(with_item), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(with_item)))))), opt(deferred(WS)), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT))), opt(deferred(WS)), deferred(block))))
    )).into()
}

fn for_stmt() -> Combinator {
    tag("for_stmt", choice!(
        seq!(python_literal("for"), opt(deferred(WS)), deferred(star_targets), opt(deferred(WS)), python_literal("in"), opt(deferred(WS)), deferred(star_expressions), opt(deferred(WS)), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT))), opt(deferred(WS)), deferred(block), opt(seq!(opt(deferred(WS)), deferred(else_block)))),
        seq!(python_literal("async"), opt(deferred(WS)), python_literal("for"), opt(deferred(WS)), deferred(star_targets), opt(deferred(WS)), python_literal("in"), opt(deferred(WS)), deferred(star_expressions), opt(deferred(WS)), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT))), opt(deferred(WS)), deferred(block), opt(seq!(opt(deferred(WS)), deferred(else_block))))
    )).into()
}

fn while_stmt() -> Combinator {
    tag("while_stmt", seq!(
        python_literal("while"),
         opt(deferred(WS)),
         deferred(named_expression),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block),
         opt(seq!(opt(deferred(WS)), deferred(else_block)))
    )).into()
}

fn else_block() -> Combinator {
    tag("else_block", seq!(
        python_literal("else"),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block)
    )).into()
}

fn elif_stmt() -> Combinator {
    tag("elif_stmt", seq!(
        python_literal("elif"),
         opt(deferred(WS)),
         deferred(named_expression),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block),
         opt(seq!(opt(deferred(WS)), choice!(deferred(elif_stmt), deferred(else_block))))
    )).into()
}

fn if_stmt() -> Combinator {
    tag("if_stmt", seq!(
        python_literal("if"),
         opt(deferred(WS)),
         deferred(named_expression),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block),
         opt(seq!(opt(deferred(WS)), choice!(deferred(elif_stmt), deferred(else_block))))
    )).into()
}

fn default() -> Combinator {
    tag("default", seq!(python_literal("="), opt(deferred(WS)), deferred(expression))).into()
}

fn star_annotation() -> Combinator {
    tag("star_annotation", seq!(python_literal(":"), opt(deferred(WS)), deferred(star_expression))).into()
}

fn annotation() -> Combinator {
    tag("annotation", seq!(python_literal(":"), opt(deferred(WS)), deferred(expression))).into()
}

fn param_star_annotation() -> Combinator {
    tag("param_star_annotation", seq!(deferred(NAME), opt(deferred(WS)), deferred(star_annotation))).into()
}

fn param() -> Combinator {
    tag("param", seq!(deferred(NAME), opt(seq!(opt(deferred(WS)), deferred(annotation))))).into()
}

fn param_maybe_default() -> Combinator {
    tag("param_maybe_default", seq!(deferred(param), opt(seq!(opt(deferred(WS)), deferred(default))), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal(","), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT)))), deferred(TYPE_COMMENT)))))).into()
}

fn param_with_default() -> Combinator {
    tag("param_with_default", seq!(deferred(param), opt(deferred(WS)), deferred(default), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal(","), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT)))), deferred(TYPE_COMMENT)))))).into()
}

fn param_no_default_star_annotation() -> Combinator {
    tag("param_no_default_star_annotation", seq!(deferred(param_star_annotation), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal(","), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT)))), deferred(TYPE_COMMENT)))))).into()
}

fn param_no_default() -> Combinator {
    tag("param_no_default", seq!(deferred(param), opt(seq!(opt(deferred(WS)), choice!(seq!(python_literal(","), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT)))), deferred(TYPE_COMMENT)))))).into()
}

fn kwds() -> Combinator {
    tag("kwds", seq!(python_literal("**"), opt(deferred(WS)), deferred(param_no_default))).into()
}

fn star_etc() -> Combinator {
    tag("star_etc", choice!(
        seq!(python_literal("*"), opt(deferred(WS)), choice!(seq!(deferred(param_no_default), opt(seq!(opt(deferred(WS)), deferred(param_maybe_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_maybe_default)))))), opt(seq!(opt(deferred(WS)), deferred(kwds)))), seq!(deferred(param_no_default_star_annotation), opt(seq!(opt(deferred(WS)), deferred(param_maybe_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_maybe_default)))))), opt(seq!(opt(deferred(WS)), deferred(kwds)))), seq!(python_literal(","), opt(deferred(WS)), deferred(param_maybe_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_maybe_default)))), opt(seq!(opt(deferred(WS)), deferred(kwds)))))),
        deferred(kwds)
    )).into()
}

fn slash_with_default() -> Combinator {
    tag("slash_with_default", seq!(
        opt(seq!(deferred(param_no_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_no_default)))), opt(deferred(WS)))),
         deferred(param_with_default),
         opt(repeat1(seq!(opt(deferred(WS)), deferred(param_with_default)))),
         opt(deferred(WS)),
         python_literal("/"),
         opt(seq!(opt(deferred(WS)), python_literal(",")))
    )).into()
}

fn slash_no_default() -> Combinator {
    tag("slash_no_default", seq!(
        deferred(param_no_default),
         opt(repeat1(seq!(opt(deferred(WS)), deferred(param_no_default)))),
         opt(deferred(WS)),
         python_literal("/"),
         opt(seq!(opt(deferred(WS)), python_literal(",")))
    )).into()
}

fn parameters() -> Combinator {
    tag("parameters", choice!(
        seq!(deferred(slash_no_default), opt(seq!(opt(deferred(WS)), deferred(param_no_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_no_default)))))), opt(seq!(opt(deferred(WS)), deferred(param_with_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_with_default)))))), opt(seq!(opt(deferred(WS)), deferred(star_etc)))),
        seq!(deferred(slash_with_default), opt(seq!(opt(deferred(WS)), deferred(param_with_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_with_default)))))), opt(seq!(opt(deferred(WS)), deferred(star_etc)))),
        seq!(deferred(param_no_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_no_default)))), opt(seq!(opt(deferred(WS)), deferred(param_with_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_with_default)))))), opt(seq!(opt(deferred(WS)), deferred(star_etc)))),
        seq!(deferred(param_with_default), opt(repeat1(seq!(opt(deferred(WS)), deferred(param_with_default)))), opt(seq!(opt(deferred(WS)), deferred(star_etc)))),
        deferred(star_etc)
    )).into()
}

fn params() -> Combinator {
    tag("params", deferred(parameters)).into()
}

fn function_def_raw() -> Combinator {
    tag("function_def_raw", choice!(
        seq!(python_literal("def"), opt(deferred(WS)), deferred(NAME), opt(seq!(opt(deferred(WS)), deferred(type_params))), opt(deferred(WS)), python_literal("("), opt(seq!(opt(deferred(WS)), deferred(params))), opt(deferred(WS)), python_literal(")"), opt(seq!(opt(deferred(WS)), python_literal("->"), opt(deferred(WS)), deferred(expression))), opt(deferred(WS)), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(func_type_comment))), opt(deferred(WS)), deferred(block)),
        seq!(python_literal("async"), opt(deferred(WS)), python_literal("def"), opt(deferred(WS)), deferred(NAME), opt(seq!(opt(deferred(WS)), deferred(type_params))), opt(deferred(WS)), python_literal("("), opt(seq!(opt(deferred(WS)), deferred(params))), opt(deferred(WS)), python_literal(")"), opt(seq!(opt(deferred(WS)), python_literal("->"), opt(deferred(WS)), deferred(expression))), opt(deferred(WS)), python_literal(":"), opt(seq!(opt(deferred(WS)), deferred(func_type_comment))), opt(deferred(WS)), deferred(block))
    )).into()
}

fn function_def() -> Combinator {
    tag("function_def", choice!(
        seq!(python_literal("@"), opt(deferred(WS)), deferred(named_expression), opt(deferred(WS)), deferred(NEWLINE), opt(seq!(opt(deferred(WS)), python_literal("@"), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), opt(seq!(deferred(WS), opt(seq!(opt(deferred(WS)), deferred(WS))), opt(deferred(WS)))), deferred(named_expression), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), opt(seq!(deferred(WS), opt(seq!(opt(deferred(WS)), deferred(WS))), opt(deferred(WS)))), deferred(NEWLINE), opt(repeat1(seq!(opt(deferred(WS)), python_literal("@"), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), opt(seq!(deferred(WS), opt(seq!(opt(deferred(WS)), deferred(WS))), opt(deferred(WS)))), deferred(named_expression), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), opt(seq!(deferred(WS), opt(seq!(opt(deferred(WS)), deferred(WS))), opt(deferred(WS)))), deferred(NEWLINE)))))), opt(deferred(WS)), deferred(function_def_raw)),
        deferred(function_def_raw)
    )).into()
}

fn class_def_raw() -> Combinator {
    tag("class_def_raw", seq!(
        python_literal("class"),
         opt(deferred(WS)),
         deferred(NAME),
         opt(seq!(opt(deferred(WS)), deferred(type_params))),
         opt(seq!(opt(deferred(WS)), python_literal("("), opt(seq!(opt(deferred(WS)), deferred(arguments))), opt(deferred(WS)), python_literal(")"))),
         opt(deferred(WS)),
         python_literal(":"),
         opt(deferred(WS)),
         deferred(block)
    )).into()
}

fn class_def() -> Combinator {
    tag("class_def", choice!(
        seq!(python_literal("@"), opt(deferred(WS)), deferred(named_expression), opt(deferred(WS)), deferred(NEWLINE), opt(seq!(opt(deferred(WS)), python_literal("@"), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(named_expression), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(NEWLINE), opt(repeat1(seq!(opt(deferred(WS)), python_literal("@"), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(named_expression), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(NEWLINE)))))), opt(deferred(WS)), deferred(class_def_raw)),
        deferred(class_def_raw)
    )).into()
}

fn decorators() -> Combinator {
    tag("decorators", seq!(
        python_literal("@"),
         opt(deferred(WS)),
         deferred(named_expression),
         opt(deferred(WS)),
         deferred(NEWLINE),
         opt(seq!(opt(deferred(WS)), python_literal("@"), opt(deferred(WS)), deferred(named_expression), opt(deferred(WS)), deferred(NEWLINE), opt(repeat1(seq!(opt(deferred(WS)), python_literal("@"), opt(deferred(WS)), deferred(named_expression), opt(deferred(WS)), deferred(NEWLINE))))))
    )).into()
}

fn block() -> Combinator {
    cached(tag("block", choice!(
        seq!(deferred(NEWLINE), opt(deferred(WS)), deferred(INDENT), opt(deferred(WS)), deferred(statements), opt(deferred(WS)), deferred(DEDENT)),
        seq!(deferred(simple_stmt), opt(deferred(WS)), choice!(deferred(NEWLINE), seq!(opt(seq!(python_literal(";"), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(simple_stmt), opt(repeat1(seq!(opt(deferred(WS)), python_literal(";"), opt(deferred(WS)), opt(seq!(deferred(WS), opt(deferred(WS)))), deferred(simple_stmt)))), opt(deferred(WS)))), opt(seq!(python_literal(";"), opt(deferred(WS)))), deferred(NEWLINE))))
    ))).into()
}

fn dotted_name() -> Combinator {
    tag("dotted_name", seq!(deferred(NAME), opt(seq!(opt(deferred(WS)), python_literal("."), opt(deferred(WS)), deferred(NAME), opt(repeat1(seq!(opt(deferred(WS)), python_literal("."), opt(deferred(WS)), deferred(NAME)))))))).into()
}

fn dotted_as_name() -> Combinator {
    tag("dotted_as_name", seq!(deferred(dotted_name), opt(seq!(opt(deferred(WS)), python_literal("as"), opt(deferred(WS)), deferred(NAME))))).into()
}

fn dotted_as_names() -> Combinator {
    tag("dotted_as_names", seq!(deferred(dotted_as_name), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(dotted_as_name), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(dotted_as_name)))))))).into()
}

fn import_from_as_name() -> Combinator {
    tag("import_from_as_name", seq!(deferred(NAME), opt(seq!(opt(deferred(WS)), python_literal("as"), opt(deferred(WS)), deferred(NAME))))).into()
}

fn import_from_as_names() -> Combinator {
    tag("import_from_as_names", seq!(deferred(import_from_as_name), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(import_from_as_name), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(import_from_as_name)))))))).into()
}

fn import_from_targets() -> Combinator {
    tag("import_from_targets", choice!(
        seq!(python_literal("("), opt(deferred(WS)), deferred(import_from_as_names), opt(seq!(opt(deferred(WS)), python_literal(","))), opt(deferred(WS)), python_literal(")")),
        deferred(import_from_as_names),
        python_literal("*")
    )).into()
}

fn import_from() -> Combinator {
    tag("import_from", seq!(python_literal("from"), opt(deferred(WS)), choice!(seq!(opt(seq!(choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(deferred(WS)), choice!(python_literal("."), python_literal("..."))))), opt(deferred(WS)))), deferred(dotted_name), opt(deferred(WS)), python_literal("import"), opt(deferred(WS)), deferred(import_from_targets)), seq!(choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(deferred(WS)), choice!(python_literal("."), python_literal("..."))))), opt(deferred(WS)), python_literal("import"), opt(deferred(WS)), deferred(import_from_targets))))).into()
}

fn import_name() -> Combinator {
    tag("import_name", seq!(python_literal("import"), opt(deferred(WS)), deferred(dotted_as_names))).into()
}

fn import_stmt() -> Combinator {
    tag("import_stmt", choice!(
        deferred(import_name),
        deferred(import_from)
    )).into()
}

fn assert_stmt() -> Combinator {
    tag("assert_stmt", seq!(python_literal("assert"), opt(deferred(WS)), deferred(expression), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(expression))))).into()
}

fn yield_stmt() -> Combinator {
    tag("yield_stmt", deferred(yield_expr)).into()
}

fn del_stmt() -> Combinator {
    tag("del_stmt", seq!(python_literal("del"), opt(deferred(WS)), deferred(del_targets))).into()
}

fn nonlocal_stmt() -> Combinator {
    tag("nonlocal_stmt", seq!(python_literal("nonlocal"), opt(deferred(WS)), deferred(NAME), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(NAME), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(NAME)))))))).into()
}

fn global_stmt() -> Combinator {
    tag("global_stmt", seq!(python_literal("global"), opt(deferred(WS)), deferred(NAME), opt(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(NAME), opt(repeat1(seq!(opt(deferred(WS)), python_literal(","), opt(deferred(WS)), deferred(NAME)))))))).into()
}

fn raise_stmt() -> Combinator {
    tag("raise_stmt", seq!(python_literal("raise"), opt(seq!(opt(deferred(WS)), deferred(expression), opt(seq!(opt(deferred(WS)), python_literal("from"), opt(deferred(WS)), deferred(expression))))))).into()
}

fn return_stmt() -> Combinator {
    tag("return_stmt", seq!(python_literal("return"), opt(seq!(opt(deferred(WS)), deferred(star_expressions))))).into()
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
        deferred(yield_expr),
        deferred(star_expressions)
    )).into()
}

fn assignment() -> Combinator {
    tag("assignment", choice!(
        seq!(deferred(NAME), opt(deferred(WS)), python_literal(":"), opt(deferred(WS)), deferred(expression), opt(seq!(opt(deferred(WS)), python_literal("="), opt(deferred(WS)), deferred(annotated_rhs)))),
        seq!(choice!(seq!(python_literal("("), opt(deferred(WS)), deferred(single_target), opt(deferred(WS)), python_literal(")")), deferred(single_subscript_attribute_target)), opt(deferred(WS)), python_literal(":"), opt(deferred(WS)), deferred(expression), opt(seq!(opt(deferred(WS)), python_literal("="), opt(deferred(WS)), deferred(annotated_rhs)))),
        seq!(deferred(star_targets), opt(deferred(WS)), python_literal("="), opt(seq!(opt(deferred(WS)), deferred(star_targets), opt(deferred(WS)), python_literal("="), opt(repeat1(seq!(opt(deferred(WS)), deferred(star_targets), opt(deferred(WS)), python_literal("=")))))), opt(deferred(WS)), choice!(deferred(yield_expr), deferred(star_expressions)), opt(seq!(opt(deferred(WS)), deferred(TYPE_COMMENT)))),
        seq!(deferred(single_target), opt(deferred(WS)), deferred(augassign), opt(deferred(WS)), choice!(deferred(yield_expr), deferred(star_expressions)))
    )).into()
}

fn compound_stmt() -> Combinator {
    tag("compound_stmt", choice!(
        deferred(function_def),
        deferred(if_stmt),
        deferred(class_def),
        deferred(with_stmt),
        deferred(for_stmt),
        deferred(try_stmt),
        deferred(while_stmt),
        deferred(match_stmt)
    )).into()
}

fn simple_stmt() -> Combinator {
    cached(tag("simple_stmt", choice!(
        deferred(assignment),
        deferred(type_alias),
        deferred(star_expressions),
        deferred(return_stmt),
        deferred(import_stmt),
        deferred(raise_stmt),
        python_literal("pass"),
        deferred(del_stmt),
        deferred(yield_stmt),
        deferred(assert_stmt),
        python_literal("break"),
        python_literal("continue"),
        deferred(global_stmt),
        deferred(nonlocal_stmt)
    ))).into()
}

fn simple_stmts() -> Combinator {
    tag("simple_stmts", seq!(deferred(simple_stmt), opt(deferred(WS)), choice!(deferred(NEWLINE), seq!(opt(seq!(python_literal(";"), opt(deferred(WS)), deferred(simple_stmt), opt(repeat1(seq!(opt(deferred(WS)), python_literal(";"), opt(deferred(WS)), deferred(simple_stmt)))), opt(deferred(WS)))), opt(seq!(python_literal(";"), opt(deferred(WS)))), deferred(NEWLINE))))).into()
}

fn statement_newline() -> Combinator {
    tag("statement_newline", choice!(
        seq!(deferred(compound_stmt), opt(deferred(WS)), deferred(NEWLINE)),
        deferred(simple_stmts),
        deferred(NEWLINE),
        deferred(ENDMARKER)
    )).into()
}

fn statement() -> Combinator {
    tag("statement", choice!(
        deferred(compound_stmt),
        deferred(simple_stmts)
    )).into()
}

fn statements() -> Combinator {
    tag("statements", seq!(deferred(statement), opt(repeat1(seq!(opt(deferred(WS)), deferred(statement)))))).into()
}

fn func_type() -> Combinator {
    tag("func_type", seq!(
        python_literal("("),
         opt(seq!(opt(deferred(WS)), deferred(type_expressions))),
         opt(deferred(WS)),
         python_literal(")"),
         opt(deferred(WS)),
         python_literal("->"),
         opt(deferred(WS)),
         deferred(expression),
         opt(seq!(opt(deferred(WS)), deferred(NEWLINE), opt(repeat1(seq!(opt(deferred(WS)), deferred(NEWLINE)))))),
         opt(deferred(WS)),
         deferred(ENDMARKER)
    )).into()
}

fn eval() -> Combinator {
    tag("eval", seq!(deferred(expressions), opt(seq!(opt(deferred(WS)), deferred(NEWLINE), opt(repeat1(seq!(opt(deferred(WS)), deferred(NEWLINE)))))), opt(deferred(WS)), deferred(ENDMARKER))).into()
}

fn interactive() -> Combinator {
    tag("interactive", deferred(statement_newline)).into()
}

fn file() -> Combinator {
    tag("file", seq!(opt(seq!(deferred(statements), opt(deferred(WS)))), deferred(ENDMARKER))).into()
}

pub fn python_file() -> Combinator {

    cache_context(seq!(opt(deferred(NEWLINE)), deferred(file))).into()
}
