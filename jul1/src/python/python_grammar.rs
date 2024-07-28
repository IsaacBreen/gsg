use std::rc::Rc;
use crate::{cache_context, cached, choice, Choice, Combinator, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, forward_decls, forward_ref, opt, Repeat1, seprep0, seprep1, Seq, tag};
use super::python_tokenizer::{WS, NAME, TYPE_COMMENT, FSTRING_START, FSTRING_MIDDLE, FSTRING_END, NUMBER, STRING, NEWLINE, INDENT, DEDENT, ENDMARKER};
use super::python_tokenizer::python_literal;
use crate::{seq, repeat0, repeat1};

pub fn python_file() -> Combinator {
    let WS = tag("WS", seq!(forbid_follows_check_not("WS"), WS(), forbid_follows(&["DEDENT","INDENT","NEWLINE"])));
    let NAME = tag("NAME", seq!(forbid_follows_check_not("NAME"), NAME(), forbid_follows(&["NAME","NUMBER"])));
    let TYPE_COMMENT = tag("TYPE_COMMENT", seq!(forbid_follows_check_not("TYPE_COMMENT"), TYPE_COMMENT(), forbid_follows(&[])));
    let FSTRING_START = tag("FSTRING_START", seq!(forbid_follows_check_not("FSTRING_START"), FSTRING_START(), forbid_follows(&["WS"])));
    let FSTRING_MIDDLE = tag("FSTRING_MIDDLE", seq!(forbid_follows_check_not("FSTRING_MIDDLE"), FSTRING_MIDDLE(), forbid_follows(&["WS"])));
    let FSTRING_END = tag("FSTRING_END", seq!(forbid_follows_check_not("FSTRING_END"), FSTRING_END(), forbid_follows(&[])));
    let NUMBER = tag("NUMBER", seq!(forbid_follows_check_not("NUMBER"), NUMBER(), forbid_follows(&["NUMBER"])));
    let STRING = tag("STRING", seq!(forbid_follows_check_not("STRING"), STRING(), forbid_follows(&[])));
    let NEWLINE = tag("NEWLINE", seq!(forbid_follows_check_not("NEWLINE"), NEWLINE(), forbid_follows(&["WS"])));
    let INDENT = tag("INDENT", seq!(forbid_follows_check_not("INDENT"), INDENT(), forbid_follows(&["WS"])));
    let DEDENT = tag("DEDENT", seq!(forbid_follows_check_not("DEDENT"), DEDENT(), forbid_follows(&["WS"])));
    let ENDMARKER = tag("ENDMARKER", seq!(forbid_follows_check_not("ENDMARKER"), ENDMARKER(), forbid_follows(&[])));

    forward_decls!(expression_without_invalid, func_type_comment, type_expressions, del_t_atom, del_target, del_targets, t_lookahead, t_primary, single_subscript_attribute_target, single_target, star_atom, target_with_star_atom, star_target, star_targets_tuple_seq, star_targets_list_seq, star_targets, kwarg_or_double_starred, kwarg_or_starred, starred_expression, kwargs, args, arguments, dictcomp, genexp, setcomp, listcomp, for_if_clause, for_if_clauses, kvpair, double_starred_kvpair, double_starred_kvpairs, dict, set, tuple, list, strings, string, fstring, fstring_format_spec, fstring_full_format_spec, fstring_conversion, fstring_replacement_field, fstring_middle, lambda_param, lambda_param_maybe_default, lambda_param_with_default, lambda_param_no_default, lambda_kwds, lambda_star_etc, lambda_slash_with_default, lambda_slash_no_default, lambda_parameters, lambda_params, lambdef, group, atom, slice, slices, primary, await_primary, power, factor, term, sum, shift_expr, bitwise_and, bitwise_xor, bitwise_or, is_bitwise_or, isnot_bitwise_or, in_bitwise_or, notin_bitwise_or, gt_bitwise_or, gte_bitwise_or, lt_bitwise_or, lte_bitwise_or, noteq_bitwise_or, eq_bitwise_or, compare_op_bitwise_or_pair, comparison, inversion, conjunction, disjunction, named_expression, assignment_expression, star_named_expression, star_named_expressions, star_expression, star_expressions, yield_expr, expression, expressions, type_param_starred_default, type_param_default, type_param_bound, type_param, type_param_seq, type_params, type_alias, keyword_pattern, keyword_patterns, positional_patterns, class_pattern, double_star_pattern, key_value_pattern, items_pattern, mapping_pattern, star_pattern, maybe_star_pattern, maybe_sequence_pattern, open_sequence_pattern, sequence_pattern, group_pattern, name_or_attr, attr, value_pattern, wildcard_pattern, pattern_capture_target, capture_pattern, imaginary_number, real_number, signed_real_number, signed_number, complex_number, literal_expr, literal_pattern, closed_pattern, or_pattern, as_pattern, pattern, patterns, guard, case_block, subject_expr, match_stmt, finally_block, except_star_block, except_block, try_stmt, with_item, with_stmt, for_stmt, while_stmt, else_block, elif_stmt, if_stmt, default, star_annotation, annotation, param_star_annotation, param, param_maybe_default, param_with_default, param_no_default_star_annotation, param_no_default, kwds, star_etc, slash_with_default, slash_no_default, parameters, params, function_def_raw, function_def, class_def_raw, class_def, decorators, block, dotted_name, dotted_as_name, dotted_as_names, import_from_as_name, import_from_as_names, import_from_targets, import_from, import_name, import_stmt, assert_stmt, yield_stmt, del_stmt, nonlocal_stmt, global_stmt, raise_stmt, return_stmt, augassign, annotated_rhs, assignment, compound_stmt, simple_stmt, simple_stmts, statement_newline, statement, statements, func_type, eval, interactive, file);

    let expression_without_invalid = expression_without_invalid.set(tag("expression_without_invalid", choice!(
        seq!(conjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("or"), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), conjunction.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("or"), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), conjunction.clone()))))), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))),
        seq!(python_literal("lambda"), opt(seq!(opt(WS.clone()), lambda_params.clone())), opt(WS.clone()), python_literal(":"), opt(WS.clone()), expression.clone())
    )));
    let func_type_comment = func_type_comment.set(tag("func_type_comment", choice!(
        seq!(NEWLINE.clone(), opt(WS.clone()), TYPE_COMMENT.clone()),
        TYPE_COMMENT.clone()
    )));
    let type_expressions = type_expressions.set(tag("type_expressions", choice!(
        seq!(choice!(seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))), lambdef.clone()), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), expression.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), expression.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), choice!(seq!(python_literal("*"), opt(WS.clone()), expression.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), python_literal("**"), opt(WS.clone()), expression.clone()))), seq!(python_literal("**"), opt(WS.clone()), expression.clone()))))),
        seq!(python_literal("*"), opt(WS.clone()), expression.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), python_literal("**"), opt(WS.clone()), expression.clone()))),
        seq!(python_literal("**"), opt(WS.clone()), expression.clone())
    )));
    let del_t_atom = del_t_atom.set(tag("del_t_atom", choice!(
        NAME.clone(),
        seq!(python_literal("("), opt(WS.clone()), choice!(seq!(del_target.clone(), opt(WS.clone()), python_literal(")")), seq!(opt(seq!(del_targets.clone(), opt(WS.clone()))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(opt(WS.clone()), del_targets.clone())), opt(WS.clone()), python_literal("]"))
    )));
    let del_target = del_target.set(cached(tag("del_target", choice!(
        seq!(choice!(NAME.clone(), python_literal("True"), python_literal("False"), python_literal("None"), strings.clone(), NUMBER.clone(), tuple.clone(), group.clone(), genexp.clone(), list.clone(), listcomp.clone(), dict.clone(), set.clone(), dictcomp.clone(), setcomp.clone(), python_literal("...")), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), NAME.clone()), seq!(python_literal("["), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), slices.clone(), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), python_literal("]")), genexp.clone(), seq!(python_literal("("), opt(seq!(opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), arguments.clone())), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), python_literal(")"))), opt(repeat1(seq!(opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), NAME.clone()), seq!(python_literal("["), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), slices.clone(), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), python_literal("]")), genexp.clone(), seq!(python_literal("("), opt(seq!(opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), arguments.clone())), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), python_literal(")")))))))), opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), NAME.clone()), seq!(python_literal("["), opt(WS.clone()), slices.clone(), opt(WS.clone()), python_literal("]")))),
        del_t_atom.clone()
    ))));
    let del_targets = del_targets.set(tag("del_targets", seq!(del_target.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), del_target.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), del_target.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let t_lookahead = t_lookahead.set(tag("t_lookahead", choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    )));
    let t_primary = t_primary.set(tag("t_primary", seq!(choice!(NAME.clone(), python_literal("True"), python_literal("False"), python_literal("None"), strings.clone(), NUMBER.clone(), tuple.clone(), group.clone(), genexp.clone(), list.clone(), listcomp.clone(), dict.clone(), set.clone(), dictcomp.clone(), setcomp.clone(), python_literal("...")), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), NAME.clone()), seq!(python_literal("["), opt(WS.clone()), slices.clone(), opt(WS.clone()), python_literal("]")), genexp.clone(), seq!(python_literal("("), opt(seq!(opt(WS.clone()), arguments.clone())), opt(WS.clone()), python_literal(")"))), opt(repeat1(seq!(opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), NAME.clone()), seq!(python_literal("["), opt(WS.clone()), slices.clone(), opt(WS.clone()), python_literal("]")), genexp.clone(), seq!(python_literal("("), opt(seq!(opt(WS.clone()), arguments.clone())), opt(WS.clone()), python_literal(")")))))))))));
    let single_subscript_attribute_target = single_subscript_attribute_target.set(tag("single_subscript_attribute_target", seq!(t_primary.clone(), opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), NAME.clone()), seq!(python_literal("["), opt(WS.clone()), slices.clone(), opt(WS.clone()), python_literal("]"))))));
    let single_target = single_target.set(tag("single_target", choice!(
        single_subscript_attribute_target.clone(),
        NAME.clone(),
        seq!(python_literal("("), opt(WS.clone()), single_target.clone(), opt(WS.clone()), python_literal(")"))
    )));
    let star_atom = star_atom.set(tag("star_atom", choice!(
        NAME.clone(),
        seq!(python_literal("("), opt(WS.clone()), choice!(seq!(target_with_star_atom.clone(), opt(WS.clone()), python_literal(")")), seq!(opt(seq!(star_targets_tuple_seq.clone(), opt(WS.clone()))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(opt(WS.clone()), star_targets_list_seq.clone())), opt(WS.clone()), python_literal("]"))
    )));
    let target_with_star_atom = target_with_star_atom.set(cached(tag("target_with_star_atom", choice!(
        seq!(t_primary.clone(), opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), NAME.clone()), seq!(python_literal("["), opt(WS.clone()), slices.clone(), opt(WS.clone()), python_literal("]")))),
        star_atom.clone()
    ))));
    let star_target = star_target.set(cached(tag("star_target", choice!(
        seq!(python_literal("*"), opt(WS.clone()), star_target.clone()),
        target_with_star_atom.clone()
    ))));
    let star_targets_tuple_seq = star_targets_tuple_seq.set(tag("star_targets_tuple_seq", seq!(star_target.clone(), opt(WS.clone()), python_literal(","), opt(seq!(opt(WS.clone()), star_target.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), star_target.clone()))), opt(seq!(opt(WS.clone()), python_literal(","))))))));
    let star_targets_list_seq = star_targets_list_seq.set(tag("star_targets_list_seq", seq!(star_target.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), star_target.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), star_target.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let star_targets = star_targets.set(tag("star_targets", seq!(star_target.clone(), opt(seq!(opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), star_target.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), star_target.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))))))));
    let kwarg_or_double_starred = kwarg_or_double_starred.set(tag("kwarg_or_double_starred", choice!(
        seq!(NAME.clone(), opt(WS.clone()), python_literal("="), opt(WS.clone()), expression.clone()),
        seq!(python_literal("**"), opt(WS.clone()), expression.clone())
    )));
    let kwarg_or_starred = kwarg_or_starred.set(tag("kwarg_or_starred", choice!(
        seq!(NAME.clone(), opt(WS.clone()), python_literal("="), opt(WS.clone()), expression.clone()),
        seq!(python_literal("*"), opt(WS.clone()), expression.clone())
    )));
    let starred_expression = starred_expression.set(tag("starred_expression", seq!(python_literal("*"), opt(WS.clone()), expression.clone())));
    let kwargs = kwargs.set(tag("kwargs", choice!(
        seq!(kwarg_or_starred.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), kwarg_or_starred.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), kwarg_or_starred.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), kwarg_or_double_starred.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), kwarg_or_double_starred.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), kwarg_or_double_starred.clone())))))))),
        seq!(kwarg_or_double_starred.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), kwarg_or_double_starred.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), kwarg_or_double_starred.clone()))))))
    )));
    let args = args.set(tag("args", choice!(
        seq!(choice!(starred_expression.clone(), seq!(NAME.clone(), opt(WS.clone()), python_literal(":="), opt(WS.clone()), expression.clone()), seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))), lambdef.clone()), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), choice!(starred_expression.clone(), seq!(NAME.clone(), opt(WS.clone()), python_literal(":="), opt(WS.clone()), expression.clone()), seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))), lambdef.clone()), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), choice!(starred_expression.clone(), seq!(NAME.clone(), opt(WS.clone()), python_literal(":="), opt(WS.clone()), expression.clone()), seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))), lambdef.clone())))))), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), kwargs.clone()))),
        kwargs.clone()
    )));
    let arguments = arguments.set(cached(tag("arguments", seq!(args.clone(), opt(seq!(opt(WS.clone()), python_literal(",")))))));
    let dictcomp = dictcomp.set(tag("dictcomp", seq!(
        python_literal("{"),
        opt(WS.clone()),
        kvpair.clone(),
        opt(WS.clone()),
        for_if_clauses.clone(),
        opt(WS.clone()),
        python_literal("}")
    )));
    let genexp = genexp.set(tag("genexp", seq!(
        python_literal("("),
        opt(WS.clone()),
        choice!(assignment_expression.clone(), expression.clone()),
        opt(WS.clone()),
        for_if_clauses.clone(),
        opt(WS.clone()),
        python_literal(")")
    )));
    let setcomp = setcomp.set(tag("setcomp", seq!(
        python_literal("{"),
        opt(WS.clone()),
        named_expression.clone(),
        opt(WS.clone()),
        for_if_clauses.clone(),
        opt(WS.clone()),
        python_literal("}")
    )));
    let listcomp = listcomp.set(tag("listcomp", seq!(
        python_literal("["),
        opt(WS.clone()),
        named_expression.clone(),
        opt(WS.clone()),
        for_if_clauses.clone(),
        opt(WS.clone()),
        python_literal("]")
    )));
    let for_if_clause = for_if_clause.set(tag("for_if_clause", choice!(
        seq!(python_literal("async"), opt(WS.clone()), python_literal("for"), opt(WS.clone()), star_targets.clone(), opt(WS.clone()), python_literal("in"), opt(WS.clone()), disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone())))))),
        seq!(python_literal("for"), opt(WS.clone()), star_targets.clone(), opt(WS.clone()), python_literal("in"), opt(WS.clone()), disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone()))))))
    )));
    let for_if_clauses = for_if_clauses.set(tag("for_if_clauses", seq!(for_if_clause.clone(), opt(repeat1(seq!(opt(WS.clone()), for_if_clause.clone()))))));
    let kvpair = kvpair.set(tag("kvpair", seq!(
        choice!(seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))), lambdef.clone()),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        expression.clone()
    )));
    let double_starred_kvpair = double_starred_kvpair.set(tag("double_starred_kvpair", choice!(
        seq!(python_literal("**"), opt(WS.clone()), bitwise_or.clone()),
        kvpair.clone()
    )));
    let double_starred_kvpairs = double_starred_kvpairs.set(tag("double_starred_kvpairs", seq!(double_starred_kvpair.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), double_starred_kvpair.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), double_starred_kvpair.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let dict = dict.set(tag("dict", seq!(python_literal("{"), opt(seq!(opt(WS.clone()), double_starred_kvpairs.clone())), opt(WS.clone()), python_literal("}"))));
    let set = set.set(tag("set", seq!(
        python_literal("{"),
        opt(WS.clone()),
        star_named_expressions.clone(),
        opt(WS.clone()),
        python_literal("}")
    )));
    let tuple = tuple.set(tag("tuple", seq!(python_literal("("), opt(seq!(opt(WS.clone()), star_named_expression.clone(), opt(WS.clone()), python_literal(","), opt(seq!(opt(WS.clone()), star_named_expressions.clone())))), opt(WS.clone()), python_literal(")"))));
    let list = list.set(tag("list", seq!(python_literal("["), opt(seq!(opt(WS.clone()), star_named_expressions.clone())), opt(WS.clone()), python_literal("]"))));
    let strings = strings.set(cached(tag("strings", seq!(choice!(seq!(FSTRING_START.clone(), opt(seq!(opt(WS.clone()), fstring_middle.clone(), opt(repeat1(seq!(opt(WS.clone()), fstring_middle.clone()))))), opt(WS.clone()), FSTRING_END.clone()), STRING.clone()), opt(repeat1(seq!(opt(WS.clone()), choice!(seq!(FSTRING_START.clone(), opt(seq!(opt(WS.clone()), fstring_middle.clone(), opt(repeat1(seq!(opt(WS.clone()), fstring_middle.clone()))))), opt(WS.clone()), FSTRING_END.clone()), STRING.clone()))))))));
    let string = string.set(tag("string", STRING.clone()));
    let fstring = fstring.set(tag("fstring", seq!(FSTRING_START.clone(), opt(seq!(opt(WS.clone()), fstring_middle.clone(), opt(repeat1(seq!(opt(WS.clone()), fstring_middle.clone()))))), opt(WS.clone()), FSTRING_END.clone())));
    let fstring_format_spec = fstring_format_spec.set(tag("fstring_format_spec", choice!(
        FSTRING_MIDDLE.clone(),
        seq!(python_literal("{"), opt(WS.clone()), annotated_rhs.clone(), opt(seq!(opt(WS.clone()), python_literal("="))), opt(seq!(opt(WS.clone()), fstring_conversion.clone())), opt(seq!(opt(WS.clone()), fstring_full_format_spec.clone())), opt(WS.clone()), python_literal("}"))
    )));
    let fstring_full_format_spec = fstring_full_format_spec.set(tag("fstring_full_format_spec", seq!(python_literal(":"), opt(seq!(opt(WS.clone()), fstring_format_spec.clone(), opt(repeat1(seq!(opt(WS.clone()), fstring_format_spec.clone()))))))));
    let fstring_conversion = fstring_conversion.set(tag("fstring_conversion", seq!(python_literal("!"), opt(WS.clone()), NAME.clone())));
    let fstring_replacement_field = fstring_replacement_field.set(tag("fstring_replacement_field", seq!(
        python_literal("{"),
        opt(WS.clone()),
        annotated_rhs.clone(),
        opt(seq!(opt(WS.clone()), python_literal("="))),
        opt(seq!(opt(WS.clone()), fstring_conversion.clone())),
        opt(seq!(opt(WS.clone()), fstring_full_format_spec.clone())),
        opt(WS.clone()),
        python_literal("}")
    )));
    let fstring_middle = fstring_middle.set(tag("fstring_middle", choice!(
        fstring_replacement_field.clone(),
        FSTRING_MIDDLE.clone()
    )));
    let lambda_param = lambda_param.set(tag("lambda_param", NAME.clone()));
    let lambda_param_maybe_default = lambda_param_maybe_default.set(tag("lambda_param_maybe_default", seq!(lambda_param.clone(), opt(seq!(opt(WS.clone()), default.clone())), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let lambda_param_with_default = lambda_param_with_default.set(tag("lambda_param_with_default", seq!(lambda_param.clone(), opt(WS.clone()), default.clone(), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let lambda_param_no_default = lambda_param_no_default.set(tag("lambda_param_no_default", seq!(lambda_param.clone(), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let lambda_kwds = lambda_kwds.set(tag("lambda_kwds", seq!(python_literal("**"), opt(WS.clone()), lambda_param_no_default.clone())));
    let lambda_star_etc = lambda_star_etc.set(tag("lambda_star_etc", choice!(
        seq!(python_literal("*"), opt(WS.clone()), choice!(seq!(lambda_param_no_default.clone(), opt(seq!(opt(WS.clone()), lambda_param_maybe_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_maybe_default.clone()))))), opt(seq!(opt(WS.clone()), lambda_kwds.clone()))), seq!(python_literal(","), opt(WS.clone()), lambda_param_maybe_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_maybe_default.clone()))), opt(seq!(opt(WS.clone()), lambda_kwds.clone()))))),
        lambda_kwds.clone()
    )));
    let lambda_slash_with_default = lambda_slash_with_default.set(tag("lambda_slash_with_default", seq!(
        opt(seq!(lambda_param_no_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_no_default.clone()))), opt(WS.clone()))),
        lambda_param_with_default.clone(),
        opt(repeat1(seq!(opt(WS.clone()), lambda_param_with_default.clone()))),
        opt(WS.clone()),
        python_literal("/"),
        opt(seq!(opt(WS.clone()), python_literal(",")))
    )));
    let lambda_slash_no_default = lambda_slash_no_default.set(tag("lambda_slash_no_default", seq!(
        lambda_param_no_default.clone(),
        opt(repeat1(seq!(opt(WS.clone()), lambda_param_no_default.clone()))),
        opt(WS.clone()),
        python_literal("/"),
        opt(seq!(opt(WS.clone()), python_literal(",")))
    )));
    let lambda_parameters = lambda_parameters.set(tag("lambda_parameters", choice!(
        seq!(lambda_slash_no_default.clone(), opt(seq!(opt(WS.clone()), lambda_param_no_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_no_default.clone()))))), opt(seq!(opt(WS.clone()), lambda_param_with_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_with_default.clone()))))), opt(seq!(opt(WS.clone()), lambda_star_etc.clone()))),
        seq!(lambda_slash_with_default.clone(), opt(seq!(opt(WS.clone()), lambda_param_with_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_with_default.clone()))))), opt(seq!(opt(WS.clone()), lambda_star_etc.clone()))),
        seq!(lambda_param_no_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_no_default.clone()))), opt(seq!(opt(WS.clone()), lambda_param_with_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_with_default.clone()))))), opt(seq!(opt(WS.clone()), lambda_star_etc.clone()))),
        seq!(lambda_param_with_default.clone(), opt(repeat1(seq!(opt(WS.clone()), lambda_param_with_default.clone()))), opt(seq!(opt(WS.clone()), lambda_star_etc.clone()))),
        lambda_star_etc.clone()
    )));
    let lambda_params = lambda_params.set(tag("lambda_params", lambda_parameters.clone()));
    let lambdef = lambdef.set(tag("lambdef", seq!(
        python_literal("lambda"),
        opt(seq!(opt(WS.clone()), lambda_params.clone())),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        expression.clone()
    )));
    let group = group.set(tag("group", seq!(
        python_literal("("),
        opt(WS.clone()),
        choice!(yield_expr.clone(), named_expression.clone()),
        opt(WS.clone()),
        python_literal(")")
    )));
    let atom = atom.set(tag("atom", choice!(
        NAME.clone(),
        python_literal("True"),
        python_literal("False"),
        python_literal("None"),
        strings.clone(),
        NUMBER.clone(),
        tuple.clone(),
        group.clone(),
        genexp.clone(),
        list.clone(),
        listcomp.clone(),
        dict.clone(),
        set.clone(),
        dictcomp.clone(),
        setcomp.clone(),
        python_literal("...")
    )));
    let slice = slice.set(tag("slice", choice!(
        seq!(opt(choice!(seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone())), opt(WS.clone())), seq!(lambdef.clone(), opt(WS.clone())))), python_literal(":"), opt(seq!(opt(WS.clone()), expression.clone())), opt(seq!(opt(WS.clone()), python_literal(":"), opt(seq!(opt(WS.clone()), expression.clone()))))),
        seq!(NAME.clone(), opt(WS.clone()), python_literal(":="), opt(WS.clone()), expression.clone()),
        seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))),
        lambdef.clone()
    )));
    let slices = slices.set(tag("slices", choice!(
        slice.clone(),
        seq!(choice!(slice.clone(), starred_expression.clone()), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), choice!(slice.clone(), starred_expression.clone()), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), choice!(slice.clone(), starred_expression.clone())))))), opt(seq!(opt(WS.clone()), python_literal(","))))
    )));
    let primary = primary.set(tag("primary", seq!(atom.clone(), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), NAME.clone()), genexp.clone(), seq!(python_literal("("), opt(seq!(opt(WS.clone()), arguments.clone())), opt(WS.clone()), python_literal(")")), seq!(python_literal("["), opt(WS.clone()), slices.clone(), opt(WS.clone()), python_literal("]"))), opt(repeat1(seq!(opt(WS.clone()), choice!(seq!(python_literal("."), opt(WS.clone()), NAME.clone()), genexp.clone(), seq!(python_literal("("), opt(seq!(opt(WS.clone()), arguments.clone())), opt(WS.clone()), python_literal(")")), seq!(python_literal("["), opt(WS.clone()), slices.clone(), opt(WS.clone()), python_literal("]")))))))))));
    let await_primary = await_primary.set(cached(tag("await_primary", choice!(
        seq!(python_literal("await"), opt(WS.clone()), primary.clone()),
        primary.clone()
    ))));
    let power = power.set(tag("power", seq!(await_primary.clone(), opt(seq!(opt(WS.clone()), python_literal("**"), opt(WS.clone()), factor.clone())))));
    let factor = factor.set(cached(tag("factor", choice!(
        seq!(python_literal("+"), opt(WS.clone()), factor.clone()),
        seq!(python_literal("-"), opt(WS.clone()), factor.clone()),
        seq!(python_literal("~"), opt(WS.clone()), factor.clone()),
        power.clone()
    ))));
    let term = term.set(tag("term", seq!(factor.clone(), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal("*"), opt(WS.clone()), factor.clone()), seq!(python_literal("/"), opt(WS.clone()), factor.clone()), seq!(python_literal("//"), opt(WS.clone()), factor.clone()), seq!(python_literal("%"), opt(WS.clone()), factor.clone()), seq!(python_literal("@"), opt(WS.clone()), factor.clone())), opt(repeat1(seq!(opt(WS.clone()), choice!(seq!(python_literal("*"), opt(WS.clone()), factor.clone()), seq!(python_literal("/"), opt(WS.clone()), factor.clone()), seq!(python_literal("//"), opt(WS.clone()), factor.clone()), seq!(python_literal("%"), opt(WS.clone()), factor.clone()), seq!(python_literal("@"), opt(WS.clone()), factor.clone()))))))))));
    let sum = sum.set(tag("sum", seq!(term.clone(), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal("+"), opt(WS.clone()), term.clone()), seq!(python_literal("-"), opt(WS.clone()), term.clone())), opt(repeat1(seq!(opt(WS.clone()), choice!(seq!(python_literal("+"), opt(WS.clone()), term.clone()), seq!(python_literal("-"), opt(WS.clone()), term.clone()))))))))));
    let shift_expr = shift_expr.set(tag("shift_expr", seq!(sum.clone(), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal("<<"), opt(WS.clone()), sum.clone()), seq!(python_literal(">>"), opt(WS.clone()), sum.clone())), opt(repeat1(seq!(opt(WS.clone()), choice!(seq!(python_literal("<<"), opt(WS.clone()), sum.clone()), seq!(python_literal(">>"), opt(WS.clone()), sum.clone()))))))))));
    let bitwise_and = bitwise_and.set(tag("bitwise_and", seq!(shift_expr.clone(), opt(seq!(opt(WS.clone()), python_literal("&"), opt(WS.clone()), shift_expr.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("&"), opt(WS.clone()), shift_expr.clone()))))))));
    let bitwise_xor = bitwise_xor.set(tag("bitwise_xor", seq!(bitwise_and.clone(), opt(seq!(opt(WS.clone()), python_literal("^"), opt(WS.clone()), bitwise_and.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("^"), opt(WS.clone()), bitwise_and.clone()))))))));
    let bitwise_or = bitwise_or.set(tag("bitwise_or", seq!(bitwise_xor.clone(), opt(seq!(opt(WS.clone()), python_literal("|"), opt(WS.clone()), bitwise_xor.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("|"), opt(WS.clone()), bitwise_xor.clone()))))))));
    let is_bitwise_or = is_bitwise_or.set(tag("is_bitwise_or", seq!(python_literal("is"), opt(WS.clone()), bitwise_or.clone())));
    let isnot_bitwise_or = isnot_bitwise_or.set(tag("isnot_bitwise_or", seq!(
        python_literal("is"),
        opt(WS.clone()),
        python_literal("not"),
        opt(WS.clone()),
        bitwise_or.clone()
    )));
    let in_bitwise_or = in_bitwise_or.set(tag("in_bitwise_or", seq!(python_literal("in"), opt(WS.clone()), bitwise_or.clone())));
    let notin_bitwise_or = notin_bitwise_or.set(tag("notin_bitwise_or", seq!(
        python_literal("not"),
        opt(WS.clone()),
        python_literal("in"),
        opt(WS.clone()),
        bitwise_or.clone()
    )));
    let gt_bitwise_or = gt_bitwise_or.set(tag("gt_bitwise_or", seq!(python_literal(">"), opt(WS.clone()), bitwise_or.clone())));
    let gte_bitwise_or = gte_bitwise_or.set(tag("gte_bitwise_or", seq!(python_literal(">="), opt(WS.clone()), bitwise_or.clone())));
    let lt_bitwise_or = lt_bitwise_or.set(tag("lt_bitwise_or", seq!(python_literal("<"), opt(WS.clone()), bitwise_or.clone())));
    let lte_bitwise_or = lte_bitwise_or.set(tag("lte_bitwise_or", seq!(python_literal("<="), opt(WS.clone()), bitwise_or.clone())));
    let noteq_bitwise_or = noteq_bitwise_or.set(tag("noteq_bitwise_or", seq!(python_literal("!="), opt(WS.clone()), bitwise_or.clone())));
    let eq_bitwise_or = eq_bitwise_or.set(tag("eq_bitwise_or", seq!(python_literal("=="), opt(WS.clone()), bitwise_or.clone())));
    let compare_op_bitwise_or_pair = compare_op_bitwise_or_pair.set(tag("compare_op_bitwise_or_pair", choice!(
        eq_bitwise_or.clone(),
        noteq_bitwise_or.clone(),
        lte_bitwise_or.clone(),
        lt_bitwise_or.clone(),
        gte_bitwise_or.clone(),
        gt_bitwise_or.clone(),
        notin_bitwise_or.clone(),
        in_bitwise_or.clone(),
        isnot_bitwise_or.clone(),
        is_bitwise_or.clone()
    )));
    let comparison = comparison.set(tag("comparison", seq!(bitwise_or.clone(), opt(seq!(opt(WS.clone()), compare_op_bitwise_or_pair.clone(), opt(repeat1(seq!(opt(WS.clone()), compare_op_bitwise_or_pair.clone()))))))));
    let inversion = inversion.set(cached(tag("inversion", choice!(
        seq!(python_literal("not"), opt(WS.clone()), inversion.clone()),
        comparison.clone()
    ))));
    let conjunction = conjunction.set(cached(tag("conjunction", seq!(inversion.clone(), opt(seq!(opt(WS.clone()), python_literal("and"), opt(WS.clone()), inversion.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("and"), opt(WS.clone()), inversion.clone())))))))));
    let disjunction = disjunction.set(cached(tag("disjunction", seq!(conjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("or"), opt(WS.clone()), conjunction.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("or"), opt(WS.clone()), conjunction.clone())))))))));
    let named_expression = named_expression.set(tag("named_expression", choice!(
        seq!(NAME.clone(), opt(WS.clone()), python_literal(":="), opt(WS.clone()), expression.clone()),
        seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))),
        lambdef.clone()
    )));
    let assignment_expression = assignment_expression.set(tag("assignment_expression", seq!(
        NAME.clone(),
        opt(WS.clone()),
        python_literal(":="),
        opt(WS.clone()),
        expression.clone()
    )));
    let star_named_expression = star_named_expression.set(tag("star_named_expression", choice!(
        seq!(python_literal("*"), opt(WS.clone()), bitwise_or.clone()),
        named_expression.clone()
    )));
    let star_named_expressions = star_named_expressions.set(tag("star_named_expressions", seq!(star_named_expression.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), star_named_expression.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), star_named_expression.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let star_expression = star_expression.set(cached(tag("star_expression", choice!(
        seq!(python_literal("*"), opt(WS.clone()), bitwise_or.clone()),
        seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))),
        lambdef.clone()
    ))));
    let star_expressions = star_expressions.set(tag("star_expressions", seq!(star_expression.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(seq!(opt(WS.clone()), star_expression.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), star_expression.clone()))), opt(seq!(opt(WS.clone()), python_literal(","))))))))));
    let yield_expr = yield_expr.set(tag("yield_expr", seq!(python_literal("yield"), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal("from"), opt(WS.clone()), expression.clone()), star_expressions.clone()))))));
    let expression = expression.set(cached(tag("expression", choice!(
        seq!(disjunction.clone(), opt(seq!(opt(WS.clone()), python_literal("if"), opt(WS.clone()), disjunction.clone(), opt(WS.clone()), python_literal("else"), opt(WS.clone()), expression.clone()))),
        lambdef.clone()
    ))));
    let expressions = expressions.set(tag("expressions", seq!(expression.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(seq!(opt(WS.clone()), expression.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), expression.clone()))), opt(seq!(opt(WS.clone()), python_literal(","))))))))));
    let type_param_starred_default = type_param_starred_default.set(tag("type_param_starred_default", seq!(python_literal("="), opt(WS.clone()), star_expression.clone())));
    let type_param_default = type_param_default.set(tag("type_param_default", seq!(python_literal("="), opt(WS.clone()), expression.clone())));
    let type_param_bound = type_param_bound.set(tag("type_param_bound", seq!(python_literal(":"), opt(WS.clone()), expression.clone())));
    let type_param = type_param.set(cached(tag("type_param", choice!(
        seq!(NAME.clone(), opt(seq!(opt(WS.clone()), type_param_bound.clone())), opt(seq!(opt(WS.clone()), type_param_default.clone()))),
        seq!(python_literal("*"), opt(WS.clone()), NAME.clone(), opt(seq!(opt(WS.clone()), type_param_starred_default.clone()))),
        seq!(python_literal("**"), opt(WS.clone()), NAME.clone(), opt(seq!(opt(WS.clone()), type_param_default.clone())))
    ))));
    let type_param_seq = type_param_seq.set(tag("type_param_seq", seq!(type_param.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), type_param.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), type_param.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let type_params = type_params.set(tag("type_params", seq!(
        python_literal("["),
        opt(WS.clone()),
        type_param_seq.clone(),
        opt(WS.clone()),
        python_literal("]")
    )));
    let type_alias = type_alias.set(tag("type_alias", seq!(
        python_literal("type"),
        opt(WS.clone()),
        NAME.clone(),
        opt(seq!(opt(WS.clone()), type_params.clone())),
        opt(WS.clone()),
        python_literal("="),
        opt(WS.clone()),
        expression.clone()
    )));
    let keyword_pattern = keyword_pattern.set(tag("keyword_pattern", seq!(
        NAME.clone(),
        opt(WS.clone()),
        python_literal("="),
        opt(WS.clone()),
        pattern.clone()
    )));
    let keyword_patterns = keyword_patterns.set(tag("keyword_patterns", seq!(keyword_pattern.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), keyword_pattern.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), keyword_pattern.clone()))))))));
    let positional_patterns = positional_patterns.set(tag("positional_patterns", seq!(choice!(as_pattern.clone(), or_pattern.clone()), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), pattern.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), pattern.clone()))))))));
    let class_pattern = class_pattern.set(tag("class_pattern", seq!(
        NAME.clone(),
        opt(seq!(opt(WS.clone()), python_literal("."), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), NAME.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("."), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), NAME.clone()))))),
        opt(WS.clone()),
        python_literal("("),
        opt(WS.clone()),
        choice!(python_literal(")"), seq!(positional_patterns.clone(), opt(WS.clone()), choice!(seq!(opt(seq!(python_literal(","), opt(WS.clone()))), python_literal(")")), seq!(python_literal(","), opt(WS.clone()), keyword_patterns.clone(), opt(seq!(opt(WS.clone()), python_literal(","))), opt(WS.clone()), python_literal(")")))), seq!(keyword_patterns.clone(), opt(seq!(opt(WS.clone()), python_literal(","))), opt(WS.clone()), python_literal(")")))
    )));
    let double_star_pattern = double_star_pattern.set(tag("double_star_pattern", seq!(python_literal("**"), opt(WS.clone()), pattern_capture_target.clone())));
    let key_value_pattern = key_value_pattern.set(tag("key_value_pattern", seq!(
        choice!(signed_number.clone(), complex_number.clone(), strings.clone(), python_literal("None"), python_literal("True"), python_literal("False"), seq!(name_or_attr.clone(), opt(WS.clone()), python_literal("."), opt(WS.clone()), NAME.clone())),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        pattern.clone()
    )));
    let items_pattern = items_pattern.set(tag("items_pattern", seq!(key_value_pattern.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), key_value_pattern.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), key_value_pattern.clone()))))))));
    let mapping_pattern = mapping_pattern.set(tag("mapping_pattern", seq!(python_literal("{"), opt(WS.clone()), choice!(python_literal("}"), seq!(double_star_pattern.clone(), opt(seq!(opt(WS.clone()), python_literal(","))), opt(WS.clone()), python_literal("}")), seq!(items_pattern.clone(), opt(WS.clone()), choice!(seq!(python_literal(","), opt(WS.clone()), double_star_pattern.clone(), opt(seq!(opt(WS.clone()), python_literal(","))), opt(WS.clone()), python_literal("}")), seq!(opt(seq!(python_literal(","), opt(WS.clone()))), python_literal("}"))))))));
    let star_pattern = star_pattern.set(cached(tag("star_pattern", seq!(python_literal("*"), opt(WS.clone()), choice!(pattern_capture_target.clone(), wildcard_pattern.clone())))));
    let maybe_star_pattern = maybe_star_pattern.set(tag("maybe_star_pattern", choice!(
        star_pattern.clone(),
        as_pattern.clone(),
        or_pattern.clone()
    )));
    let maybe_sequence_pattern = maybe_sequence_pattern.set(tag("maybe_sequence_pattern", seq!(maybe_star_pattern.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), maybe_star_pattern.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), maybe_star_pattern.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))))));
    let open_sequence_pattern = open_sequence_pattern.set(tag("open_sequence_pattern", seq!(maybe_star_pattern.clone(), opt(WS.clone()), python_literal(","), opt(seq!(opt(WS.clone()), maybe_sequence_pattern.clone())))));
    let sequence_pattern = sequence_pattern.set(tag("sequence_pattern", choice!(
        seq!(python_literal("["), opt(seq!(opt(WS.clone()), maybe_sequence_pattern.clone())), opt(WS.clone()), python_literal("]")),
        seq!(python_literal("("), opt(seq!(opt(WS.clone()), open_sequence_pattern.clone())), opt(WS.clone()), python_literal(")"))
    )));
    let group_pattern = group_pattern.set(tag("group_pattern", seq!(
        python_literal("("),
        opt(WS.clone()),
        pattern.clone(),
        opt(WS.clone()),
        python_literal(")")
    )));
    let name_or_attr = name_or_attr.set(tag("name_or_attr", seq!(NAME.clone(), opt(seq!(opt(WS.clone()), python_literal("."), opt(WS.clone()), NAME.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("."), opt(WS.clone()), NAME.clone()))))))));
    let attr = attr.set(tag("attr", seq!(
        name_or_attr.clone(),
        opt(WS.clone()),
        python_literal("."),
        opt(WS.clone()),
        NAME.clone()
    )));
    let value_pattern = value_pattern.set(tag("value_pattern", attr.clone()));
    let wildcard_pattern = wildcard_pattern.set(tag("wildcard_pattern", python_literal("_")));
    let pattern_capture_target = pattern_capture_target.set(tag("pattern_capture_target", NAME.clone()));
    let capture_pattern = capture_pattern.set(tag("capture_pattern", pattern_capture_target.clone()));
    let imaginary_number = imaginary_number.set(tag("imaginary_number", NUMBER.clone()));
    let real_number = real_number.set(tag("real_number", NUMBER.clone()));
    let signed_real_number = signed_real_number.set(tag("signed_real_number", choice!(
        real_number.clone(),
        seq!(python_literal("-"), opt(WS.clone()), real_number.clone())
    )));
    let signed_number = signed_number.set(tag("signed_number", choice!(
        NUMBER.clone(),
        seq!(python_literal("-"), opt(WS.clone()), NUMBER.clone())
    )));
    let complex_number = complex_number.set(tag("complex_number", seq!(signed_real_number.clone(), opt(WS.clone()), choice!(seq!(python_literal("+"), opt(WS.clone()), imaginary_number.clone()), seq!(python_literal("-"), opt(WS.clone()), imaginary_number.clone())))));
    let literal_expr = literal_expr.set(tag("literal_expr", choice!(
        signed_number.clone(),
        complex_number.clone(),
        strings.clone(),
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )));
    let literal_pattern = literal_pattern.set(tag("literal_pattern", choice!(
        signed_number.clone(),
        complex_number.clone(),
        strings.clone(),
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )));
    let closed_pattern = closed_pattern.set(cached(tag("closed_pattern", choice!(
        literal_pattern.clone(),
        capture_pattern.clone(),
        wildcard_pattern.clone(),
        value_pattern.clone(),
        group_pattern.clone(),
        sequence_pattern.clone(),
        mapping_pattern.clone(),
        class_pattern.clone()
    ))));
    let or_pattern = or_pattern.set(tag("or_pattern", seq!(closed_pattern.clone(), opt(seq!(opt(WS.clone()), python_literal("|"), opt(WS.clone()), closed_pattern.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("|"), opt(WS.clone()), closed_pattern.clone()))))))));
    let as_pattern = as_pattern.set(tag("as_pattern", seq!(
        or_pattern.clone(),
        opt(WS.clone()),
        python_literal("as"),
        opt(WS.clone()),
        pattern_capture_target.clone()
    )));
    let pattern = pattern.set(tag("pattern", choice!(
        as_pattern.clone(),
        or_pattern.clone()
    )));
    let patterns = patterns.set(tag("patterns", choice!(
        open_sequence_pattern.clone(),
        pattern.clone()
    )));
    let guard = guard.set(tag("guard", seq!(python_literal("if"), opt(WS.clone()), named_expression.clone())));
    let case_block = case_block.set(tag("case_block", seq!(
        python_literal("case"),
        opt(WS.clone()),
        patterns.clone(),
        opt(seq!(opt(WS.clone()), guard.clone())),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone()
    )));
    let subject_expr = subject_expr.set(tag("subject_expr", choice!(
        seq!(star_named_expression.clone(), opt(WS.clone()), python_literal(","), opt(seq!(opt(WS.clone()), star_named_expressions.clone()))),
        named_expression.clone()
    )));
    let match_stmt = match_stmt.set(tag("match_stmt", seq!(
        python_literal("match"),
        opt(WS.clone()),
        subject_expr.clone(),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        NEWLINE.clone(),
        opt(WS.clone()),
        INDENT.clone(),
        opt(WS.clone()),
        case_block.clone(),
        opt(repeat1(seq!(opt(WS.clone()), case_block.clone()))),
        opt(WS.clone()),
        DEDENT.clone()
    )));
    let finally_block = finally_block.set(tag("finally_block", seq!(
        python_literal("finally"),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone()
    )));
    let except_star_block = except_star_block.set(tag("except_star_block", seq!(
        python_literal("except"),
        opt(WS.clone()),
        python_literal("*"),
        opt(WS.clone()),
        expression.clone(),
        opt(seq!(opt(WS.clone()), python_literal("as"), opt(WS.clone()), NAME.clone())),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone()
    )));
    let except_block = except_block.set(tag("except_block", seq!(python_literal("except"), opt(WS.clone()), choice!(seq!(expression.clone(), opt(seq!(opt(WS.clone()), python_literal("as"), opt(WS.clone()), NAME.clone())), opt(WS.clone()), python_literal(":"), opt(WS.clone()), block.clone()), seq!(python_literal(":"), opt(WS.clone()), block.clone())))));
    let try_stmt = try_stmt.set(tag("try_stmt", seq!(
        python_literal("try"),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone(),
        opt(WS.clone()),
        choice!(finally_block.clone(), seq!(except_block.clone(), opt(repeat1(seq!(opt(WS.clone()), except_block.clone()))), opt(seq!(opt(WS.clone()), else_block.clone())), opt(seq!(opt(WS.clone()), finally_block.clone()))), seq!(except_star_block.clone(), opt(repeat1(seq!(opt(WS.clone()), except_star_block.clone()))), opt(seq!(opt(WS.clone()), else_block.clone())), opt(seq!(opt(WS.clone()), finally_block.clone()))))
    )));
    let with_item = with_item.set(tag("with_item", seq!(expression.clone(), opt(seq!(opt(WS.clone()), python_literal("as"), opt(WS.clone()), star_target.clone())))));
    let with_stmt = with_stmt.set(tag("with_stmt", choice!(
        seq!(python_literal("with"), opt(WS.clone()), choice!(seq!(python_literal("("), opt(WS.clone()), with_item.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), with_item.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), with_item.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))), opt(WS.clone()), python_literal(")"), opt(WS.clone()), python_literal(":"), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone())), opt(WS.clone()), block.clone()), seq!(with_item.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), with_item.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), with_item.clone()))))), opt(WS.clone()), python_literal(":"), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone())), opt(WS.clone()), block.clone()))),
        seq!(python_literal("async"), opt(WS.clone()), python_literal("with"), opt(WS.clone()), choice!(seq!(python_literal("("), opt(WS.clone()), with_item.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), with_item.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), with_item.clone()))))), opt(seq!(opt(WS.clone()), python_literal(","))), opt(WS.clone()), python_literal(")"), opt(WS.clone()), python_literal(":"), opt(WS.clone()), block.clone()), seq!(with_item.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), with_item.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), with_item.clone()))))), opt(WS.clone()), python_literal(":"), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone())), opt(WS.clone()), block.clone())))
    )));
    let for_stmt = for_stmt.set(tag("for_stmt", choice!(
        seq!(python_literal("for"), opt(WS.clone()), star_targets.clone(), opt(WS.clone()), python_literal("in"), opt(WS.clone()), star_expressions.clone(), opt(WS.clone()), python_literal(":"), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone())), opt(WS.clone()), block.clone(), opt(seq!(opt(WS.clone()), else_block.clone()))),
        seq!(python_literal("async"), opt(WS.clone()), python_literal("for"), opt(WS.clone()), star_targets.clone(), opt(WS.clone()), python_literal("in"), opt(WS.clone()), star_expressions.clone(), opt(WS.clone()), python_literal(":"), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone())), opt(WS.clone()), block.clone(), opt(seq!(opt(WS.clone()), else_block.clone())))
    )));
    let while_stmt = while_stmt.set(tag("while_stmt", seq!(
        python_literal("while"),
        opt(WS.clone()),
        named_expression.clone(),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone(),
        opt(seq!(opt(WS.clone()), else_block.clone()))
    )));
    let else_block = else_block.set(tag("else_block", seq!(
        python_literal("else"),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone()
    )));
    let elif_stmt = elif_stmt.set(tag("elif_stmt", seq!(
        python_literal("elif"),
        opt(WS.clone()),
        named_expression.clone(),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone(),
        opt(seq!(opt(WS.clone()), choice!(elif_stmt.clone(), else_block.clone())))
    )));
    let if_stmt = if_stmt.set(tag("if_stmt", seq!(
        python_literal("if"),
        opt(WS.clone()),
        named_expression.clone(),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone(),
        opt(seq!(opt(WS.clone()), choice!(elif_stmt.clone(), else_block.clone())))
    )));
    let default = default.set(tag("default", seq!(python_literal("="), opt(WS.clone()), expression.clone())));
    let star_annotation = star_annotation.set(tag("star_annotation", seq!(python_literal(":"), opt(WS.clone()), star_expression.clone())));
    let annotation = annotation.set(tag("annotation", seq!(python_literal(":"), opt(WS.clone()), expression.clone())));
    let param_star_annotation = param_star_annotation.set(tag("param_star_annotation", seq!(NAME.clone(), opt(WS.clone()), star_annotation.clone())));
    let param = param.set(tag("param", seq!(NAME.clone(), opt(seq!(opt(WS.clone()), annotation.clone())))));
    let param_maybe_default = param_maybe_default.set(tag("param_maybe_default", seq!(param.clone(), opt(seq!(opt(WS.clone()), default.clone())), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal(","), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone()))), TYPE_COMMENT.clone()))))));
    let param_with_default = param_with_default.set(tag("param_with_default", seq!(param.clone(), opt(WS.clone()), default.clone(), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal(","), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone()))), TYPE_COMMENT.clone()))))));
    let param_no_default_star_annotation = param_no_default_star_annotation.set(tag("param_no_default_star_annotation", seq!(param_star_annotation.clone(), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal(","), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone()))), TYPE_COMMENT.clone()))))));
    let param_no_default = param_no_default.set(tag("param_no_default", seq!(param.clone(), opt(seq!(opt(WS.clone()), choice!(seq!(python_literal(","), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone()))), TYPE_COMMENT.clone()))))));
    let kwds = kwds.set(tag("kwds", seq!(python_literal("**"), opt(WS.clone()), param_no_default.clone())));
    let star_etc = star_etc.set(tag("star_etc", choice!(
        seq!(python_literal("*"), opt(WS.clone()), choice!(seq!(param_no_default.clone(), opt(seq!(opt(WS.clone()), param_maybe_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_maybe_default.clone()))))), opt(seq!(opt(WS.clone()), kwds.clone()))), seq!(param_no_default_star_annotation.clone(), opt(seq!(opt(WS.clone()), param_maybe_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_maybe_default.clone()))))), opt(seq!(opt(WS.clone()), kwds.clone()))), seq!(python_literal(","), opt(WS.clone()), param_maybe_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_maybe_default.clone()))), opt(seq!(opt(WS.clone()), kwds.clone()))))),
        kwds.clone()
    )));
    let slash_with_default = slash_with_default.set(tag("slash_with_default", seq!(
        opt(seq!(param_no_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_no_default.clone()))), opt(WS.clone()))),
        param_with_default.clone(),
        opt(repeat1(seq!(opt(WS.clone()), param_with_default.clone()))),
        opt(WS.clone()),
        python_literal("/"),
        opt(seq!(opt(WS.clone()), python_literal(",")))
    )));
    let slash_no_default = slash_no_default.set(tag("slash_no_default", seq!(
        param_no_default.clone(),
        opt(repeat1(seq!(opt(WS.clone()), param_no_default.clone()))),
        opt(WS.clone()),
        python_literal("/"),
        opt(seq!(opt(WS.clone()), python_literal(",")))
    )));
    let parameters = parameters.set(tag("parameters", choice!(
        seq!(slash_no_default.clone(), opt(seq!(opt(WS.clone()), param_no_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_no_default.clone()))))), opt(seq!(opt(WS.clone()), param_with_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_with_default.clone()))))), opt(seq!(opt(WS.clone()), star_etc.clone()))),
        seq!(slash_with_default.clone(), opt(seq!(opt(WS.clone()), param_with_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_with_default.clone()))))), opt(seq!(opt(WS.clone()), star_etc.clone()))),
        seq!(param_no_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_no_default.clone()))), opt(seq!(opt(WS.clone()), param_with_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_with_default.clone()))))), opt(seq!(opt(WS.clone()), star_etc.clone()))),
        seq!(param_with_default.clone(), opt(repeat1(seq!(opt(WS.clone()), param_with_default.clone()))), opt(seq!(opt(WS.clone()), star_etc.clone()))),
        star_etc.clone()
    )));
    let params = params.set(tag("params", parameters.clone()));
    let function_def_raw = function_def_raw.set(tag("function_def_raw", choice!(
        seq!(python_literal("def"), opt(WS.clone()), NAME.clone(), opt(seq!(opt(WS.clone()), type_params.clone())), opt(WS.clone()), python_literal("("), opt(seq!(opt(WS.clone()), params.clone())), opt(WS.clone()), python_literal(")"), opt(seq!(opt(WS.clone()), python_literal("->"), opt(WS.clone()), expression.clone())), opt(WS.clone()), python_literal(":"), opt(seq!(opt(WS.clone()), func_type_comment.clone())), opt(WS.clone()), block.clone()),
        seq!(python_literal("async"), opt(WS.clone()), python_literal("def"), opt(WS.clone()), NAME.clone(), opt(seq!(opt(WS.clone()), type_params.clone())), opt(WS.clone()), python_literal("("), opt(seq!(opt(WS.clone()), params.clone())), opt(WS.clone()), python_literal(")"), opt(seq!(opt(WS.clone()), python_literal("->"), opt(WS.clone()), expression.clone())), opt(WS.clone()), python_literal(":"), opt(seq!(opt(WS.clone()), func_type_comment.clone())), opt(WS.clone()), block.clone())
    )));
    let function_def = function_def.set(tag("function_def", choice!(
        seq!(python_literal("@"), opt(WS.clone()), named_expression.clone(), opt(WS.clone()), NEWLINE.clone(), opt(seq!(opt(WS.clone()), python_literal("@"), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), opt(seq!(WS.clone(), opt(seq!(opt(WS.clone()), WS.clone())), opt(WS.clone()))), named_expression.clone(), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), opt(seq!(WS.clone(), opt(seq!(opt(WS.clone()), WS.clone())), opt(WS.clone()))), NEWLINE.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("@"), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), opt(seq!(WS.clone(), opt(seq!(opt(WS.clone()), WS.clone())), opt(WS.clone()))), named_expression.clone(), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), opt(seq!(WS.clone(), opt(seq!(opt(WS.clone()), WS.clone())), opt(WS.clone()))), NEWLINE.clone()))))), opt(WS.clone()), function_def_raw.clone()),
        function_def_raw.clone()
    )));
    let class_def_raw = class_def_raw.set(tag("class_def_raw", seq!(
        python_literal("class"),
        opt(WS.clone()),
        NAME.clone(),
        opt(seq!(opt(WS.clone()), type_params.clone())),
        opt(seq!(opt(WS.clone()), python_literal("("), opt(seq!(opt(WS.clone()), arguments.clone())), opt(WS.clone()), python_literal(")"))),
        opt(WS.clone()),
        python_literal(":"),
        opt(WS.clone()),
        block.clone()
    )));
    let class_def = class_def.set(tag("class_def", choice!(
        seq!(python_literal("@"), opt(WS.clone()), named_expression.clone(), opt(WS.clone()), NEWLINE.clone(), opt(seq!(opt(WS.clone()), python_literal("@"), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), named_expression.clone(), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), NEWLINE.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("@"), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), named_expression.clone(), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), NEWLINE.clone()))))), opt(WS.clone()), class_def_raw.clone()),
        class_def_raw.clone()
    )));
    let decorators = decorators.set(tag("decorators", seq!(
        python_literal("@"),
        opt(WS.clone()),
        named_expression.clone(),
        opt(WS.clone()),
        NEWLINE.clone(),
        opt(seq!(opt(WS.clone()), python_literal("@"), opt(WS.clone()), named_expression.clone(), opt(WS.clone()), NEWLINE.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("@"), opt(WS.clone()), named_expression.clone(), opt(WS.clone()), NEWLINE.clone())))))
    )));
    let block = block.set(cached(tag("block", choice!(
        seq!(NEWLINE.clone(), opt(WS.clone()), INDENT.clone(), opt(WS.clone()), statements.clone(), opt(WS.clone()), DEDENT.clone()),
        seq!(simple_stmt.clone(), opt(WS.clone()), choice!(NEWLINE.clone(), seq!(opt(seq!(python_literal(";"), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), simple_stmt.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(";"), opt(WS.clone()), opt(seq!(WS.clone(), opt(WS.clone()))), simple_stmt.clone()))), opt(WS.clone()))), opt(seq!(python_literal(";"), opt(WS.clone()))), NEWLINE.clone())))
    ))));
    let dotted_name = dotted_name.set(tag("dotted_name", seq!(NAME.clone(), opt(seq!(opt(WS.clone()), python_literal("."), opt(WS.clone()), NAME.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal("."), opt(WS.clone()), NAME.clone()))))))));
    let dotted_as_name = dotted_as_name.set(tag("dotted_as_name", seq!(dotted_name.clone(), opt(seq!(opt(WS.clone()), python_literal("as"), opt(WS.clone()), NAME.clone())))));
    let dotted_as_names = dotted_as_names.set(tag("dotted_as_names", seq!(dotted_as_name.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), dotted_as_name.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), dotted_as_name.clone()))))))));
    let import_from_as_name = import_from_as_name.set(tag("import_from_as_name", seq!(NAME.clone(), opt(seq!(opt(WS.clone()), python_literal("as"), opt(WS.clone()), NAME.clone())))));
    let import_from_as_names = import_from_as_names.set(tag("import_from_as_names", seq!(import_from_as_name.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), import_from_as_name.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), import_from_as_name.clone()))))))));
    let import_from_targets = import_from_targets.set(tag("import_from_targets", choice!(
        seq!(python_literal("("), opt(WS.clone()), import_from_as_names.clone(), opt(seq!(opt(WS.clone()), python_literal(","))), opt(WS.clone()), python_literal(")")),
        import_from_as_names.clone(),
        python_literal("*")
    )));
    let import_from = import_from.set(tag("import_from", seq!(python_literal("from"), opt(WS.clone()), choice!(seq!(opt(seq!(choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(WS.clone()), choice!(python_literal("."), python_literal("..."))))), opt(WS.clone()))), dotted_name.clone(), opt(WS.clone()), python_literal("import"), opt(WS.clone()), import_from_targets.clone()), seq!(choice!(python_literal("."), python_literal("...")), opt(repeat1(seq!(opt(WS.clone()), choice!(python_literal("."), python_literal("..."))))), opt(WS.clone()), python_literal("import"), opt(WS.clone()), import_from_targets.clone())))));
    let import_name = import_name.set(tag("import_name", seq!(python_literal("import"), opt(WS.clone()), dotted_as_names.clone())));
    let import_stmt = import_stmt.set(tag("import_stmt", choice!(
        import_name.clone(),
        import_from.clone()
    )));
    let assert_stmt = assert_stmt.set(tag("assert_stmt", seq!(python_literal("assert"), opt(WS.clone()), expression.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), expression.clone())))));
    let yield_stmt = yield_stmt.set(tag("yield_stmt", yield_expr.clone()));
    let del_stmt = del_stmt.set(tag("del_stmt", seq!(python_literal("del"), opt(WS.clone()), del_targets.clone())));
    let nonlocal_stmt = nonlocal_stmt.set(tag("nonlocal_stmt", seq!(python_literal("nonlocal"), opt(WS.clone()), NAME.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), NAME.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), NAME.clone()))))))));
    let global_stmt = global_stmt.set(tag("global_stmt", seq!(python_literal("global"), opt(WS.clone()), NAME.clone(), opt(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), NAME.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(","), opt(WS.clone()), NAME.clone()))))))));
    let raise_stmt = raise_stmt.set(tag("raise_stmt", seq!(python_literal("raise"), opt(seq!(opt(WS.clone()), expression.clone(), opt(seq!(opt(WS.clone()), python_literal("from"), opt(WS.clone()), expression.clone())))))));
    let return_stmt = return_stmt.set(tag("return_stmt", seq!(python_literal("return"), opt(seq!(opt(WS.clone()), star_expressions.clone())))));
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
    )));
    let annotated_rhs = annotated_rhs.set(tag("annotated_rhs", choice!(
        yield_expr.clone(),
        star_expressions.clone()
    )));
    let assignment = assignment.set(tag("assignment", choice!(
        seq!(NAME.clone(), opt(WS.clone()), python_literal(":"), opt(WS.clone()), expression.clone(), opt(seq!(opt(WS.clone()), python_literal("="), opt(WS.clone()), annotated_rhs.clone()))),
        seq!(choice!(seq!(python_literal("("), opt(WS.clone()), single_target.clone(), opt(WS.clone()), python_literal(")")), single_subscript_attribute_target.clone()), opt(WS.clone()), python_literal(":"), opt(WS.clone()), expression.clone(), opt(seq!(opt(WS.clone()), python_literal("="), opt(WS.clone()), annotated_rhs.clone()))),
        seq!(star_targets.clone(), opt(WS.clone()), python_literal("="), opt(seq!(opt(WS.clone()), star_targets.clone(), opt(WS.clone()), python_literal("="), opt(repeat1(seq!(opt(WS.clone()), star_targets.clone(), opt(WS.clone()), python_literal("=")))))), opt(WS.clone()), choice!(yield_expr.clone(), star_expressions.clone()), opt(seq!(opt(WS.clone()), TYPE_COMMENT.clone()))),
        seq!(single_target.clone(), opt(WS.clone()), augassign.clone(), opt(WS.clone()), choice!(yield_expr.clone(), star_expressions.clone()))
    )));
    let compound_stmt = compound_stmt.set(tag("compound_stmt", choice!(
        function_def.clone(),
        if_stmt.clone(),
        class_def.clone(),
        with_stmt.clone(),
        for_stmt.clone(),
        try_stmt.clone(),
        while_stmt.clone(),
        match_stmt.clone()
    )));
    let simple_stmt = simple_stmt.set(cached(tag("simple_stmt", choice!(
        assignment.clone(),
        type_alias.clone(),
        star_expressions.clone(),
        return_stmt.clone(),
        import_stmt.clone(),
        raise_stmt.clone(),
        python_literal("pass"),
        del_stmt.clone(),
        yield_stmt.clone(),
        assert_stmt.clone(),
        python_literal("break"),
        python_literal("continue"),
        global_stmt.clone(),
        nonlocal_stmt.clone()
    ))));
    let simple_stmts = simple_stmts.set(tag("simple_stmts", seq!(simple_stmt.clone(), opt(WS.clone()), choice!(NEWLINE.clone(), seq!(opt(seq!(python_literal(";"), opt(WS.clone()), simple_stmt.clone(), opt(repeat1(seq!(opt(WS.clone()), python_literal(";"), opt(WS.clone()), simple_stmt.clone()))), opt(WS.clone()))), opt(seq!(python_literal(";"), opt(WS.clone()))), NEWLINE.clone())))));
    let statement_newline = statement_newline.set(tag("statement_newline", choice!(
        seq!(compound_stmt.clone(), opt(WS.clone()), NEWLINE.clone()),
        simple_stmts.clone(),
        NEWLINE.clone(),
        ENDMARKER.clone()
    )));
    let statement = statement.set(tag("statement", choice!(
        compound_stmt.clone(),
        simple_stmts.clone()
    )));
    let statements = statements.set(tag("statements", seq!(statement.clone(), opt(repeat1(seq!(opt(WS.clone()), statement.clone()))))));
    let func_type = func_type.set(tag("func_type", seq!(
        python_literal("("),
        opt(seq!(opt(WS.clone()), type_expressions.clone())),
        opt(WS.clone()),
        python_literal(")"),
        opt(WS.clone()),
        python_literal("->"),
        opt(WS.clone()),
        expression.clone(),
        opt(seq!(opt(WS.clone()), NEWLINE.clone(), opt(repeat1(seq!(opt(WS.clone()), NEWLINE.clone()))))),
        opt(WS.clone()),
        ENDMARKER.clone()
    )));
    let eval = eval.set(tag("eval", seq!(expressions.clone(), opt(seq!(opt(WS.clone()), NEWLINE.clone(), opt(repeat1(seq!(opt(WS.clone()), NEWLINE.clone()))))), opt(WS.clone()), ENDMARKER.clone())));
    let interactive = interactive.set(tag("interactive", statement_newline.clone()));
    let file = file.set(tag("file", seq!(opt(seq!(statements.clone(), opt(WS.clone()))), ENDMARKER.clone())));

    cache_context(seq!(opt(NEWLINE), file))
}
