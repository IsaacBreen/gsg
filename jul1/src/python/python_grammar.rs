use std::rc::Rc;
use crate::{choice, seq, repeat0, repeat1, opt, eat_char_choice, eat_string, eat_char_range, forward_ref, eps, cut, tag, DynCombinator, CombinatorTrait, forward_decls};
use super::python_tokenizer::{NAME, TYPE_COMMENT, FSTRING_START, FSTRING_MIDDLE, FSTRING_END, NUMBER, STRING, NEWLINE, INDENT, DEDENT, ENDMARKER};
use super::python_tokenizer::python_literal;

pub fn python_file() -> Rc<DynCombinator> {
    let NAME = tag("NAME", NAME()).into_rc_dyn();
    let TYPE_COMMENT = tag("TYPE_COMMENT", TYPE_COMMENT()).into_rc_dyn();
    let FSTRING_START = tag("FSTRING_START", FSTRING_START()).into_rc_dyn();
    let FSTRING_MIDDLE = tag("FSTRING_MIDDLE", FSTRING_MIDDLE()).into_rc_dyn();
    let FSTRING_END = tag("FSTRING_END", FSTRING_END()).into_rc_dyn();
    let NUMBER = tag("NUMBER", NUMBER()).into_rc_dyn();
    let STRING = tag("STRING", STRING()).into_rc_dyn();
    let NEWLINE = tag("NEWLINE", NEWLINE()).into_rc_dyn();
    let INDENT = tag("INDENT", INDENT()).into_rc_dyn();
    let DEDENT = tag("DEDENT", DEDENT()).into_rc_dyn();
    let ENDMARKER = tag("ENDMARKER", ENDMARKER()).into_rc_dyn();

    forward_decls!(expression_without_invalid, func_type_comment, type_expressions, del_t_atom, del_target, del_targets, t_lookahead, t_primary, single_subscript_attribute_target, single_target, star_atom, target_with_star_atom, star_target, star_targets_tuple_seq, star_targets_list_seq, star_targets, kwarg_or_double_starred, kwarg_or_starred, starred_expression, kwargs, args, arguments, dictcomp, genexp, setcomp, listcomp, for_if_clause, for_if_clauses, kvpair, double_starred_kvpair, double_starred_kvpairs, dict, set, tuple, list, strings, string, fstring, fstring_format_spec, fstring_full_format_spec, fstring_conversion, fstring_replacement_field, fstring_middle, lambda_param, lambda_param_maybe_default, lambda_param_with_default, lambda_param_no_default, lambda_kwds, lambda_star_etc, lambda_slash_with_default, lambda_slash_no_default, lambda_parameters, lambda_params, lambdef, group, atom, slice, slices, primary, await_primary, power, factor, term, sum, shift_expr, bitwise_and, bitwise_xor, bitwise_or, is_bitwise_or, isnot_bitwise_or, in_bitwise_or, notin_bitwise_or, gt_bitwise_or, gte_bitwise_or, lt_bitwise_or, lte_bitwise_or, noteq_bitwise_or, eq_bitwise_or, compare_op_bitwise_or_pair, comparison, inversion, conjunction, disjunction, named_expression, assignment_expression, star_named_expression, star_named_expressions, star_expression, star_expressions, yield_expr, expression, expressions, type_param_starred_default, type_param_default, type_param_bound, type_param, type_param_seq, type_params, type_alias, keyword_pattern, keyword_patterns, positional_patterns, class_pattern, double_star_pattern, key_value_pattern, items_pattern, mapping_pattern, star_pattern, maybe_star_pattern, maybe_sequence_pattern, open_sequence_pattern, sequence_pattern, group_pattern, name_or_attr, attr, value_pattern, wildcard_pattern, pattern_capture_target, capture_pattern, imaginary_number, real_number, signed_real_number, signed_number, complex_number, literal_expr, literal_pattern, closed_pattern, or_pattern, as_pattern, pattern, patterns, guard, case_block, subject_expr, match_stmt, finally_block, except_star_block, except_block, try_stmt, with_item, with_stmt, for_stmt, while_stmt, else_block, elif_stmt, if_stmt, default, star_annotation, annotation, param_star_annotation, param, param_maybe_default, param_with_default, param_no_default_star_annotation, param_no_default, kwds, star_etc, slash_with_default, slash_no_default, parameters, params, function_def_raw, function_def, class_def_raw, class_def, decorators, block, dotted_name, dotted_as_name, dotted_as_names, import_from_as_name, import_from_as_names, import_from_targets, import_from, import_name, import_stmt, assert_stmt, yield_stmt, del_stmt, nonlocal_stmt, global_stmt, raise_stmt, return_stmt, augassign, annotated_rhs, assignment, compound_stmt, simple_stmt, simple_stmts, statement_newline, statement, statements, func_type, eval, interactive, file);

    let expression_without_invalid = expression_without_invalid.set(tag("expression_without_invalid", choice!(
        seq!(&conjunction, opt(repeat1(seq!(python_literal("or"), &conjunction))), opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), &expression)
    ))).into_rc_dyn();
    let func_type_comment = func_type_comment.set(tag("func_type_comment", choice!(
        seq!(&NEWLINE, &TYPE_COMMENT),
        &TYPE_COMMENT
    ))).into_rc_dyn();
    let type_expressions = type_expressions.set(tag("type_expressions", choice!(
        seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), opt(repeat1(seq!(python_literal(","), &expression))), opt(seq!(python_literal(","), choice!(seq!(python_literal("*"), &expression, opt(seq!(python_literal(","), python_literal("**"), &expression))), seq!(python_literal("**"), &expression))))),
        seq!(python_literal("*"), &expression, opt(seq!(python_literal(","), python_literal("**"), &expression))),
        seq!(python_literal("**"), &expression)
    ))).into_rc_dyn();
    let del_t_atom = del_t_atom.set(tag("del_t_atom", choice!(
        &NAME,
        seq!(python_literal("("), choice!(seq!(&del_target, python_literal(")")), seq!(opt(&del_targets), python_literal(")")))),
        seq!(python_literal("["), opt(&del_targets), python_literal("]"))
    ))).into_rc_dyn();
    let del_target = del_target.set(tag("del_target", choice!(
        seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), &strings, &NUMBER, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp, python_literal("...")), opt(repeat1(choice!(&strings, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp))), opt(repeat1(choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")), &genexp, seq!(python_literal("("), opt(&arguments), python_literal(")"))))), choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")))),
        &del_t_atom
    ))).into_rc_dyn();
    let del_targets = del_targets.set(tag("del_targets", seq!(&del_target, opt(repeat1(seq!(python_literal(","), &del_target))), opt(python_literal(","))))).into_rc_dyn();
    let t_lookahead = t_lookahead.set(tag("t_lookahead", choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    ))).into_rc_dyn();
    let t_primary = t_primary.set(tag("t_primary", seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), &strings, &NUMBER, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp, python_literal("...")), opt(repeat1(choice!(&strings, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp))), opt(repeat1(choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")), &genexp, seq!(python_literal("("), opt(&arguments), python_literal(")")))))))).into_rc_dyn();
    let single_subscript_attribute_target = single_subscript_attribute_target.set(tag("single_subscript_attribute_target", seq!(&t_primary, choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")))))).into_rc_dyn();
    let single_target = single_target.set(tag("single_target", choice!(
        &single_subscript_attribute_target,
        &NAME,
        seq!(python_literal("("), &single_target, python_literal(")"))
    ))).into_rc_dyn();
    let star_atom = star_atom.set(tag("star_atom", choice!(
        &NAME,
        seq!(python_literal("("), choice!(seq!(&target_with_star_atom, python_literal(")")), seq!(opt(&star_targets_tuple_seq), python_literal(")")))),
        seq!(python_literal("["), opt(&star_targets_list_seq), python_literal("]"))
    ))).into_rc_dyn();
    let target_with_star_atom = target_with_star_atom.set(tag("target_with_star_atom", choice!(
        seq!(&t_primary, choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")))),
        &star_atom
    ))).into_rc_dyn();
    let star_target = star_target.set(tag("star_target", choice!(
        seq!(python_literal("*"), &star_target),
        &target_with_star_atom
    ))).into_rc_dyn();
    let star_targets_tuple_seq = star_targets_tuple_seq.set(tag("star_targets_tuple_seq", seq!(&star_target, choice!(seq!(repeat1(seq!(python_literal(","), &star_target)), opt(python_literal(","))), python_literal(","))))).into_rc_dyn();
    let star_targets_list_seq = star_targets_list_seq.set(tag("star_targets_list_seq", seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(","))))).into_rc_dyn();
    let star_targets = star_targets.set(tag("star_targets", seq!(&star_target, opt(seq!(opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(","))))))).into_rc_dyn();
    let kwarg_or_double_starred = kwarg_or_double_starred.set(tag("kwarg_or_double_starred", choice!(
        seq!(&NAME, python_literal("="), &expression),
        seq!(python_literal("**"), &expression)
    ))).into_rc_dyn();
    let kwarg_or_starred = kwarg_or_starred.set(tag("kwarg_or_starred", choice!(
        seq!(&NAME, python_literal("="), &expression),
        seq!(python_literal("*"), &expression)
    ))).into_rc_dyn();
    let starred_expression = starred_expression.set(tag("starred_expression", seq!(python_literal("*"), &expression))).into_rc_dyn();
    let kwargs = kwargs.set(tag("kwargs", choice!(
        seq!(&kwarg_or_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_starred))), opt(seq!(python_literal(","), &kwarg_or_double_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_double_starred)))))),
        seq!(&kwarg_or_double_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_double_starred))))
    ))).into_rc_dyn();
    let args = args.set(tag("args", choice!(
        seq!(choice!(&starred_expression, seq!(&NAME, python_literal(":="), cut(), &expression), seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(&NAME, python_literal(":="), cut(), &expression), seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)))), opt(seq!(python_literal(","), &kwargs))),
        &kwargs
    ))).into_rc_dyn();
    let arguments = arguments.set(tag("arguments", seq!(&args, opt(python_literal(","))))).into_rc_dyn();
    let dictcomp = dictcomp.set(tag("dictcomp", seq!(python_literal("{"), &kvpair, &for_if_clauses, python_literal("}")))).into_rc_dyn();
    let genexp = genexp.set(tag("genexp", seq!(python_literal("("), choice!(&assignment_expression, &expression), &for_if_clauses, python_literal(")")))).into_rc_dyn();
    let setcomp = setcomp.set(tag("setcomp", seq!(python_literal("{"), &named_expression, &for_if_clauses, python_literal("}")))).into_rc_dyn();
    let listcomp = listcomp.set(tag("listcomp", seq!(python_literal("["), &named_expression, &for_if_clauses, python_literal("]")))).into_rc_dyn();
    let for_if_clause = for_if_clause.set(tag("for_if_clause", choice!(
        seq!(python_literal("async"), python_literal("for"), &star_targets, python_literal("in"), cut(), &disjunction, opt(repeat1(seq!(python_literal("if"), &disjunction)))),
        seq!(python_literal("for"), &star_targets, python_literal("in"), cut(), &disjunction, opt(repeat1(seq!(python_literal("if"), &disjunction))))
    ))).into_rc_dyn();
    let for_if_clauses = for_if_clauses.set(tag("for_if_clauses", repeat1(&for_if_clause))).into_rc_dyn();
    let kvpair = kvpair.set(tag("kvpair", seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), python_literal(":"), &expression))).into_rc_dyn();
    let double_starred_kvpair = double_starred_kvpair.set(tag("double_starred_kvpair", choice!(
        seq!(python_literal("**"), &bitwise_or),
        &kvpair
    ))).into_rc_dyn();
    let double_starred_kvpairs = double_starred_kvpairs.set(tag("double_starred_kvpairs", seq!(&double_starred_kvpair, opt(repeat1(seq!(python_literal(","), &double_starred_kvpair))), opt(python_literal(","))))).into_rc_dyn();
    let dict = dict.set(tag("dict", seq!(python_literal("{"), opt(&double_starred_kvpairs), python_literal("}")))).into_rc_dyn();
    let set = set.set(tag("set", seq!(python_literal("{"), &star_named_expressions, python_literal("}")))).into_rc_dyn();
    let tuple = tuple.set(tag("tuple", seq!(python_literal("("), opt(seq!(&star_named_expression, python_literal(","), opt(&star_named_expressions))), python_literal(")")))).into_rc_dyn();
    let list = list.set(tag("list", seq!(python_literal("["), opt(&star_named_expressions), python_literal("]")))).into_rc_dyn();
    let strings = strings.set(tag("strings", repeat1(choice!(seq!(&FSTRING_START, opt(repeat1(&fstring_middle)), &FSTRING_END), &STRING)))).into_rc_dyn();
    let string = string.set(tag("string", &STRING)).into_rc_dyn();
    let fstring = fstring.set(tag("fstring", seq!(&FSTRING_START, opt(repeat1(&fstring_middle)), &FSTRING_END))).into_rc_dyn();
    let fstring_format_spec = fstring_format_spec.set(tag("fstring_format_spec", choice!(
        &FSTRING_MIDDLE,
        seq!(python_literal("{"), &annotated_rhs, opt(python_literal("=")), opt(&fstring_conversion), opt(&fstring_full_format_spec), python_literal("}"))
    ))).into_rc_dyn();
    let fstring_full_format_spec = fstring_full_format_spec.set(tag("fstring_full_format_spec", seq!(python_literal(":"), opt(repeat1(&fstring_format_spec))))).into_rc_dyn();
    let fstring_conversion = fstring_conversion.set(tag("fstring_conversion", seq!(python_literal(""), &NAME))).into_rc_dyn();
    let fstring_replacement_field = fstring_replacement_field.set(tag("fstring_replacement_field", seq!(python_literal("{"), &annotated_rhs, opt(python_literal("=")), opt(&fstring_conversion), opt(&fstring_full_format_spec), python_literal("}")))).into_rc_dyn();
    let fstring_middle = fstring_middle.set(tag("fstring_middle", choice!(
        &fstring_replacement_field,
        &FSTRING_MIDDLE
    ))).into_rc_dyn();
    let lambda_param = lambda_param.set(tag("lambda_param", &NAME)).into_rc_dyn();
    let lambda_param_maybe_default = lambda_param_maybe_default.set(tag("lambda_param_maybe_default", seq!(&lambda_param, opt(&default), opt(python_literal(","))))).into_rc_dyn();
    let lambda_param_with_default = lambda_param_with_default.set(tag("lambda_param_with_default", seq!(&lambda_param, &default, opt(python_literal(","))))).into_rc_dyn();
    let lambda_param_no_default = lambda_param_no_default.set(tag("lambda_param_no_default", seq!(&lambda_param, opt(python_literal(","))))).into_rc_dyn();
    let lambda_kwds = lambda_kwds.set(tag("lambda_kwds", seq!(python_literal("**"), &lambda_param_no_default))).into_rc_dyn();
    let lambda_star_etc = lambda_star_etc.set(tag("lambda_star_etc", choice!(
        seq!(python_literal("*"), choice!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_maybe_default)), opt(&lambda_kwds)), seq!(python_literal(","), repeat1(&lambda_param_maybe_default), opt(&lambda_kwds)))),
        &lambda_kwds
    ))).into_rc_dyn();
    let lambda_slash_with_default = lambda_slash_with_default.set(tag("lambda_slash_with_default", seq!(opt(repeat1(&lambda_param_no_default)), repeat1(&lambda_param_with_default), python_literal("/"), opt(python_literal(",")), opt(repeat1(seq!(repeat1(&lambda_param_with_default), python_literal("/"), opt(python_literal(",")))))))).into_rc_dyn();
    let lambda_slash_no_default = lambda_slash_no_default.set(tag("lambda_slash_no_default", seq!(repeat1(&lambda_param_no_default), python_literal("/"), opt(python_literal(","))))).into_rc_dyn();
    let lambda_parameters = lambda_parameters.set(tag("lambda_parameters", choice!(
        seq!(&lambda_slash_no_default, opt(repeat1(&lambda_param_no_default)), opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(&lambda_slash_with_default, opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(repeat1(&lambda_param_no_default), opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(repeat1(&lambda_param_with_default), opt(&lambda_star_etc)),
        &lambda_star_etc
    ))).into_rc_dyn();
    let lambda_params = lambda_params.set(tag("lambda_params", &lambda_parameters)).into_rc_dyn();
    let lambdef = lambdef.set(tag("lambdef", seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), &expression))).into_rc_dyn();
    let group = group.set(tag("group", seq!(python_literal("("), choice!(&yield_expr, &named_expression), python_literal(")")))).into_rc_dyn();
    let atom = atom.set(tag("atom", seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), &strings, &NUMBER, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp, python_literal("...")), opt(repeat1(choice!(&strings, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp)))))).into_rc_dyn();
    let slice = slice.set(tag("slice", seq!(choice!(seq!(opt(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), python_literal(":"), opt(&expression), opt(seq!(python_literal(":"), opt(&expression)))), seq!(&NAME, python_literal(":="), cut(), &expression), seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), opt(repeat1(seq!(python_literal(":"), opt(&expression), opt(seq!(python_literal(":"), opt(&expression))))))))).into_rc_dyn();
    let slices = slices.set(tag("slices", choice!(
        &slice,
        seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression)))), opt(python_literal(",")))
    ))).into_rc_dyn();
    let primary = primary.set(tag("primary", seq!(&atom, opt(repeat1(choice!(seq!(python_literal("."), &NAME), &genexp, seq!(python_literal("("), opt(&arguments), python_literal(")")), seq!(python_literal("["), &slices, python_literal("]")))))))).into_rc_dyn();
    let await_primary = await_primary.set(tag("await_primary", choice!(
        seq!(python_literal("await"), &primary),
        &primary
    ))).into_rc_dyn();
    let power = power.set(tag("power", seq!(&await_primary, opt(seq!(python_literal("**"), &factor))))).into_rc_dyn();
    let factor = factor.set(tag("factor", choice!(
        seq!(python_literal("+"), &factor),
        seq!(python_literal("-"), &factor),
        seq!(python_literal("~"), &factor),
        &power
    ))).into_rc_dyn();
    let term = term.set(tag("term", seq!(&factor, opt(repeat1(choice!(seq!(python_literal("*"), &factor), seq!(python_literal("/"), &factor), seq!(python_literal("//"), &factor), seq!(python_literal("%"), &factor), seq!(python_literal("@"), &factor))))))).into_rc_dyn();
    let sum = sum.set(tag("sum", seq!(&term, opt(repeat1(choice!(seq!(python_literal("+"), &term), seq!(python_literal("-"), &term))))))).into_rc_dyn();
    let shift_expr = shift_expr.set(tag("shift_expr", seq!(&sum, opt(repeat1(choice!(seq!(python_literal("<<"), &sum), seq!(python_literal(">>"), &sum))))))).into_rc_dyn();
    let bitwise_and = bitwise_and.set(tag("bitwise_and", seq!(&shift_expr, opt(repeat1(seq!(python_literal("&"), &shift_expr)))))).into_rc_dyn();
    let bitwise_xor = bitwise_xor.set(tag("bitwise_xor", seq!(&bitwise_and, opt(repeat1(seq!(python_literal("^"), &bitwise_and)))))).into_rc_dyn();
    let bitwise_or = bitwise_or.set(tag("bitwise_or", seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor)))))).into_rc_dyn();
    let is_bitwise_or = is_bitwise_or.set(tag("is_bitwise_or", seq!(python_literal("is"), &bitwise_or))).into_rc_dyn();
    let isnot_bitwise_or = isnot_bitwise_or.set(tag("isnot_bitwise_or", seq!(python_literal("is"), python_literal("not"), &bitwise_or))).into_rc_dyn();
    let in_bitwise_or = in_bitwise_or.set(tag("in_bitwise_or", seq!(python_literal("in"), &bitwise_or))).into_rc_dyn();
    let notin_bitwise_or = notin_bitwise_or.set(tag("notin_bitwise_or", seq!(python_literal("not"), python_literal("in"), &bitwise_or))).into_rc_dyn();
    let gt_bitwise_or = gt_bitwise_or.set(tag("gt_bitwise_or", seq!(python_literal(">"), &bitwise_or))).into_rc_dyn();
    let gte_bitwise_or = gte_bitwise_or.set(tag("gte_bitwise_or", seq!(python_literal(">="), &bitwise_or))).into_rc_dyn();
    let lt_bitwise_or = lt_bitwise_or.set(tag("lt_bitwise_or", seq!(python_literal("<"), &bitwise_or))).into_rc_dyn();
    let lte_bitwise_or = lte_bitwise_or.set(tag("lte_bitwise_or", seq!(python_literal("<="), &bitwise_or))).into_rc_dyn();
    let noteq_bitwise_or = noteq_bitwise_or.set(tag("noteq_bitwise_or", seq!(python_literal("!="), &bitwise_or))).into_rc_dyn();
    let eq_bitwise_or = eq_bitwise_or.set(tag("eq_bitwise_or", seq!(python_literal("=="), &bitwise_or))).into_rc_dyn();
    let compare_op_bitwise_or_pair = compare_op_bitwise_or_pair.set(tag("compare_op_bitwise_or_pair", choice!(
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
    ))).into_rc_dyn();
    let comparison = comparison.set(tag("comparison", seq!(&bitwise_or, opt(repeat1(&compare_op_bitwise_or_pair))))).into_rc_dyn();
    let inversion = inversion.set(tag("inversion", choice!(
        seq!(python_literal("not"), &inversion),
        &comparison
    ))).into_rc_dyn();
    let conjunction = conjunction.set(tag("conjunction", seq!(&inversion, opt(repeat1(seq!(python_literal("and"), &inversion)))))).into_rc_dyn();
    let disjunction = disjunction.set(tag("disjunction", seq!(&conjunction, opt(repeat1(seq!(python_literal("or"), &conjunction)))))).into_rc_dyn();
    let named_expression = named_expression.set(tag("named_expression", choice!(
        seq!(&NAME, python_literal(":="), cut(), &expression),
        seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        &lambdef
    ))).into_rc_dyn();
    let assignment_expression = assignment_expression.set(tag("assignment_expression", seq!(&NAME, python_literal(":="), cut(), &expression))).into_rc_dyn();
    let star_named_expression = star_named_expression.set(tag("star_named_expression", choice!(
        seq!(python_literal("*"), &bitwise_or),
        &named_expression
    ))).into_rc_dyn();
    let star_named_expressions = star_named_expressions.set(tag("star_named_expressions", seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression))), opt(python_literal(","))))).into_rc_dyn();
    let star_expression = star_expression.set(tag("star_expression", choice!(
        seq!(python_literal("*"), &bitwise_or),
        seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        &lambdef
    ))).into_rc_dyn();
    let star_expressions = star_expressions.set(tag("star_expressions", seq!(&star_expression, opt(choice!(seq!(repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), python_literal(",")))))).into_rc_dyn();
    let yield_expr = yield_expr.set(tag("yield_expr", seq!(python_literal("yield"), opt(choice!(seq!(python_literal("from"), &expression), &star_expressions))))).into_rc_dyn();
    let expression = expression.set(tag("expression", choice!(
        seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        &lambdef
    ))).into_rc_dyn();
    let expressions = expressions.set(tag("expressions", seq!(&expression, opt(choice!(seq!(repeat1(seq!(python_literal(","), &expression)), opt(python_literal(","))), python_literal(",")))))).into_rc_dyn();
    let type_param_starred_default = type_param_starred_default.set(tag("type_param_starred_default", seq!(python_literal("="), &star_expression))).into_rc_dyn();
    let type_param_default = type_param_default.set(tag("type_param_default", seq!(python_literal("="), &expression))).into_rc_dyn();
    let type_param_bound = type_param_bound.set(tag("type_param_bound", seq!(python_literal(":"), &expression))).into_rc_dyn();
    let type_param = type_param.set(tag("type_param", choice!(
        seq!(&NAME, opt(&type_param_bound), opt(&type_param_default)),
        seq!(python_literal("*"), &NAME, opt(&type_param_starred_default)),
        seq!(python_literal("**"), &NAME, opt(&type_param_default))
    ))).into_rc_dyn();
    let type_param_seq = type_param_seq.set(tag("type_param_seq", seq!(&type_param, opt(repeat1(seq!(python_literal(","), &type_param))), opt(python_literal(","))))).into_rc_dyn();
    let type_params = type_params.set(tag("type_params", seq!(python_literal("["), &type_param_seq, python_literal("]")))).into_rc_dyn();
    let type_alias = type_alias.set(tag("type_alias", seq!(python_literal("yp"), &NAME, opt(&type_params), python_literal("="), &expression))).into_rc_dyn();
    let keyword_pattern = keyword_pattern.set(tag("keyword_pattern", seq!(&NAME, python_literal("="), &pattern))).into_rc_dyn();
    let keyword_patterns = keyword_patterns.set(tag("keyword_patterns", seq!(&keyword_pattern, opt(repeat1(seq!(python_literal(","), &keyword_pattern)))))).into_rc_dyn();
    let positional_patterns = positional_patterns.set(tag("positional_patterns", seq!(choice!(&as_pattern, &or_pattern), opt(repeat1(seq!(python_literal(","), &pattern)))))).into_rc_dyn();
    let class_pattern = class_pattern.set(tag("class_pattern", seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME))), python_literal("("), choice!(python_literal(")"), seq!(&positional_patterns, choice!(seq!(opt(python_literal(",")), python_literal(")")), seq!(python_literal(","), &keyword_patterns, opt(python_literal(",")), python_literal(")")))), seq!(&keyword_patterns, opt(python_literal(",")), python_literal(")")))))).into_rc_dyn();
    let double_star_pattern = double_star_pattern.set(tag("double_star_pattern", seq!(python_literal("**"), &pattern_capture_target))).into_rc_dyn();
    let key_value_pattern = key_value_pattern.set(tag("key_value_pattern", seq!(choice!(&signed_number, &complex_number, &strings, python_literal("None"), python_literal("True"), python_literal("False"), seq!(&name_or_attr, python_literal("."), &NAME)), python_literal(":"), &pattern))).into_rc_dyn();
    let items_pattern = items_pattern.set(tag("items_pattern", seq!(&key_value_pattern, opt(repeat1(seq!(python_literal(","), &key_value_pattern)))))).into_rc_dyn();
    let mapping_pattern = mapping_pattern.set(tag("mapping_pattern", seq!(python_literal("{"), choice!(python_literal("}"), seq!(&double_star_pattern, opt(python_literal(",")), python_literal("}")), seq!(&items_pattern, choice!(seq!(python_literal(","), &double_star_pattern, opt(python_literal(",")), python_literal("}")), seq!(opt(python_literal(",")), python_literal("}")))))))).into_rc_dyn();
    let star_pattern = star_pattern.set(tag("star_pattern", seq!(python_literal("*"), choice!(&pattern_capture_target, &wildcard_pattern)))).into_rc_dyn();
    let maybe_star_pattern = maybe_star_pattern.set(tag("maybe_star_pattern", choice!(
        &star_pattern,
        &as_pattern,
        &or_pattern
    ))).into_rc_dyn();
    let maybe_sequence_pattern = maybe_sequence_pattern.set(tag("maybe_sequence_pattern", seq!(&maybe_star_pattern, opt(repeat1(seq!(python_literal(","), &maybe_star_pattern))), opt(python_literal(","))))).into_rc_dyn();
    let open_sequence_pattern = open_sequence_pattern.set(tag("open_sequence_pattern", seq!(&maybe_star_pattern, python_literal(","), opt(&maybe_sequence_pattern)))).into_rc_dyn();
    let sequence_pattern = sequence_pattern.set(tag("sequence_pattern", choice!(
        seq!(python_literal("["), opt(&maybe_sequence_pattern), python_literal("]")),
        seq!(python_literal("("), opt(&open_sequence_pattern), python_literal(")"))
    ))).into_rc_dyn();
    let group_pattern = group_pattern.set(tag("group_pattern", seq!(python_literal("("), &pattern, python_literal(")")))).into_rc_dyn();
    let name_or_attr = name_or_attr.set(tag("name_or_attr", seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME)))))).into_rc_dyn();
    let attr = attr.set(tag("attr", seq!(&name_or_attr, python_literal("."), &NAME))).into_rc_dyn();
    let value_pattern = value_pattern.set(tag("value_pattern", &attr)).into_rc_dyn();
    let wildcard_pattern = wildcard_pattern.set(tag("wildcard_pattern", python_literal(""))).into_rc_dyn();
    let pattern_capture_target = pattern_capture_target.set(tag("pattern_capture_target", repeat1(&NAME))).into_rc_dyn();
    let capture_pattern = capture_pattern.set(tag("capture_pattern", &pattern_capture_target)).into_rc_dyn();
    let imaginary_number = imaginary_number.set(tag("imaginary_number", &NUMBER)).into_rc_dyn();
    let real_number = real_number.set(tag("real_number", &NUMBER)).into_rc_dyn();
    let signed_real_number = signed_real_number.set(tag("signed_real_number", choice!(
        &real_number,
        seq!(python_literal("-"), &real_number)
    ))).into_rc_dyn();
    let signed_number = signed_number.set(tag("signed_number", choice!(
        &NUMBER,
        seq!(python_literal("-"), &NUMBER)
    ))).into_rc_dyn();
    let complex_number = complex_number.set(tag("complex_number", seq!(&signed_real_number, choice!(seq!(python_literal("+"), &imaginary_number), seq!(python_literal("-"), &imaginary_number))))).into_rc_dyn();
    let literal_expr = literal_expr.set(tag("literal_expr", choice!(
        &signed_number,
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    ))).into_rc_dyn();
    let literal_pattern = literal_pattern.set(tag("literal_pattern", choice!(
        &signed_number,
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    ))).into_rc_dyn();
    let closed_pattern = closed_pattern.set(tag("closed_pattern", choice!(
        &literal_pattern,
        &capture_pattern,
        &wildcard_pattern,
        &value_pattern,
        &group_pattern,
        &sequence_pattern,
        &mapping_pattern,
        &class_pattern
    ))).into_rc_dyn();
    let or_pattern = or_pattern.set(tag("or_pattern", seq!(&closed_pattern, opt(repeat1(seq!(python_literal("|"), &closed_pattern)))))).into_rc_dyn();
    let as_pattern = as_pattern.set(tag("as_pattern", seq!(&or_pattern, python_literal("as"), &pattern_capture_target))).into_rc_dyn();
    let pattern = pattern.set(tag("pattern", choice!(
        &as_pattern,
        &or_pattern
    ))).into_rc_dyn();
    let patterns = patterns.set(tag("patterns", choice!(
        &open_sequence_pattern,
        &pattern
    ))).into_rc_dyn();
    let guard = guard.set(tag("guard", seq!(python_literal("if"), &named_expression))).into_rc_dyn();
    let case_block = case_block.set(tag("case_block", seq!(python_literal("as"), &patterns, opt(&guard), python_literal(":"), &block))).into_rc_dyn();
    let subject_expr = subject_expr.set(tag("subject_expr", choice!(
        seq!(&star_named_expression, python_literal(","), opt(&star_named_expressions)),
        &named_expression
    ))).into_rc_dyn();
    let match_stmt = match_stmt.set(tag("match_stmt", seq!(python_literal("atc"), &subject_expr, python_literal(":"), &NEWLINE, &INDENT, repeat1(&case_block), &DEDENT))).into_rc_dyn();
    let finally_block = finally_block.set(tag("finally_block", seq!(python_literal("finally"), python_literal(":"), &block))).into_rc_dyn();
    let except_star_block = except_star_block.set(tag("except_star_block", seq!(python_literal("except"), python_literal("*"), &expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), &block))).into_rc_dyn();
    let except_block = except_block.set(tag("except_block", seq!(python_literal("except"), choice!(seq!(&expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), &block), seq!(python_literal(":"), &block))))).into_rc_dyn();
    let try_stmt = try_stmt.set(tag("try_stmt", seq!(python_literal("try"), python_literal(":"), &block, choice!(&finally_block, seq!(repeat1(&except_block), opt(&else_block), opt(&finally_block)), seq!(repeat1(&except_star_block), opt(&else_block), opt(&finally_block)))))).into_rc_dyn();
    let with_item = with_item.set(tag("with_item", seq!(&expression, opt(seq!(python_literal("as"), &star_target))))).into_rc_dyn();
    let with_stmt = with_stmt.set(tag("with_stmt", choice!(
        seq!(python_literal("with"), choice!(seq!(python_literal("("), &with_item, opt(repeat1(seq!(python_literal(","), &with_item))), opt(python_literal(",")), python_literal(")"), python_literal(":"), opt(&TYPE_COMMENT), &block), seq!(&with_item, opt(repeat1(seq!(python_literal(","), &with_item))), python_literal(":"), opt(&TYPE_COMMENT), &block))),
        seq!(python_literal("async"), python_literal("with"), choice!(seq!(python_literal("("), &with_item, opt(repeat1(seq!(python_literal(","), &with_item))), opt(python_literal(",")), python_literal(")"), python_literal(":"), &block), seq!(&with_item, opt(repeat1(seq!(python_literal(","), &with_item))), python_literal(":"), opt(&TYPE_COMMENT), &block)))
    ))).into_rc_dyn();
    let for_stmt = for_stmt.set(tag("for_stmt", choice!(
        seq!(python_literal("for"), &star_targets, python_literal("in"), cut(), &star_expressions, python_literal(":"), opt(&TYPE_COMMENT), &block, opt(&else_block)),
        seq!(python_literal("async"), python_literal("for"), &star_targets, python_literal("in"), cut(), &star_expressions, python_literal(":"), opt(&TYPE_COMMENT), &block, opt(&else_block))
    ))).into_rc_dyn();
    let while_stmt = while_stmt.set(tag("while_stmt", seq!(python_literal("while"), &named_expression, python_literal(":"), &block, opt(&else_block)))).into_rc_dyn();
    let else_block = else_block.set(tag("else_block", seq!(python_literal("else"), python_literal(":"), &block))).into_rc_dyn();
    let elif_stmt = elif_stmt.set(tag("elif_stmt", seq!(python_literal("elif"), &named_expression, python_literal(":"), &block, opt(choice!(&elif_stmt, &else_block))))).into_rc_dyn();
    let if_stmt = if_stmt.set(tag("if_stmt", seq!(python_literal("if"), &named_expression, python_literal(":"), &block, opt(choice!(&elif_stmt, &else_block))))).into_rc_dyn();
    let default = default.set(tag("default", seq!(python_literal("="), &expression))).into_rc_dyn();
    let star_annotation = star_annotation.set(tag("star_annotation", seq!(python_literal(":"), &star_expression))).into_rc_dyn();
    let annotation = annotation.set(tag("annotation", seq!(python_literal(":"), &expression))).into_rc_dyn();
    let param_star_annotation = param_star_annotation.set(tag("param_star_annotation", seq!(&NAME, &star_annotation))).into_rc_dyn();
    let param = param.set(tag("param", seq!(&NAME, opt(&annotation)))).into_rc_dyn();
    let param_maybe_default = param_maybe_default.set(tag("param_maybe_default", seq!(&param, opt(&default), opt(choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), &TYPE_COMMENT))))).into_rc_dyn();
    let param_with_default = param_with_default.set(tag("param_with_default", seq!(&param, &default, opt(choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), &TYPE_COMMENT))))).into_rc_dyn();
    let param_no_default_star_annotation = param_no_default_star_annotation.set(tag("param_no_default_star_annotation", seq!(&param_star_annotation, opt(choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), &TYPE_COMMENT))))).into_rc_dyn();
    let param_no_default = param_no_default.set(tag("param_no_default", seq!(&param, opt(choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), &TYPE_COMMENT))))).into_rc_dyn();
    let kwds = kwds.set(tag("kwds", seq!(python_literal("**"), &param_no_default))).into_rc_dyn();
    let star_etc = star_etc.set(tag("star_etc", choice!(
        seq!(python_literal("*"), choice!(seq!(&param_no_default, opt(repeat1(&param_maybe_default)), opt(&kwds)), seq!(&param_no_default_star_annotation, opt(repeat1(&param_maybe_default)), opt(&kwds)), seq!(python_literal(","), repeat1(&param_maybe_default), opt(&kwds)))),
        &kwds
    ))).into_rc_dyn();
    let slash_with_default = slash_with_default.set(tag("slash_with_default", seq!(opt(repeat1(&param_no_default)), repeat1(&param_with_default), python_literal("/"), opt(python_literal(",")), opt(repeat1(seq!(repeat1(&param_with_default), python_literal("/"), opt(python_literal(",")))))))).into_rc_dyn();
    let slash_no_default = slash_no_default.set(tag("slash_no_default", seq!(repeat1(&param_no_default), python_literal("/"), opt(python_literal(","))))).into_rc_dyn();
    let parameters = parameters.set(tag("parameters", choice!(
        seq!(&slash_no_default, opt(repeat1(&param_no_default)), opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(&slash_with_default, opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(repeat1(&param_no_default), opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(repeat1(&param_with_default), opt(&star_etc)),
        &star_etc
    ))).into_rc_dyn();
    let params = params.set(tag("params", &parameters)).into_rc_dyn();
    let function_def_raw = function_def_raw.set(tag("function_def_raw", choice!(
        seq!(python_literal("def"), &NAME, opt(&type_params), python_literal("("), opt(&params), python_literal(")"), opt(seq!(python_literal("->"), &expression)), python_literal(":"), opt(&func_type_comment), &block),
        seq!(python_literal("async"), python_literal("def"), &NAME, opt(&type_params), python_literal("("), opt(&params), python_literal(")"), opt(seq!(python_literal("->"), &expression)), python_literal(":"), opt(&func_type_comment), &block)
    ))).into_rc_dyn();
    let function_def = function_def.set(tag("function_def", choice!(
        seq!(python_literal("@"), &named_expression, &NEWLINE, opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE))), &function_def_raw),
        &function_def_raw
    ))).into_rc_dyn();
    let class_def_raw = class_def_raw.set(tag("class_def_raw", seq!(python_literal("class"), &NAME, opt(&type_params), opt(seq!(python_literal("("), opt(&arguments), python_literal(")"))), python_literal(":"), &block))).into_rc_dyn();
    let class_def = class_def.set(tag("class_def", choice!(
        seq!(python_literal("@"), &named_expression, &NEWLINE, opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE))), &class_def_raw),
        &class_def_raw
    ))).into_rc_dyn();
    let decorators = decorators.set(tag("decorators", seq!(python_literal("@"), &named_expression, &NEWLINE, opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE)))))).into_rc_dyn();
    let block = block.set(tag("block", choice!(
        seq!(&NEWLINE, &INDENT, &statements, &DEDENT),
        seq!(&simple_stmt, choice!(&NEWLINE, seq!(opt(repeat1(seq!(python_literal(";"), &simple_stmt))), opt(python_literal(";")), &NEWLINE)))
    ))).into_rc_dyn();
    let dotted_name = dotted_name.set(tag("dotted_name", seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME)))))).into_rc_dyn();
    let dotted_as_name = dotted_as_name.set(tag("dotted_as_name", seq!(&dotted_name, opt(seq!(python_literal("as"), &NAME))))).into_rc_dyn();
    let dotted_as_names = dotted_as_names.set(tag("dotted_as_names", seq!(&dotted_as_name, opt(repeat1(seq!(python_literal(","), &dotted_as_name)))))).into_rc_dyn();
    let import_from_as_name = import_from_as_name.set(tag("import_from_as_name", seq!(&NAME, opt(seq!(python_literal("as"), &NAME))))).into_rc_dyn();
    let import_from_as_names = import_from_as_names.set(tag("import_from_as_names", seq!(&import_from_as_name, opt(repeat1(seq!(python_literal(","), &import_from_as_name)))))).into_rc_dyn();
    let import_from_targets = import_from_targets.set(tag("import_from_targets", choice!(
        seq!(python_literal("("), &import_from_as_names, opt(python_literal(",")), python_literal(")")),
        &import_from_as_names,
        python_literal("*")
    ))).into_rc_dyn();
    let import_from = import_from.set(tag("import_from", seq!(python_literal("from"), choice!(seq!(opt(repeat1(choice!(python_literal("."), python_literal("...")))), &dotted_name, python_literal("import"), &import_from_targets), seq!(repeat1(choice!(python_literal("."), python_literal("..."))), python_literal("import"), &import_from_targets))))).into_rc_dyn();
    let import_name = import_name.set(tag("import_name", seq!(python_literal("import"), &dotted_as_names))).into_rc_dyn();
    let import_stmt = import_stmt.set(tag("import_stmt", choice!(
        &import_name,
        &import_from
    ))).into_rc_dyn();
    let assert_stmt = assert_stmt.set(tag("assert_stmt", seq!(python_literal("assert"), &expression, opt(seq!(python_literal(","), &expression))))).into_rc_dyn();
    let yield_stmt = yield_stmt.set(tag("yield_stmt", &yield_expr)).into_rc_dyn();
    let del_stmt = del_stmt.set(tag("del_stmt", seq!(python_literal("del"), &del_targets))).into_rc_dyn();
    let nonlocal_stmt = nonlocal_stmt.set(tag("nonlocal_stmt", seq!(python_literal("nonlocal"), &NAME, opt(repeat1(seq!(python_literal(","), &NAME)))))).into_rc_dyn();
    let global_stmt = global_stmt.set(tag("global_stmt", seq!(python_literal("global"), &NAME, opt(repeat1(seq!(python_literal(","), &NAME)))))).into_rc_dyn();
    let raise_stmt = raise_stmt.set(tag("raise_stmt", seq!(python_literal("raise"), opt(seq!(&expression, opt(seq!(python_literal("from"), &expression))))))).into_rc_dyn();
    let return_stmt = return_stmt.set(tag("return_stmt", seq!(python_literal("return"), opt(&star_expressions)))).into_rc_dyn();
    let augassign = augassign.set(tag("augassign", choice!(
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
    ))).into_rc_dyn();
    let annotated_rhs = annotated_rhs.set(tag("annotated_rhs", choice!(
        &yield_expr,
        &star_expressions
    ))).into_rc_dyn();
    let assignment = assignment.set(tag("assignment", choice!(
        seq!(&NAME, python_literal(":"), &expression, opt(seq!(python_literal("="), &annotated_rhs))),
        seq!(choice!(seq!(python_literal("("), &single_target, python_literal(")")), &single_subscript_attribute_target), python_literal(":"), &expression, opt(seq!(python_literal("="), &annotated_rhs))),
        seq!(&star_targets, python_literal("="), opt(repeat1(seq!(&star_targets, python_literal("=")))), choice!(&yield_expr, &star_expressions), opt(&TYPE_COMMENT)),
        seq!(&single_target, &augassign, cut(), choice!(&yield_expr, &star_expressions))
    ))).into_rc_dyn();
    let compound_stmt = compound_stmt.set(tag("compound_stmt", seq!(choice!(&function_def, &if_stmt, &class_def, &with_stmt, &for_stmt, &try_stmt, &while_stmt, &match_stmt), opt(repeat1(choice!(&function_def, &if_stmt, &class_def, &with_stmt, &for_stmt, &try_stmt, &while_stmt)))))).into_rc_dyn();
    let simple_stmt = simple_stmt.set(tag("simple_stmt", seq!(choice!(&assignment, &type_alias, &star_expressions, &return_stmt, &import_stmt, &raise_stmt, python_literal("pass"), &del_stmt, &yield_stmt, &assert_stmt, python_literal("break"), python_literal("continue"), &global_stmt, &nonlocal_stmt), opt(repeat1(choice!(&type_alias, &return_stmt, &import_stmt, &raise_stmt, &del_stmt, &yield_stmt, &assert_stmt, &global_stmt, &nonlocal_stmt)))))).into_rc_dyn();
    let simple_stmts = simple_stmts.set(tag("simple_stmts", seq!(&simple_stmt, choice!(&NEWLINE, seq!(opt(repeat1(seq!(python_literal(";"), &simple_stmt))), opt(python_literal(";")), &NEWLINE))))).into_rc_dyn();
    let statement_newline = statement_newline.set(tag("statement_newline", choice!(
        seq!(&compound_stmt, &NEWLINE),
        &simple_stmts,
        &NEWLINE,
        &ENDMARKER
    ))).into_rc_dyn();
    let statement = statement.set(tag("statement", choice!(
        &compound_stmt,
        &simple_stmts
    ))).into_rc_dyn();
    let statements = statements.set(tag("statements", repeat1(&statement))).into_rc_dyn();
    let func_type = func_type.set(tag("func_type", seq!(python_literal("("), opt(&type_expressions), python_literal(")"), python_literal("->"), &expression, opt(repeat1(&NEWLINE)), &ENDMARKER))).into_rc_dyn();
    let eval = eval.set(tag("eval", seq!(&expressions, opt(repeat1(&NEWLINE)), &ENDMARKER))).into_rc_dyn();
    let interactive = interactive.set(tag("interactive", &statement_newline)).into_rc_dyn();
    let file = file.set(tag("file", seq!(opt(&statements), repeat1(&ENDMARKER)))).into_rc_dyn();

    seq!(repeat0(NEWLINE), file).into_rc_dyn()
}
