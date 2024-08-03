use std::rc::Rc;
use crate::{cache_context, cached, cache_first_context, cache_first, symbol, Symbol, Choice, deferred, Combinator, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, forward_decls, forward_ref, Repeat1, Seq, tag, Compile, lookahead, negative_lookahead};
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

pub fn python_file() -> Combinator {
    use super::python_tokenizer as token;
    let WS = symbol(cached(tag("WS", seq!(forbid_follows_check_not(Forbidden::WS as usize), token::WS().compile(), forbid_follows(&[Forbidden::DEDENT as usize, Forbidden::INDENT as usize, Forbidden::NEWLINE as usize])))));
    let NAME = symbol(cached(tag("NAME", seq!(forbid_follows_check_not(Forbidden::NAME as usize), token::NAME().compile(), forbid_follows(&[Forbidden::NAME as usize, Forbidden::NUMBER as usize])))));
    let TYPE_COMMENT = symbol(cached(tag("TYPE_COMMENT", seq!(forbid_follows_clear(), token::TYPE_COMMENT().compile()))));
    let FSTRING_START = symbol(cached(tag("FSTRING_START", seq!(token::FSTRING_START().compile(), forbid_follows(&[Forbidden::WS as usize])))));
    let FSTRING_MIDDLE = symbol(cached(tag("FSTRING_MIDDLE", seq!(forbid_follows_check_not(Forbidden::FSTRING_MIDDLE as usize), token::FSTRING_MIDDLE().compile(), forbid_follows(&[Forbidden::FSTRING_MIDDLE as usize, Forbidden::WS as usize])))));
    let FSTRING_END = symbol(cached(tag("FSTRING_END", seq!(forbid_follows_clear(), token::FSTRING_END().compile()))));
    let NUMBER = symbol(cached(tag("NUMBER", seq!(forbid_follows_check_not(Forbidden::NUMBER as usize), token::NUMBER().compile(), forbid_follows(&[Forbidden::NUMBER as usize])))));
    let STRING = symbol(cached(tag("STRING", seq!(forbid_follows_clear(), token::STRING().compile()))));
    let NEWLINE = symbol(cached(tag("NEWLINE", seq!(forbid_follows_check_not(Forbidden::NEWLINE as usize), token::NEWLINE().compile(), forbid_follows(&[Forbidden::WS as usize])))));
    let INDENT = symbol(cached(tag("INDENT", seq!(forbid_follows_check_not(Forbidden::INDENT as usize), token::INDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))));
    let DEDENT = symbol(cached(tag("DEDENT", seq!(forbid_follows_check_not(Forbidden::DEDENT as usize), token::DEDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))));
    let ENDMARKER = symbol(cached(tag("ENDMARKER", seq!(forbid_follows_clear(), token::ENDMARKER().compile()))));

    forward_decls!(expression_without_invalid, func_type_comment, type_expressions, del_t_atom, del_target, del_targets, t_lookahead, t_primary, single_subscript_attribute_target, single_target, star_atom, target_with_star_atom, star_target, star_targets_tuple_seq, star_targets_list_seq, star_targets, kwarg_or_double_starred, kwarg_or_starred, starred_expression, kwargs, args, arguments, dictcomp, genexp, setcomp, listcomp, for_if_clause, for_if_clauses, kvpair, double_starred_kvpair, double_starred_kvpairs, dict, set, tuple, list, strings, string, fstring, fstring_format_spec, fstring_full_format_spec, fstring_conversion, fstring_replacement_field, fstring_middle, lambda_param, lambda_param_maybe_default, lambda_param_with_default, lambda_param_no_default, lambda_kwds, lambda_star_etc, lambda_slash_with_default, lambda_slash_no_default, lambda_parameters, lambda_params, lambdef, group, atom, slice, slices, primary, await_primary, power, factor, term, sum, shift_expr, bitwise_and, bitwise_xor, bitwise_or, is_bitwise_or, isnot_bitwise_or, in_bitwise_or, notin_bitwise_or, gt_bitwise_or, gte_bitwise_or, lt_bitwise_or, lte_bitwise_or, noteq_bitwise_or, eq_bitwise_or, compare_op_bitwise_or_pair, comparison, inversion, conjunction, disjunction, named_expression, assignment_expression, star_named_expression, star_named_expressions, star_expression, star_expressions, yield_expr, expression, expressions, type_param_starred_default, type_param_default, type_param_bound, type_param, type_param_seq, type_params, type_alias, keyword_pattern, keyword_patterns, positional_patterns, class_pattern, double_star_pattern, key_value_pattern, items_pattern, mapping_pattern, star_pattern, maybe_star_pattern, maybe_sequence_pattern, open_sequence_pattern, sequence_pattern, group_pattern, name_or_attr, attr, value_pattern, wildcard_pattern, pattern_capture_target, capture_pattern, imaginary_number, real_number, signed_real_number, signed_number, complex_number, literal_expr, literal_pattern, closed_pattern, or_pattern, as_pattern, pattern, patterns, guard, case_block, subject_expr, match_stmt, finally_block, except_star_block, except_block, try_stmt, with_item, with_stmt, for_stmt, while_stmt, else_block, elif_stmt, if_stmt, default, star_annotation, annotation, param_star_annotation, param, param_maybe_default, param_with_default, param_no_default_star_annotation, param_no_default, kwds, star_etc, slash_with_default, slash_no_default, parameters, params, function_def_raw, function_def, class_def_raw, class_def, decorators, block, dotted_name, dotted_as_name, dotted_as_names, import_from_as_name, import_from_as_names, import_from_targets, import_from, import_name, import_stmt, assert_stmt, yield_stmt, del_stmt, nonlocal_stmt, global_stmt, raise_stmt, return_stmt, augassign, annotated_rhs, assignment, compound_stmt, simple_stmt, simple_stmts, statement_newline, statement, statements, func_type, eval, interactive, file, );
    let expression_without_invalid = expression_without_invalid.set(tag("expression_without_invalid", crate::choice!(
        seq!(&conjunction, opt(seq!(opt(&WS), python_literal("or"), opt(&WS), opt(seq!(&WS, opt(&WS))), &conjunction, opt(repeat1(seq!(opt(&WS), python_literal("or"), opt(&WS), opt(seq!(&WS, opt(&WS))), &conjunction))))), opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
        seq!(python_literal("lambda"), opt(seq!(opt(&WS), &lambda_params)), opt(&WS), python_literal(":"), opt(&WS), &expression)
    )));
    let func_type_comment = func_type_comment.set(tag("func_type_comment", crate::choice!(
        seq!(&NEWLINE, opt(&WS), &TYPE_COMMENT, lookahead(seq!(&NEWLINE, &INDENT))),
        &TYPE_COMMENT
    )));
    let type_expressions = type_expressions.set(tag("type_expressions", crate::choice!(
        seq!(crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &expression))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), crate::choice!(seq!(python_literal("*"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), python_literal("**"), opt(&WS), &expression))), seq!(python_literal("**"), opt(&WS), &expression))))),
        seq!(python_literal("*"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), python_literal("**"), opt(&WS), &expression))),
        seq!(python_literal("**"), opt(&WS), &expression)
    )));
    let del_t_atom = del_t_atom.set(tag("del_t_atom", crate::choice!(
        &NAME,
        seq!(python_literal("("), opt(&WS), crate::choice!(seq!(&del_target, opt(&WS), python_literal(")")), seq!(opt(seq!(&del_targets, opt(&WS))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(opt(&WS), &del_targets)), opt(&WS), python_literal("]"))
    )));
    let del_target = del_target.set(cached(tag("del_target", crate::choice!(
        seq!(crate::choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(crate::choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), crate::choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), crate::choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), crate::choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead), opt(seq!(opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), opt(&WS), opt(seq!(&WS, opt(&WS))), &slices, opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal("]"), lookahead(&t_lookahead)), seq!(&genexp, lookahead(&t_lookahead)), seq!(python_literal("("), opt(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &arguments)), opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal(")"), lookahead(&t_lookahead))), opt(repeat1(seq!(opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), opt(&WS), opt(seq!(&WS, opt(&WS))), &slices, opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal("]"), lookahead(&t_lookahead)), seq!(&genexp, lookahead(&t_lookahead)), seq!(python_literal("("), opt(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &arguments)), opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal(")"), lookahead(&t_lookahead)))))))), opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), &NAME, negative_lookahead(&t_lookahead)), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), negative_lookahead(&t_lookahead)))),
        &del_t_atom
    ))));
    let del_targets = del_targets.set(tag("del_targets", seq!(&del_target, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &del_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &del_target))))), opt(seq!(opt(&WS), python_literal(","))))));
    let t_lookahead = t_lookahead.set(tag("t_lookahead", crate::choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    )));
    let t_primary = t_primary.set(tag("t_primary", seq!(crate::choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(crate::choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), crate::choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), crate::choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), crate::choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead), opt(seq!(opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), lookahead(&t_lookahead)), seq!(&genexp, lookahead(&t_lookahead)), seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")"), lookahead(&t_lookahead))), opt(repeat1(seq!(opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), lookahead(&t_lookahead)), seq!(&genexp, lookahead(&t_lookahead)), seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")"), lookahead(&t_lookahead)))))))))));
    let single_subscript_attribute_target = single_subscript_attribute_target.set(tag("single_subscript_attribute_target", seq!(&t_primary, opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), &NAME, negative_lookahead(&t_lookahead)), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), negative_lookahead(&t_lookahead))))));
    let single_target = single_target.set(tag("single_target", crate::choice!(
        &single_subscript_attribute_target,
        &NAME,
        seq!(python_literal("("), opt(&WS), &single_target, opt(&WS), python_literal(")"))
    )));
    let star_atom = star_atom.set(tag("star_atom", crate::choice!(
        &NAME,
        seq!(python_literal("("), opt(&WS), crate::choice!(seq!(&target_with_star_atom, opt(&WS), python_literal(")")), seq!(opt(seq!(&star_targets_tuple_seq, opt(&WS))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(opt(&WS), &star_targets_list_seq)), opt(&WS), python_literal("]"))
    )));
    let target_with_star_atom = target_with_star_atom.set(cached(tag("target_with_star_atom", crate::choice!(
        seq!(&t_primary, opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), &NAME, negative_lookahead(&t_lookahead)), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"), negative_lookahead(&t_lookahead)))),
        &star_atom
    ))));
    let star_target = star_target.set(cached(tag("star_target", crate::choice!(
        seq!(python_literal("*"), negative_lookahead(python_literal("*")), opt(&WS), &star_target),
        &target_with_star_atom
    ))));
    let star_targets_tuple_seq = star_targets_tuple_seq.set(tag("star_targets_tuple_seq", seq!(&star_target, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))), opt(seq!(opt(&WS), python_literal(","))))))));
    let star_targets_list_seq = star_targets_list_seq.set(tag("star_targets_list_seq", seq!(&star_target, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))))), opt(seq!(opt(&WS), python_literal(","))))));
    let star_targets = star_targets.set(tag("star_targets", seq!(&star_target, crate::choice!(negative_lookahead(python_literal(",")), seq!(opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))))), opt(seq!(opt(&WS), python_literal(","))))))));
    let kwarg_or_double_starred = kwarg_or_double_starred.set(tag("kwarg_or_double_starred", crate::choice!(
        seq!(&NAME, opt(&WS), python_literal("="), opt(&WS), &expression),
        seq!(python_literal("**"), opt(&WS), &expression)
    )));
    let kwarg_or_starred = kwarg_or_starred.set(tag("kwarg_or_starred", crate::choice!(
        seq!(&NAME, opt(&WS), python_literal("="), opt(&WS), &expression),
        seq!(python_literal("*"), opt(&WS), &expression)
    )));
    let starred_expression = starred_expression.set(tag("starred_expression", seq!(python_literal("*"), opt(&WS), &expression)));
    let kwargs = kwargs.set(tag("kwargs", crate::choice!(
        seq!(&kwarg_or_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_starred))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred)))))))),
        seq!(&kwarg_or_double_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred))))))
    )));
    let args = args.set(tag("args", crate::choice!(
        seq!(crate::choice!(&starred_expression, seq!(crate::choice!(seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), crate::choice!(&starred_expression, seq!(crate::choice!(seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), crate::choice!(&starred_expression, seq!(crate::choice!(seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("="))))))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwargs))),
        &kwargs
    )));
    let arguments = arguments.set(cached(tag("arguments", seq!(&args, opt(seq!(opt(&WS), python_literal(","))), lookahead(python_literal(")"))))));
    let dictcomp = dictcomp.set(tag("dictcomp", seq!(
        python_literal("{"),
         opt(&WS),
         &kvpair,
         opt(&WS),
         &for_if_clauses,
         opt(&WS),
         python_literal("}")
    )));
    let genexp = genexp.set(tag("genexp", seq!(
        python_literal("("),
         opt(&WS),
         crate::choice!(&assignment_expression, seq!(&expression, negative_lookahead(python_literal(":=")))),
         opt(&WS),
         &for_if_clauses,
         opt(&WS),
         python_literal(")")
    )));
    let setcomp = setcomp.set(tag("setcomp", seq!(
        python_literal("{"),
         opt(&WS),
         &named_expression,
         opt(&WS),
         &for_if_clauses,
         opt(&WS),
         python_literal("}")
    )));
    let listcomp = listcomp.set(tag("listcomp", seq!(
        python_literal("["),
         opt(&WS),
         &named_expression,
         opt(&WS),
         &for_if_clauses,
         opt(&WS),
         python_literal("]")
    )));
    let for_if_clause = for_if_clause.set(tag("for_if_clause", crate::choice!(
        seq!(python_literal("async"), opt(&WS), python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(repeat1(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction)))))),
        seq!(python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(repeat1(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction))))))
    )));
    let for_if_clauses = for_if_clauses.set(tag("for_if_clauses", seq!(&for_if_clause, opt(repeat1(seq!(opt(&WS), &for_if_clause))))));
    let kvpair = kvpair.set(tag("kvpair", seq!(
        crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &expression
    )));
    let double_starred_kvpair = double_starred_kvpair.set(tag("double_starred_kvpair", crate::choice!(
        seq!(python_literal("**"), opt(&WS), &bitwise_or),
        &kvpair
    )));
    let double_starred_kvpairs = double_starred_kvpairs.set(tag("double_starred_kvpairs", seq!(&double_starred_kvpair, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &double_starred_kvpair, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &double_starred_kvpair))))), opt(seq!(opt(&WS), python_literal(","))))));
    let dict = dict.set(tag("dict", seq!(python_literal("{"), opt(seq!(opt(&WS), &double_starred_kvpairs)), opt(&WS), python_literal("}"))));
    let set = set.set(tag("set", seq!(
        python_literal("{"),
         opt(&WS),
         &star_named_expressions,
         opt(&WS),
         python_literal("}")
    )));
    let tuple = tuple.set(tag("tuple", seq!(python_literal("("), opt(seq!(opt(&WS), &star_named_expression, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_named_expressions)))), opt(&WS), python_literal(")"))));
    let list = list.set(tag("list", seq!(python_literal("["), opt(seq!(opt(&WS), &star_named_expressions)), opt(&WS), python_literal("]"))));
    let strings = strings.set(cached(tag("strings", seq!(crate::choice!(seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END), &STRING), opt(repeat1(seq!(opt(&WS), crate::choice!(seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END), &STRING))))))));
    let string = string.set(tag("string", &STRING));
    let fstring = fstring.set(tag("fstring", seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END)));
    let fstring_format_spec = fstring_format_spec.set(tag("fstring_format_spec", crate::choice!(
        &FSTRING_MIDDLE,
        seq!(python_literal("{"), opt(&WS), &annotated_rhs, opt(seq!(opt(&WS), python_literal("="))), opt(seq!(opt(&WS), &fstring_conversion)), opt(seq!(opt(&WS), &fstring_full_format_spec)), opt(&WS), python_literal("}"))
    )));
    let fstring_full_format_spec = fstring_full_format_spec.set(tag("fstring_full_format_spec", seq!(python_literal(":"), opt(seq!(opt(&WS), &fstring_format_spec, opt(repeat1(seq!(opt(&WS), &fstring_format_spec))))))));
    let fstring_conversion = fstring_conversion.set(tag("fstring_conversion", seq!(python_literal("!"), opt(&WS), &NAME)));
    let fstring_replacement_field = fstring_replacement_field.set(tag("fstring_replacement_field", seq!(
        python_literal("{"),
         opt(&WS),
         &annotated_rhs,
         opt(seq!(opt(&WS), python_literal("="))),
         opt(seq!(opt(&WS), &fstring_conversion)),
         opt(seq!(opt(&WS), &fstring_full_format_spec)),
         opt(&WS),
         python_literal("}")
    )));
    let fstring_middle = fstring_middle.set(tag("fstring_middle", crate::choice!(
        &fstring_replacement_field,
        &FSTRING_MIDDLE
    )));
    let lambda_param = lambda_param.set(tag("lambda_param", &NAME));
    let lambda_param_maybe_default = lambda_param_maybe_default.set(tag("lambda_param_maybe_default", seq!(&lambda_param, opt(seq!(opt(&WS), &default)), crate::choice!(seq!(opt(&WS), python_literal(",")), lookahead(python_literal(":"))))));
    let lambda_param_with_default = lambda_param_with_default.set(tag("lambda_param_with_default", seq!(&lambda_param, opt(&WS), &default, crate::choice!(seq!(opt(&WS), python_literal(",")), lookahead(python_literal(":"))))));
    let lambda_param_no_default = lambda_param_no_default.set(tag("lambda_param_no_default", seq!(&lambda_param, crate::choice!(seq!(opt(&WS), python_literal(",")), lookahead(python_literal(":"))))));
    let lambda_kwds = lambda_kwds.set(tag("lambda_kwds", seq!(python_literal("**"), opt(&WS), &lambda_param_no_default)));
    let lambda_star_etc = lambda_star_etc.set(tag("lambda_star_etc", crate::choice!(
        seq!(python_literal("*"), opt(&WS), crate::choice!(seq!(&lambda_param_no_default, opt(seq!(opt(&WS), &lambda_param_maybe_default, opt(repeat1(seq!(opt(&WS), &lambda_param_maybe_default))))), opt(seq!(opt(&WS), &lambda_kwds))), seq!(python_literal(","), opt(&WS), &lambda_param_maybe_default, opt(repeat1(seq!(opt(&WS), &lambda_param_maybe_default))), opt(seq!(opt(&WS), &lambda_kwds))))),
        &lambda_kwds
    )));
    let lambda_slash_with_default = lambda_slash_with_default.set(tag("lambda_slash_with_default", seq!(
        opt(seq!(&lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))), opt(&WS))),
         &lambda_param_with_default,
         opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))),
         opt(&WS),
         python_literal("/"),
         crate::choice!(seq!(opt(&WS), python_literal(",")), lookahead(python_literal(":")))
    )));
    let lambda_slash_no_default = lambda_slash_no_default.set(tag("lambda_slash_no_default", seq!(
        &lambda_param_no_default,
         opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))),
         opt(&WS),
         python_literal("/"),
         crate::choice!(seq!(opt(&WS), python_literal(",")), lookahead(python_literal(":")))
    )));
    let lambda_parameters = lambda_parameters.set(tag("lambda_parameters", crate::choice!(
        seq!(&lambda_slash_no_default, opt(seq!(opt(&WS), &lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))))), opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
        seq!(&lambda_slash_with_default, opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
        seq!(&lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))), opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
        seq!(&lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))), opt(seq!(opt(&WS), &lambda_star_etc))),
        &lambda_star_etc
    )));
    let lambda_params = lambda_params.set(tag("lambda_params", &lambda_parameters));
    let lambdef = lambdef.set(tag("lambdef", seq!(
        python_literal("lambda"),
         opt(seq!(opt(&WS), &lambda_params)),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &expression
    )));
    let group = group.set(tag("group", seq!(
        python_literal("("),
         opt(&WS),
         crate::choice!(&yield_expr, &named_expression),
         opt(&WS),
         python_literal(")")
    )));
    let atom = atom.set(tag("atom", crate::choice!(
        &NAME,
        python_literal("True"),
        python_literal("False"),
        python_literal("None"),
        seq!(lookahead(crate::choice!(&STRING, &FSTRING_START)), &strings),
        &NUMBER,
        seq!(lookahead(python_literal("(")), crate::choice!(&tuple, &group, &genexp)),
        seq!(lookahead(python_literal("[")), crate::choice!(&list, &listcomp)),
        seq!(lookahead(python_literal("{")), crate::choice!(&dict, &set, &dictcomp, &setcomp)),
        python_literal("...")
    )));
    let slice = slice.set(tag("slice", crate::choice!(
        seq!(opt(seq!(crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), opt(&WS))), python_literal(":"), opt(seq!(opt(&WS), &expression)), opt(seq!(opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &expression))))),
        crate::choice!(seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(python_literal(":="))))
    )));
    let slices = slices.set(tag("slices", crate::choice!(
        seq!(&slice, negative_lookahead(python_literal(","))),
        seq!(crate::choice!(&slice, &starred_expression), opt(seq!(opt(&WS), python_literal(","), opt(&WS), crate::choice!(&slice, &starred_expression), opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), crate::choice!(&slice, &starred_expression)))))), opt(seq!(opt(&WS), python_literal(","))))
    )));
    let primary = primary.set(tag("primary", seq!(&atom, opt(seq!(opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), &NAME), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")")), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"))), opt(repeat1(seq!(opt(&WS), crate::choice!(seq!(python_literal("."), opt(&WS), &NAME), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")")), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]")))))))))));
    let await_primary = await_primary.set(cached(tag("await_primary", crate::choice!(
        seq!(python_literal("await"), opt(&WS), &primary),
        &primary
    ))));
    let power = power.set(tag("power", seq!(&await_primary, opt(seq!(opt(&WS), python_literal("**"), opt(&WS), &factor)))));
    let factor = factor.set(cached(tag("factor", crate::choice!(
        seq!(python_literal("+"), opt(&WS), &factor),
        seq!(python_literal("-"), opt(&WS), &factor),
        seq!(python_literal("~"), opt(&WS), &factor),
        &power
    ))));
    let term = term.set(tag("term", seq!(&factor, opt(seq!(opt(&WS), crate::choice!(seq!(python_literal("*"), opt(&WS), &factor), seq!(python_literal("/"), opt(&WS), &factor), seq!(python_literal("//"), opt(&WS), &factor), seq!(python_literal("%"), opt(&WS), &factor), seq!(python_literal("@"), opt(&WS), &factor)), opt(repeat1(seq!(opt(&WS), crate::choice!(seq!(python_literal("*"), opt(&WS), &factor), seq!(python_literal("/"), opt(&WS), &factor), seq!(python_literal("//"), opt(&WS), &factor), seq!(python_literal("%"), opt(&WS), &factor), seq!(python_literal("@"), opt(&WS), &factor))))))))));
    let sum = sum.set(tag("sum", seq!(&term, opt(seq!(opt(&WS), crate::choice!(seq!(python_literal("+"), opt(&WS), &term), seq!(python_literal("-"), opt(&WS), &term)), opt(repeat1(seq!(opt(&WS), crate::choice!(seq!(python_literal("+"), opt(&WS), &term), seq!(python_literal("-"), opt(&WS), &term))))))))));
    let shift_expr = shift_expr.set(tag("shift_expr", seq!(&sum, opt(seq!(opt(&WS), crate::choice!(seq!(python_literal("<<"), opt(&WS), &sum), seq!(python_literal(">>"), opt(&WS), &sum)), opt(repeat1(seq!(opt(&WS), crate::choice!(seq!(python_literal("<<"), opt(&WS), &sum), seq!(python_literal(">>"), opt(&WS), &sum))))))))));
    let bitwise_and = bitwise_and.set(tag("bitwise_and", seq!(&shift_expr, opt(seq!(opt(&WS), python_literal("&"), opt(&WS), &shift_expr, opt(repeat1(seq!(opt(&WS), python_literal("&"), opt(&WS), &shift_expr))))))));
    let bitwise_xor = bitwise_xor.set(tag("bitwise_xor", seq!(&bitwise_and, opt(seq!(opt(&WS), python_literal("^"), opt(&WS), &bitwise_and, opt(repeat1(seq!(opt(&WS), python_literal("^"), opt(&WS), &bitwise_and))))))));
    let bitwise_or = bitwise_or.set(tag("bitwise_or", seq!(&bitwise_xor, opt(seq!(opt(&WS), python_literal("|"), opt(&WS), &bitwise_xor, opt(repeat1(seq!(opt(&WS), python_literal("|"), opt(&WS), &bitwise_xor))))))));
    let is_bitwise_or = is_bitwise_or.set(tag("is_bitwise_or", seq!(python_literal("is"), opt(&WS), &bitwise_or)));
    let isnot_bitwise_or = isnot_bitwise_or.set(tag("isnot_bitwise_or", seq!(
        python_literal("is"),
         opt(&WS),
         python_literal("not"),
         opt(&WS),
         &bitwise_or
    )));
    let in_bitwise_or = in_bitwise_or.set(tag("in_bitwise_or", seq!(python_literal("in"), opt(&WS), &bitwise_or)));
    let notin_bitwise_or = notin_bitwise_or.set(tag("notin_bitwise_or", seq!(
        python_literal("not"),
         opt(&WS),
         python_literal("in"),
         opt(&WS),
         &bitwise_or
    )));
    let gt_bitwise_or = gt_bitwise_or.set(tag("gt_bitwise_or", seq!(python_literal(">"), opt(&WS), &bitwise_or)));
    let gte_bitwise_or = gte_bitwise_or.set(tag("gte_bitwise_or", seq!(python_literal(">="), opt(&WS), &bitwise_or)));
    let lt_bitwise_or = lt_bitwise_or.set(tag("lt_bitwise_or", seq!(python_literal("<"), opt(&WS), &bitwise_or)));
    let lte_bitwise_or = lte_bitwise_or.set(tag("lte_bitwise_or", seq!(python_literal("<="), opt(&WS), &bitwise_or)));
    let noteq_bitwise_or = noteq_bitwise_or.set(tag("noteq_bitwise_or", seq!(python_literal("!="), opt(&WS), &bitwise_or)));
    let eq_bitwise_or = eq_bitwise_or.set(tag("eq_bitwise_or", seq!(python_literal("=="), opt(&WS), &bitwise_or)));
    let compare_op_bitwise_or_pair = compare_op_bitwise_or_pair.set(tag("compare_op_bitwise_or_pair", crate::choice!(
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
    )));
    let comparison = comparison.set(tag("comparison", seq!(&bitwise_or, opt(seq!(opt(&WS), &compare_op_bitwise_or_pair, opt(repeat1(seq!(opt(&WS), &compare_op_bitwise_or_pair))))))));
    let inversion = inversion.set(cached(tag("inversion", crate::choice!(
        seq!(python_literal("not"), opt(&WS), &inversion),
        &comparison
    ))));
    let conjunction = conjunction.set(cached(tag("conjunction", seq!(&inversion, opt(seq!(opt(&WS), python_literal("and"), opt(&WS), &inversion, opt(repeat1(seq!(opt(&WS), python_literal("and"), opt(&WS), &inversion)))))))));
    let disjunction = disjunction.set(cached(tag("disjunction", seq!(&conjunction, opt(seq!(opt(&WS), python_literal("or"), opt(&WS), &conjunction, opt(repeat1(seq!(opt(&WS), python_literal("or"), opt(&WS), &conjunction)))))))));
    let named_expression = named_expression.set(tag("named_expression", crate::choice!(
        seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression),
        seq!(crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), negative_lookahead(python_literal(":=")))
    )));
    let assignment_expression = assignment_expression.set(tag("assignment_expression", seq!(
        &NAME,
         opt(&WS),
         python_literal(":="),
         opt(&WS),
         &expression
    )));
    let star_named_expression = star_named_expression.set(tag("star_named_expression", crate::choice!(
        seq!(python_literal("*"), opt(&WS), &bitwise_or),
        &named_expression
    )));
    let star_named_expressions = star_named_expressions.set(tag("star_named_expressions", seq!(&star_named_expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_named_expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_named_expression))))), opt(seq!(opt(&WS), python_literal(","))))));
    let star_expression = star_expression.set(cached(tag("star_expression", crate::choice!(
        seq!(python_literal("*"), opt(&WS), &bitwise_or),
        crate::choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef)
    ))));
    let star_expressions = star_expressions.set(tag("star_expressions", seq!(&star_expression, opt(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_expression))), opt(seq!(opt(&WS), python_literal(","))))))))));
    let yield_expr = yield_expr.set(tag("yield_expr", seq!(python_literal("yield"), crate::choice!(seq!(opt(&WS), python_literal("from"), opt(&WS), &expression), opt(seq!(opt(&WS), &star_expressions))))));
    let expression = expression.set(cached(tag("expression", crate::choice!(
        seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
        &lambdef
    ))));
    let expressions = expressions.set(tag("expressions", seq!(&expression, opt(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &expression))), opt(seq!(opt(&WS), python_literal(","))))))))));
    let type_param_starred_default = type_param_starred_default.set(tag("type_param_starred_default", seq!(python_literal("="), opt(&WS), &star_expression)));
    let type_param_default = type_param_default.set(tag("type_param_default", seq!(python_literal("="), opt(&WS), &expression)));
    let type_param_bound = type_param_bound.set(tag("type_param_bound", seq!(python_literal(":"), opt(&WS), &expression)));
    let type_param = type_param.set(cached(tag("type_param", crate::choice!(
        seq!(&NAME, opt(seq!(opt(&WS), &type_param_bound)), opt(seq!(opt(&WS), &type_param_default))),
        seq!(python_literal("*"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_param_starred_default))),
        seq!(python_literal("**"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_param_default)))
    ))));
    let type_param_seq = type_param_seq.set(tag("type_param_seq", seq!(&type_param, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &type_param, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &type_param))))), opt(seq!(opt(&WS), python_literal(","))))));
    let type_params = type_params.set(tag("type_params", seq!(
        python_literal("["),
         opt(&WS),
         &type_param_seq,
         opt(&WS),
         python_literal("]")
    )));
    let type_alias = type_alias.set(tag("type_alias", seq!(
        python_literal("type"),
         opt(&WS),
         &NAME,
         opt(seq!(opt(&WS), &type_params)),
         opt(&WS),
         python_literal("="),
         opt(&WS),
         &expression
    )));
    let keyword_pattern = keyword_pattern.set(tag("keyword_pattern", seq!(
        &NAME,
         opt(&WS),
         python_literal("="),
         opt(&WS),
         &pattern
    )));
    let keyword_patterns = keyword_patterns.set(tag("keyword_patterns", seq!(&keyword_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &keyword_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &keyword_pattern))))))));
    let positional_patterns = positional_patterns.set(tag("positional_patterns", seq!(crate::choice!(&as_pattern, &or_pattern), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &pattern))))))));
    let class_pattern = class_pattern.set(tag("class_pattern", seq!(
        &NAME,
         opt(seq!(opt(&WS), python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME))))),
         opt(&WS),
         python_literal("("),
         opt(&WS),
         crate::choice!(python_literal(")"), seq!(&positional_patterns, opt(&WS), crate::choice!(seq!(opt(seq!(python_literal(","), opt(&WS))), python_literal(")")), seq!(python_literal(","), opt(&WS), &keyword_patterns, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")))), seq!(&keyword_patterns, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")))
    )));
    let double_star_pattern = double_star_pattern.set(tag("double_star_pattern", seq!(python_literal("**"), opt(&WS), &pattern_capture_target)));
    let key_value_pattern = key_value_pattern.set(tag("key_value_pattern", seq!(
        crate::choice!(crate::choice!(seq!(&signed_number, negative_lookahead(crate::choice!(python_literal("+"), python_literal("-")))), &complex_number, &strings, python_literal("None"), python_literal("True"), python_literal("False")), seq!(&name_or_attr, opt(&WS), python_literal("."), opt(&WS), &NAME)),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &pattern
    )));
    let items_pattern = items_pattern.set(tag("items_pattern", seq!(&key_value_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &key_value_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &key_value_pattern))))))));
    let mapping_pattern = mapping_pattern.set(tag("mapping_pattern", seq!(python_literal("{"), opt(&WS), crate::choice!(python_literal("}"), seq!(&double_star_pattern, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal("}")), seq!(&items_pattern, opt(&WS), crate::choice!(seq!(python_literal(","), opt(&WS), &double_star_pattern, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal("}")), seq!(opt(seq!(python_literal(","), opt(&WS))), python_literal("}"))))))));
    let star_pattern = star_pattern.set(cached(tag("star_pattern", seq!(python_literal("*"), opt(&WS), crate::choice!(&pattern_capture_target, &wildcard_pattern)))));
    let maybe_star_pattern = maybe_star_pattern.set(tag("maybe_star_pattern", crate::choice!(
        &star_pattern,
        crate::choice!(&as_pattern, &or_pattern)
    )));
    let maybe_sequence_pattern = maybe_sequence_pattern.set(tag("maybe_sequence_pattern", seq!(&maybe_star_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &maybe_star_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &maybe_star_pattern))))), opt(seq!(opt(&WS), python_literal(","))))));
    let open_sequence_pattern = open_sequence_pattern.set(tag("open_sequence_pattern", seq!(&maybe_star_pattern, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &maybe_sequence_pattern)))));
    let sequence_pattern = sequence_pattern.set(tag("sequence_pattern", crate::choice!(
        seq!(python_literal("["), opt(seq!(opt(&WS), &maybe_sequence_pattern)), opt(&WS), python_literal("]")),
        seq!(python_literal("("), opt(seq!(opt(&WS), &open_sequence_pattern)), opt(&WS), python_literal(")"))
    )));
    let group_pattern = group_pattern.set(tag("group_pattern", seq!(
        python_literal("("),
         opt(&WS),
         &pattern,
         opt(&WS),
         python_literal(")")
    )));
    let name_or_attr = name_or_attr.set(tag("name_or_attr", seq!(&NAME, opt(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME))))))));
    let attr = attr.set(tag("attr", seq!(
        &name_or_attr,
         opt(&WS),
         python_literal("."),
         opt(&WS),
         &NAME
    )));
    let value_pattern = value_pattern.set(tag("value_pattern", seq!(&attr, negative_lookahead(crate::choice!(python_literal("."), python_literal("("), python_literal("="))))));
    let wildcard_pattern = wildcard_pattern.set(tag("wildcard_pattern", python_literal("_")));
    let pattern_capture_target = pattern_capture_target.set(tag("pattern_capture_target", seq!(negative_lookahead(python_literal("_")), &NAME, negative_lookahead(crate::choice!(python_literal("."), python_literal("("), python_literal("="))))));
    let capture_pattern = capture_pattern.set(tag("capture_pattern", &pattern_capture_target));
    let imaginary_number = imaginary_number.set(tag("imaginary_number", &NUMBER));
    let real_number = real_number.set(tag("real_number", &NUMBER));
    let signed_real_number = signed_real_number.set(tag("signed_real_number", crate::choice!(
        &real_number,
        seq!(python_literal("-"), opt(&WS), &real_number)
    )));
    let signed_number = signed_number.set(tag("signed_number", crate::choice!(
        &NUMBER,
        seq!(python_literal("-"), opt(&WS), &NUMBER)
    )));
    let complex_number = complex_number.set(tag("complex_number", seq!(&signed_real_number, opt(&WS), crate::choice!(seq!(python_literal("+"), opt(&WS), &imaginary_number), seq!(python_literal("-"), opt(&WS), &imaginary_number)))));
    let literal_expr = literal_expr.set(tag("literal_expr", crate::choice!(
        seq!(&signed_number, negative_lookahead(crate::choice!(python_literal("+"), python_literal("-")))),
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )));
    let literal_pattern = literal_pattern.set(tag("literal_pattern", crate::choice!(
        seq!(&signed_number, negative_lookahead(crate::choice!(python_literal("+"), python_literal("-")))),
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )));
    let closed_pattern = closed_pattern.set(cached(tag("closed_pattern", crate::choice!(
        &literal_pattern,
        &capture_pattern,
        &wildcard_pattern,
        &value_pattern,
        &group_pattern,
        &sequence_pattern,
        &mapping_pattern,
        &class_pattern
    ))));
    let or_pattern = or_pattern.set(tag("or_pattern", seq!(&closed_pattern, opt(seq!(opt(&WS), python_literal("|"), opt(&WS), &closed_pattern, opt(repeat1(seq!(opt(&WS), python_literal("|"), opt(&WS), &closed_pattern))))))));
    let as_pattern = as_pattern.set(tag("as_pattern", seq!(
        &or_pattern,
         opt(&WS),
         python_literal("as"),
         opt(&WS),
         &pattern_capture_target
    )));
    let pattern = pattern.set(tag("pattern", crate::choice!(
        &as_pattern,
        &or_pattern
    )));
    let patterns = patterns.set(tag("patterns", crate::choice!(
        &open_sequence_pattern,
        &pattern
    )));
    let guard = guard.set(tag("guard", seq!(python_literal("if"), opt(&WS), &named_expression)));
    let case_block = case_block.set(tag("case_block", seq!(
        python_literal("case"),
         opt(&WS),
         &patterns,
         opt(seq!(opt(&WS), &guard)),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block
    )));
    let subject_expr = subject_expr.set(tag("subject_expr", crate::choice!(
        seq!(&star_named_expression, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_named_expressions))),
        &named_expression
    )));
    let match_stmt = match_stmt.set(tag("match_stmt", seq!(
        python_literal("match"),
         opt(&WS),
         &subject_expr,
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &NEWLINE,
         opt(&WS),
         &INDENT,
         opt(&WS),
         &case_block,
         opt(repeat1(seq!(opt(&WS), &case_block))),
         opt(&WS),
         &DEDENT
    )));
    let finally_block = finally_block.set(tag("finally_block", seq!(
        python_literal("finally"),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block
    )));
    let except_star_block = except_star_block.set(tag("except_star_block", seq!(
        python_literal("except"),
         opt(&WS),
         python_literal("*"),
         opt(&WS),
         &expression,
         opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block
    )));
    let except_block = except_block.set(tag("except_block", seq!(python_literal("except"), opt(&WS), crate::choice!(seq!(&expression, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)), opt(&WS), python_literal(":"), opt(&WS), &block), seq!(python_literal(":"), opt(&WS), &block)))));
    let try_stmt = try_stmt.set(tag("try_stmt", seq!(
        python_literal("try"),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block,
         opt(&WS),
         crate::choice!(&finally_block, seq!(&except_block, opt(repeat1(seq!(opt(&WS), &except_block))), opt(seq!(opt(&WS), &else_block)), opt(seq!(opt(&WS), &finally_block))), seq!(&except_star_block, opt(repeat1(seq!(opt(&WS), &except_star_block))), opt(seq!(opt(&WS), &else_block)), opt(seq!(opt(&WS), &finally_block))))
    )));
    let with_item = with_item.set(tag("with_item", seq!(&expression, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &star_target, lookahead(crate::choice!(python_literal(","), python_literal(")"), python_literal(":"))))))));
    let with_stmt = with_stmt.set(tag("with_stmt", crate::choice!(
        seq!(python_literal("with"), opt(&WS), crate::choice!(seq!(python_literal("("), opt(&WS), &with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")"), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block), seq!(&with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block))),
        seq!(python_literal("async"), opt(&WS), python_literal("with"), opt(&WS), crate::choice!(seq!(python_literal("("), opt(&WS), &with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")"), opt(&WS), python_literal(":"), opt(&WS), &block), seq!(&with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block)))
    )));
    let for_stmt = for_stmt.set(tag("for_stmt", crate::choice!(
        seq!(python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &star_expressions, opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block, opt(seq!(opt(&WS), &else_block))),
        seq!(python_literal("async"), opt(&WS), python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &star_expressions, opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block, opt(seq!(opt(&WS), &else_block)))
    )));
    let while_stmt = while_stmt.set(tag("while_stmt", seq!(
        python_literal("while"),
         opt(&WS),
         &named_expression,
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block,
         opt(seq!(opt(&WS), &else_block))
    )));
    let else_block = else_block.set(tag("else_block", seq!(
        python_literal("else"),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block
    )));
    let elif_stmt = elif_stmt.set(tag("elif_stmt", seq!(
        python_literal("elif"),
         opt(&WS),
         &named_expression,
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block,
         crate::choice!(seq!(opt(&WS), &elif_stmt), opt(seq!(opt(&WS), &else_block)))
    )));
    let if_stmt = if_stmt.set(tag("if_stmt", seq!(
        python_literal("if"),
         opt(&WS),
         &named_expression,
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block,
         crate::choice!(seq!(opt(&WS), &elif_stmt), opt(seq!(opt(&WS), &else_block)))
    )));
    let default = default.set(tag("default", seq!(python_literal("="), opt(&WS), &expression)));
    let star_annotation = star_annotation.set(tag("star_annotation", seq!(python_literal(":"), opt(&WS), &star_expression)));
    let annotation = annotation.set(tag("annotation", seq!(python_literal(":"), opt(&WS), &expression)));
    let param_star_annotation = param_star_annotation.set(tag("param_star_annotation", seq!(&NAME, opt(&WS), &star_annotation)));
    let param = param.set(tag("param", seq!(&NAME, opt(seq!(opt(&WS), &annotation)))));
    let param_maybe_default = param_maybe_default.set(tag("param_maybe_default", seq!(&param, opt(seq!(opt(&WS), &default)), crate::choice!(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), seq!(opt(seq!(opt(&WS), &TYPE_COMMENT)), lookahead(python_literal(")")))))));
    let param_with_default = param_with_default.set(tag("param_with_default", seq!(&param, opt(&WS), &default, crate::choice!(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), seq!(opt(seq!(opt(&WS), &TYPE_COMMENT)), lookahead(python_literal(")")))))));
    let param_no_default_star_annotation = param_no_default_star_annotation.set(tag("param_no_default_star_annotation", seq!(&param_star_annotation, crate::choice!(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), seq!(opt(seq!(opt(&WS), &TYPE_COMMENT)), lookahead(python_literal(")")))))));
    let param_no_default = param_no_default.set(tag("param_no_default", seq!(&param, crate::choice!(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), seq!(opt(seq!(opt(&WS), &TYPE_COMMENT)), lookahead(python_literal(")")))))));
    let kwds = kwds.set(tag("kwds", seq!(python_literal("**"), opt(&WS), &param_no_default)));
    let star_etc = star_etc.set(tag("star_etc", crate::choice!(
        seq!(python_literal("*"), opt(&WS), crate::choice!(seq!(&param_no_default, opt(seq!(opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))))), opt(seq!(opt(&WS), &kwds))), seq!(&param_no_default_star_annotation, opt(seq!(opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))))), opt(seq!(opt(&WS), &kwds))), seq!(python_literal(","), opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))), opt(seq!(opt(&WS), &kwds))))),
        &kwds
    )));
    let slash_with_default = slash_with_default.set(tag("slash_with_default", seq!(
        opt(seq!(&param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))), opt(&WS))),
         &param_with_default,
         opt(repeat1(seq!(opt(&WS), &param_with_default))),
         opt(&WS),
         python_literal("/"),
         crate::choice!(seq!(opt(&WS), python_literal(",")), lookahead(python_literal(")")))
    )));
    let slash_no_default = slash_no_default.set(tag("slash_no_default", seq!(
        &param_no_default,
         opt(repeat1(seq!(opt(&WS), &param_no_default))),
         opt(&WS),
         python_literal("/"),
         crate::choice!(seq!(opt(&WS), python_literal(",")), lookahead(python_literal(")")))
    )));
    let parameters = parameters.set(tag("parameters", crate::choice!(
        seq!(&slash_no_default, opt(seq!(opt(&WS), &param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))))), opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
        seq!(&slash_with_default, opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
        seq!(&param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))), opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
        seq!(&param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))), opt(seq!(opt(&WS), &star_etc))),
        &star_etc
    )));
    let params = params.set(tag("params", &parameters));
    let function_def_raw = function_def_raw.set(tag("function_def_raw", crate::choice!(
        seq!(python_literal("def"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_params)), opt(&WS), python_literal("("), opt(seq!(opt(&WS), &params)), opt(&WS), python_literal(")"), opt(seq!(opt(&WS), python_literal("->"), opt(&WS), &expression)), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &func_type_comment)), opt(&WS), &block),
        seq!(python_literal("async"), opt(&WS), python_literal("def"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_params)), opt(&WS), python_literal("("), opt(seq!(opt(&WS), &params)), opt(&WS), python_literal(")"), opt(seq!(opt(&WS), python_literal("->"), opt(&WS), &expression)), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &func_type_comment)), opt(&WS), &block)
    )));
    let function_def = function_def.set(tag("function_def", crate::choice!(
        seq!(python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &NEWLINE))))), opt(&WS), &function_def_raw),
        &function_def_raw
    )));
    let class_def_raw = class_def_raw.set(tag("class_def_raw", seq!(
        python_literal("class"),
         opt(&WS),
         &NAME,
         opt(seq!(opt(&WS), &type_params)),
         opt(seq!(opt(&WS), python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")"))),
         opt(&WS),
         python_literal(":"),
         opt(&WS),
         &block
    )));
    let class_def = class_def.set(tag("class_def", crate::choice!(
        seq!(python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), &NEWLINE))))), opt(&WS), &class_def_raw),
        &class_def_raw
    )));
    let decorators = decorators.set(tag("decorators", seq!(
        python_literal("@"),
         opt(&WS),
         &named_expression,
         opt(&WS),
         &NEWLINE,
         opt(seq!(opt(&WS), python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE)))))
    )));
    let block = block.set(cached(tag("block", crate::choice!(
        seq!(&NEWLINE, opt(&WS), &INDENT, opt(&WS), &statements, opt(&WS), &DEDENT),
        seq!(&simple_stmt, opt(&WS), crate::choice!(seq!(negative_lookahead(python_literal(";")), &NEWLINE), seq!(opt(seq!(python_literal(";"), opt(&WS), opt(seq!(&WS, opt(&WS))), &simple_stmt, opt(repeat1(seq!(opt(&WS), python_literal(";"), opt(&WS), opt(seq!(&WS, opt(&WS))), &simple_stmt))), opt(&WS))), opt(seq!(python_literal(";"), opt(&WS))), &NEWLINE)))
    ))));
    let dotted_name = dotted_name.set(tag("dotted_name", seq!(&NAME, opt(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME))))))));
    let dotted_as_name = dotted_as_name.set(tag("dotted_as_name", seq!(&dotted_name, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)))));
    let dotted_as_names = dotted_as_names.set(tag("dotted_as_names", seq!(&dotted_as_name, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &dotted_as_name, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &dotted_as_name))))))));
    let import_from_as_name = import_from_as_name.set(tag("import_from_as_name", seq!(&NAME, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)))));
    let import_from_as_names = import_from_as_names.set(tag("import_from_as_names", seq!(&import_from_as_name, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &import_from_as_name, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &import_from_as_name))))))));
    let import_from_targets = import_from_targets.set(tag("import_from_targets", crate::choice!(
        seq!(python_literal("("), opt(&WS), &import_from_as_names, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")),
        seq!(&import_from_as_names, negative_lookahead(python_literal(","))),
        python_literal("*")
    )));
    let import_from = import_from.set(tag("import_from", seq!(python_literal("from"), opt(&WS), crate::choice!(seq!(opt(seq!(crate::choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(&WS), crate::choice!(python_literal("."), python_literal("..."))))), opt(&WS))), &dotted_name, opt(&WS), python_literal("import"), opt(&WS), &import_from_targets), seq!(crate::choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(&WS), crate::choice!(python_literal("."), python_literal("..."))))), opt(&WS), python_literal("import"), opt(&WS), &import_from_targets)))));
    let import_name = import_name.set(tag("import_name", seq!(python_literal("import"), opt(&WS), &dotted_as_names)));
    let import_stmt = import_stmt.set(tag("import_stmt", crate::choice!(
        &import_name,
        &import_from
    )));
    let assert_stmt = assert_stmt.set(tag("assert_stmt", seq!(python_literal("assert"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &expression)))));
    let yield_stmt = yield_stmt.set(tag("yield_stmt", &yield_expr));
    let del_stmt = del_stmt.set(tag("del_stmt", seq!(python_literal("del"), opt(&WS), &del_targets, lookahead(crate::choice!(python_literal(";"), &NEWLINE)))));
    let nonlocal_stmt = nonlocal_stmt.set(tag("nonlocal_stmt", seq!(python_literal("nonlocal"), opt(&WS), &NAME, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME))))))));
    let global_stmt = global_stmt.set(tag("global_stmt", seq!(python_literal("global"), opt(&WS), &NAME, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME))))))));
    let raise_stmt = raise_stmt.set(tag("raise_stmt", seq!(python_literal("raise"), opt(seq!(opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("from"), opt(&WS), &expression)))))));
    let return_stmt = return_stmt.set(tag("return_stmt", seq!(python_literal("return"), opt(seq!(opt(&WS), &star_expressions)))));
    let augassign = augassign.set(tag("augassign", crate::choice!(
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
    )));
    let annotated_rhs = annotated_rhs.set(tag("annotated_rhs", crate::choice!(
        &yield_expr,
        &star_expressions
    )));
    let assignment = assignment.set(tag("assignment", crate::choice!(
        seq!(&NAME, opt(&WS), python_literal(":"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("="), opt(&WS), &annotated_rhs))),
        seq!(crate::choice!(seq!(python_literal("("), opt(&WS), &single_target, opt(&WS), python_literal(")")), &single_subscript_attribute_target), opt(&WS), python_literal(":"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("="), opt(&WS), &annotated_rhs))),
        seq!(&star_targets, opt(&WS), python_literal("="), opt(seq!(opt(&WS), &star_targets, opt(&WS), python_literal("="), opt(repeat1(seq!(opt(&WS), &star_targets, opt(&WS), python_literal("=")))))), opt(&WS), crate::choice!(&yield_expr, &star_expressions), negative_lookahead(python_literal("=")), opt(seq!(opt(&WS), &TYPE_COMMENT))),
        seq!(&single_target, opt(&WS), &augassign, opt(&WS), crate::choice!(&yield_expr, &star_expressions))
    )));
    let compound_stmt = compound_stmt.set(tag("compound_stmt", crate::choice!(
        seq!(lookahead(crate::choice!(python_literal("def"), python_literal("@"), python_literal("async"))), &function_def),
        seq!(lookahead(python_literal("if")), &if_stmt),
        seq!(lookahead(crate::choice!(python_literal("class"), python_literal("@"))), &class_def),
        seq!(lookahead(crate::choice!(python_literal("with"), python_literal("async"))), &with_stmt),
        seq!(lookahead(crate::choice!(python_literal("for"), python_literal("async"))), &for_stmt),
        seq!(lookahead(python_literal("try")), &try_stmt),
        seq!(lookahead(python_literal("while")), &while_stmt),
        &match_stmt
    )));
    let simple_stmt = simple_stmt.set(cached(tag("simple_stmt", crate::choice!(
        &assignment,
        seq!(lookahead(python_literal("type")), &type_alias),
        &star_expressions,
        seq!(lookahead(python_literal("return")), &return_stmt),
        seq!(lookahead(crate::choice!(python_literal("import"), python_literal("from"))), &import_stmt),
        seq!(lookahead(python_literal("raise")), &raise_stmt),
        python_literal("pass"),
        seq!(lookahead(python_literal("del")), &del_stmt),
        seq!(lookahead(python_literal("yield")), &yield_stmt),
        seq!(lookahead(python_literal("assert")), &assert_stmt),
        python_literal("break"),
        python_literal("continue"),
        seq!(lookahead(python_literal("global")), &global_stmt),
        seq!(lookahead(python_literal("nonlocal")), &nonlocal_stmt)
    ))));
    let simple_stmts = simple_stmts.set(tag("simple_stmts", seq!(&simple_stmt, opt(&WS), crate::choice!(seq!(negative_lookahead(python_literal(";")), &NEWLINE), seq!(opt(seq!(python_literal(";"), opt(&WS), &simple_stmt, opt(repeat1(seq!(opt(&WS), python_literal(";"), opt(&WS), &simple_stmt))), opt(&WS))), opt(seq!(python_literal(";"), opt(&WS))), &NEWLINE)))));
    let statement_newline = statement_newline.set(tag("statement_newline", crate::choice!(
        seq!(&compound_stmt, opt(&WS), &NEWLINE),
        &simple_stmts,
        &NEWLINE,
        &ENDMARKER
    )));
    let statement = statement.set(tag("statement", crate::choice!(
        &compound_stmt,
        &simple_stmts
    )));
    let statements = statements.set(tag("statements", seq!(&statement, opt(repeat1(seq!(opt(&WS), &statement))))));
    let func_type = func_type.set(tag("func_type", seq!(
        python_literal("("),
         opt(seq!(opt(&WS), &type_expressions)),
         opt(&WS),
         python_literal(")"),
         opt(&WS),
         python_literal("->"),
         opt(&WS),
         &expression,
         opt(seq!(opt(&WS), &NEWLINE, opt(repeat1(seq!(opt(&WS), &NEWLINE))))),
         opt(&WS),
         &ENDMARKER
    )));
    let eval = eval.set(tag("eval", seq!(&expressions, opt(seq!(opt(&WS), &NEWLINE, opt(repeat1(seq!(opt(&WS), &NEWLINE))))), opt(&WS), &ENDMARKER)));
    let interactive = interactive.set(tag("interactive", &statement_newline));
    let file = file.set(tag("file", seq!(opt(seq!(&statements, opt(&WS))), &ENDMARKER)));


    cache_context(cache_first_context(seq!(opt(&NEWLINE), &file))).into()
}
