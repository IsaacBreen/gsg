// use std::rc::Rc;
// use crate::{cache_context, cached, lookahead_context, symbol, Symbol, Choice, deferred, Combinator, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, forward_decls, forward_ref, Repeat1, Seq, tag, lookahead, negative_lookahead};
// use super::python_tokenizer::python_literal;
// use crate::seq;
// use crate::{opt_greedy as opt, choice_greedy as choice, seprep0_greedy as seprep0, seprep1_greedy as seprep1, repeat0_greedy as repeat0, repeat1_greedy as repeat1};
//
// enum Forbidden {
//     WS,
//     NAME,
//     TYPE_COMMENT,
//     FSTRING_START,
//     FSTRING_MIDDLE,
//     FSTRING_END,
//     NUMBER,
//     STRING,
//     NEWLINE,
//     INDENT,
//     DEDENT,
//     ENDMARKER,
// }
//
// use super::python_tokenizer as token;
// fn WS() -> Combinator { cached(tag("WS", seq!(forbid_follows_check_not(Forbidden::WS as usize), token::WS().compile(), forbid_follows(&[Forbidden::DEDENT as usize, Forbidden::INDENT as usize, Forbidden::NEWLINE as usize])))).into() }
// fn NAME() -> Combinator { cached(tag("NAME", seq!(forbid_follows_check_not(Forbidden::NAME as usize), token::NAME().compile(), forbid_follows(&[Forbidden::NAME as usize, Forbidden::NUMBER as usize])))).into() }
// fn TYPE_COMMENT() -> Combinator { cached(tag("TYPE_COMMENT", seq!(forbid_follows_clear(), token::TYPE_COMMENT().compile()))).into() }
// fn FSTRING_START() -> Combinator { cached(tag("FSTRING_START", seq!(token::FSTRING_START().compile(), forbid_follows(&[Forbidden::WS as usize])))).into() }
// fn FSTRING_MIDDLE() -> Combinator { cached(tag("FSTRING_MIDDLE", seq!(forbid_follows_check_not(Forbidden::FSTRING_MIDDLE as usize), token::FSTRING_MIDDLE().compile(), forbid_follows(&[Forbidden::FSTRING_MIDDLE as usize, Forbidden::WS as usize])))).into() }
// fn FSTRING_END() -> Combinator { cached(tag("FSTRING_END", seq!(forbid_follows_clear(), token::FSTRING_END().compile()))).into() }
// fn NUMBER() -> Combinator { cached(tag("NUMBER", seq!(forbid_follows_check_not(Forbidden::NUMBER as usize), token::NUMBER().compile(), forbid_follows(&[Forbidden::NUMBER as usize])))).into() }
// fn STRING() -> Combinator { cached(tag("STRING", seq!(forbid_follows_clear(), token::STRING().compile()))).into() }
// fn NEWLINE() -> Combinator { cached(tag("NEWLINE", seq!(forbid_follows_check_not(Forbidden::NEWLINE as usize), token::NEWLINE().compile(), forbid_follows(&[Forbidden::WS as usize])))).into() }
// fn INDENT() -> Combinator { cached(tag("INDENT", seq!(forbid_follows_check_not(Forbidden::INDENT as usize), token::INDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))).into() }
// fn DEDENT() -> Combinator { cached(tag("DEDENT", seq!(forbid_follows_check_not(Forbidden::DEDENT as usize), token::DEDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))).into() }
// fn ENDMARKER() -> Combinator { cached(tag("ENDMARKER", seq!(forbid_follows_clear(), token::ENDMARKER().compile()))).into() }
//
// fn expression_without_invalid() -> Combinator {
//     tag("expression_without_invalid", choice!(
//         seq!(&conjunction, opt(seq!(opt(&WS), python_literal("or"), opt(&WS), opt(seq!(&WS, opt(&WS))), &conjunction, opt(repeat1(seq!(opt(&WS), python_literal("or"), opt(&WS), opt(seq!(&WS, opt(&WS))), &conjunction))))), opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
//         seq!(python_literal("lambda"), opt(seq!(opt(&WS), &lambda_params)), opt(&WS), python_literal(":"), opt(&WS), &expression)
//     )).into()
// }
//
// fn func_type_comment() -> Combinator {
//     tag("func_type_comment", choice!(
//         seq!(&NEWLINE, opt(&WS), &TYPE_COMMENT, lookahead(seq!(opt(&WS), &NEWLINE, opt(&WS), &INDENT))),
//         &TYPE_COMMENT
//     )).into()
// }
//
// fn type_expressions() -> Combinator {
//     tag("type_expressions", choice!(
//         seq!(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &expression))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(seq!(python_literal("*"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), python_literal("**"), opt(&WS), &expression))), seq!(python_literal("**"), opt(&WS), &expression))))),
//         seq!(python_literal("*"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), python_literal("**"), opt(&WS), &expression))),
//         seq!(python_literal("**"), opt(&WS), &expression)
//     )).into()
// }
//
// fn del_t_atom() -> Combinator {
//     tag("del_t_atom", choice!(
//         &NAME,
//         seq!(python_literal("("), opt(&WS), choice!(seq!(&del_target, opt(&WS), python_literal(")")), seq!(opt(seq!(&del_targets, opt(&WS))), python_literal(")")))),
//         seq!(python_literal("["), opt(seq!(opt(&WS), &del_targets)), opt(&WS), python_literal("]"))
//     )).into()
// }
//
// fn del_target() -> Combinator {
//     cached(tag("del_target", choice!(
//         seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(seq!(choice!(&STRING, &FSTRING_START), opt(&WS))), &strings), &NUMBER, seq!(lookahead(seq!(python_literal("("), opt(&WS))), choice!(&tuple, &group, &genexp)), seq!(lookahead(seq!(python_literal("["), opt(&WS))), choice!(&list, &listcomp)), seq!(lookahead(seq!(python_literal("{"), opt(&WS))), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(seq!(opt(&WS), &t_lookahead)), opt(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME, lookahead(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(python_literal("["), opt(&WS), opt(seq!(&WS, opt(&WS))), &slices, opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal("]"), lookahead(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(&genexp, lookahead(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(python_literal("("), opt(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &arguments)), opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal(")"), lookahead(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &t_lookahead)))), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME, lookahead(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(python_literal("["), opt(&WS), opt(seq!(&WS, opt(&WS))), &slices, opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal("]"), lookahead(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(&genexp, lookahead(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(python_literal("("), opt(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &arguments)), opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal(")"), lookahead(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &t_lookahead))))))))), opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME, negative_lookahead(seq!(opt(&WS), &t_lookahead))), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), negative_lookahead(seq!(opt(&WS), &t_lookahead))))),
//         &del_t_atom
//     ))).into()
// }
//
// fn del_targets() -> Combinator {
//     tag("del_targets", seq!(&del_target, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &del_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &del_target))))), opt(seq!(opt(&WS), python_literal(","))))).into()
// }
//
// fn t_lookahead() -> Combinator {
//     tag("t_lookahead", choice!(
//         python_literal("("),
//         python_literal("["),
//         python_literal(".")
//     )).into()
// }
//
// fn t_primary() -> Combinator {
//     tag("t_primary", seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(seq!(choice!(&STRING, &FSTRING_START), opt(&WS))), &strings), &NUMBER, seq!(lookahead(seq!(python_literal("("), opt(&WS))), choice!(&tuple, &group, &genexp)), seq!(lookahead(seq!(python_literal("["), opt(&WS))), choice!(&list, &listcomp)), seq!(lookahead(seq!(python_literal("{"), opt(&WS))), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(seq!(opt(&WS), &t_lookahead)), opt(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME, lookahead(seq!(opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), lookahead(seq!(opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(&genexp, lookahead(seq!(opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")"), lookahead(seq!(opt(seq!(&WS, opt(&WS))), &t_lookahead)))), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME, lookahead(seq!(opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), lookahead(seq!(opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(&genexp, lookahead(seq!(opt(seq!(&WS, opt(&WS))), &t_lookahead))), seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")"), lookahead(seq!(opt(seq!(&WS, opt(&WS))), &t_lookahead))))))))))).into()
// }
//
// fn single_subscript_attribute_target() -> Combinator {
//     tag("single_subscript_attribute_target", seq!(&t_primary, opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME, negative_lookahead(seq!(opt(&WS), &t_lookahead))), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), negative_lookahead(seq!(opt(&WS), &t_lookahead)))))).into()
// }
//
// fn single_target() -> Combinator {
//     tag("single_target", choice!(
//         &single_subscript_attribute_target,
//         &NAME,
//         seq!(python_literal("("), opt(&WS), &single_target, opt(&WS), python_literal(")"))
//     )).into()
// }
//
// fn star_atom() -> Combinator {
//     tag("star_atom", choice!(
//         &NAME,
//         seq!(python_literal("("), opt(&WS), choice!(seq!(&target_with_star_atom, opt(&WS), python_literal(")")), seq!(opt(seq!(&star_targets_tuple_seq, opt(&WS))), python_literal(")")))),
//         seq!(python_literal("["), opt(seq!(opt(&WS), &star_targets_list_seq)), opt(&WS), python_literal("]"))
//     )).into()
// }
//
// fn target_with_star_atom() -> Combinator {
//     cached(tag("target_with_star_atom", choice!(
//         seq!(&t_primary, opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME, negative_lookahead(seq!(opt(&WS), &t_lookahead))), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), negative_lookahead(seq!(opt(&WS), &t_lookahead))))),
//         &star_atom
//     ))).into()
// }
//
// fn star_target() -> Combinator {
//     cached(tag("star_target", choice!(
//         seq!(python_literal("*"), negative_lookahead(seq!(opt(&WS), python_literal("*"))), opt(&WS), &star_target),
//         &target_with_star_atom
//     ))).into()
// }
//
// fn star_targets_tuple_seq() -> Combinator {
//     tag("star_targets_tuple_seq", seq!(&star_target, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))), opt(seq!(opt(&WS), python_literal(","))))))).into()
// }
//
// fn star_targets_list_seq() -> Combinator {
//     tag("star_targets_list_seq", seq!(&star_target, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))))), opt(seq!(opt(&WS), python_literal(","))))).into()
// }
//
// fn star_targets() -> Combinator {
//     tag("star_targets", seq!(&star_target, choice!(negative_lookahead(seq!(opt(&WS), python_literal(","))), seq!(opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))))), opt(seq!(opt(&WS), python_literal(","))))))).into()
// }
//
// fn kwarg_or_double_starred() -> Combinator {
//     tag("kwarg_or_double_starred", choice!(
//         seq!(&NAME, opt(&WS), python_literal("="), opt(&WS), &expression),
//         seq!(python_literal("**"), opt(&WS), &expression)
//     )).into()
// }
//
// fn kwarg_or_starred() -> Combinator {
//     tag("kwarg_or_starred", choice!(
//         seq!(&NAME, opt(&WS), python_literal("="), opt(&WS), &expression),
//         seq!(python_literal("*"), opt(&WS), &expression)
//     )).into()
// }
//
// fn starred_expression() -> Combinator {
//     tag("starred_expression", seq!(python_literal("*"), opt(&WS), &expression)).into()
// }
//
// fn kwargs() -> Combinator {
//     tag("kwargs", choice!(
//         seq!(&kwarg_or_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_starred))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred)))))))),
//         seq!(&kwarg_or_double_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred))))))
//     )).into()
// }
//
// fn args() -> Combinator {
//     tag("args", choice!(
//         seq!(choice!(&starred_expression, seq!(choice!(seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(seq!(opt(&WS), python_literal(":="))))), negative_lookahead(seq!(opt(&WS), python_literal("="))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(&starred_expression, seq!(choice!(seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(seq!(opt(&WS), python_literal(":="))))), negative_lookahead(seq!(opt(&WS), python_literal("="))))), opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(&starred_expression, seq!(choice!(seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(seq!(opt(&WS), python_literal(":="))))), negative_lookahead(seq!(opt(&WS), python_literal("=")))))))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwargs))),
//         &kwargs
//     )).into()
// }
//
// fn arguments() -> Combinator {
//     cached(tag("arguments", seq!(&args, opt(seq!(opt(&WS), python_literal(","))), lookahead(seq!(opt(&WS), python_literal(")")))))).into()
// }
//
// fn dictcomp() -> Combinator {
//     tag("dictcomp", seq!(
//         python_literal("{"),
//          opt(&WS),
//          &kvpair,
//          opt(&WS),
//          &for_if_clauses,
//          opt(&WS),
//          python_literal("}")
//     )).into()
// }
//
// fn genexp() -> Combinator {
//     tag("genexp", seq!(
//         python_literal("("),
//          opt(&WS),
//          choice!(&assignment_expression, seq!(&expression, negative_lookahead(seq!(opt(&WS), python_literal(":="))))),
//          opt(&WS),
//          &for_if_clauses,
//          opt(&WS),
//          python_literal(")")
//     )).into()
// }
//
// fn setcomp() -> Combinator {
//     tag("setcomp", seq!(
//         python_literal("{"),
//          opt(&WS),
//          &named_expression,
//          opt(&WS),
//          &for_if_clauses,
//          opt(&WS),
//          python_literal("}")
//     )).into()
// }
//
// fn listcomp() -> Combinator {
//     tag("listcomp", seq!(
//         python_literal("["),
//          opt(&WS),
//          &named_expression,
//          opt(&WS),
//          &for_if_clauses,
//          opt(&WS),
//          python_literal("]")
//     )).into()
// }
//
// fn for_if_clause() -> Combinator {
//     tag("for_if_clause", choice!(
//         seq!(python_literal("async"), opt(&WS), python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(repeat1(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction)))))),
//         seq!(python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(repeat1(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction))))))
//     )).into()
// }
//
// fn for_if_clauses() -> Combinator {
//     tag("for_if_clauses", seq!(&for_if_clause, opt(repeat1(seq!(opt(&WS), &for_if_clause))))).into()
// }
//
// fn kvpair() -> Combinator {
//     tag("kvpair", seq!(
//         choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &expression
//     )).into()
// }
//
// fn double_starred_kvpair() -> Combinator {
//     tag("double_starred_kvpair", choice!(
//         seq!(python_literal("**"), opt(&WS), &bitwise_or),
//         &kvpair
//     )).into()
// }
//
// fn double_starred_kvpairs() -> Combinator {
//     tag("double_starred_kvpairs", seq!(&double_starred_kvpair, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &double_starred_kvpair, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &double_starred_kvpair))))), opt(seq!(opt(&WS), python_literal(","))))).into()
// }
//
// fn dict() -> Combinator {
//     tag("dict", seq!(python_literal("{"), opt(seq!(opt(&WS), &double_starred_kvpairs)), opt(&WS), python_literal("}"))).into()
// }
//
// fn set() -> Combinator {
//     tag("set", seq!(
//         python_literal("{"),
//          opt(&WS),
//          &star_named_expressions,
//          opt(&WS),
//          python_literal("}")
//     )).into()
// }
//
// fn tuple() -> Combinator {
//     tag("tuple", seq!(python_literal("("), opt(seq!(opt(&WS), &star_named_expression, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_named_expressions)))), opt(&WS), python_literal(")"))).into()
// }
//
// fn list() -> Combinator {
//     tag("list", seq!(python_literal("["), opt(seq!(opt(&WS), &star_named_expressions)), opt(&WS), python_literal("]"))).into()
// }
//
// fn strings() -> Combinator {
//     cached(tag("strings", seq!(choice!(seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END), &STRING), opt(repeat1(seq!(opt(&WS), choice!(seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END), &STRING))))))).into()
// }
//
// fn string() -> Combinator {
//     tag("string", &STRING).into()
// }
//
// fn fstring() -> Combinator {
//     tag("fstring", seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END)).into()
// }
//
// fn fstring_format_spec() -> Combinator {
//     tag("fstring_format_spec", choice!(
//         &FSTRING_MIDDLE,
//         seq!(python_literal("{"), opt(&WS), &annotated_rhs, opt(seq!(opt(&WS), python_literal("="))), opt(seq!(opt(&WS), &fstring_conversion)), opt(seq!(opt(&WS), &fstring_full_format_spec)), opt(&WS), python_literal("}"))
//     )).into()
// }
//
// fn fstring_full_format_spec() -> Combinator {
//     tag("fstring_full_format_spec", seq!(python_literal(":"), opt(seq!(opt(&WS), &fstring_format_spec, opt(repeat1(seq!(opt(&WS), &fstring_format_spec))))))).into()
// }
//
// fn fstring_conversion() -> Combinator {
//     tag("fstring_conversion", seq!(python_literal("!"), opt(&WS), &NAME)).into()
// }
//
// fn fstring_replacement_field() -> Combinator {
//     tag("fstring_replacement_field", seq!(
//         python_literal("{"),
//          opt(&WS),
//          &annotated_rhs,
//          opt(seq!(opt(&WS), python_literal("="))),
//          opt(seq!(opt(&WS), &fstring_conversion)),
//          opt(seq!(opt(&WS), &fstring_full_format_spec)),
//          opt(&WS),
//          python_literal("}")
//     )).into()
// }
//
// fn fstring_middle() -> Combinator {
//     tag("fstring_middle", choice!(
//         &fstring_replacement_field,
//         &FSTRING_MIDDLE
//     )).into()
// }
//
// fn lambda_param() -> Combinator {
//     tag("lambda_param", &NAME).into()
// }
//
// fn lambda_param_maybe_default() -> Combinator {
//     tag("lambda_param_maybe_default", seq!(&lambda_param, opt(seq!(opt(&WS), &default)), choice!(seq!(opt(&WS), python_literal(",")), lookahead(seq!(opt(&WS), python_literal(":")))))).into()
// }
//
// fn lambda_param_with_default() -> Combinator {
//     tag("lambda_param_with_default", seq!(&lambda_param, opt(&WS), &default, choice!(seq!(opt(&WS), python_literal(",")), lookahead(seq!(opt(&WS), python_literal(":")))))).into()
// }
//
// fn lambda_param_no_default() -> Combinator {
//     tag("lambda_param_no_default", seq!(&lambda_param, choice!(seq!(opt(&WS), python_literal(",")), lookahead(seq!(opt(&WS), python_literal(":")))))).into()
// }
//
// fn lambda_kwds() -> Combinator {
//     tag("lambda_kwds", seq!(python_literal("**"), opt(&WS), &lambda_param_no_default)).into()
// }
//
// fn lambda_star_etc() -> Combinator {
//     tag("lambda_star_etc", choice!(
//         seq!(python_literal("*"), opt(&WS), choice!(seq!(&lambda_param_no_default, opt(seq!(opt(&WS), &lambda_param_maybe_default, opt(repeat1(seq!(opt(&WS), &lambda_param_maybe_default))))), opt(seq!(opt(&WS), &lambda_kwds))), seq!(python_literal(","), opt(&WS), &lambda_param_maybe_default, opt(repeat1(seq!(opt(&WS), &lambda_param_maybe_default))), opt(seq!(opt(&WS), &lambda_kwds))))),
//         &lambda_kwds
//     )).into()
// }
//
// fn lambda_slash_with_default() -> Combinator {
//     tag("lambda_slash_with_default", seq!(
//         opt(seq!(&lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))), opt(&WS))),
//          &lambda_param_with_default,
//          opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))),
//          opt(&WS),
//          python_literal("/"),
//          choice!(seq!(opt(&WS), python_literal(",")), lookahead(seq!(opt(&WS), python_literal(":"))))
//     )).into()
// }
//
// fn lambda_slash_no_default() -> Combinator {
//     tag("lambda_slash_no_default", seq!(
//         &lambda_param_no_default,
//          opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))),
//          opt(&WS),
//          python_literal("/"),
//          choice!(seq!(opt(&WS), python_literal(",")), lookahead(seq!(opt(&WS), python_literal(":"))))
//     )).into()
// }
//
// fn lambda_parameters() -> Combinator {
//     tag("lambda_parameters", choice!(
//         seq!(&lambda_slash_no_default, opt(seq!(opt(&WS), &lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))))), opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
//         seq!(&lambda_slash_with_default, opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
//         seq!(&lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))), opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
//         seq!(&lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))), opt(seq!(opt(&WS), &lambda_star_etc))),
//         &lambda_star_etc
//     )).into()
// }
//
// fn lambda_params() -> Combinator {
//     tag("lambda_params", &lambda_parameters).into()
// }
//
// fn lambdef() -> Combinator {
//     tag("lambdef", seq!(
//         python_literal("lambda"),
//          opt(seq!(opt(&WS), &lambda_params)),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &expression
//     )).into()
// }
//
// fn group() -> Combinator {
//     tag("group", seq!(
//         python_literal("("),
//          opt(&WS),
//          choice!(&yield_expr, &named_expression),
//          opt(&WS),
//          python_literal(")")
//     )).into()
// }
//
// fn atom() -> Combinator {
//     tag("atom", choice!(
//         &NAME,
//         python_literal("True"),
//         python_literal("False"),
//         python_literal("None"),
//         seq!(lookahead(seq!(choice!(&STRING, &FSTRING_START), opt(&WS))), &strings),
//         &NUMBER,
//         seq!(lookahead(seq!(python_literal("("), opt(&WS))), choice!(&tuple, &group, &genexp)),
//         seq!(lookahead(seq!(python_literal("["), opt(&WS))), choice!(&list, &listcomp)),
//         seq!(lookahead(seq!(python_literal("{"), opt(&WS))), choice!(&dict, &set, &dictcomp, &setcomp)),
//         python_literal("...")
//     )).into()
// }
//
// fn slice() -> Combinator {
//     tag("slice", choice!(
//         seq!(opt(seq!(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), opt(&WS))), python_literal(":"), opt(seq!(opt(&WS), &expression)), opt(seq!(opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &expression))))),
//         choice!(seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(seq!(opt(&WS), python_literal(":=")))))
//     )).into()
// }
//
// fn slices() -> Combinator {
//     tag("slices", choice!(
//         seq!(&slice, negative_lookahead(seq!(opt(&WS), python_literal(",")))),
//         seq!(choice!(&slice, &starred_expression), opt(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(&slice, &starred_expression), opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(&slice, &starred_expression)))))), opt(seq!(opt(&WS), python_literal(","))))
//     )).into()
// }
//
// fn primary() -> Combinator {
//     tag("primary", seq!(&atom, opt(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")")), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"))), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")")), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]")))))))))).into()
// }
//
// fn await_primary() -> Combinator {
//     cached(tag("await_primary", choice!(
//         seq!(python_literal("await"), opt(&WS), &primary),
//         &primary
//     ))).into()
// }
//
// fn power() -> Combinator {
//     tag("power", seq!(&await_primary, opt(seq!(opt(&WS), python_literal("**"), opt(&WS), &factor)))).into()
// }
//
// fn factor() -> Combinator {
//     cached(tag("factor", choice!(
//         seq!(python_literal("+"), opt(&WS), &factor),
//         seq!(python_literal("-"), opt(&WS), &factor),
//         seq!(python_literal("~"), opt(&WS), &factor),
//         &power
//     ))).into()
// }
//
// fn term() -> Combinator {
//     tag("term", seq!(&factor, opt(seq!(opt(&WS), choice!(seq!(python_literal("*"), opt(&WS), &factor), seq!(python_literal("/"), opt(&WS), &factor), seq!(python_literal("//"), opt(&WS), &factor), seq!(python_literal("%"), opt(&WS), &factor), seq!(python_literal("@"), opt(&WS), &factor)), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("*"), opt(&WS), &factor), seq!(python_literal("/"), opt(&WS), &factor), seq!(python_literal("//"), opt(&WS), &factor), seq!(python_literal("%"), opt(&WS), &factor), seq!(python_literal("@"), opt(&WS), &factor))))))))).into()
// }
//
// fn sum() -> Combinator {
//     tag("sum", seq!(&term, opt(seq!(opt(&WS), choice!(seq!(python_literal("+"), opt(&WS), &term), seq!(python_literal("-"), opt(&WS), &term)), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("+"), opt(&WS), &term), seq!(python_literal("-"), opt(&WS), &term))))))))).into()
// }
//
// fn shift_expr() -> Combinator {
//     tag("shift_expr", seq!(&sum, opt(seq!(opt(&WS), choice!(seq!(python_literal("<<"), opt(&WS), &sum), seq!(python_literal(">>"), opt(&WS), &sum)), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("<<"), opt(&WS), &sum), seq!(python_literal(">>"), opt(&WS), &sum))))))))).into()
// }
//
// fn bitwise_and() -> Combinator {
//     tag("bitwise_and", seq!(&shift_expr, opt(seq!(opt(&WS), python_literal("&"), opt(&WS), &shift_expr, opt(repeat1(seq!(opt(&WS), python_literal("&"), opt(&WS), &shift_expr))))))).into()
// }
//
// fn bitwise_xor() -> Combinator {
//     tag("bitwise_xor", seq!(&bitwise_and, opt(seq!(opt(&WS), python_literal("^"), opt(&WS), &bitwise_and, opt(repeat1(seq!(opt(&WS), python_literal("^"), opt(&WS), &bitwise_and))))))).into()
// }
//
// fn bitwise_or() -> Combinator {
//     tag("bitwise_or", seq!(&bitwise_xor, opt(seq!(opt(&WS), python_literal("|"), opt(&WS), &bitwise_xor, opt(repeat1(seq!(opt(&WS), python_literal("|"), opt(&WS), &bitwise_xor))))))).into()
// }
//
// fn is_bitwise_or() -> Combinator {
//     tag("is_bitwise_or", seq!(python_literal("is"), opt(&WS), &bitwise_or)).into()
// }
//
// fn isnot_bitwise_or() -> Combinator {
//     tag("isnot_bitwise_or", seq!(
//         python_literal("is"),
//          opt(&WS),
//          python_literal("not"),
//          opt(&WS),
//          &bitwise_or
//     )).into()
// }
//
// fn in_bitwise_or() -> Combinator {
//     tag("in_bitwise_or", seq!(python_literal("in"), opt(&WS), &bitwise_or)).into()
// }
//
// fn notin_bitwise_or() -> Combinator {
//     tag("notin_bitwise_or", seq!(
//         python_literal("not"),
//          opt(&WS),
//          python_literal("in"),
//          opt(&WS),
//          &bitwise_or
//     )).into()
// }
//
// fn gt_bitwise_or() -> Combinator {
//     tag("gt_bitwise_or", seq!(python_literal(">"), opt(&WS), &bitwise_or)).into()
// }
//
// fn gte_bitwise_or() -> Combinator {
//     tag("gte_bitwise_or", seq!(python_literal(">="), opt(&WS), &bitwise_or)).into()
// }
//
// fn lt_bitwise_or() -> Combinator {
//     tag("lt_bitwise_or", seq!(python_literal("<"), opt(&WS), &bitwise_or)).into()
// }
//
// fn lte_bitwise_or() -> Combinator {
//     tag("lte_bitwise_or", seq!(python_literal("<="), opt(&WS), &bitwise_or)).into()
// }
//
// fn noteq_bitwise_or() -> Combinator {
//     tag("noteq_bitwise_or", seq!(python_literal("!="), opt(&WS), &bitwise_or)).into()
// }
//
// fn eq_bitwise_or() -> Combinator {
//     tag("eq_bitwise_or", seq!(python_literal("=="), opt(&WS), &bitwise_or)).into()
// }
//
// fn compare_op_bitwise_or_pair() -> Combinator {
//     tag("compare_op_bitwise_or_pair", choice!(
//         &eq_bitwise_or,
//         &noteq_bitwise_or,
//         &lte_bitwise_or,
//         &lt_bitwise_or,
//         &gte_bitwise_or,
//         &gt_bitwise_or,
//         &notin_bitwise_or,
//         &in_bitwise_or,
//         &isnot_bitwise_or,
//         &is_bitwise_or
//     )).into()
// }
//
// fn comparison() -> Combinator {
//     tag("comparison", seq!(&bitwise_or, opt(seq!(opt(&WS), &compare_op_bitwise_or_pair, opt(repeat1(seq!(opt(&WS), &compare_op_bitwise_or_pair))))))).into()
// }
//
// fn inversion() -> Combinator {
//     cached(tag("inversion", choice!(
//         seq!(python_literal("not"), opt(&WS), &inversion),
//         &comparison
//     ))).into()
// }
//
// fn conjunction() -> Combinator {
//     cached(tag("conjunction", seq!(&inversion, opt(seq!(opt(&WS), python_literal("and"), opt(&WS), &inversion, opt(repeat1(seq!(opt(&WS), python_literal("and"), opt(&WS), &inversion)))))))).into()
// }
//
// fn disjunction() -> Combinator {
//     cached(tag("disjunction", seq!(&conjunction, opt(seq!(opt(&WS), python_literal("or"), opt(&WS), &conjunction, opt(repeat1(seq!(opt(&WS), python_literal("or"), opt(&WS), &conjunction)))))))).into()
// }
//
// fn named_expression() -> Combinator {
//     tag("named_expression", choice!(
//         seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression),
//         seq!(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(seq!(opt(&WS), python_literal(":="))))
//     )).into()
// }
//
// fn assignment_expression() -> Combinator {
//     tag("assignment_expression", seq!(
//         &NAME,
//          opt(&WS),
//          python_literal(":="),
//          opt(&WS),
//          &expression
//     )).into()
// }
//
// fn star_named_expression() -> Combinator {
//     tag("star_named_expression", choice!(
//         seq!(python_literal("*"), opt(&WS), &bitwise_or),
//         &named_expression
//     )).into()
// }
//
// fn star_named_expressions() -> Combinator {
//     tag("star_named_expressions", seq!(&star_named_expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_named_expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_named_expression))))), opt(seq!(opt(&WS), python_literal(","))))).into()
// }
//
// fn star_expression() -> Combinator {
//     cached(tag("star_expression", choice!(
//         seq!(python_literal("*"), opt(&WS), &bitwise_or),
//         choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef)
//     ))).into()
// }
//
// fn star_expressions() -> Combinator {
//     tag("star_expressions", seq!(&star_expression, opt(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_expression))), opt(seq!(opt(&WS), python_literal(","))))))))).into()
// }
//
// fn yield_expr() -> Combinator {
//     tag("yield_expr", seq!(python_literal("yield"), choice!(seq!(opt(&WS), python_literal("from"), opt(&WS), &expression), opt(seq!(opt(&WS), &star_expressions))))).into()
// }
//
// fn expression() -> Combinator {
//     cached(tag("expression", choice!(
//         seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
//         &lambdef
//     ))).into()
// }
//
// fn expressions() -> Combinator {
//     tag("expressions", seq!(&expression, opt(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &expression))), opt(seq!(opt(&WS), python_literal(","))))))))).into()
// }
//
// fn type_param_starred_default() -> Combinator {
//     tag("type_param_starred_default", seq!(python_literal("="), opt(&WS), &star_expression)).into()
// }
//
// fn type_param_default() -> Combinator {
//     tag("type_param_default", seq!(python_literal("="), opt(&WS), &expression)).into()
// }
//
// fn type_param_bound() -> Combinator {
//     tag("type_param_bound", seq!(python_literal(":"), opt(&WS), &expression)).into()
// }
//
// fn type_param() -> Combinator {
//     cached(tag("type_param", choice!(
//         seq!(&NAME, opt(seq!(opt(&WS), &type_param_bound)), opt(seq!(opt(&WS), &type_param_default))),
//         seq!(python_literal("*"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_param_starred_default))),
//         seq!(python_literal("**"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_param_default)))
//     ))).into()
// }
//
// fn type_param_seq() -> Combinator {
//     tag("type_param_seq", seq!(&type_param, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &type_param, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &type_param))))), opt(seq!(opt(&WS), python_literal(","))))).into()
// }
//
// fn type_params() -> Combinator {
//     tag("type_params", seq!(
//         python_literal("["),
//          opt(&WS),
//          &type_param_seq,
//          opt(&WS),
//          python_literal("]")
//     )).into()
// }
//
// fn type_alias() -> Combinator {
//     tag("type_alias", seq!(
//         python_literal("type"),
//          opt(&WS),
//          &NAME,
//          opt(seq!(opt(&WS), &type_params)),
//          opt(&WS),
//          python_literal("="),
//          opt(&WS),
//          &expression
//     )).into()
// }
//
// fn keyword_pattern() -> Combinator {
//     tag("keyword_pattern", seq!(
//         &NAME,
//          opt(&WS),
//          python_literal("="),
//          opt(&WS),
//          &pattern
//     )).into()
// }
//
// fn keyword_patterns() -> Combinator {
//     tag("keyword_patterns", seq!(&keyword_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &keyword_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &keyword_pattern))))))).into()
// }
//
// fn positional_patterns() -> Combinator {
//     tag("positional_patterns", seq!(choice!(&as_pattern, &or_pattern), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &pattern))))))).into()
// }
//
// fn class_pattern() -> Combinator {
//     tag("class_pattern", seq!(
//         &NAME,
//          opt(seq!(opt(&WS), python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME))))),
//          opt(&WS),
//          python_literal("("),
//          opt(&WS),
//          choice!(python_literal(")"), seq!(&positional_patterns, opt(&WS), choice!(seq!(opt(seq!(python_literal(","), opt(&WS))), python_literal(")")), seq!(python_literal(","), opt(&WS), &keyword_patterns, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")))), seq!(&keyword_patterns, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")))
//     )).into()
// }
//
// fn double_star_pattern() -> Combinator {
//     tag("double_star_pattern", seq!(python_literal("**"), opt(&WS), &pattern_capture_target)).into()
// }
//
// fn key_value_pattern() -> Combinator {
//     tag("key_value_pattern", seq!(
//         choice!(choice!(seq!(&signed_number, negative_lookahead(seq!(opt(&WS), choice!(python_literal("+"), python_literal("-"))))), &complex_number, &strings, python_literal("None"), python_literal("True"), python_literal("False")), seq!(&name_or_attr, opt(&WS), python_literal("."), opt(&WS), &NAME)),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &pattern
//     )).into()
// }
//
// fn items_pattern() -> Combinator {
//     tag("items_pattern", seq!(&key_value_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &key_value_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &key_value_pattern))))))).into()
// }
//
// fn mapping_pattern() -> Combinator {
//     tag("mapping_pattern", seq!(python_literal("{"), opt(&WS), choice!(python_literal("}"), seq!(&double_star_pattern, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal("}")), seq!(&items_pattern, opt(&WS), choice!(seq!(python_literal(","), opt(&WS), &double_star_pattern, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal("}")), seq!(opt(seq!(python_literal(","), opt(&WS))), python_literal("}"))))))).into()
// }
//
// fn star_pattern() -> Combinator {
//     cached(tag("star_pattern", seq!(python_literal("*"), opt(&WS), choice!(&pattern_capture_target, &wildcard_pattern)))).into()
// }
//
// fn maybe_star_pattern() -> Combinator {
//     tag("maybe_star_pattern", choice!(
//         &star_pattern,
//         choice!(&as_pattern, &or_pattern)
//     )).into()
// }
//
// fn maybe_sequence_pattern() -> Combinator {
//     tag("maybe_sequence_pattern", seq!(&maybe_star_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &maybe_star_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &maybe_star_pattern))))), opt(seq!(opt(&WS), python_literal(","))))).into()
// }
//
// fn open_sequence_pattern() -> Combinator {
//     tag("open_sequence_pattern", seq!(&maybe_star_pattern, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &maybe_sequence_pattern)))).into()
// }
//
// fn sequence_pattern() -> Combinator {
//     tag("sequence_pattern", choice!(
//         seq!(python_literal("["), opt(seq!(opt(&WS), &maybe_sequence_pattern)), opt(&WS), python_literal("]")),
//         seq!(python_literal("("), opt(seq!(opt(&WS), &open_sequence_pattern)), opt(&WS), python_literal(")"))
//     )).into()
// }
//
// fn group_pattern() -> Combinator {
//     tag("group_pattern", seq!(
//         python_literal("("),
//          opt(&WS),
//          &pattern,
//          opt(&WS),
//          python_literal(")")
//     )).into()
// }
//
// fn name_or_attr() -> Combinator {
//     tag("name_or_attr", seq!(&NAME, opt(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME))))))).into()
// }
//
// fn attr() -> Combinator {
//     tag("attr", seq!(
//         &name_or_attr,
//          opt(&WS),
//          python_literal("."),
//          opt(&WS),
//          &NAME
//     )).into()
// }
//
// fn value_pattern() -> Combinator {
//     tag("value_pattern", seq!(&attr, negative_lookahead(seq!(opt(&WS), choice!(python_literal("."), python_literal("("), python_literal("=")))))).into()
// }
//
// fn wildcard_pattern() -> Combinator {
//     tag("wildcard_pattern", python_literal("_")).into()
// }
//
// fn pattern_capture_target() -> Combinator {
//     tag("pattern_capture_target", seq!(negative_lookahead(seq!(python_literal("_"), opt(&WS))), &NAME, negative_lookahead(seq!(opt(&WS), choice!(python_literal("."), python_literal("("), python_literal("=")))))).into()
// }
//
// fn capture_pattern() -> Combinator {
//     tag("capture_pattern", &pattern_capture_target).into()
// }
//
// fn imaginary_number() -> Combinator {
//     tag("imaginary_number", &NUMBER).into()
// }
//
// fn real_number() -> Combinator {
//     tag("real_number", &NUMBER).into()
// }
//
// fn signed_real_number() -> Combinator {
//     tag("signed_real_number", choice!(
//         &real_number,
//         seq!(python_literal("-"), opt(&WS), &real_number)
//     )).into()
// }
//
// fn signed_number() -> Combinator {
//     tag("signed_number", choice!(
//         &NUMBER,
//         seq!(python_literal("-"), opt(&WS), &NUMBER)
//     )).into()
// }
//
// fn complex_number() -> Combinator {
//     tag("complex_number", seq!(&signed_real_number, opt(&WS), choice!(seq!(python_literal("+"), opt(&WS), &imaginary_number), seq!(python_literal("-"), opt(&WS), &imaginary_number)))).into()
// }
//
// fn literal_expr() -> Combinator {
//     tag("literal_expr", choice!(
//         seq!(&signed_number, negative_lookahead(seq!(opt(&WS), choice!(python_literal("+"), python_literal("-"))))),
//         &complex_number,
//         &strings,
//         python_literal("None"),
//         python_literal("True"),
//         python_literal("False")
//     )).into()
// }
//
// fn literal_pattern() -> Combinator {
//     tag("literal_pattern", choice!(
//         seq!(&signed_number, negative_lookahead(seq!(opt(&WS), choice!(python_literal("+"), python_literal("-"))))),
//         &complex_number,
//         &strings,
//         python_literal("None"),
//         python_literal("True"),
//         python_literal("False")
//     )).into()
// }
//
// fn closed_pattern() -> Combinator {
//     cached(tag("closed_pattern", choice!(
//         &literal_pattern,
//         &capture_pattern,
//         &wildcard_pattern,
//         &value_pattern,
//         &group_pattern,
//         &sequence_pattern,
//         &mapping_pattern,
//         &class_pattern
//     ))).into()
// }
//
// fn or_pattern() -> Combinator {
//     tag("or_pattern", seq!(&closed_pattern, opt(seq!(opt(&WS), python_literal("|"), opt(&WS), &closed_pattern, opt(repeat1(seq!(opt(&WS), python_literal("|"), opt(&WS), &closed_pattern))))))).into()
// }
//
// fn as_pattern() -> Combinator {
//     tag("as_pattern", seq!(
//         &or_pattern,
//          opt(&WS),
//          python_literal("as"),
//          opt(&WS),
//          &pattern_capture_target
//     )).into()
// }
//
// fn pattern() -> Combinator {
//     tag("pattern", choice!(
//         &as_pattern,
//         &or_pattern
//     )).into()
// }
//
// fn patterns() -> Combinator {
//     tag("patterns", choice!(
//         &open_sequence_pattern,
//         &pattern
//     )).into()
// }
//
// fn guard() -> Combinator {
//     tag("guard", seq!(python_literal("if"), opt(&WS), &named_expression)).into()
// }
//
// fn case_block() -> Combinator {
//     tag("case_block", seq!(
//         python_literal("case"),
//          opt(&WS),
//          &patterns,
//          opt(seq!(opt(&WS), &guard)),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block
//     )).into()
// }
//
// fn subject_expr() -> Combinator {
//     tag("subject_expr", choice!(
//         seq!(&star_named_expression, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_named_expressions))),
//         &named_expression
//     )).into()
// }
//
// fn match_stmt() -> Combinator {
//     tag("match_stmt", seq!(
//         python_literal("match"),
//          opt(&WS),
//          &subject_expr,
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &NEWLINE,
//          opt(&WS),
//          &INDENT,
//          opt(&WS),
//          &case_block,
//          opt(repeat1(seq!(opt(&WS), &case_block))),
//          opt(&WS),
//          &DEDENT
//     )).into()
// }
//
// fn finally_block() -> Combinator {
//     tag("finally_block", seq!(
//         python_literal("finally"),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block
//     )).into()
// }
//
// fn except_star_block() -> Combinator {
//     tag("except_star_block", seq!(
//         python_literal("except"),
//          opt(&WS),
//          python_literal("*"),
//          opt(&WS),
//          &expression,
//          opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block
//     )).into()
// }
//
// fn except_block() -> Combinator {
//     tag("except_block", seq!(python_literal("except"), opt(&WS), choice!(seq!(&expression, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)), opt(&WS), python_literal(":"), opt(&WS), &block), seq!(python_literal(":"), opt(&WS), &block)))).into()
// }
//
// fn try_stmt() -> Combinator {
//     tag("try_stmt", seq!(
//         python_literal("try"),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block,
//          opt(&WS),
//          choice!(&finally_block, seq!(&except_block, opt(repeat1(seq!(opt(&WS), &except_block))), opt(seq!(opt(&WS), &else_block)), opt(seq!(opt(&WS), &finally_block))), seq!(&except_star_block, opt(repeat1(seq!(opt(&WS), &except_star_block))), opt(seq!(opt(&WS), &else_block)), opt(seq!(opt(&WS), &finally_block))))
//     )).into()
// }
//
// fn with_item() -> Combinator {
//     tag("with_item", seq!(&expression, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &star_target, lookahead(seq!(opt(&WS), choice!(python_literal(","), python_literal(")"), python_literal(":")))))))).into()
// }
//
// fn with_stmt() -> Combinator {
//     tag("with_stmt", choice!(
//         seq!(python_literal("with"), opt(&WS), choice!(seq!(python_literal("("), opt(&WS), &with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")"), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block), seq!(&with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block))),
//         seq!(python_literal("async"), opt(&WS), python_literal("with"), opt(&WS), choice!(seq!(python_literal("("), opt(&WS), &with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")"), opt(&WS), python_literal(":"), opt(&WS), &block), seq!(&with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block)))
//     )).into()
// }
//
// fn for_stmt() -> Combinator {
//     tag("for_stmt", choice!(
//         seq!(python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &star_expressions, opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block, opt(seq!(opt(&WS), &else_block))),
//         seq!(python_literal("async"), opt(&WS), python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &star_expressions, opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block, opt(seq!(opt(&WS), &else_block)))
//     )).into()
// }
//
// fn while_stmt() -> Combinator {
//     tag("while_stmt", seq!(
//         python_literal("while"),
//          opt(&WS),
//          &named_expression,
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block,
//          opt(seq!(opt(&WS), &else_block))
//     )).into()
// }
//
// fn else_block() -> Combinator {
//     tag("else_block", seq!(
//         python_literal("else"),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block
//     )).into()
// }
//
// fn elif_stmt() -> Combinator {
//     tag("elif_stmt", seq!(
//         python_literal("elif"),
//          opt(&WS),
//          &named_expression,
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block,
//          choice!(seq!(opt(&WS), &elif_stmt), opt(seq!(opt(&WS), &else_block)))
//     )).into()
// }
//
// fn if_stmt() -> Combinator {
//     tag("if_stmt", seq!(
//         python_literal("if"),
//          opt(&WS),
//          &named_expression,
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block,
//          choice!(seq!(opt(&WS), &elif_stmt), opt(seq!(opt(&WS), &else_block)))
//     )).into()
// }
//
// fn default() -> Combinator {
//     tag("default", seq!(python_literal("="), opt(&WS), &expression)).into()
// }
//
// fn star_annotation() -> Combinator {
//     tag("star_annotation", seq!(python_literal(":"), opt(&WS), &star_expression)).into()
// }
//
// fn annotation() -> Combinator {
//     tag("annotation", seq!(python_literal(":"), opt(&WS), &expression)).into()
// }
//
// fn param_star_annotation() -> Combinator {
//     tag("param_star_annotation", seq!(&NAME, opt(&WS), &star_annotation)).into()
// }
//
// fn param() -> Combinator {
//     tag("param", seq!(&NAME, opt(seq!(opt(&WS), &annotation)))).into()
// }
//
// fn param_maybe_default() -> Combinator {
//     tag("param_maybe_default", seq!(&param, opt(seq!(opt(&WS), &default)), choice!(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), seq!(opt(seq!(opt(&WS), &TYPE_COMMENT)), lookahead(seq!(opt(&WS), python_literal(")"))))))).into()
// }
//
// fn param_with_default() -> Combinator {
//     tag("param_with_default", seq!(&param, opt(&WS), &default, choice!(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), seq!(opt(seq!(opt(&WS), &TYPE_COMMENT)), lookahead(seq!(opt(&WS), python_literal(")"))))))).into()
// }
//
// fn param_no_default_star_annotation() -> Combinator {
//     tag("param_no_default_star_annotation", seq!(&param_star_annotation, choice!(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), seq!(opt(seq!(opt(&WS), &TYPE_COMMENT)), lookahead(seq!(opt(&WS), python_literal(")"))))))).into()
// }
//
// fn param_no_default() -> Combinator {
//     tag("param_no_default", seq!(&param, choice!(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), seq!(opt(seq!(opt(&WS), &TYPE_COMMENT)), lookahead(seq!(opt(&WS), python_literal(")"))))))).into()
// }
//
// fn kwds() -> Combinator {
//     tag("kwds", seq!(python_literal("**"), opt(&WS), &param_no_default)).into()
// }
//
// fn star_etc() -> Combinator {
//     tag("star_etc", choice!(
//         seq!(python_literal("*"), opt(&WS), choice!(seq!(&param_no_default, opt(seq!(opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))))), opt(seq!(opt(&WS), &kwds))), seq!(&param_no_default_star_annotation, opt(seq!(opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))))), opt(seq!(opt(&WS), &kwds))), seq!(python_literal(","), opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))), opt(seq!(opt(&WS), &kwds))))),
//         &kwds
//     )).into()
// }
//
// fn slash_with_default() -> Combinator {
//     tag("slash_with_default", seq!(
//         opt(seq!(&param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))), opt(&WS))),
//          &param_with_default,
//          opt(repeat1(seq!(opt(&WS), &param_with_default))),
//          opt(&WS),
//          python_literal("/"),
//          choice!(seq!(opt(&WS), python_literal(",")), lookahead(seq!(opt(&WS), python_literal(")"))))
//     )).into()
// }
//
// fn slash_no_default() -> Combinator {
//     tag("slash_no_default", seq!(
//         &param_no_default,
//          opt(repeat1(seq!(opt(&WS), &param_no_default))),
//          opt(&WS),
//          python_literal("/"),
//          choice!(seq!(opt(&WS), python_literal(",")), lookahead(seq!(opt(&WS), python_literal(")"))))
//     )).into()
// }
//
// fn parameters() -> Combinator {
//     tag("parameters", choice!(
//         seq!(&slash_no_default, opt(seq!(opt(&WS), &param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))))), opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
//         seq!(&slash_with_default, opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
//         seq!(&param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))), opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
//         seq!(&param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))), opt(seq!(opt(&WS), &star_etc))),
//         &star_etc
//     )).into()
// }
//
// fn params() -> Combinator {
//     tag("params", &parameters).into()
// }
//
// fn function_def_raw() -> Combinator {
//     tag("function_def_raw", choice!(
//         seq!(python_literal("def"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_params)), opt(&WS), python_literal("("), opt(seq!(opt(&WS), &params)), opt(&WS), python_literal(")"), opt(seq!(opt(&WS), python_literal("->"), opt(&WS), &expression)), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &func_type_comment)), opt(&WS), &block),
//         seq!(python_literal("async"), opt(&WS), python_literal("def"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_params)), opt(&WS), python_literal("("), opt(seq!(opt(&WS), &params)), opt(&WS), python_literal(")"), opt(seq!(opt(&WS), python_literal("->"), opt(&WS), &expression)), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &func_type_comment)), opt(&WS), &block)
//     )).into()
// }
//
// fn function_def() -> Combinator {
//     tag("function_def", choice!(
//         seq!(python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &NEWLINE))))), opt(&WS), &function_def_raw),
//         &function_def_raw
//     )).into()
// }
//
// fn class_def_raw() -> Combinator {
//     tag("class_def_raw", seq!(
//         python_literal("class"),
//          opt(&WS),
//          &NAME,
//          opt(seq!(opt(&WS), &type_params)),
//          opt(seq!(opt(&WS), python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")"))),
//          opt(&WS),
//          python_literal(":"),
//          opt(&WS),
//          &block
//     )).into()
// }
//
// fn class_def() -> Combinator {
//     tag("class_def", choice!(
//         seq!(python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), &NEWLINE))))), opt(&WS), &class_def_raw),
//         &class_def_raw
//     )).into()
// }
//
// fn decorators() -> Combinator {
//     tag("decorators", seq!(
//         python_literal("@"),
//          opt(&WS),
//          &named_expression,
//          opt(&WS),
//          &NEWLINE,
//          opt(seq!(opt(&WS), python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE)))))
//     )).into()
// }
//
// fn block() -> Combinator {
//     cached(tag("block", choice!(
//         seq!(&NEWLINE, opt(&WS), &INDENT, opt(&WS), &statements, opt(&WS), &DEDENT),
//         seq!(&simple_stmt, opt(&WS), choice!(seq!(negative_lookahead(seq!(python_literal(";"), opt(&WS))), &NEWLINE), seq!(opt(seq!(python_literal(";"), opt(&WS), opt(seq!(&WS, opt(&WS))), &simple_stmt, opt(repeat1(seq!(opt(&WS), python_literal(";"), opt(&WS), opt(seq!(&WS, opt(&WS))), &simple_stmt))), opt(&WS))), opt(seq!(python_literal(";"), opt(&WS))), &NEWLINE)))
//     ))).into()
// }
//
// fn dotted_name() -> Combinator {
//     tag("dotted_name", seq!(&NAME, opt(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME))))))).into()
// }
//
// fn dotted_as_name() -> Combinator {
//     tag("dotted_as_name", seq!(&dotted_name, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)))).into()
// }
//
// fn dotted_as_names() -> Combinator {
//     tag("dotted_as_names", seq!(&dotted_as_name, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &dotted_as_name, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &dotted_as_name))))))).into()
// }
//
// fn import_from_as_name() -> Combinator {
//     tag("import_from_as_name", seq!(&NAME, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)))).into()
// }
//
// fn import_from_as_names() -> Combinator {
//     tag("import_from_as_names", seq!(&import_from_as_name, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &import_from_as_name, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &import_from_as_name))))))).into()
// }
//
// fn import_from_targets() -> Combinator {
//     tag("import_from_targets", choice!(
//         seq!(python_literal("("), opt(&WS), &import_from_as_names, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")),
//         seq!(&import_from_as_names, negative_lookahead(seq!(opt(&WS), python_literal(",")))),
//         python_literal("*")
//     )).into()
// }
//
// fn import_from() -> Combinator {
//     tag("import_from", seq!(python_literal("from"), opt(&WS), choice!(seq!(opt(seq!(choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(&WS), choice!(python_literal("."), python_literal("..."))))), opt(&WS))), &dotted_name, opt(&WS), python_literal("import"), opt(&WS), &import_from_targets), seq!(choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(&WS), choice!(python_literal("."), python_literal("..."))))), opt(&WS), python_literal("import"), opt(&WS), &import_from_targets)))).into()
// }
//
// fn import_name() -> Combinator {
//     tag("import_name", seq!(python_literal("import"), opt(&WS), &dotted_as_names)).into()
// }
//
// fn import_stmt() -> Combinator {
//     tag("import_stmt", choice!(
//         &import_name,
//         &import_from
//     )).into()
// }
//
// fn assert_stmt() -> Combinator {
//     tag("assert_stmt", seq!(python_literal("assert"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &expression)))).into()
// }
//
// fn yield_stmt() -> Combinator {
//     tag("yield_stmt", &yield_expr).into()
// }
//
// fn del_stmt() -> Combinator {
//     tag("del_stmt", seq!(python_literal("del"), opt(&WS), &del_targets, lookahead(seq!(opt(&WS), choice!(python_literal(";"), &NEWLINE))))).into()
// }
//
// fn nonlocal_stmt() -> Combinator {
//     tag("nonlocal_stmt", seq!(python_literal("nonlocal"), opt(&WS), &NAME, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME))))))).into()
// }
//
// fn global_stmt() -> Combinator {
//     tag("global_stmt", seq!(python_literal("global"), opt(&WS), &NAME, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME))))))).into()
// }
//
// fn raise_stmt() -> Combinator {
//     tag("raise_stmt", seq!(python_literal("raise"), opt(seq!(opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("from"), opt(&WS), &expression)))))).into()
// }
//
// fn return_stmt() -> Combinator {
//     tag("return_stmt", seq!(python_literal("return"), opt(seq!(opt(&WS), &star_expressions)))).into()
// }
//
// fn augassign() -> Combinator {
//     tag("augassign", choice!(
//         python_literal("+="),
//         python_literal("-="),
//         python_literal("*="),
//         python_literal("@="),
//         python_literal("/="),
//         python_literal("%="),
//         python_literal("&="),
//         python_literal("|="),
//         python_literal("^="),
//         python_literal("<<="),
//         python_literal(">>="),
//         python_literal("**="),
//         python_literal("//=")
//     )).into()
// }
//
// fn annotated_rhs() -> Combinator {
//     tag("annotated_rhs", choice!(
//         &yield_expr,
//         &star_expressions
//     )).into()
// }
//
// fn assignment() -> Combinator {
//     tag("assignment", choice!(
//         seq!(&NAME, opt(&WS), python_literal(":"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("="), opt(&WS), &annotated_rhs))),
//         seq!(choice!(seq!(python_literal("("), opt(&WS), &single_target, opt(&WS), python_literal(")")), &single_subscript_attribute_target), opt(&WS), python_literal(":"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("="), opt(&WS), &annotated_rhs))),
//         seq!(&star_targets, opt(&WS), python_literal("="), opt(seq!(opt(&WS), &star_targets, opt(&WS), python_literal("="), opt(repeat1(seq!(opt(&WS), &star_targets, opt(&WS), python_literal("=")))))), opt(&WS), choice!(&yield_expr, &star_expressions), negative_lookahead(seq!(opt(&WS), python_literal("="))), opt(seq!(opt(&WS), &TYPE_COMMENT))),
//         seq!(&single_target, opt(&WS), &augassign, opt(&WS), choice!(&yield_expr, &star_expressions))
//     )).into()
// }
//
// fn compound_stmt() -> Combinator {
//     tag("compound_stmt", choice!(
//         seq!(lookahead(seq!(choice!(python_literal("def"), python_literal("@"), python_literal("async")), opt(&WS))), &function_def),
//         seq!(lookahead(seq!(python_literal("if"), opt(&WS))), &if_stmt),
//         seq!(lookahead(seq!(choice!(python_literal("class"), python_literal("@")), opt(&WS))), &class_def),
//         seq!(lookahead(seq!(choice!(python_literal("with"), python_literal("async")), opt(&WS))), &with_stmt),
//         seq!(lookahead(seq!(choice!(python_literal("for"), python_literal("async")), opt(&WS))), &for_stmt),
//         seq!(lookahead(seq!(python_literal("try"), opt(&WS))), &try_stmt),
//         seq!(lookahead(seq!(python_literal("while"), opt(&WS))), &while_stmt),
//         &match_stmt
//     )).into()
// }
//
// fn simple_stmt() -> Combinator {
//     cached(tag("simple_stmt", choice!(
//         &assignment,
//         seq!(lookahead(seq!(python_literal("type"), opt(&WS))), &type_alias),
//         &star_expressions,
//         seq!(lookahead(seq!(python_literal("return"), opt(&WS))), &return_stmt),
//         seq!(lookahead(seq!(choice!(python_literal("import"), python_literal("from")), opt(&WS))), &import_stmt),
//         seq!(lookahead(seq!(python_literal("raise"), opt(&WS))), &raise_stmt),
//         python_literal("pass"),
//         seq!(lookahead(seq!(python_literal("del"), opt(&WS))), &del_stmt),
//         seq!(lookahead(seq!(python_literal("yield"), opt(&WS))), &yield_stmt),
//         seq!(lookahead(seq!(python_literal("assert"), opt(&WS))), &assert_stmt),
//         python_literal("break"),
//         python_literal("continue"),
//         seq!(lookahead(seq!(python_literal("global"), opt(&WS))), &global_stmt),
//         seq!(lookahead(seq!(python_literal("nonlocal"), opt(&WS))), &nonlocal_stmt)
//     ))).into()
// }
//
// fn simple_stmts() -> Combinator {
//     tag("simple_stmts", seq!(&simple_stmt, opt(&WS), choice!(seq!(negative_lookahead(seq!(python_literal(";"), opt(&WS))), &NEWLINE), seq!(opt(seq!(python_literal(";"), opt(&WS), &simple_stmt, opt(repeat1(seq!(opt(&WS), python_literal(";"), opt(&WS), &simple_stmt))), opt(&WS))), opt(seq!(python_literal(";"), opt(&WS))), &NEWLINE)))).into()
// }
//
// fn statement_newline() -> Combinator {
//     tag("statement_newline", choice!(
//         seq!(&compound_stmt, opt(&WS), &NEWLINE),
//         &simple_stmts,
//         &NEWLINE,
//         &ENDMARKER
//     )).into()
// }
//
// fn statement() -> Combinator {
//     tag("statement", choice!(
//         &compound_stmt,
//         &simple_stmts
//     )).into()
// }
//
// fn statements() -> Combinator {
//     tag("statements", seq!(&statement, opt(repeat1(seq!(opt(&WS), &statement))))).into()
// }
//
// fn func_type() -> Combinator {
//     tag("func_type", seq!(
//         python_literal("("),
//          opt(seq!(opt(&WS), &type_expressions)),
//          opt(&WS),
//          python_literal(")"),
//          opt(&WS),
//          python_literal("->"),
//          opt(&WS),
//          &expression,
//          opt(seq!(opt(&WS), &NEWLINE, opt(repeat1(seq!(opt(&WS), &NEWLINE))))),
//          opt(&WS),
//          &ENDMARKER
//     )).into()
// }
//
// fn eval() -> Combinator {
//     tag("eval", seq!(&expressions, opt(seq!(opt(&WS), &NEWLINE, opt(repeat1(seq!(opt(&WS), &NEWLINE))))), opt(&WS), &ENDMARKER)).into()
// }
//
// fn interactive() -> Combinator {
//     tag("interactive", &statement_newline).into()
// }
//
// fn file() -> Combinator {
//     tag("file", seq!(opt(seq!(&statements, opt(&WS))), &ENDMARKER)).into()
// }
//
//
// pub fn python_file() -> Combinator {
//
//     cache_context(tag("main", seq!(opt(&NEWLINE), &file))).compile()
// }
