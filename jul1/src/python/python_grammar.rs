use std::rc::Rc;
use crate::{cache_context, cached, symbol, choice, Choice, Combinator, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, forward_decls, forward_ref, opt, Repeat1, seprep0, seprep1, Seq, tag, Compile};
use super::python_tokenizer::{WS, NAME, TYPE_COMMENT, FSTRING_START, FSTRING_MIDDLE, FSTRING_END, NUMBER, STRING, NEWLINE, INDENT, DEDENT, ENDMARKER};
use super::python_tokenizer::python_literal;
use crate::{seq, repeat0, repeat1};

pub fn python_file() -> Combinator {
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

    let WS = symbol(cached(tag("WS", seq!(forbid_follows_check_not(Forbidden::WS as usize), WS().compile(), forbid_follows(&[Forbidden::DEDENT as usize, Forbidden::INDENT as usize, Forbidden::NEWLINE as usize])))));
    let NAME = symbol(cached(tag("NAME", seq!(forbid_follows_check_not(Forbidden::NAME as usize), NAME().compile(), forbid_follows(&[Forbidden::NAME as usize, Forbidden::NUMBER as usize])))));
    let TYPE_COMMENT = symbol(cached(tag("TYPE_COMMENT", seq!(TYPE_COMMENT().compile()))));
    let FSTRING_START = symbol(cached(tag("FSTRING_START", seq!(FSTRING_START().compile(), forbid_follows(&[Forbidden::WS as usize])))));
    let FSTRING_MIDDLE = symbol(cached(tag("FSTRING_MIDDLE", seq!(forbid_follows_check_not(Forbidden::FSTRING_MIDDLE as usize), FSTRING_MIDDLE().compile(), forbid_follows(&[Forbidden::FSTRING_MIDDLE as usize, Forbidden::WS as usize])))));
    let FSTRING_END = symbol(cached(tag("FSTRING_END", seq!(FSTRING_END().compile()))));
    let NUMBER = symbol(cached(tag("NUMBER", seq!(forbid_follows_check_not(Forbidden::NUMBER as usize), NUMBER().compile(), forbid_follows(&[Forbidden::NUMBER as usize])))));
    let STRING = symbol(cached(tag("STRING", seq!(STRING().compile()))));
    let NEWLINE = symbol(cached(tag("NEWLINE", seq!(forbid_follows_check_not(Forbidden::NEWLINE as usize), NEWLINE().compile(), forbid_follows(&[Forbidden::WS as usize])))));
    let INDENT = symbol(cached(tag("INDENT", seq!(forbid_follows_check_not(Forbidden::INDENT as usize), INDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))));
    let DEDENT = symbol(cached(tag("DEDENT", seq!(forbid_follows_check_not(Forbidden::DEDENT as usize), DEDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))));
    let ENDMARKER = symbol(cached(tag("ENDMARKER", seq!(ENDMARKER().compile()))));

    forward_decls!(expression_without_invalid, func_type_comment, type_expressions, del_t_atom, del_target, del_targets, t_lookahead, t_primary, single_subscript_attribute_target, single_target, star_atom, target_with_star_atom, star_target, star_targets_tuple_seq, star_targets_list_seq, star_targets, kwarg_or_double_starred, kwarg_or_starred, starred_expression, kwargs, args, arguments, dictcomp, genexp, setcomp, listcomp, for_if_clause, for_if_clauses, kvpair, double_starred_kvpair, double_starred_kvpairs, dict, set, tuple, list, strings, string, fstring, fstring_format_spec, fstring_full_format_spec, fstring_conversion, fstring_replacement_field, fstring_middle, lambda_param, lambda_param_maybe_default, lambda_param_with_default, lambda_param_no_default, lambda_kwds, lambda_star_etc, lambda_slash_with_default, lambda_slash_no_default, lambda_parameters, lambda_params, lambdef, group, atom, slice, slices, primary, await_primary, power, factor, term, sum, shift_expr, bitwise_and, bitwise_xor, bitwise_or, is_bitwise_or, isnot_bitwise_or, in_bitwise_or, notin_bitwise_or, gt_bitwise_or, gte_bitwise_or, lt_bitwise_or, lte_bitwise_or, noteq_bitwise_or, eq_bitwise_or, compare_op_bitwise_or_pair, comparison, inversion, conjunction, disjunction, named_expression, assignment_expression, star_named_expression, star_named_expressions, star_expression, star_expressions, yield_expr, expression, expressions, type_param_starred_default, type_param_default, type_param_bound, type_param, type_param_seq, type_params, type_alias, keyword_pattern, keyword_patterns, positional_patterns, class_pattern, double_star_pattern, key_value_pattern, items_pattern, mapping_pattern, star_pattern, maybe_star_pattern, maybe_sequence_pattern, open_sequence_pattern, sequence_pattern, group_pattern, name_or_attr, attr, value_pattern, wildcard_pattern, pattern_capture_target, capture_pattern, imaginary_number, real_number, signed_real_number, signed_number, complex_number, literal_expr, literal_pattern, closed_pattern, or_pattern, as_pattern, pattern, patterns, guard, case_block, subject_expr, match_stmt, finally_block, except_star_block, except_block, try_stmt, with_item, with_stmt, for_stmt, while_stmt, else_block, elif_stmt, if_stmt, default, star_annotation, annotation, param_star_annotation, param, param_maybe_default, param_with_default, param_no_default_star_annotation, param_no_default, kwds, star_etc, slash_with_default, slash_no_default, parameters, params, function_def_raw, function_def, class_def_raw, class_def, decorators, block, dotted_name, dotted_as_name, dotted_as_names, import_from_as_name, import_from_as_names, import_from_targets, import_from, import_name, import_stmt, assert_stmt, yield_stmt, del_stmt, nonlocal_stmt, global_stmt, raise_stmt, return_stmt, augassign, annotated_rhs, assignment, compound_stmt, simple_stmt, simple_stmts, statement_newline, statement, statements, func_type, eval, interactive, file);

    let expression_without_invalid = expression_without_invalid.set(tag("expression_without_invalid", Combinator::from(choice!(
        seq!(&conjunction, opt(seq!(opt(&WS), python_literal("or"), opt(&WS), opt(seq!(&WS, opt(&WS))), &conjunction, opt(repeat1(seq!(opt(&WS), python_literal("or"), opt(&WS), opt(seq!(&WS, opt(&WS))), &conjunction))))), opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
        seq!(python_literal("lambda"), opt(seq!(opt(&WS), &lambda_params)), opt(&WS), python_literal(":"), opt(&WS), &expression)
    )).compile()));
    let func_type_comment = func_type_comment.set(tag("func_type_comment", Combinator::from(choice!(
        seq!(&NEWLINE, opt(&WS), &TYPE_COMMENT),
        &TYPE_COMMENT
    )).compile()));
    let type_expressions = type_expressions.set(tag("type_expressions", Combinator::from(choice!(
        seq!(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &expression))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(seq!(python_literal("*"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), python_literal("**"), opt(&WS), &expression))), seq!(python_literal("**"), opt(&WS), &expression))))),
        seq!(python_literal("*"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), python_literal("**"), opt(&WS), &expression))),
        seq!(python_literal("**"), opt(&WS), &expression)
    )).compile()));
    let del_t_atom = del_t_atom.set(tag("del_t_atom", Combinator::from(choice!(
        &NAME,
        seq!(python_literal("("), opt(&WS), choice!(seq!(&del_target, opt(&WS), python_literal(")")), seq!(opt(seq!(&del_targets, opt(&WS))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(opt(&WS), &del_targets)), opt(&WS), python_literal("]"))
    )).compile()));
    let del_target = del_target.set(cached(tag("del_target", Combinator::from(choice!(
        seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), &strings, &NUMBER, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp, python_literal("...")), opt(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME), seq!(python_literal("["), opt(&WS), opt(seq!(&WS, opt(&WS))), &slices, opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal("]")), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &arguments)), opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal(")"))), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME), seq!(python_literal("["), opt(&WS), opt(seq!(&WS, opt(&WS))), &slices, opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal("]")), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), opt(seq!(&WS, opt(&WS))), &arguments)), opt(&WS), opt(seq!(&WS, opt(&WS))), python_literal(")")))))))), opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]")))),
        &del_t_atom
    )).compile())));
    let del_targets = del_targets.set(tag("del_targets", Combinator::from(seq!(&del_target, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &del_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &del_target))))), opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let t_lookahead = t_lookahead.set(tag("t_lookahead", Combinator::from(choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    )).compile()));
    let t_primary = t_primary.set(tag("t_primary", Combinator::from(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), &strings, &NUMBER, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp, python_literal("...")), opt(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]")), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")"))), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]")), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")")))))))))).compile()));
    let single_subscript_attribute_target = single_subscript_attribute_target.set(tag("single_subscript_attribute_target", Combinator::from(seq!(&t_primary, opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"))))).compile()));
    let single_target = single_target.set(tag("single_target", Combinator::from(choice!(
        &single_subscript_attribute_target,
        &NAME,
        seq!(python_literal("("), opt(&WS), &single_target, opt(&WS), python_literal(")"))
    )).compile()));
    let star_atom = star_atom.set(tag("star_atom", Combinator::from(choice!(
        &NAME,
        seq!(python_literal("("), opt(&WS), choice!(seq!(&target_with_star_atom, opt(&WS), python_literal(")")), seq!(opt(seq!(&star_targets_tuple_seq, opt(&WS))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(opt(&WS), &star_targets_list_seq)), opt(&WS), python_literal("]"))
    )).compile()));
    let target_with_star_atom = target_with_star_atom.set(cached(tag("target_with_star_atom", Combinator::from(choice!(
        seq!(&t_primary, opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]")))),
        &star_atom
    )).compile())));
    let star_target = star_target.set(cached(tag("star_target", Combinator::from(choice!(
        seq!(python_literal("*"), opt(&WS), &star_target),
        &target_with_star_atom
    )).compile())));
    let star_targets_tuple_seq = star_targets_tuple_seq.set(tag("star_targets_tuple_seq", Combinator::from(seq!(&star_target, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))), opt(seq!(opt(&WS), python_literal(","))))))).compile()));
    let star_targets_list_seq = star_targets_list_seq.set(tag("star_targets_list_seq", Combinator::from(seq!(&star_target, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))))), opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let star_targets = star_targets.set(tag("star_targets", Combinator::from(seq!(&star_target, opt(seq!(opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_target))))), opt(seq!(opt(&WS), python_literal(","))))))).compile()));
    let kwarg_or_double_starred = kwarg_or_double_starred.set(tag("kwarg_or_double_starred", Combinator::from(choice!(
        seq!(&NAME, opt(&WS), python_literal("="), opt(&WS), &expression),
        seq!(python_literal("**"), opt(&WS), &expression)
    )).compile()));
    let kwarg_or_starred = kwarg_or_starred.set(tag("kwarg_or_starred", Combinator::from(choice!(
        seq!(&NAME, opt(&WS), python_literal("="), opt(&WS), &expression),
        seq!(python_literal("*"), opt(&WS), &expression)
    )).compile()));
    let starred_expression = starred_expression.set(tag("starred_expression", Combinator::from(seq!(python_literal("*"), opt(&WS), &expression)).compile()));
    let kwargs = kwargs.set(tag("kwargs", Combinator::from(choice!(
        seq!(&kwarg_or_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_starred))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred)))))))),
        seq!(&kwarg_or_double_starred, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &kwarg_or_double_starred))))))
    )).compile()));
    let args = args.set(tag("args", Combinator::from(choice!(
        seq!(choice!(&starred_expression, seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), opt(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(&starred_expression, seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef), opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(&starred_expression, seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression), seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef)))))), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &kwargs))),
        &kwargs
    )).compile()));
    let arguments = arguments.set(cached(tag("arguments", Combinator::from(seq!(&args, opt(seq!(opt(&WS), python_literal(","))))).compile())));
    let dictcomp = dictcomp.set(tag("dictcomp", Combinator::from(seq!(
        python_literal("{"),
        opt(&WS),
        &kvpair,
        opt(&WS),
        &for_if_clauses,
        opt(&WS),
        python_literal("}")
    )).compile()));
    let genexp = genexp.set(tag("genexp", Combinator::from(seq!(
        python_literal("("),
        opt(&WS),
        choice!(&assignment_expression, &expression),
        opt(&WS),
        &for_if_clauses,
        opt(&WS),
        python_literal(")")
    )).compile()));
    let setcomp = setcomp.set(tag("setcomp", Combinator::from(seq!(
        python_literal("{"),
        opt(&WS),
        &named_expression,
        opt(&WS),
        &for_if_clauses,
        opt(&WS),
        python_literal("}")
    )).compile()));
    let listcomp = listcomp.set(tag("listcomp", Combinator::from(seq!(
        python_literal("["),
        opt(&WS),
        &named_expression,
        opt(&WS),
        &for_if_clauses,
        opt(&WS),
        python_literal("]")
    )).compile()));
    let for_if_clause = for_if_clause.set(tag("for_if_clause", Combinator::from(choice!(
        seq!(python_literal("async"), opt(&WS), python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(repeat1(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction)))))),
        seq!(python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(repeat1(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction))))))
    )).compile()));
    let for_if_clauses = for_if_clauses.set(tag("for_if_clauses", Combinator::from(seq!(&for_if_clause, opt(repeat1(seq!(opt(&WS), &for_if_clause))))).compile()));
    let kvpair = kvpair.set(tag("kvpair", Combinator::from(seq!(
        choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))), &lambdef),
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &expression
    )).compile()));
    let double_starred_kvpair = double_starred_kvpair.set(tag("double_starred_kvpair", Combinator::from(choice!(
        seq!(python_literal("**"), opt(&WS), &bitwise_or),
        &kvpair
    )).compile()));
    let double_starred_kvpairs = double_starred_kvpairs.set(tag("double_starred_kvpairs", Combinator::from(seq!(&double_starred_kvpair, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &double_starred_kvpair, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &double_starred_kvpair))))), opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let dict = dict.set(tag("dict", Combinator::from(seq!(python_literal("{"), opt(seq!(opt(&WS), &double_starred_kvpairs)), opt(&WS), python_literal("}"))).compile()));
    let set = set.set(tag("set", Combinator::from(seq!(
        python_literal("{"),
        opt(&WS),
        &star_named_expressions,
        opt(&WS),
        python_literal("}")
    )).compile()));
    let tuple = tuple.set(tag("tuple", Combinator::from(seq!(python_literal("("), opt(seq!(opt(&WS), &star_named_expression, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_named_expressions)))), opt(&WS), python_literal(")"))).compile()));
    let list = list.set(tag("list", Combinator::from(seq!(python_literal("["), opt(seq!(opt(&WS), &star_named_expressions)), opt(&WS), python_literal("]"))).compile()));
    let strings = strings.set(cached(tag("strings", Combinator::from(seq!(choice!(seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END), &STRING), opt(repeat1(seq!(opt(&WS), choice!(seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END), &STRING)))))).compile())));
    let string = string.set(tag("string", Combinator::from(&STRING).compile()));
    let fstring = fstring.set(tag("fstring", Combinator::from(seq!(&FSTRING_START, opt(seq!(opt(&WS), &fstring_middle, opt(repeat1(seq!(opt(&WS), &fstring_middle))))), opt(&WS), &FSTRING_END)).compile()));
    let fstring_format_spec = fstring_format_spec.set(tag("fstring_format_spec", Combinator::from(choice!(
        &FSTRING_MIDDLE,
        seq!(python_literal("{"), opt(&WS), &annotated_rhs, opt(seq!(opt(&WS), python_literal("="))), opt(seq!(opt(&WS), &fstring_conversion)), opt(seq!(opt(&WS), &fstring_full_format_spec)), opt(&WS), python_literal("}"))
    )).compile()));
    let fstring_full_format_spec = fstring_full_format_spec.set(tag("fstring_full_format_spec", Combinator::from(seq!(python_literal(":"), opt(seq!(opt(&WS), &fstring_format_spec, opt(repeat1(seq!(opt(&WS), &fstring_format_spec))))))).compile()));
    let fstring_conversion = fstring_conversion.set(tag("fstring_conversion", Combinator::from(seq!(python_literal("!"), opt(&WS), &NAME)).compile()));
    let fstring_replacement_field = fstring_replacement_field.set(tag("fstring_replacement_field", Combinator::from(seq!(
        python_literal("{"),
        opt(&WS),
        &annotated_rhs,
        opt(seq!(opt(&WS), python_literal("="))),
        opt(seq!(opt(&WS), &fstring_conversion)),
        opt(seq!(opt(&WS), &fstring_full_format_spec)),
        opt(&WS),
        python_literal("}")
    )).compile()));
    let fstring_middle = fstring_middle.set(tag("fstring_middle", Combinator::from(choice!(
        &fstring_replacement_field,
        &FSTRING_MIDDLE
    )).compile()));
    let lambda_param = lambda_param.set(tag("lambda_param", Combinator::from(&NAME).compile()));
    let lambda_param_maybe_default = lambda_param_maybe_default.set(tag("lambda_param_maybe_default", Combinator::from(seq!(&lambda_param, opt(seq!(opt(&WS), &default)), opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let lambda_param_with_default = lambda_param_with_default.set(tag("lambda_param_with_default", Combinator::from(seq!(&lambda_param, opt(&WS), &default, opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let lambda_param_no_default = lambda_param_no_default.set(tag("lambda_param_no_default", Combinator::from(seq!(&lambda_param, opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let lambda_kwds = lambda_kwds.set(tag("lambda_kwds", Combinator::from(seq!(python_literal("**"), opt(&WS), &lambda_param_no_default)).compile()));
    let lambda_star_etc = lambda_star_etc.set(tag("lambda_star_etc", Combinator::from(choice!(
        seq!(python_literal("*"), opt(&WS), choice!(seq!(&lambda_param_no_default, opt(seq!(opt(&WS), &lambda_param_maybe_default, opt(repeat1(seq!(opt(&WS), &lambda_param_maybe_default))))), opt(seq!(opt(&WS), &lambda_kwds))), seq!(python_literal(","), opt(&WS), &lambda_param_maybe_default, opt(repeat1(seq!(opt(&WS), &lambda_param_maybe_default))), opt(seq!(opt(&WS), &lambda_kwds))))),
        &lambda_kwds
    )).compile()));
    let lambda_slash_with_default = lambda_slash_with_default.set(tag("lambda_slash_with_default", Combinator::from(seq!(
        opt(seq!(&lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))), opt(&WS))),
        &lambda_param_with_default,
        opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))),
        opt(&WS),
        python_literal("/"),
        opt(seq!(opt(&WS), python_literal(",")))
    )).compile()));
    let lambda_slash_no_default = lambda_slash_no_default.set(tag("lambda_slash_no_default", Combinator::from(seq!(
        &lambda_param_no_default,
        opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))),
        opt(&WS),
        python_literal("/"),
        opt(seq!(opt(&WS), python_literal(",")))
    )).compile()));
    let lambda_parameters = lambda_parameters.set(tag("lambda_parameters", Combinator::from(choice!(
        seq!(&lambda_slash_no_default, opt(seq!(opt(&WS), &lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))))), opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
        seq!(&lambda_slash_with_default, opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
        seq!(&lambda_param_no_default, opt(repeat1(seq!(opt(&WS), &lambda_param_no_default))), opt(seq!(opt(&WS), &lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))))), opt(seq!(opt(&WS), &lambda_star_etc))),
        seq!(&lambda_param_with_default, opt(repeat1(seq!(opt(&WS), &lambda_param_with_default))), opt(seq!(opt(&WS), &lambda_star_etc))),
        &lambda_star_etc
    )).compile()));
    let lambda_params = lambda_params.set(tag("lambda_params", Combinator::from(&lambda_parameters).compile()));
    let lambdef = lambdef.set(tag("lambdef", Combinator::from(seq!(
        python_literal("lambda"),
        opt(seq!(opt(&WS), &lambda_params)),
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &expression
    )).compile()));
    let group = group.set(tag("group", Combinator::from(seq!(
        python_literal("("),
        opt(&WS),
        choice!(&yield_expr, &named_expression),
        opt(&WS),
        python_literal(")")
    )).compile()));
    let atom = atom.set(tag("atom", Combinator::from(choice!(
        &NAME,
        python_literal("True"),
        python_literal("False"),
        python_literal("None"),
        &strings,
        &NUMBER,
        &tuple,
        &group,
        &genexp,
        &list,
        &listcomp,
        &dict,
        &set,
        &dictcomp,
        &setcomp,
        python_literal("...")
    )).compile()));
    let slice = slice.set(tag("slice", Combinator::from(choice!(
        seq!(opt(choice!(seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression)), opt(&WS)), seq!(&lambdef, opt(&WS)))), python_literal(":"), opt(seq!(opt(&WS), &expression)), opt(seq!(opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &expression))))),
        seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression),
        seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
        &lambdef
    )).compile()));
    let slices = slices.set(tag("slices", Combinator::from(choice!(
        &slice,
        seq!(choice!(&slice, &starred_expression), opt(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(&slice, &starred_expression), opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), choice!(&slice, &starred_expression)))))), opt(seq!(opt(&WS), python_literal(","))))
    )).compile()));
    let primary = primary.set(tag("primary", Combinator::from(seq!(&atom, opt(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")")), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]"))), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("."), opt(&WS), &NAME), &genexp, seq!(python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")")), seq!(python_literal("["), opt(&WS), &slices, opt(&WS), python_literal("]")))))))))).compile()));
    let await_primary = await_primary.set(cached(tag("await_primary", Combinator::from(choice!(
        seq!(python_literal("await"), opt(&WS), &primary),
        &primary
    )).compile())));
    let power = power.set(tag("power", Combinator::from(seq!(&await_primary, opt(seq!(opt(&WS), python_literal("**"), opt(&WS), &factor)))).compile()));
    let factor = factor.set(cached(tag("factor", Combinator::from(choice!(
        seq!(python_literal("+"), opt(&WS), &factor),
        seq!(python_literal("-"), opt(&WS), &factor),
        seq!(python_literal("~"), opt(&WS), &factor),
        &power
    )).compile())));
    let term = term.set(tag("term", Combinator::from(seq!(&factor, opt(seq!(opt(&WS), choice!(seq!(python_literal("*"), opt(&WS), &factor), seq!(python_literal("/"), opt(&WS), &factor), seq!(python_literal("//"), opt(&WS), &factor), seq!(python_literal("%"), opt(&WS), &factor), seq!(python_literal("@"), opt(&WS), &factor)), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("*"), opt(&WS), &factor), seq!(python_literal("/"), opt(&WS), &factor), seq!(python_literal("//"), opt(&WS), &factor), seq!(python_literal("%"), opt(&WS), &factor), seq!(python_literal("@"), opt(&WS), &factor))))))))).compile()));
    let sum = sum.set(tag("sum", Combinator::from(seq!(&term, opt(seq!(opt(&WS), choice!(seq!(python_literal("+"), opt(&WS), &term), seq!(python_literal("-"), opt(&WS), &term)), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("+"), opt(&WS), &term), seq!(python_literal("-"), opt(&WS), &term))))))))).compile()));
    let shift_expr = shift_expr.set(tag("shift_expr", Combinator::from(seq!(&sum, opt(seq!(opt(&WS), choice!(seq!(python_literal("<<"), opt(&WS), &sum), seq!(python_literal(">>"), opt(&WS), &sum)), opt(repeat1(seq!(opt(&WS), choice!(seq!(python_literal("<<"), opt(&WS), &sum), seq!(python_literal(">>"), opt(&WS), &sum))))))))).compile()));
    let bitwise_and = bitwise_and.set(tag("bitwise_and", Combinator::from(seq!(&shift_expr, opt(seq!(opt(&WS), python_literal("&"), opt(&WS), &shift_expr, opt(repeat1(seq!(opt(&WS), python_literal("&"), opt(&WS), &shift_expr))))))).compile()));
    let bitwise_xor = bitwise_xor.set(tag("bitwise_xor", Combinator::from(seq!(&bitwise_and, opt(seq!(opt(&WS), python_literal("^"), opt(&WS), &bitwise_and, opt(repeat1(seq!(opt(&WS), python_literal("^"), opt(&WS), &bitwise_and))))))).compile()));
    let bitwise_or = bitwise_or.set(tag("bitwise_or", Combinator::from(seq!(&bitwise_xor, opt(seq!(opt(&WS), python_literal("|"), opt(&WS), &bitwise_xor, opt(repeat1(seq!(opt(&WS), python_literal("|"), opt(&WS), &bitwise_xor))))))).compile()));
    let is_bitwise_or = is_bitwise_or.set(tag("is_bitwise_or", Combinator::from(seq!(python_literal("is"), opt(&WS), &bitwise_or)).compile()));
    let isnot_bitwise_or = isnot_bitwise_or.set(tag("isnot_bitwise_or", Combinator::from(seq!(
        python_literal("is"),
        opt(&WS),
        python_literal("not"),
        opt(&WS),
        &bitwise_or
    )).compile()));
    let in_bitwise_or = in_bitwise_or.set(tag("in_bitwise_or", Combinator::from(seq!(python_literal("in"), opt(&WS), &bitwise_or)).compile()));
    let notin_bitwise_or = notin_bitwise_or.set(tag("notin_bitwise_or", Combinator::from(seq!(
        python_literal("not"),
        opt(&WS),
        python_literal("in"),
        opt(&WS),
        &bitwise_or
    )).compile()));
    let gt_bitwise_or = gt_bitwise_or.set(tag("gt_bitwise_or", Combinator::from(seq!(python_literal(">"), opt(&WS), &bitwise_or)).compile()));
    let gte_bitwise_or = gte_bitwise_or.set(tag("gte_bitwise_or", Combinator::from(seq!(python_literal(">="), opt(&WS), &bitwise_or)).compile()));
    let lt_bitwise_or = lt_bitwise_or.set(tag("lt_bitwise_or", Combinator::from(seq!(python_literal("<"), opt(&WS), &bitwise_or)).compile()));
    let lte_bitwise_or = lte_bitwise_or.set(tag("lte_bitwise_or", Combinator::from(seq!(python_literal("<="), opt(&WS), &bitwise_or)).compile()));
    let noteq_bitwise_or = noteq_bitwise_or.set(tag("noteq_bitwise_or", Combinator::from(seq!(python_literal("!="), opt(&WS), &bitwise_or)).compile()));
    let eq_bitwise_or = eq_bitwise_or.set(tag("eq_bitwise_or", Combinator::from(seq!(python_literal("=="), opt(&WS), &bitwise_or)).compile()));
    let compare_op_bitwise_or_pair = compare_op_bitwise_or_pair.set(tag("compare_op_bitwise_or_pair", Combinator::from(choice!(
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
    )).compile()));
    let comparison = comparison.set(tag("comparison", Combinator::from(seq!(&bitwise_or, opt(seq!(opt(&WS), &compare_op_bitwise_or_pair, opt(repeat1(seq!(opt(&WS), &compare_op_bitwise_or_pair))))))).compile()));
    let inversion = inversion.set(cached(tag("inversion", Combinator::from(choice!(
        seq!(python_literal("not"), opt(&WS), &inversion),
        &comparison
    )).compile())));
    let conjunction = conjunction.set(cached(tag("conjunction", Combinator::from(seq!(&inversion, opt(seq!(opt(&WS), python_literal("and"), opt(&WS), &inversion, opt(repeat1(seq!(opt(&WS), python_literal("and"), opt(&WS), &inversion))))))).compile())));
    let disjunction = disjunction.set(cached(tag("disjunction", Combinator::from(seq!(&conjunction, opt(seq!(opt(&WS), python_literal("or"), opt(&WS), &conjunction, opt(repeat1(seq!(opt(&WS), python_literal("or"), opt(&WS), &conjunction))))))).compile())));
    let named_expression = named_expression.set(tag("named_expression", Combinator::from(choice!(
        seq!(&NAME, opt(&WS), python_literal(":="), opt(&WS), &expression),
        seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
        &lambdef
    )).compile()));
    let assignment_expression = assignment_expression.set(tag("assignment_expression", Combinator::from(seq!(
        &NAME,
        opt(&WS),
        python_literal(":="),
        opt(&WS),
        &expression
    )).compile()));
    let star_named_expression = star_named_expression.set(tag("star_named_expression", Combinator::from(choice!(
        seq!(python_literal("*"), opt(&WS), &bitwise_or),
        &named_expression
    )).compile()));
    let star_named_expressions = star_named_expressions.set(tag("star_named_expressions", Combinator::from(seq!(&star_named_expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &star_named_expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_named_expression))))), opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let star_expression = star_expression.set(cached(tag("star_expression", Combinator::from(choice!(
        seq!(python_literal("*"), opt(&WS), &bitwise_or),
        seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
        &lambdef
    )).compile())));
    let star_expressions = star_expressions.set(tag("star_expressions", Combinator::from(seq!(&star_expression, opt(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &star_expression))), opt(seq!(opt(&WS), python_literal(","))))))))).compile()));
    let yield_expr = yield_expr.set(tag("yield_expr", Combinator::from(seq!(python_literal("yield"), opt(seq!(opt(&WS), choice!(seq!(python_literal("from"), opt(&WS), &expression), &star_expressions))))).compile()));
    let expression = expression.set(cached(tag("expression", Combinator::from(choice!(
        seq!(&disjunction, opt(seq!(opt(&WS), python_literal("if"), opt(&WS), &disjunction, opt(&WS), python_literal("else"), opt(&WS), &expression))),
        &lambdef
    )).compile())));
    let expressions = expressions.set(tag("expressions", Combinator::from(seq!(&expression, opt(seq!(opt(&WS), python_literal(","), opt(seq!(opt(&WS), &expression, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &expression))), opt(seq!(opt(&WS), python_literal(","))))))))).compile()));
    let type_param_starred_default = type_param_starred_default.set(tag("type_param_starred_default", Combinator::from(seq!(python_literal("="), opt(&WS), &star_expression)).compile()));
    let type_param_default = type_param_default.set(tag("type_param_default", Combinator::from(seq!(python_literal("="), opt(&WS), &expression)).compile()));
    let type_param_bound = type_param_bound.set(tag("type_param_bound", Combinator::from(seq!(python_literal(":"), opt(&WS), &expression)).compile()));
    let type_param = type_param.set(cached(tag("type_param", Combinator::from(choice!(
        seq!(&NAME, opt(seq!(opt(&WS), &type_param_bound)), opt(seq!(opt(&WS), &type_param_default))),
        seq!(python_literal("*"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_param_starred_default))),
        seq!(python_literal("**"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_param_default)))
    )).compile())));
    let type_param_seq = type_param_seq.set(tag("type_param_seq", Combinator::from(seq!(&type_param, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &type_param, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &type_param))))), opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let type_params = type_params.set(tag("type_params", Combinator::from(seq!(
        python_literal("["),
        opt(&WS),
        &type_param_seq,
        opt(&WS),
        python_literal("]")
    )).compile()));
    let type_alias = type_alias.set(tag("type_alias", Combinator::from(seq!(
        python_literal("type"),
        opt(&WS),
        &NAME,
        opt(seq!(opt(&WS), &type_params)),
        opt(&WS),
        python_literal("="),
        opt(&WS),
        &expression
    )).compile()));
    let keyword_pattern = keyword_pattern.set(tag("keyword_pattern", Combinator::from(seq!(
        &NAME,
        opt(&WS),
        python_literal("="),
        opt(&WS),
        &pattern
    )).compile()));
    let keyword_patterns = keyword_patterns.set(tag("keyword_patterns", Combinator::from(seq!(&keyword_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &keyword_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &keyword_pattern))))))).compile()));
    let positional_patterns = positional_patterns.set(tag("positional_patterns", Combinator::from(seq!(choice!(&as_pattern, &or_pattern), opt(seq!(opt(&WS), python_literal(","), opt(&WS), &pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &pattern))))))).compile()));
    let class_pattern = class_pattern.set(tag("class_pattern", Combinator::from(seq!(
        &NAME,
        opt(seq!(opt(&WS), python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), opt(seq!(&WS, opt(&WS))), &NAME))))),
        opt(&WS),
        python_literal("("),
        opt(&WS),
        choice!(python_literal(")"), seq!(&positional_patterns, opt(&WS), choice!(seq!(opt(seq!(python_literal(","), opt(&WS))), python_literal(")")), seq!(python_literal(","), opt(&WS), &keyword_patterns, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")))), seq!(&keyword_patterns, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")))
    )).compile()));
    let double_star_pattern = double_star_pattern.set(tag("double_star_pattern", Combinator::from(seq!(python_literal("**"), opt(&WS), &pattern_capture_target)).compile()));
    let key_value_pattern = key_value_pattern.set(tag("key_value_pattern", Combinator::from(seq!(
        choice!(&signed_number, &complex_number, &strings, python_literal("None"), python_literal("True"), python_literal("False"), seq!(&name_or_attr, opt(&WS), python_literal("."), opt(&WS), &NAME)),
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &pattern
    )).compile()));
    let items_pattern = items_pattern.set(tag("items_pattern", Combinator::from(seq!(&key_value_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &key_value_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &key_value_pattern))))))).compile()));
    let mapping_pattern = mapping_pattern.set(tag("mapping_pattern", Combinator::from(seq!(python_literal("{"), opt(&WS), choice!(python_literal("}"), seq!(&double_star_pattern, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal("}")), seq!(&items_pattern, opt(&WS), choice!(seq!(python_literal(","), opt(&WS), &double_star_pattern, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal("}")), seq!(opt(seq!(python_literal(","), opt(&WS))), python_literal("}"))))))).compile()));
    let star_pattern = star_pattern.set(cached(tag("star_pattern", Combinator::from(seq!(python_literal("*"), opt(&WS), choice!(&pattern_capture_target, &wildcard_pattern))).compile())));
    let maybe_star_pattern = maybe_star_pattern.set(tag("maybe_star_pattern", Combinator::from(choice!(
        &star_pattern,
        &as_pattern,
        &or_pattern
    )).compile()));
    let maybe_sequence_pattern = maybe_sequence_pattern.set(tag("maybe_sequence_pattern", Combinator::from(seq!(&maybe_star_pattern, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &maybe_star_pattern, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &maybe_star_pattern))))), opt(seq!(opt(&WS), python_literal(","))))).compile()));
    let open_sequence_pattern = open_sequence_pattern.set(tag("open_sequence_pattern", Combinator::from(seq!(&maybe_star_pattern, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &maybe_sequence_pattern)))).compile()));
    let sequence_pattern = sequence_pattern.set(tag("sequence_pattern", Combinator::from(choice!(
        seq!(python_literal("["), opt(seq!(opt(&WS), &maybe_sequence_pattern)), opt(&WS), python_literal("]")),
        seq!(python_literal("("), opt(seq!(opt(&WS), &open_sequence_pattern)), opt(&WS), python_literal(")"))
    )).compile()));
    let group_pattern = group_pattern.set(tag("group_pattern", Combinator::from(seq!(
        python_literal("("),
        opt(&WS),
        &pattern,
        opt(&WS),
        python_literal(")")
    )).compile()));
    let name_or_attr = name_or_attr.set(tag("name_or_attr", Combinator::from(seq!(&NAME, opt(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME))))))).compile()));
    let attr = attr.set(tag("attr", Combinator::from(seq!(
        &name_or_attr,
        opt(&WS),
        python_literal("."),
        opt(&WS),
        &NAME
    )).compile()));
    let value_pattern = value_pattern.set(tag("value_pattern", Combinator::from(&attr).compile()));
    let wildcard_pattern = wildcard_pattern.set(tag("wildcard_pattern", Combinator::from(python_literal("_")).compile()));
    let pattern_capture_target = pattern_capture_target.set(tag("pattern_capture_target", Combinator::from(&NAME).compile()));
    let capture_pattern = capture_pattern.set(tag("capture_pattern", Combinator::from(&pattern_capture_target).compile()));
    let imaginary_number = imaginary_number.set(tag("imaginary_number", Combinator::from(&NUMBER).compile()));
    let real_number = real_number.set(tag("real_number", Combinator::from(&NUMBER).compile()));
    let signed_real_number = signed_real_number.set(tag("signed_real_number", Combinator::from(choice!(
        &real_number,
        seq!(python_literal("-"), opt(&WS), &real_number)
    )).compile()));
    let signed_number = signed_number.set(tag("signed_number", Combinator::from(choice!(
        &NUMBER,
        seq!(python_literal("-"), opt(&WS), &NUMBER)
    )).compile()));
    let complex_number = complex_number.set(tag("complex_number", Combinator::from(seq!(&signed_real_number, opt(&WS), choice!(seq!(python_literal("+"), opt(&WS), &imaginary_number), seq!(python_literal("-"), opt(&WS), &imaginary_number)))).compile()));
    let literal_expr = literal_expr.set(tag("literal_expr", Combinator::from(choice!(
        &signed_number,
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).compile()));
    let literal_pattern = literal_pattern.set(tag("literal_pattern", Combinator::from(choice!(
        &signed_number,
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).compile()));
    let closed_pattern = closed_pattern.set(cached(tag("closed_pattern", Combinator::from(choice!(
        &literal_pattern,
        &capture_pattern,
        &wildcard_pattern,
        &value_pattern,
        &group_pattern,
        &sequence_pattern,
        &mapping_pattern,
        &class_pattern
    )).compile())));
    let or_pattern = or_pattern.set(tag("or_pattern", Combinator::from(seq!(&closed_pattern, opt(seq!(opt(&WS), python_literal("|"), opt(&WS), &closed_pattern, opt(repeat1(seq!(opt(&WS), python_literal("|"), opt(&WS), &closed_pattern))))))).compile()));
    let as_pattern = as_pattern.set(tag("as_pattern", Combinator::from(seq!(
        &or_pattern,
        opt(&WS),
        python_literal("as"),
        opt(&WS),
        &pattern_capture_target
    )).compile()));
    let pattern = pattern.set(tag("pattern", Combinator::from(choice!(
        &as_pattern,
        &or_pattern
    )).compile()));
    let patterns = patterns.set(tag("patterns", Combinator::from(choice!(
        &open_sequence_pattern,
        &pattern
    )).compile()));
    let guard = guard.set(tag("guard", Combinator::from(seq!(python_literal("if"), opt(&WS), &named_expression)).compile()));
    let case_block = case_block.set(tag("case_block", Combinator::from(seq!(
        python_literal("case"),
        opt(&WS),
        &patterns,
        opt(seq!(opt(&WS), &guard)),
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &block
    )).compile()));
    let subject_expr = subject_expr.set(tag("subject_expr", Combinator::from(choice!(
        seq!(&star_named_expression, opt(&WS), python_literal(","), opt(seq!(opt(&WS), &star_named_expressions))),
        &named_expression
    )).compile()));
    let match_stmt = match_stmt.set(tag("match_stmt", Combinator::from(seq!(
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
    )).compile()));
    let finally_block = finally_block.set(tag("finally_block", Combinator::from(seq!(
        python_literal("finally"),
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &block
    )).compile()));
    let except_star_block = except_star_block.set(tag("except_star_block", Combinator::from(seq!(
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
    )).compile()));
    let except_block = except_block.set(tag("except_block", Combinator::from(seq!(python_literal("except"), opt(&WS), choice!(seq!(&expression, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)), opt(&WS), python_literal(":"), opt(&WS), &block), seq!(python_literal(":"), opt(&WS), &block)))).compile()));
    let try_stmt = try_stmt.set(tag("try_stmt", Combinator::from(seq!(
        python_literal("try"),
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &block,
        opt(&WS),
        choice!(&finally_block, seq!(&except_block, opt(repeat1(seq!(opt(&WS), &except_block))), opt(seq!(opt(&WS), &else_block)), opt(seq!(opt(&WS), &finally_block))), seq!(&except_star_block, opt(repeat1(seq!(opt(&WS), &except_star_block))), opt(seq!(opt(&WS), &else_block)), opt(seq!(opt(&WS), &finally_block))))
    )).compile()));
    let with_item = with_item.set(tag("with_item", Combinator::from(seq!(&expression, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &star_target)))).compile()));
    let with_stmt = with_stmt.set(tag("with_stmt", Combinator::from(choice!(
        seq!(python_literal("with"), opt(&WS), choice!(seq!(python_literal("("), opt(&WS), &with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")"), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block), seq!(&with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block))),
        seq!(python_literal("async"), opt(&WS), python_literal("with"), opt(&WS), choice!(seq!(python_literal("("), opt(&WS), &with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")"), opt(&WS), python_literal(":"), opt(&WS), &block), seq!(&with_item, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &with_item))))), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block)))
    )).compile()));
    let for_stmt = for_stmt.set(tag("for_stmt", Combinator::from(choice!(
        seq!(python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &star_expressions, opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block, opt(seq!(opt(&WS), &else_block))),
        seq!(python_literal("async"), opt(&WS), python_literal("for"), opt(&WS), &star_targets, opt(&WS), python_literal("in"), opt(&WS), &star_expressions, opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &TYPE_COMMENT)), opt(&WS), &block, opt(seq!(opt(&WS), &else_block)))
    )).compile()));
    let while_stmt = while_stmt.set(tag("while_stmt", Combinator::from(seq!(
        python_literal("while"),
        opt(&WS),
        &named_expression,
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &block,
        opt(seq!(opt(&WS), &else_block))
    )).compile()));
    let else_block = else_block.set(tag("else_block", Combinator::from(seq!(
        python_literal("else"),
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &block
    )).compile()));
    let elif_stmt = elif_stmt.set(tag("elif_stmt", Combinator::from(seq!(
        python_literal("elif"),
        opt(&WS),
        &named_expression,
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &block,
        opt(seq!(opt(&WS), choice!(&elif_stmt, &else_block)))
    )).compile()));
    let if_stmt = if_stmt.set(tag("if_stmt", Combinator::from(seq!(
        python_literal("if"),
        opt(&WS),
        &named_expression,
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &block,
        opt(seq!(opt(&WS), choice!(&elif_stmt, &else_block)))
    )).compile()));
    let default = default.set(tag("default", Combinator::from(seq!(python_literal("="), opt(&WS), &expression)).compile()));
    let star_annotation = star_annotation.set(tag("star_annotation", Combinator::from(seq!(python_literal(":"), opt(&WS), &star_expression)).compile()));
    let annotation = annotation.set(tag("annotation", Combinator::from(seq!(python_literal(":"), opt(&WS), &expression)).compile()));
    let param_star_annotation = param_star_annotation.set(tag("param_star_annotation", Combinator::from(seq!(&NAME, opt(&WS), &star_annotation)).compile()));
    let param = param.set(tag("param", Combinator::from(seq!(&NAME, opt(seq!(opt(&WS), &annotation)))).compile()));
    let param_maybe_default = param_maybe_default.set(tag("param_maybe_default", Combinator::from(seq!(&param, opt(seq!(opt(&WS), &default)), opt(seq!(opt(&WS), choice!(seq!(python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), &TYPE_COMMENT))))).compile()));
    let param_with_default = param_with_default.set(tag("param_with_default", Combinator::from(seq!(&param, opt(&WS), &default, opt(seq!(opt(&WS), choice!(seq!(python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), &TYPE_COMMENT))))).compile()));
    let param_no_default_star_annotation = param_no_default_star_annotation.set(tag("param_no_default_star_annotation", Combinator::from(seq!(&param_star_annotation, opt(seq!(opt(&WS), choice!(seq!(python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), &TYPE_COMMENT))))).compile()));
    let param_no_default = param_no_default.set(tag("param_no_default", Combinator::from(seq!(&param, opt(seq!(opt(&WS), choice!(seq!(python_literal(","), opt(seq!(opt(&WS), &TYPE_COMMENT))), &TYPE_COMMENT))))).compile()));
    let kwds = kwds.set(tag("kwds", Combinator::from(seq!(python_literal("**"), opt(&WS), &param_no_default)).compile()));
    let star_etc = star_etc.set(tag("star_etc", Combinator::from(choice!(
        seq!(python_literal("*"), opt(&WS), choice!(seq!(&param_no_default, opt(seq!(opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))))), opt(seq!(opt(&WS), &kwds))), seq!(&param_no_default_star_annotation, opt(seq!(opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))))), opt(seq!(opt(&WS), &kwds))), seq!(python_literal(","), opt(&WS), &param_maybe_default, opt(repeat1(seq!(opt(&WS), &param_maybe_default))), opt(seq!(opt(&WS), &kwds))))),
        &kwds
    )).compile()));
    let slash_with_default = slash_with_default.set(tag("slash_with_default", Combinator::from(seq!(
        opt(seq!(&param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))), opt(&WS))),
        &param_with_default,
        opt(repeat1(seq!(opt(&WS), &param_with_default))),
        opt(&WS),
        python_literal("/"),
        opt(seq!(opt(&WS), python_literal(",")))
    )).compile()));
    let slash_no_default = slash_no_default.set(tag("slash_no_default", Combinator::from(seq!(
        &param_no_default,
        opt(repeat1(seq!(opt(&WS), &param_no_default))),
        opt(&WS),
        python_literal("/"),
        opt(seq!(opt(&WS), python_literal(",")))
    )).compile()));
    let parameters = parameters.set(tag("parameters", Combinator::from(choice!(
        seq!(&slash_no_default, opt(seq!(opt(&WS), &param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))))), opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
        seq!(&slash_with_default, opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
        seq!(&param_no_default, opt(repeat1(seq!(opt(&WS), &param_no_default))), opt(seq!(opt(&WS), &param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))))), opt(seq!(opt(&WS), &star_etc))),
        seq!(&param_with_default, opt(repeat1(seq!(opt(&WS), &param_with_default))), opt(seq!(opt(&WS), &star_etc))),
        &star_etc
    )).compile()));
    let params = params.set(tag("params", Combinator::from(&parameters).compile()));
    let function_def_raw = function_def_raw.set(tag("function_def_raw", Combinator::from(choice!(
        seq!(python_literal("def"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_params)), opt(&WS), python_literal("("), opt(seq!(opt(&WS), &params)), opt(&WS), python_literal(")"), opt(seq!(opt(&WS), python_literal("->"), opt(&WS), &expression)), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &func_type_comment)), opt(&WS), &block),
        seq!(python_literal("async"), opt(&WS), python_literal("def"), opt(&WS), &NAME, opt(seq!(opt(&WS), &type_params)), opt(&WS), python_literal("("), opt(seq!(opt(&WS), &params)), opt(&WS), python_literal(")"), opt(seq!(opt(&WS), python_literal("->"), opt(&WS), &expression)), opt(&WS), python_literal(":"), opt(seq!(opt(&WS), &func_type_comment)), opt(&WS), &block)
    )).compile()));
    let function_def = function_def.set(tag("function_def", Combinator::from(choice!(
        seq!(python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), opt(seq!(&WS, opt(seq!(opt(&WS), &WS)), opt(&WS))), &NEWLINE))))), opt(&WS), &function_def_raw),
        &function_def_raw
    )).compile()));
    let class_def_raw = class_def_raw.set(tag("class_def_raw", Combinator::from(seq!(
        python_literal("class"),
        opt(&WS),
        &NAME,
        opt(seq!(opt(&WS), &type_params)),
        opt(seq!(opt(&WS), python_literal("("), opt(seq!(opt(&WS), &arguments)), opt(&WS), python_literal(")"))),
        opt(&WS),
        python_literal(":"),
        opt(&WS),
        &block
    )).compile()));
    let class_def = class_def.set(tag("class_def", Combinator::from(choice!(
        seq!(python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), opt(seq!(&WS, opt(&WS))), &named_expression, opt(&WS), opt(seq!(&WS, opt(&WS))), &NEWLINE))))), opt(&WS), &class_def_raw),
        &class_def_raw
    )).compile()));
    let decorators = decorators.set(tag("decorators", Combinator::from(seq!(
        python_literal("@"),
        opt(&WS),
        &named_expression,
        opt(&WS),
        &NEWLINE,
        opt(seq!(opt(&WS), python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE, opt(repeat1(seq!(opt(&WS), python_literal("@"), opt(&WS), &named_expression, opt(&WS), &NEWLINE)))))
    )).compile()));
    let block = block.set(cached(tag("block", Combinator::from(choice!(
        seq!(&NEWLINE, opt(&WS), &INDENT, opt(&WS), &statements, opt(&WS), &DEDENT),
        seq!(&simple_stmt, opt(&WS), choice!(&NEWLINE, seq!(opt(seq!(python_literal(";"), opt(&WS), opt(seq!(&WS, opt(&WS))), &simple_stmt, opt(repeat1(seq!(opt(&WS), python_literal(";"), opt(&WS), opt(seq!(&WS, opt(&WS))), &simple_stmt))), opt(&WS))), opt(seq!(python_literal(";"), opt(&WS))), &NEWLINE)))
    )).compile())));
    let dotted_name = dotted_name.set(tag("dotted_name", Combinator::from(seq!(&NAME, opt(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal("."), opt(&WS), &NAME))))))).compile()));
    let dotted_as_name = dotted_as_name.set(tag("dotted_as_name", Combinator::from(seq!(&dotted_name, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)))).compile()));
    let dotted_as_names = dotted_as_names.set(tag("dotted_as_names", Combinator::from(seq!(&dotted_as_name, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &dotted_as_name, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &dotted_as_name))))))).compile()));
    let import_from_as_name = import_from_as_name.set(tag("import_from_as_name", Combinator::from(seq!(&NAME, opt(seq!(opt(&WS), python_literal("as"), opt(&WS), &NAME)))).compile()));
    let import_from_as_names = import_from_as_names.set(tag("import_from_as_names", Combinator::from(seq!(&import_from_as_name, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &import_from_as_name, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &import_from_as_name))))))).compile()));
    let import_from_targets = import_from_targets.set(tag("import_from_targets", Combinator::from(choice!(
        seq!(python_literal("("), opt(&WS), &import_from_as_names, opt(seq!(opt(&WS), python_literal(","))), opt(&WS), python_literal(")")),
        &import_from_as_names,
        python_literal("*")
    )).compile()));
    let import_from = import_from.set(tag("import_from", Combinator::from(seq!(python_literal("from"), opt(&WS), choice!(seq!(opt(seq!(choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(&WS), choice!(python_literal("."), python_literal("..."))))), opt(&WS))), &dotted_name, opt(&WS), python_literal("import"), opt(&WS), &import_from_targets), seq!(choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(&WS), choice!(python_literal("."), python_literal("..."))))), opt(&WS), python_literal("import"), opt(&WS), &import_from_targets)))).compile()));
    let import_name = import_name.set(tag("import_name", Combinator::from(seq!(python_literal("import"), opt(&WS), &dotted_as_names)).compile()));
    let import_stmt = import_stmt.set(tag("import_stmt", Combinator::from(choice!(
        &import_name,
        &import_from
    )).compile()));
    let assert_stmt = assert_stmt.set(tag("assert_stmt", Combinator::from(seq!(python_literal("assert"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &expression)))).compile()));
    let yield_stmt = yield_stmt.set(tag("yield_stmt", Combinator::from(&yield_expr).compile()));
    let del_stmt = del_stmt.set(tag("del_stmt", Combinator::from(seq!(python_literal("del"), opt(&WS), &del_targets)).compile()));
    let nonlocal_stmt = nonlocal_stmt.set(tag("nonlocal_stmt", Combinator::from(seq!(python_literal("nonlocal"), opt(&WS), &NAME, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME))))))).compile()));
    let global_stmt = global_stmt.set(tag("global_stmt", Combinator::from(seq!(python_literal("global"), opt(&WS), &NAME, opt(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME, opt(repeat1(seq!(opt(&WS), python_literal(","), opt(&WS), &NAME))))))).compile()));
    let raise_stmt = raise_stmt.set(tag("raise_stmt", Combinator::from(seq!(python_literal("raise"), opt(seq!(opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("from"), opt(&WS), &expression)))))).compile()));
    let return_stmt = return_stmt.set(tag("return_stmt", Combinator::from(seq!(python_literal("return"), opt(seq!(opt(&WS), &star_expressions)))).compile()));
    let augassign = augassign.set(tag("augassign", Combinator::from(choice!(
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
    )).compile()));
    let annotated_rhs = annotated_rhs.set(tag("annotated_rhs", Combinator::from(choice!(
        &yield_expr,
        &star_expressions
    )).compile()));
    let assignment = assignment.set(tag("assignment", Combinator::from(choice!(
        seq!(&NAME, opt(&WS), python_literal(":"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("="), opt(&WS), &annotated_rhs))),
        seq!(choice!(seq!(python_literal("("), opt(&WS), &single_target, opt(&WS), python_literal(")")), &single_subscript_attribute_target), opt(&WS), python_literal(":"), opt(&WS), &expression, opt(seq!(opt(&WS), python_literal("="), opt(&WS), &annotated_rhs))),
        seq!(&star_targets, opt(&WS), python_literal("="), opt(seq!(opt(&WS), &star_targets, opt(&WS), python_literal("="), opt(repeat1(seq!(opt(&WS), &star_targets, opt(&WS), python_literal("=")))))), opt(&WS), choice!(&yield_expr, &star_expressions), opt(seq!(opt(&WS), &TYPE_COMMENT))),
        seq!(&single_target, opt(&WS), &augassign, opt(&WS), choice!(&yield_expr, &star_expressions))
    )).compile()));
    let compound_stmt = compound_stmt.set(tag("compound_stmt", Combinator::from(choice!(
        &function_def,
        &if_stmt,
        &class_def,
        &with_stmt,
        &for_stmt,
        &try_stmt,
        &while_stmt,
        &match_stmt
    )).compile()));
    let simple_stmt = simple_stmt.set(cached(tag("simple_stmt", Combinator::from(choice!(
        &assignment,
        &type_alias,
        &star_expressions,
        &return_stmt,
        &import_stmt,
        &raise_stmt,
        python_literal("pass"),
        &del_stmt,
        &yield_stmt,
        &assert_stmt,
        python_literal("break"),
        python_literal("continue"),
        &global_stmt,
        &nonlocal_stmt
    )).compile())));
    let simple_stmts = simple_stmts.set(tag("simple_stmts", Combinator::from(seq!(&simple_stmt, opt(&WS), choice!(&NEWLINE, seq!(opt(seq!(python_literal(";"), opt(&WS), &simple_stmt, opt(repeat1(seq!(opt(&WS), python_literal(";"), opt(&WS), &simple_stmt))), opt(&WS))), opt(seq!(python_literal(";"), opt(&WS))), &NEWLINE)))).compile()));
    let statement_newline = statement_newline.set(tag("statement_newline", Combinator::from(choice!(
        seq!(&compound_stmt, opt(&WS), &NEWLINE),
        &simple_stmts,
        &NEWLINE,
        &ENDMARKER
    )).compile()));
    let statement = statement.set(tag("statement", Combinator::from(choice!(
        &compound_stmt,
        &simple_stmts
    )).compile()));
    let statements = statements.set(tag("statements", Combinator::from(seq!(&statement, opt(repeat1(seq!(opt(&WS), &statement))))).compile()));
    let func_type = func_type.set(tag("func_type", Combinator::from(seq!(
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
    )).compile()));
    let eval = eval.set(tag("eval", Combinator::from(seq!(&expressions, opt(seq!(opt(&WS), &NEWLINE, opt(repeat1(seq!(opt(&WS), &NEWLINE))))), opt(&WS), &ENDMARKER)).compile()));
    let interactive = interactive.set(tag("interactive", Combinator::from(&statement_newline).compile()));
    let file = file.set(tag("file", Combinator::from(seq!(opt(seq!(&statements, opt(&WS))), &ENDMARKER)).compile()));

    cache_context(seq!(opt(&NEWLINE), &file)).into()
}
