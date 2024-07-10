use crate::{choice, seq, repeat, repeat as repeat0, repeat1, opt, eat_char_choice, eat_string};

fn main() {
    let file = forward_ref();
    let interactive = forward_ref();
    let eval = forward_ref();
    let func_type = forward_ref();
    let statements = forward_ref();
    let statement = forward_ref();
    let statement_newline = forward_ref();
    let simple_stmts = forward_ref();
    let simple_stmt = forward_ref();
    let compound_stmt = forward_ref();
    let assignment = forward_ref();
    let annotated_rhs = forward_ref();
    let augassign = forward_ref();
    let return_stmt = forward_ref();
    let raise_stmt = forward_ref();
    let global_stmt = forward_ref();
    let nonlocal_stmt = forward_ref();
    let del_stmt = forward_ref();
    let yield_stmt = forward_ref();
    let assert_stmt = forward_ref();
    let import_stmt = forward_ref();
    let import_name = forward_ref();
    let import_from = forward_ref();
    let import_from_targets = forward_ref();
    let import_from_as_names = forward_ref();
    let import_from_as_name = forward_ref();
    let dotted_as_names = forward_ref();
    let dotted_as_name = forward_ref();
    let dotted_name = forward_ref();
    let block = forward_ref();
    let decorators = forward_ref();
    let class_def = forward_ref();
    let class_def_raw = forward_ref();
    let function_def = forward_ref();
    let function_def_raw = forward_ref();
    let params = forward_ref();
    let parameters = forward_ref();
    let slash_no_default = forward_ref();
    let slash_with_default = forward_ref();
    let star_etc = forward_ref();
    let kwds = forward_ref();
    let param_no_default = forward_ref();
    let param_no_default_star_annotation = forward_ref();
    let param_with_default = forward_ref();
    let param_maybe_default = forward_ref();
    let param = forward_ref();
    let param_star_annotation = forward_ref();
    let annotation = forward_ref();
    let star_annotation = forward_ref();
    let default = forward_ref();
    let if_stmt = forward_ref();
    let elif_stmt = forward_ref();
    let else_block = forward_ref();
    let while_stmt = forward_ref();
    let for_stmt = forward_ref();
    let with_stmt = forward_ref();
    let with_item = forward_ref();
    let try_stmt = forward_ref();
    let except_block = forward_ref();
    let except_star_block = forward_ref();
    let finally_block = forward_ref();
    let match_stmt = forward_ref();
    let subject_expr = forward_ref();
    let case_block = forward_ref();
    let guard = forward_ref();
    let patterns = forward_ref();
    let pattern = forward_ref();
    let as_pattern = forward_ref();
    let or_pattern = forward_ref();
    let closed_pattern = forward_ref();
    let literal_pattern = forward_ref();
    let literal_expr = forward_ref();
    let complex_number = forward_ref();
    let signed_number = forward_ref();
    let signed_real_number = forward_ref();
    let real_number = forward_ref();
    let imaginary_number = forward_ref();
    let capture_pattern = forward_ref();
    let pattern_capture_target = forward_ref();
    let wildcard_pattern = forward_ref();
    let value_pattern = forward_ref();
    let attr = forward_ref();
    let name_or_attr = forward_ref();
    let group_pattern = forward_ref();
    let sequence_pattern = forward_ref();
    let open_sequence_pattern = forward_ref();
    let maybe_sequence_pattern = forward_ref();
    let maybe_star_pattern = forward_ref();
    let star_pattern = forward_ref();
    let mapping_pattern = forward_ref();
    let items_pattern = forward_ref();
    let key_value_pattern = forward_ref();
    let double_star_pattern = forward_ref();
    let class_pattern = forward_ref();
    let positional_patterns = forward_ref();
    let keyword_patterns = forward_ref();
    let keyword_pattern = forward_ref();
    let type_alias = forward_ref();
    let type_params = forward_ref();
    let type_param_seq = forward_ref();
    let type_param = forward_ref();
    let type_param_bound = forward_ref();
    let type_param_default = forward_ref();
    let type_param_starred_default = forward_ref();
    let expressions = forward_ref();
    let expression = forward_ref();
    let yield_expr = forward_ref();
    let star_expressions = forward_ref();
    let star_expression = forward_ref();
    let star_named_expressions = forward_ref();
    let star_named_expression = forward_ref();
    let assignment_expression = forward_ref();
    let named_expression = forward_ref();
    let disjunction = forward_ref();
    let conjunction = forward_ref();
    let inversion = forward_ref();
    let comparison = forward_ref();
    let compare_op_bitwise_or_pair = forward_ref();
    let eq_bitwise_or = forward_ref();
    let noteq_bitwise_or = forward_ref();
    let lte_bitwise_or = forward_ref();
    let lt_bitwise_or = forward_ref();
    let gte_bitwise_or = forward_ref();
    let gt_bitwise_or = forward_ref();
    let notin_bitwise_or = forward_ref();
    let in_bitwise_or = forward_ref();
    let isnot_bitwise_or = forward_ref();
    let is_bitwise_or = forward_ref();
    let bitwise_or = forward_ref();
    let bitwise_xor = forward_ref();
    let bitwise_and = forward_ref();
    let shift_expr = forward_ref();
    let sum = forward_ref();
    let term = forward_ref();
    let factor = forward_ref();
    let power = forward_ref();
    let await_primary = forward_ref();
    let primary = forward_ref();
    let slices = forward_ref();
    let slice = forward_ref();
    let atom = forward_ref();
    let group = forward_ref();
    let lambdef = forward_ref();
    let lambda_params = forward_ref();
    let lambda_parameters = forward_ref();
    let lambda_slash_no_default = forward_ref();
    let lambda_slash_with_default = forward_ref();
    let lambda_star_etc = forward_ref();
    let lambda_kwds = forward_ref();
    let lambda_param_no_default = forward_ref();
    let lambda_param_with_default = forward_ref();
    let lambda_param_maybe_default = forward_ref();
    let lambda_param = forward_ref();
    let fstring_middle = forward_ref();
    let fstring_replacement_field = forward_ref();
    let fstring_conversion = forward_ref();
    let fstring_full_format_spec = forward_ref();
    let fstring_format_spec = forward_ref();
    let fstring = forward_ref();
    let string = forward_ref();
    let strings = forward_ref();
    let list = forward_ref();
    let tuple = forward_ref();
    let set = forward_ref();
    let dict = forward_ref();
    let double_starred_kvpairs = forward_ref();
    let double_starred_kvpair = forward_ref();
    let kvpair = forward_ref();
    let for_if_clauses = forward_ref();
    let for_if_clause = forward_ref();
    let listcomp = forward_ref();
    let setcomp = forward_ref();
    let genexp = forward_ref();
    let dictcomp = forward_ref();
    let arguments = forward_ref();
    let args = forward_ref();
    let kwargs = forward_ref();
    let starred_expression = forward_ref();
    let kwarg_or_starred = forward_ref();
    let kwarg_or_double_starred = forward_ref();
    let star_targets = forward_ref();
    let star_targets_list_seq = forward_ref();
    let star_targets_tuple_seq = forward_ref();
    let star_target = forward_ref();
    let target_with_star_atom = forward_ref();
    let star_atom = forward_ref();
    let single_target = forward_ref();
    let single_subscript_attribute_target = forward_ref();
    let t_primary = forward_ref();
    let t_lookahead = forward_ref();
    let del_targets = forward_ref();
    let del_target = forward_ref();
    let del_t_atom = forward_ref();
    let type_expressions = forward_ref();
    let func_type_comment = forward_ref();
    let invalid_arguments = forward_ref();
    let invalid_kwarg = forward_ref();
    let expression_without_invalid = forward_ref();
    let invalid_legacy_expression = forward_ref();
    let invalid_type_param = forward_ref();
    let invalid_expression = forward_ref();
    let invalid_named_expression = forward_ref();
    let invalid_assignment = forward_ref();
    let invalid_ann_assign_target = forward_ref();
    let invalid_del_stmt = forward_ref();
    let invalid_block = forward_ref();
    let invalid_comprehension = forward_ref();
    let invalid_dict_comprehension = forward_ref();
    let invalid_parameters = forward_ref();
    let invalid_default = forward_ref();
    let invalid_star_etc = forward_ref();
    let invalid_kwds = forward_ref();
    let invalid_parameters_helper = forward_ref();
    let invalid_lambda_parameters = forward_ref();
    let invalid_lambda_parameters_helper = forward_ref();
    let invalid_lambda_star_etc = forward_ref();
    let invalid_lambda_kwds = forward_ref();
    let invalid_double_type_comments = forward_ref();
    let invalid_with_item = forward_ref();
    let invalid_for_if_clause = forward_ref();
    let invalid_for_target = forward_ref();
    let invalid_group = forward_ref();
    let invalid_import = forward_ref();
    let invalid_import_from_targets = forward_ref();
    let invalid_with_stmt = forward_ref();
    let invalid_with_stmt_indent = forward_ref();
    let invalid_try_stmt = forward_ref();
    let invalid_except_stmt = forward_ref();
    let invalid_finally_stmt = forward_ref();
    let invalid_except_stmt_indent = forward_ref();
    let invalid_except_star_stmt_indent = forward_ref();
    let invalid_match_stmt = forward_ref();
    let invalid_case_block = forward_ref();
    let invalid_as_pattern = forward_ref();
    let invalid_class_pattern = forward_ref();
    let invalid_class_argument_pattern = forward_ref();
    let invalid_if_stmt = forward_ref();
    let invalid_elif_stmt = forward_ref();
    let invalid_else_stmt = forward_ref();
    let invalid_while_stmt = forward_ref();
    let invalid_for_stmt = forward_ref();
    let invalid_def_raw = forward_ref();
    let invalid_class_def_raw = forward_ref();
    let invalid_double_starred_kvpairs = forward_ref();
    let invalid_kvpair = forward_ref();
    let invalid_starred_expression_unpacking = forward_ref();
    let invalid_starred_expression = forward_ref();
    let invalid_replacement_field = forward_ref();
    let invalid_conversion_character = forward_ref();
    let invalid_arithmetic = forward_ref();
    let invalid_factor = forward_ref();
    let invalid_type_params = forward_ref();    let file_copy = file.clone();
    let interactive_copy = interactive.clone();
    let eval_copy = eval.clone();
    let func_type_copy = func_type.clone();
    let statements_copy = statements.clone();
    let statement_copy = statement.clone();
    let statement_newline_copy = statement_newline.clone();
    let simple_stmts_copy = simple_stmts.clone();
    let simple_stmt_copy = simple_stmt.clone();
    let compound_stmt_copy = compound_stmt.clone();
    let assignment_copy = assignment.clone();
    let annotated_rhs_copy = annotated_rhs.clone();
    let augassign_copy = augassign.clone();
    let return_stmt_copy = return_stmt.clone();
    let raise_stmt_copy = raise_stmt.clone();
    let global_stmt_copy = global_stmt.clone();
    let nonlocal_stmt_copy = nonlocal_stmt.clone();
    let del_stmt_copy = del_stmt.clone();
    let yield_stmt_copy = yield_stmt.clone();
    let assert_stmt_copy = assert_stmt.clone();
    let import_stmt_copy = import_stmt.clone();
    let import_name_copy = import_name.clone();
    let import_from_copy = import_from.clone();
    let import_from_targets_copy = import_from_targets.clone();
    let import_from_as_names_copy = import_from_as_names.clone();
    let import_from_as_name_copy = import_from_as_name.clone();
    let dotted_as_names_copy = dotted_as_names.clone();
    let dotted_as_name_copy = dotted_as_name.clone();
    let dotted_name_copy = dotted_name.clone();
    let block_copy = block.clone();
    let decorators_copy = decorators.clone();
    let class_def_copy = class_def.clone();
    let class_def_raw_copy = class_def_raw.clone();
    let function_def_copy = function_def.clone();
    let function_def_raw_copy = function_def_raw.clone();
    let params_copy = params.clone();
    let parameters_copy = parameters.clone();
    let slash_no_default_copy = slash_no_default.clone();
    let slash_with_default_copy = slash_with_default.clone();
    let star_etc_copy = star_etc.clone();
    let kwds_copy = kwds.clone();
    let param_no_default_copy = param_no_default.clone();
    let param_no_default_star_annotation_copy = param_no_default_star_annotation.clone();
    let param_with_default_copy = param_with_default.clone();
    let param_maybe_default_copy = param_maybe_default.clone();
    let param_copy = param.clone();
    let param_star_annotation_copy = param_star_annotation.clone();
    let annotation_copy = annotation.clone();
    let star_annotation_copy = star_annotation.clone();
    let default_copy = default.clone();
    let if_stmt_copy = if_stmt.clone();
    let elif_stmt_copy = elif_stmt.clone();
    let else_block_copy = else_block.clone();
    let while_stmt_copy = while_stmt.clone();
    let for_stmt_copy = for_stmt.clone();
    let with_stmt_copy = with_stmt.clone();
    let with_item_copy = with_item.clone();
    let try_stmt_copy = try_stmt.clone();
    let except_block_copy = except_block.clone();
    let except_star_block_copy = except_star_block.clone();
    let finally_block_copy = finally_block.clone();
    let match_stmt_copy = match_stmt.clone();
    let subject_expr_copy = subject_expr.clone();
    let case_block_copy = case_block.clone();
    let guard_copy = guard.clone();
    let patterns_copy = patterns.clone();
    let pattern_copy = pattern.clone();
    let as_pattern_copy = as_pattern.clone();
    let or_pattern_copy = or_pattern.clone();
    let closed_pattern_copy = closed_pattern.clone();
    let literal_pattern_copy = literal_pattern.clone();
    let literal_expr_copy = literal_expr.clone();
    let complex_number_copy = complex_number.clone();
    let signed_number_copy = signed_number.clone();
    let signed_real_number_copy = signed_real_number.clone();
    let real_number_copy = real_number.clone();
    let imaginary_number_copy = imaginary_number.clone();
    let capture_pattern_copy = capture_pattern.clone();
    let pattern_capture_target_copy = pattern_capture_target.clone();
    let wildcard_pattern_copy = wildcard_pattern.clone();
    let value_pattern_copy = value_pattern.clone();
    let attr_copy = attr.clone();
    let name_or_attr_copy = name_or_attr.clone();
    let group_pattern_copy = group_pattern.clone();
    let sequence_pattern_copy = sequence_pattern.clone();
    let open_sequence_pattern_copy = open_sequence_pattern.clone();
    let maybe_sequence_pattern_copy = maybe_sequence_pattern.clone();
    let maybe_star_pattern_copy = maybe_star_pattern.clone();
    let star_pattern_copy = star_pattern.clone();
    let mapping_pattern_copy = mapping_pattern.clone();
    let items_pattern_copy = items_pattern.clone();
    let key_value_pattern_copy = key_value_pattern.clone();
    let double_star_pattern_copy = double_star_pattern.clone();
    let class_pattern_copy = class_pattern.clone();
    let positional_patterns_copy = positional_patterns.clone();
    let keyword_patterns_copy = keyword_patterns.clone();
    let keyword_pattern_copy = keyword_pattern.clone();
    let type_alias_copy = type_alias.clone();
    let type_params_copy = type_params.clone();
    let type_param_seq_copy = type_param_seq.clone();
    let type_param_copy = type_param.clone();
    let type_param_bound_copy = type_param_bound.clone();
    let type_param_default_copy = type_param_default.clone();
    let type_param_starred_default_copy = type_param_starred_default.clone();
    let expressions_copy = expressions.clone();
    let expression_copy = expression.clone();
    let yield_expr_copy = yield_expr.clone();
    let star_expressions_copy = star_expressions.clone();
    let star_expression_copy = star_expression.clone();
    let star_named_expressions_copy = star_named_expressions.clone();
    let star_named_expression_copy = star_named_expression.clone();
    let assignment_expression_copy = assignment_expression.clone();
    let named_expression_copy = named_expression.clone();
    let disjunction_copy = disjunction.clone();
    let conjunction_copy = conjunction.clone();
    let inversion_copy = inversion.clone();
    let comparison_copy = comparison.clone();
    let compare_op_bitwise_or_pair_copy = compare_op_bitwise_or_pair.clone();
    let eq_bitwise_or_copy = eq_bitwise_or.clone();
    let noteq_bitwise_or_copy = noteq_bitwise_or.clone();
    let lte_bitwise_or_copy = lte_bitwise_or.clone();
    let lt_bitwise_or_copy = lt_bitwise_or.clone();
    let gte_bitwise_or_copy = gte_bitwise_or.clone();
    let gt_bitwise_or_copy = gt_bitwise_or.clone();
    let notin_bitwise_or_copy = notin_bitwise_or.clone();
    let in_bitwise_or_copy = in_bitwise_or.clone();
    let isnot_bitwise_or_copy = isnot_bitwise_or.clone();
    let is_bitwise_or_copy = is_bitwise_or.clone();
    let bitwise_or_copy = bitwise_or.clone();
    let bitwise_xor_copy = bitwise_xor.clone();
    let bitwise_and_copy = bitwise_and.clone();
    let shift_expr_copy = shift_expr.clone();
    let sum_copy = sum.clone();
    let term_copy = term.clone();
    let factor_copy = factor.clone();
    let power_copy = power.clone();
    let await_primary_copy = await_primary.clone();
    let primary_copy = primary.clone();
    let slices_copy = slices.clone();
    let slice_copy = slice.clone();
    let atom_copy = atom.clone();
    let group_copy = group.clone();
    let lambdef_copy = lambdef.clone();
    let lambda_params_copy = lambda_params.clone();
    let lambda_parameters_copy = lambda_parameters.clone();
    let lambda_slash_no_default_copy = lambda_slash_no_default.clone();
    let lambda_slash_with_default_copy = lambda_slash_with_default.clone();
    let lambda_star_etc_copy = lambda_star_etc.clone();
    let lambda_kwds_copy = lambda_kwds.clone();
    let lambda_param_no_default_copy = lambda_param_no_default.clone();
    let lambda_param_with_default_copy = lambda_param_with_default.clone();
    let lambda_param_maybe_default_copy = lambda_param_maybe_default.clone();
    let lambda_param_copy = lambda_param.clone();
    let fstring_middle_copy = fstring_middle.clone();
    let fstring_replacement_field_copy = fstring_replacement_field.clone();
    let fstring_conversion_copy = fstring_conversion.clone();
    let fstring_full_format_spec_copy = fstring_full_format_spec.clone();
    let fstring_format_spec_copy = fstring_format_spec.clone();
    let fstring_copy = fstring.clone();
    let string_copy = string.clone();
    let strings_copy = strings.clone();
    let list_copy = list.clone();
    let tuple_copy = tuple.clone();
    let set_copy = set.clone();
    let dict_copy = dict.clone();
    let double_starred_kvpairs_copy = double_starred_kvpairs.clone();
    let double_starred_kvpair_copy = double_starred_kvpair.clone();
    let kvpair_copy = kvpair.clone();
    let for_if_clauses_copy = for_if_clauses.clone();
    let for_if_clause_copy = for_if_clause.clone();
    let listcomp_copy = listcomp.clone();
    let setcomp_copy = setcomp.clone();
    let genexp_copy = genexp.clone();
    let dictcomp_copy = dictcomp.clone();
    let arguments_copy = arguments.clone();
    let args_copy = args.clone();
    let kwargs_copy = kwargs.clone();
    let starred_expression_copy = starred_expression.clone();
    let kwarg_or_starred_copy = kwarg_or_starred.clone();
    let kwarg_or_double_starred_copy = kwarg_or_double_starred.clone();
    let star_targets_copy = star_targets.clone();
    let star_targets_list_seq_copy = star_targets_list_seq.clone();
    let star_targets_tuple_seq_copy = star_targets_tuple_seq.clone();
    let star_target_copy = star_target.clone();
    let target_with_star_atom_copy = target_with_star_atom.clone();
    let star_atom_copy = star_atom.clone();
    let single_target_copy = single_target.clone();
    let single_subscript_attribute_target_copy = single_subscript_attribute_target.clone();
    let t_primary_copy = t_primary.clone();
    let t_lookahead_copy = t_lookahead.clone();
    let del_targets_copy = del_targets.clone();
    let del_target_copy = del_target.clone();
    let del_t_atom_copy = del_t_atom.clone();
    let type_expressions_copy = type_expressions.clone();
    let func_type_comment_copy = func_type_comment.clone();
    let invalid_arguments_copy = invalid_arguments.clone();
    let invalid_kwarg_copy = invalid_kwarg.clone();
    let expression_without_invalid_copy = expression_without_invalid.clone();
    let invalid_legacy_expression_copy = invalid_legacy_expression.clone();
    let invalid_type_param_copy = invalid_type_param.clone();
    let invalid_expression_copy = invalid_expression.clone();
    let invalid_named_expression_copy = invalid_named_expression.clone();
    let invalid_assignment_copy = invalid_assignment.clone();
    let invalid_ann_assign_target_copy = invalid_ann_assign_target.clone();
    let invalid_del_stmt_copy = invalid_del_stmt.clone();
    let invalid_block_copy = invalid_block.clone();
    let invalid_comprehension_copy = invalid_comprehension.clone();
    let invalid_dict_comprehension_copy = invalid_dict_comprehension.clone();
    let invalid_parameters_copy = invalid_parameters.clone();
    let invalid_default_copy = invalid_default.clone();
    let invalid_star_etc_copy = invalid_star_etc.clone();
    let invalid_kwds_copy = invalid_kwds.clone();
    let invalid_parameters_helper_copy = invalid_parameters_helper.clone();
    let invalid_lambda_parameters_copy = invalid_lambda_parameters.clone();
    let invalid_lambda_parameters_helper_copy = invalid_lambda_parameters_helper.clone();
    let invalid_lambda_star_etc_copy = invalid_lambda_star_etc.clone();
    let invalid_lambda_kwds_copy = invalid_lambda_kwds.clone();
    let invalid_double_type_comments_copy = invalid_double_type_comments.clone();
    let invalid_with_item_copy = invalid_with_item.clone();
    let invalid_for_if_clause_copy = invalid_for_if_clause.clone();
    let invalid_for_target_copy = invalid_for_target.clone();
    let invalid_group_copy = invalid_group.clone();
    let invalid_import_copy = invalid_import.clone();
    let invalid_import_from_targets_copy = invalid_import_from_targets.clone();
    let invalid_with_stmt_copy = invalid_with_stmt.clone();
    let invalid_with_stmt_indent_copy = invalid_with_stmt_indent.clone();
    let invalid_try_stmt_copy = invalid_try_stmt.clone();
    let invalid_except_stmt_copy = invalid_except_stmt.clone();
    let invalid_finally_stmt_copy = invalid_finally_stmt.clone();
    let invalid_except_stmt_indent_copy = invalid_except_stmt_indent.clone();
    let invalid_except_star_stmt_indent_copy = invalid_except_star_stmt_indent.clone();
    let invalid_match_stmt_copy = invalid_match_stmt.clone();
    let invalid_case_block_copy = invalid_case_block.clone();
    let invalid_as_pattern_copy = invalid_as_pattern.clone();
    let invalid_class_pattern_copy = invalid_class_pattern.clone();
    let invalid_class_argument_pattern_copy = invalid_class_argument_pattern.clone();
    let invalid_if_stmt_copy = invalid_if_stmt.clone();
    let invalid_elif_stmt_copy = invalid_elif_stmt.clone();
    let invalid_else_stmt_copy = invalid_else_stmt.clone();
    let invalid_while_stmt_copy = invalid_while_stmt.clone();
    let invalid_for_stmt_copy = invalid_for_stmt.clone();
    let invalid_def_raw_copy = invalid_def_raw.clone();
    let invalid_class_def_raw_copy = invalid_class_def_raw.clone();
    let invalid_double_starred_kvpairs_copy = invalid_double_starred_kvpairs.clone();
    let invalid_kvpair_copy = invalid_kvpair.clone();
    let invalid_starred_expression_unpacking_copy = invalid_starred_expression_unpacking.clone();
    let invalid_starred_expression_copy = invalid_starred_expression.clone();
    let invalid_replacement_field_copy = invalid_replacement_field.clone();
    let invalid_conversion_character_copy = invalid_conversion_character.clone();
    let invalid_arithmetic_copy = invalid_arithmetic.clone();
    let invalid_factor_copy = invalid_factor.clone();
    let invalid_type_params_copy = invalid_type_params.clone();    let file = choice!(seq!(opt(choice!(seq!(statements))), ENDMARKER));
    let interactive = choice!(seq!(statement_newline));
    let eval = choice!(seq!(expressions, repeat(NEWLINE), ENDMARKER));
    let func_type = choice!(seq!(eat_char_choice("("), opt(choice!(seq!(type_expressions))), eat_char_choice(")"), eat_char_choice("->"), expression, repeat(NEWLINE), ENDMARKER));
    let statements = choice!(seq!(repeat(statement)));
    let statement = choice!(seq!(compound_stmt),
        seq!(simple_stmts));
    let statement_newline = choice!(seq!(compound_stmt, NEWLINE),
        seq!(simple_stmts),
        seq!(NEWLINE),
        seq!(ENDMARKER));
    let simple_stmts = choice!(seq!(simple_stmt, lookahead(eat_char_choice(";"), !), NEWLINE),
        seq!(gather(simple_stmt, eat_char_choice(";")), opt(choice!(seq!(eat_char_choice(";")))), NEWLINE));
    let simple_stmt = choice!(seq!(assignment),
        seq!(lookahead("type", &), type_alias),
        seq!(star_expressions),
        seq!(lookahead(eat_char_choice("return"), &), return_stmt),
        seq!(lookahead(group(choice!(seq!(eat_char_choice("import")), seq!(eat_char_choice("from")))), &), import_stmt),
        seq!(lookahead(eat_char_choice("raise"), &), raise_stmt),
        seq!(eat_char_choice("pass")),
        seq!(lookahead(eat_char_choice("del"), &), del_stmt),
        seq!(lookahead(eat_char_choice("yield"), &), yield_stmt),
        seq!(lookahead(eat_char_choice("assert"), &), assert_stmt),
        seq!(eat_char_choice("break")),
        seq!(eat_char_choice("continue")),
        seq!(lookahead(eat_char_choice("global"), &), global_stmt),
        seq!(lookahead(eat_char_choice("nonlocal"), &), nonlocal_stmt));
    let compound_stmt = choice!(seq!(lookahead(group(choice!(seq!(eat_char_choice("def")), seq!(eat_char_choice("@")), seq!(eat_char_choice("async")))), &), function_def),
        seq!(lookahead(eat_char_choice("if"), &), if_stmt),
        seq!(lookahead(group(choice!(seq!(eat_char_choice("class")), seq!(eat_char_choice("@")))), &), class_def),
        seq!(lookahead(group(choice!(seq!(eat_char_choice("with")), seq!(eat_char_choice("async")))), &), with_stmt),
        seq!(lookahead(group(choice!(seq!(eat_char_choice("for")), seq!(eat_char_choice("async")))), &), for_stmt),
        seq!(lookahead(eat_char_choice("try"), &), try_stmt),
        seq!(lookahead(eat_char_choice("while"), &), while_stmt),
        seq!(match_stmt));
    let assignment = choice!(seq!(NAME, eat_char_choice(":"), expression, opt(choice!(seq!(eat_char_choice("="), annotated_rhs)))),
        seq!(group(choice!(seq!(eat_char_choice("("), single_target, eat_char_choice(")")), seq!(single_subscript_attribute_target))), eat_char_choice(":"), expression, opt(choice!(seq!(eat_char_choice("="), annotated_rhs)))),
        seq!(repeat(group(choice!(seq!(star_targets, eat_char_choice("="))))), group(choice!(seq!(yield_expr), seq!(star_expressions))), lookahead(eat_char_choice("="), !), opt(choice!(seq!(TYPE_COMMENT)))),
        seq!(single_target, augassign, cut(), group(choice!(seq!(yield_expr), seq!(star_expressions)))),
        seq!(invalid_assignment));
    let annotated_rhs = choice!(seq!(yield_expr),
        seq!(star_expressions));
    let augassign = choice!(seq!(eat_char_choice("+=")),
        seq!(eat_char_choice("-=")),
        seq!(eat_char_choice("*=")),
        seq!(eat_char_choice("@=")),
        seq!(eat_char_choice("/=")),
        seq!(eat_char_choice("%=")),
        seq!(eat_char_choice("&=")),
        seq!(eat_char_choice("|=")),
        seq!(eat_char_choice("^=")),
        seq!(eat_char_choice("<<=")),
        seq!(eat_char_choice(">>=")),
        seq!(eat_char_choice("**=")),
        seq!(eat_char_choice("//=")));
    let return_stmt = choice!(seq!(eat_char_choice("return"), opt(choice!(seq!(star_expressions)))));
    let raise_stmt = choice!(seq!(eat_char_choice("raise"), expression, opt(choice!(seq!(eat_char_choice("from"), expression)))),
        seq!(eat_char_choice("raise")));
    let global_stmt = choice!(seq!(eat_char_choice("global"), gather(NAME, eat_char_choice(","))));
    let nonlocal_stmt = choice!(seq!(eat_char_choice("nonlocal"), gather(NAME, eat_char_choice(","))));
    let del_stmt = choice!(seq!(eat_char_choice("del"), del_targets, lookahead(group(choice!(seq!(eat_char_choice(";")), seq!(NEWLINE))), &)),
        seq!(invalid_del_stmt));
    let yield_stmt = choice!(seq!(yield_expr));
    let assert_stmt = choice!(seq!(eat_char_choice("assert"), expression, opt(choice!(seq!(eat_char_choice(","), expression)))));
    let import_stmt = choice!(seq!(invalid_import),
        seq!(import_name),
        seq!(import_from));
    let import_name = choice!(seq!(eat_char_choice("import"), dotted_as_names));
    let import_from = choice!(seq!(eat_char_choice("from"), repeat(group(choice!(seq!(eat_char_choice(".")), seq!(eat_char_choice("..."))))), dotted_name, eat_char_choice("import"), import_from_targets),
        seq!(eat_char_choice("from"), repeat(group(choice!(seq!(eat_char_choice(".")), seq!(eat_char_choice("..."))))), eat_char_choice("import"), import_from_targets));
    let import_from_targets = choice!(seq!(eat_char_choice("("), import_from_as_names, opt(choice!(seq!(eat_char_choice(",")))), eat_char_choice(")")),
        seq!(import_from_as_names, lookahead(eat_char_choice(","), !)),
        seq!(eat_char_choice("*")),
        seq!(invalid_import_from_targets));
    let import_from_as_names = choice!(seq!(gather(import_from_as_name, eat_char_choice(","))));
    let import_from_as_name = choice!(seq!(NAME, opt(choice!(seq!(eat_char_choice("as"), NAME)))));
    let dotted_as_names = choice!(seq!(gather(dotted_as_name, eat_char_choice(","))));
    let dotted_as_name = choice!(seq!(dotted_name, opt(choice!(seq!(eat_char_choice("as"), NAME)))));
    let dotted_name = choice!(seq!(dotted_name, eat_char_choice("."), NAME),
        seq!(NAME));
    let block = choice!(seq!(NEWLINE, INDENT, statements, DEDENT),
        seq!(simple_stmts),
        seq!(invalid_block));
    let decorators = choice!(seq!(repeat(group(choice!(seq!(eat_char_choice("@"), named_expression, NEWLINE))))));
    let class_def = choice!(seq!(decorators, class_def_raw),
        seq!(class_def_raw));
    let class_def_raw = choice!(seq!(invalid_class_def_raw),
        seq!(eat_char_choice("class"), NAME, opt(choice!(seq!(type_params))), opt(choice!(seq!(eat_char_choice("("), opt(choice!(seq!(arguments))), eat_char_choice(")")))), eat_char_choice(":"), block));
    let function_def = choice!(seq!(decorators, function_def_raw),
        seq!(function_def_raw));
    let function_def_raw = choice!(seq!(invalid_def_raw),
        seq!(eat_char_choice("def"), NAME, opt(choice!(seq!(type_params))), eat_char_choice("("), opt(choice!(seq!(params))), eat_char_choice(")"), opt(choice!(seq!(eat_char_choice("->"), expression))), eat_char_choice(":"), opt(choice!(seq!(func_type_comment))), block),
        seq!(eat_char_choice("async"), eat_char_choice("def"), NAME, opt(choice!(seq!(type_params))), eat_char_choice("("), opt(choice!(seq!(params))), eat_char_choice(")"), opt(choice!(seq!(eat_char_choice("->"), expression))), eat_char_choice(":"), opt(choice!(seq!(func_type_comment))), block));
    let params = choice!(seq!(invalid_parameters),
        seq!(parameters));
    let parameters = choice!(seq!(slash_no_default, repeat(param_no_default), repeat(param_with_default), opt(choice!(seq!(star_etc)))),
        seq!(slash_with_default, repeat(param_with_default), opt(choice!(seq!(star_etc)))),
        seq!(repeat(param_no_default), repeat(param_with_default), opt(choice!(seq!(star_etc)))),
        seq!(repeat(param_with_default), opt(choice!(seq!(star_etc)))),
        seq!(star_etc));
    let slash_no_default = choice!(seq!(repeat(param_no_default), eat_char_choice("/"), eat_char_choice(",")),
        seq!(repeat(param_no_default), eat_char_choice("/"), lookahead(eat_char_choice(")"), &)));
    let slash_with_default = choice!(seq!(repeat(param_no_default), repeat(param_with_default), eat_char_choice("/"), eat_char_choice(",")),
        seq!(repeat(param_no_default), repeat(param_with_default), eat_char_choice("/"), lookahead(eat_char_choice(")"), &)));
    let star_etc = choice!(seq!(invalid_star_etc),
        seq!(eat_char_choice("*"), param_no_default, repeat(param_maybe_default), opt(choice!(seq!(kwds)))),
        seq!(eat_char_choice("*"), param_no_default_star_annotation, repeat(param_maybe_default), opt(choice!(seq!(kwds)))),
        seq!(eat_char_choice("*"), eat_char_choice(","), repeat(param_maybe_default), opt(choice!(seq!(kwds)))),
        seq!(kwds));
    let kwds = choice!(seq!(invalid_kwds),
        seq!(eat_char_choice("**"), param_no_default));
    let param_no_default = choice!(seq!(param, eat_char_choice(","), opt(TYPE_COMMENT)),
        seq!(param, opt(TYPE_COMMENT), lookahead(eat_char_choice(")"), &)));
    let param_no_default_star_annotation = choice!(seq!(param_star_annotation, eat_char_choice(","), opt(TYPE_COMMENT)),
        seq!(param_star_annotation, opt(TYPE_COMMENT), lookahead(eat_char_choice(")"), &)));
    let param_with_default = choice!(seq!(param, default, eat_char_choice(","), opt(TYPE_COMMENT)),
        seq!(param, default, opt(TYPE_COMMENT), lookahead(eat_char_choice(")"), &)));
    let param_maybe_default = choice!(seq!(param, opt(default), eat_char_choice(","), opt(TYPE_COMMENT)),
        seq!(param, opt(default), opt(TYPE_COMMENT), lookahead(eat_char_choice(")"), &)));
    let param = choice!(seq!(NAME, opt(annotation)));
    let param_star_annotation = choice!(seq!(NAME, star_annotation));
    let annotation = choice!(seq!(eat_char_choice(":"), expression));
    let star_annotation = choice!(seq!(eat_char_choice(":"), star_expression));
    let default = choice!(seq!(eat_char_choice("="), expression),
        seq!(invalid_default));
    let if_stmt = choice!(seq!(invalid_if_stmt),
        seq!(eat_char_choice("if"), named_expression, eat_char_choice(":"), block, elif_stmt),
        seq!(eat_char_choice("if"), named_expression, eat_char_choice(":"), block, opt(choice!(seq!(else_block)))));
    let elif_stmt = choice!(seq!(invalid_elif_stmt),
        seq!(eat_char_choice("elif"), named_expression, eat_char_choice(":"), block, elif_stmt),
        seq!(eat_char_choice("elif"), named_expression, eat_char_choice(":"), block, opt(choice!(seq!(else_block)))));
    let else_block = choice!(seq!(invalid_else_stmt),
        seq!(eat_char_choice("else"), forced(eat_char_choice(":")), block));
    let while_stmt = choice!(seq!(invalid_while_stmt),
        seq!(eat_char_choice("while"), named_expression, eat_char_choice(":"), block, opt(choice!(seq!(else_block)))));
    let for_stmt = choice!(seq!(invalid_for_stmt),
        seq!(eat_char_choice("for"), star_targets, eat_char_choice("in"), cut(), star_expressions, eat_char_choice(":"), opt(choice!(seq!(TYPE_COMMENT))), block, opt(choice!(seq!(else_block)))),
        seq!(eat_char_choice("async"), eat_char_choice("for"), star_targets, eat_char_choice("in"), cut(), star_expressions, eat_char_choice(":"), opt(choice!(seq!(TYPE_COMMENT))), block, opt(choice!(seq!(else_block)))),
        seq!(invalid_for_target));
    let with_stmt = choice!(seq!(invalid_with_stmt_indent),
        seq!(eat_char_choice("with"), eat_char_choice("("), gather(with_item, eat_char_choice(",")), opt(eat_char_choice(",")), eat_char_choice(")"), eat_char_choice(":"), opt(choice!(seq!(TYPE_COMMENT))), block),
        seq!(eat_char_choice("with"), gather(with_item, eat_char_choice(",")), eat_char_choice(":"), opt(choice!(seq!(TYPE_COMMENT))), block),
        seq!(eat_char_choice("async"), eat_char_choice("with"), eat_char_choice("("), gather(with_item, eat_char_choice(",")), opt(eat_char_choice(",")), eat_char_choice(")"), eat_char_choice(":"), block),
        seq!(eat_char_choice("async"), eat_char_choice("with"), gather(with_item, eat_char_choice(",")), eat_char_choice(":"), opt(choice!(seq!(TYPE_COMMENT))), block),
        seq!(invalid_with_stmt));
    let with_item = choice!(seq!(expression, eat_char_choice("as"), star_target, lookahead(group(choice!(seq!(eat_char_choice(",")), seq!(eat_char_choice(")")), seq!(eat_char_choice(":")))), &)),
        seq!(invalid_with_item),
        seq!(expression));
    let try_stmt = choice!(seq!(invalid_try_stmt),
        seq!(eat_char_choice("try"), forced(eat_char_choice(":")), block, finally_block),
        seq!(eat_char_choice("try"), forced(eat_char_choice(":")), block, repeat(except_block), opt(choice!(seq!(else_block))), opt(choice!(seq!(finally_block)))),
        seq!(eat_char_choice("try"), forced(eat_char_choice(":")), block, repeat(except_star_block), opt(choice!(seq!(else_block))), opt(choice!(seq!(finally_block)))));
    let except_block = choice!(seq!(invalid_except_stmt_indent),
        seq!(eat_char_choice("except"), expression, opt(choice!(seq!(eat_char_choice("as"), NAME))), eat_char_choice(":"), block),
        seq!(eat_char_choice("except"), eat_char_choice(":"), block),
        seq!(invalid_except_stmt));
    let except_star_block = choice!(seq!(invalid_except_star_stmt_indent),
        seq!(eat_char_choice("except"), eat_char_choice("*"), expression, opt(choice!(seq!(eat_char_choice("as"), NAME))), eat_char_choice(":"), block),
        seq!(invalid_except_stmt));
    let finally_block = choice!(seq!(invalid_finally_stmt),
        seq!(eat_char_choice("finally"), forced(eat_char_choice(":")), block));
    let match_stmt = choice!(seq!("match", subject_expr, eat_char_choice(":"), NEWLINE, INDENT, repeat(case_block), DEDENT),
        seq!(invalid_match_stmt));
    let subject_expr = choice!(seq!(star_named_expression, eat_char_choice(","), opt(star_named_expressions)),
        seq!(named_expression));
    let case_block = choice!(seq!(invalid_case_block),
        seq!("case", patterns, opt(guard), eat_char_choice(":"), block));
    let guard = choice!(seq!(eat_char_choice("if"), named_expression));
    let patterns = choice!(seq!(open_sequence_pattern),
        seq!(pattern));
    let pattern = choice!(seq!(as_pattern),
        seq!(or_pattern));
    let as_pattern = choice!(seq!(or_pattern, eat_char_choice("as"), pattern_capture_target),
        seq!(invalid_as_pattern));
    let or_pattern = choice!(seq!(gather(closed_pattern, eat_char_choice("|"))));
    let closed_pattern = choice!(seq!(literal_pattern),
        seq!(capture_pattern),
        seq!(wildcard_pattern),
        seq!(value_pattern),
        seq!(group_pattern),
        seq!(sequence_pattern),
        seq!(mapping_pattern),
        seq!(class_pattern));
    let literal_pattern = choice!(seq!(signed_number, lookahead(group(choice!(seq!(eat_char_choice("+")), seq!(eat_char_choice("-")))), !)),
        seq!(complex_number),
        seq!(strings),
        seq!(eat_char_choice("None")),
        seq!(eat_char_choice("True")),
        seq!(eat_char_choice("False")));
    let literal_expr = choice!(seq!(signed_number, lookahead(group(choice!(seq!(eat_char_choice("+")), seq!(eat_char_choice("-")))), !)),
        seq!(complex_number),
        seq!(strings),
        seq!(eat_char_choice("None")),
        seq!(eat_char_choice("True")),
        seq!(eat_char_choice("False")));
    let complex_number = choice!(seq!(signed_real_number, eat_char_choice("+"), imaginary_number),
        seq!(signed_real_number, eat_char_choice("-"), imaginary_number));
    let signed_number = choice!(seq!(NUMBER),
        seq!(eat_char_choice("-"), NUMBER));
    let signed_real_number = choice!(seq!(real_number),
        seq!(eat_char_choice("-"), real_number));
    let real_number = choice!(seq!(NUMBER));
    let imaginary_number = choice!(seq!(NUMBER));
    let capture_pattern = choice!(seq!(pattern_capture_target));
    let pattern_capture_target = choice!(seq!(lookahead("_", !), NAME, lookahead(group(choice!(seq!(eat_char_choice(".")), seq!(eat_char_choice("(")), seq!(eat_char_choice("=")))), !)));
    let wildcard_pattern = choice!(seq!("_"));
    let value_pattern = choice!(seq!(attr, lookahead(group(choice!(seq!(eat_char_choice(".")), seq!(eat_char_choice("(")), seq!(eat_char_choice("=")))), !)));
    let attr = choice!(seq!(name_or_attr, eat_char_choice("."), NAME));
    let name_or_attr = choice!(seq!(attr),
        seq!(NAME));
    let group_pattern = choice!(seq!(eat_char_choice("("), pattern, eat_char_choice(")")));
    let sequence_pattern = choice!(seq!(eat_char_choice("["), opt(maybe_sequence_pattern), eat_char_choice("]")),
        seq!(eat_char_choice("("), opt(open_sequence_pattern), eat_char_choice(")")));
    let open_sequence_pattern = choice!(seq!(maybe_star_pattern, eat_char_choice(","), opt(maybe_sequence_pattern)));
    let maybe_sequence_pattern = choice!(seq!(gather(maybe_star_pattern, eat_char_choice(",")), opt(eat_char_choice(","))));
    let maybe_star_pattern = choice!(seq!(star_pattern),
        seq!(pattern));
    let star_pattern = choice!(seq!(eat_char_choice("*"), pattern_capture_target),
        seq!(eat_char_choice("*"), wildcard_pattern));
    let mapping_pattern = choice!(seq!(eat_char_choice("{"), eat_char_choice("}")),
        seq!(eat_char_choice("{"), double_star_pattern, opt(eat_char_choice(",")), eat_char_choice("}")),
        seq!(eat_char_choice("{"), items_pattern, eat_char_choice(","), double_star_pattern, opt(eat_char_choice(",")), eat_char_choice("}")),
        seq!(eat_char_choice("{"), items_pattern, opt(eat_char_choice(",")), eat_char_choice("}")));
    let items_pattern = choice!(seq!(gather(key_value_pattern, eat_char_choice(","))));
    let key_value_pattern = choice!(seq!(group(choice!(seq!(literal_expr), seq!(attr))), eat_char_choice(":"), pattern));
    let double_star_pattern = choice!(seq!(eat_char_choice("**"), pattern_capture_target));
    let class_pattern = choice!(seq!(name_or_attr, eat_char_choice("("), eat_char_choice(")")),
        seq!(name_or_attr, eat_char_choice("("), positional_patterns, opt(eat_char_choice(",")), eat_char_choice(")")),
        seq!(name_or_attr, eat_char_choice("("), keyword_patterns, opt(eat_char_choice(",")), eat_char_choice(")")),
        seq!(name_or_attr, eat_char_choice("("), positional_patterns, eat_char_choice(","), keyword_patterns, opt(eat_char_choice(",")), eat_char_choice(")")),
        seq!(invalid_class_pattern));
    let positional_patterns = choice!(seq!(gather(pattern, eat_char_choice(","))));
    let keyword_patterns = choice!(seq!(gather(keyword_pattern, eat_char_choice(","))));
    let keyword_pattern = choice!(seq!(NAME, eat_char_choice("="), pattern));
    let type_alias = choice!(seq!("type", NAME, opt(choice!(seq!(type_params))), eat_char_choice("="), expression));
    let type_params = choice!(seq!(invalid_type_params),
        seq!(eat_char_choice("["), type_param_seq, eat_char_choice("]")));
    let type_param_seq = choice!(seq!(gather(type_param, eat_char_choice(",")), opt(choice!(seq!(eat_char_choice(","))))));
    let type_param = choice!(seq!(NAME, opt(choice!(seq!(type_param_bound))), opt(choice!(seq!(type_param_default)))),
        seq!(invalid_type_param),
        seq!(eat_char_choice("*"), NAME, opt(choice!(seq!(type_param_starred_default)))),
        seq!(eat_char_choice("**"), NAME, opt(choice!(seq!(type_param_default)))));
    let type_param_bound = choice!(seq!(eat_char_choice(":"), expression));
    let type_param_default = choice!(seq!(eat_char_choice("="), expression));
    let type_param_starred_default = choice!(seq!(eat_char_choice("="), star_expression));
    let expressions = choice!(seq!(expression, repeat(group(choice!(seq!(eat_char_choice(","), expression)))), opt(choice!(seq!(eat_char_choice(","))))),
        seq!(expression, eat_char_choice(",")),
        seq!(expression));
    let expression = choice!(seq!(invalid_expression),
        seq!(invalid_legacy_expression),
        seq!(disjunction, eat_char_choice("if"), disjunction, eat_char_choice("else"), expression),
        seq!(disjunction),
        seq!(lambdef));
    let yield_expr = choice!(seq!(eat_char_choice("yield"), eat_char_choice("from"), expression),
        seq!(eat_char_choice("yield"), opt(choice!(seq!(star_expressions)))));
    let star_expressions = choice!(seq!(star_expression, repeat(group(choice!(seq!(eat_char_choice(","), star_expression)))), opt(choice!(seq!(eat_char_choice(","))))),
        seq!(star_expression, eat_char_choice(",")),
        seq!(star_expression));
    let star_expression = choice!(seq!(eat_char_choice("*"), bitwise_or),
        seq!(expression));
    let star_named_expressions = choice!(seq!(gather(star_named_expression, eat_char_choice(",")), opt(choice!(seq!(eat_char_choice(","))))));
    let star_named_expression = choice!(seq!(eat_char_choice("*"), bitwise_or),
        seq!(named_expression));
    let assignment_expression = choice!(seq!(NAME, eat_char_choice(":="), cut(), expression));
    let named_expression = choice!(seq!(assignment_expression),
        seq!(invalid_named_expression),
        seq!(expression, lookahead(eat_char_choice(":="), !)));
    let disjunction = choice!(seq!(conjunction, repeat(group(choice!(seq!(eat_char_choice("or"), conjunction))))),
        seq!(conjunction));
    let conjunction = choice!(seq!(inversion, repeat(group(choice!(seq!(eat_char_choice("and"), inversion))))),
        seq!(inversion));
    let inversion = choice!(seq!(eat_char_choice("not"), inversion),
        seq!(comparison));
    let comparison = choice!(seq!(bitwise_or, repeat(compare_op_bitwise_or_pair)),
        seq!(bitwise_or));
    let compare_op_bitwise_or_pair = choice!(seq!(eq_bitwise_or),
        seq!(noteq_bitwise_or),
        seq!(lte_bitwise_or),
        seq!(lt_bitwise_or),
        seq!(gte_bitwise_or),
        seq!(gt_bitwise_or),
        seq!(notin_bitwise_or),
        seq!(in_bitwise_or),
        seq!(isnot_bitwise_or),
        seq!(is_bitwise_or));
    let eq_bitwise_or = choice!(seq!(eat_char_choice("=="), bitwise_or));
    let noteq_bitwise_or = choice!(seq!(group(choice!(seq!(eat_char_choice("!=")))), bitwise_or));
    let lte_bitwise_or = choice!(seq!(eat_char_choice("<="), bitwise_or));
    let lt_bitwise_or = choice!(seq!(eat_char_choice("<"), bitwise_or));
    let gte_bitwise_or = choice!(seq!(eat_char_choice(">="), bitwise_or));
    let gt_bitwise_or = choice!(seq!(eat_char_choice(">"), bitwise_or));
    let notin_bitwise_or = choice!(seq!(eat_char_choice("not"), eat_char_choice("in"), bitwise_or));
    let in_bitwise_or = choice!(seq!(eat_char_choice("in"), bitwise_or));
    let isnot_bitwise_or = choice!(seq!(eat_char_choice("is"), eat_char_choice("not"), bitwise_or));
    let is_bitwise_or = choice!(seq!(eat_char_choice("is"), bitwise_or));
    let bitwise_or = choice!(seq!(bitwise_or, eat_char_choice("|"), bitwise_xor),
        seq!(bitwise_xor));
    let bitwise_xor = choice!(seq!(bitwise_xor, eat_char_choice("^"), bitwise_and),
        seq!(bitwise_and));
    let bitwise_and = choice!(seq!(bitwise_and, eat_char_choice("&"), shift_expr),
        seq!(shift_expr));
    let shift_expr = choice!(seq!(shift_expr, eat_char_choice("<<"), sum),
        seq!(shift_expr, eat_char_choice(">>"), sum),
        seq!(invalid_arithmetic),
        seq!(sum));
    let sum = choice!(seq!(sum, eat_char_choice("+"), term),
        seq!(sum, eat_char_choice("-"), term),
        seq!(term));
    let term = choice!(seq!(term, eat_char_choice("*"), factor),
        seq!(term, eat_char_choice("/"), factor),
        seq!(term, eat_char_choice("//"), factor),
        seq!(term, eat_char_choice("%"), factor),
        seq!(term, eat_char_choice("@"), factor),
        seq!(invalid_factor),
        seq!(factor));
    let factor = choice!(seq!(eat_char_choice("+"), factor),
        seq!(eat_char_choice("-"), factor),
        seq!(eat_char_choice("~"), factor),
        seq!(power));
    let power = choice!(seq!(await_primary, eat_char_choice("**"), factor),
        seq!(await_primary));
    let await_primary = choice!(seq!(eat_char_choice("await"), primary),
        seq!(primary));
    let primary = choice!(seq!(primary, eat_char_choice("."), NAME),
        seq!(primary, genexp),
        seq!(primary, eat_char_choice("("), opt(choice!(seq!(arguments))), eat_char_choice(")")),
        seq!(primary, eat_char_choice("["), slices, eat_char_choice("]")),
        seq!(atom));
    let slices = choice!(seq!(slice, lookahead(eat_char_choice(","), !)),
        seq!(gather(group(choice!(seq!(slice), seq!(starred_expression))), eat_char_choice(",")), opt(choice!(seq!(eat_char_choice(","))))));
    let slice = choice!(seq!(opt(choice!(seq!(expression))), eat_char_choice(":"), opt(choice!(seq!(expression))), opt(choice!(seq!(eat_char_choice(":"), opt(choice!(seq!(expression))))))),
        seq!(named_expression));
    let atom = choice!(seq!(NAME),
        seq!(eat_char_choice("True")),
        seq!(eat_char_choice("False")),
        seq!(eat_char_choice("None")),
        seq!(lookahead(group(choice!(seq!(STRING), seq!(FSTRING_START))), &), strings),
        seq!(NUMBER),
        seq!(lookahead(eat_char_choice("("), &), group(choice!(seq!(tuple), seq!(group), seq!(genexp)))),
        seq!(lookahead(eat_char_choice("["), &), group(choice!(seq!(list), seq!(listcomp)))),
        seq!(lookahead(eat_char_choice("{"), &), group(choice!(seq!(dict), seq!(set), seq!(dictcomp), seq!(setcomp)))),
        seq!(eat_char_choice("...")));
    let group = choice!(seq!(eat_char_choice("("), group(choice!(seq!(yield_expr), seq!(named_expression))), eat_char_choice(")")),
        seq!(invalid_group));
    let lambdef = choice!(seq!(eat_char_choice("lambda"), opt(choice!(seq!(lambda_params))), eat_char_choice(":"), expression));
    let lambda_params = choice!(seq!(invalid_lambda_parameters),
        seq!(lambda_parameters));
    let lambda_parameters = choice!(seq!(lambda_slash_no_default, repeat(lambda_param_no_default), repeat(lambda_param_with_default), opt(choice!(seq!(lambda_star_etc)))),
        seq!(lambda_slash_with_default, repeat(lambda_param_with_default), opt(choice!(seq!(lambda_star_etc)))),
        seq!(repeat(lambda_param_no_default), repeat(lambda_param_with_default), opt(choice!(seq!(lambda_star_etc)))),
        seq!(repeat(lambda_param_with_default), opt(choice!(seq!(lambda_star_etc)))),
        seq!(lambda_star_etc));
    let lambda_slash_no_default = choice!(seq!(repeat(lambda_param_no_default), eat_char_choice("/"), eat_char_choice(",")),
        seq!(repeat(lambda_param_no_default), eat_char_choice("/"), lookahead(eat_char_choice(":"), &)));
    let lambda_slash_with_default = choice!(seq!(repeat(lambda_param_no_default), repeat(lambda_param_with_default), eat_char_choice("/"), eat_char_choice(",")),
        seq!(repeat(lambda_param_no_default), repeat(lambda_param_with_default), eat_char_choice("/"), lookahead(eat_char_choice(":"), &)));
    let lambda_star_etc = choice!(seq!(invalid_lambda_star_etc),
        seq!(eat_char_choice("*"), lambda_param_no_default, repeat(lambda_param_maybe_default), opt(choice!(seq!(lambda_kwds)))),
        seq!(eat_char_choice("*"), eat_char_choice(","), repeat(lambda_param_maybe_default), opt(choice!(seq!(lambda_kwds)))),
        seq!(lambda_kwds));
    let lambda_kwds = choice!(seq!(invalid_lambda_kwds),
        seq!(eat_char_choice("**"), lambda_param_no_default));
    let lambda_param_no_default = choice!(seq!(lambda_param, eat_char_choice(",")),
        seq!(lambda_param, lookahead(eat_char_choice(":"), &)));
    let lambda_param_with_default = choice!(seq!(lambda_param, default, eat_char_choice(",")),
        seq!(lambda_param, default, lookahead(eat_char_choice(":"), &)));
    let lambda_param_maybe_default = choice!(seq!(lambda_param, opt(default), eat_char_choice(",")),
        seq!(lambda_param, opt(default), lookahead(eat_char_choice(":"), &)));
    let lambda_param = choice!(seq!(NAME));
    let fstring_middle = choice!(seq!(fstring_replacement_field),
        seq!(FSTRING_MIDDLE));
    let fstring_replacement_field = choice!(seq!(eat_char_choice("{"), annotated_rhs, opt(eat_char_choice("=")), opt(choice!(seq!(fstring_conversion))), opt(choice!(seq!(fstring_full_format_spec))), eat_char_choice("}")),
        seq!(invalid_replacement_field));
    let fstring_conversion = choice!(seq!("!", NAME));
    let fstring_full_format_spec = choice!(seq!(eat_char_choice(":"), repeat(fstring_format_spec)));
    let fstring_format_spec = choice!(seq!(FSTRING_MIDDLE),
        seq!(fstring_replacement_field));
    let fstring = choice!(seq!(FSTRING_START, repeat(fstring_middle), FSTRING_END));
    let string = choice!(seq!(STRING));
    let strings = choice!(seq!(repeat(group(choice!(seq!(fstring), seq!(string))))));
    let list = choice!(seq!(eat_char_choice("["), opt(choice!(seq!(star_named_expressions))), eat_char_choice("]")));
    let tuple = choice!(seq!(eat_char_choice("("), opt(choice!(seq!(star_named_expression, eat_char_choice(","), opt(choice!(seq!(star_named_expressions)))))), eat_char_choice(")")));
    let set = choice!(seq!(eat_char_choice("{"), star_named_expressions, eat_char_choice("}")));
    let dict = choice!(seq!(eat_char_choice("{"), opt(choice!(seq!(double_starred_kvpairs))), eat_char_choice("}")),
        seq!(eat_char_choice("{"), invalid_double_starred_kvpairs, eat_char_choice("}")));
    let double_starred_kvpairs = choice!(seq!(gather(double_starred_kvpair, eat_char_choice(",")), opt(choice!(seq!(eat_char_choice(","))))));
    let double_starred_kvpair = choice!(seq!(eat_char_choice("**"), bitwise_or),
        seq!(kvpair));
    let kvpair = choice!(seq!(expression, eat_char_choice(":"), expression));
    let for_if_clauses = choice!(seq!(repeat(for_if_clause)));
    let for_if_clause = choice!(seq!(eat_char_choice("async"), eat_char_choice("for"), star_targets, eat_char_choice("in"), cut(), disjunction, repeat(group(choice!(seq!(eat_char_choice("if"), disjunction))))),
        seq!(eat_char_choice("for"), star_targets, eat_char_choice("in"), cut(), disjunction, repeat(group(choice!(seq!(eat_char_choice("if"), disjunction))))),
        seq!(invalid_for_if_clause),
        seq!(invalid_for_target));
    let listcomp = choice!(seq!(eat_char_choice("["), named_expression, for_if_clauses, eat_char_choice("]")),
        seq!(invalid_comprehension));
    let setcomp = choice!(seq!(eat_char_choice("{"), named_expression, for_if_clauses, eat_char_choice("}")),
        seq!(invalid_comprehension));
    let genexp = choice!(seq!(eat_char_choice("("), group(choice!(seq!(assignment_expression), seq!(expression, lookahead(eat_char_choice(":="), !)))), for_if_clauses, eat_char_choice(")")),
        seq!(invalid_comprehension));
    let dictcomp = choice!(seq!(eat_char_choice("{"), kvpair, for_if_clauses, eat_char_choice("}")),
        seq!(invalid_dict_comprehension));
    let arguments = choice!(seq!(args, opt(choice!(seq!(eat_char_choice(",")))), lookahead(eat_char_choice(")"), &)),
        seq!(invalid_arguments));
    let args = choice!(seq!(gather(group(choice!(seq!(starred_expression), seq!(group(choice!(seq!(assignment_expression), seq!(expression, lookahead(eat_char_choice(":="), !)))), lookahead(eat_char_choice("="), !)))), eat_char_choice(",")), opt(choice!(seq!(eat_char_choice(","), kwargs)))),
        seq!(kwargs));
    let kwargs = choice!(seq!(gather(kwarg_or_starred, eat_char_choice(",")), eat_char_choice(","), gather(kwarg_or_double_starred, eat_char_choice(","))),
        seq!(gather(kwarg_or_starred, eat_char_choice(","))),
        seq!(gather(kwarg_or_double_starred, eat_char_choice(","))));
    let starred_expression = choice!(seq!(invalid_starred_expression_unpacking),
        seq!(eat_char_choice("*"), expression),
        seq!(invalid_starred_expression));
    let kwarg_or_starred = choice!(seq!(invalid_kwarg),
        seq!(NAME, eat_char_choice("="), expression),
        seq!(starred_expression));
    let kwarg_or_double_starred = choice!(seq!(invalid_kwarg),
        seq!(NAME, eat_char_choice("="), expression),
        seq!(eat_char_choice("**"), expression));
    let star_targets = choice!(seq!(star_target, lookahead(eat_char_choice(","), !)),
        seq!(star_target, repeat(group(choice!(seq!(eat_char_choice(","), star_target)))), opt(choice!(seq!(eat_char_choice(","))))));
    let star_targets_list_seq = choice!(seq!(gather(star_target, eat_char_choice(",")), opt(choice!(seq!(eat_char_choice(","))))));
    let star_targets_tuple_seq = choice!(seq!(star_target, repeat(group(choice!(seq!(eat_char_choice(","), star_target)))), opt(choice!(seq!(eat_char_choice(","))))),
        seq!(star_target, eat_char_choice(",")));
    let star_target = choice!(seq!(eat_char_choice("*"), group(choice!(seq!(lookahead(eat_char_choice("*"), !), star_target)))),
        seq!(target_with_star_atom));
    let target_with_star_atom = choice!(seq!(t_primary, eat_char_choice("."), NAME, lookahead(t_lookahead, !)),
        seq!(t_primary, eat_char_choice("["), slices, eat_char_choice("]"), lookahead(t_lookahead, !)),
        seq!(star_atom));
    let star_atom = choice!(seq!(NAME),
        seq!(eat_char_choice("("), target_with_star_atom, eat_char_choice(")")),
        seq!(eat_char_choice("("), opt(choice!(seq!(star_targets_tuple_seq))), eat_char_choice(")")),
        seq!(eat_char_choice("["), opt(choice!(seq!(star_targets_list_seq))), eat_char_choice("]")));
    let single_target = choice!(seq!(single_subscript_attribute_target),
        seq!(NAME),
        seq!(eat_char_choice("("), single_target, eat_char_choice(")")));
    let single_subscript_attribute_target = choice!(seq!(t_primary, eat_char_choice("."), NAME, lookahead(t_lookahead, !)),
        seq!(t_primary, eat_char_choice("["), slices, eat_char_choice("]"), lookahead(t_lookahead, !)));
    let t_primary = choice!(seq!(t_primary, eat_char_choice("."), NAME, lookahead(t_lookahead, &)),
        seq!(t_primary, eat_char_choice("["), slices, eat_char_choice("]"), lookahead(t_lookahead, &)),
        seq!(t_primary, genexp, lookahead(t_lookahead, &)),
        seq!(t_primary, eat_char_choice("("), opt(choice!(seq!(arguments))), eat_char_choice(")"), lookahead(t_lookahead, &)),
        seq!(atom, lookahead(t_lookahead, &)));
    let t_lookahead = choice!(seq!(eat_char_choice("(")),
        seq!(eat_char_choice("[")),
        seq!(eat_char_choice(".")));
    let del_targets = choice!(seq!(gather(del_target, eat_char_choice(",")), opt(choice!(seq!(eat_char_choice(","))))));
    let del_target = choice!(seq!(t_primary, eat_char_choice("."), NAME, lookahead(t_lookahead, !)),
        seq!(t_primary, eat_char_choice("["), slices, eat_char_choice("]"), lookahead(t_lookahead, !)),
        seq!(del_t_atom));
    let del_t_atom = choice!(seq!(NAME),
        seq!(eat_char_choice("("), del_target, eat_char_choice(")")),
        seq!(eat_char_choice("("), opt(choice!(seq!(del_targets))), eat_char_choice(")")),
        seq!(eat_char_choice("["), opt(choice!(seq!(del_targets))), eat_char_choice("]")));
    let type_expressions = choice!(seq!(gather(expression, eat_char_choice(",")), eat_char_choice(","), eat_char_choice("*"), expression, eat_char_choice(","), eat_char_choice("**"), expression),
        seq!(gather(expression, eat_char_choice(",")), eat_char_choice(","), eat_char_choice("*"), expression),
        seq!(gather(expression, eat_char_choice(",")), eat_char_choice(","), eat_char_choice("**"), expression),
        seq!(eat_char_choice("*"), expression, eat_char_choice(","), eat_char_choice("**"), expression),
        seq!(eat_char_choice("*"), expression),
        seq!(eat_char_choice("**"), expression),
        seq!(gather(expression, eat_char_choice(","))));
    let func_type_comment = choice!(seq!(NEWLINE, TYPE_COMMENT, lookahead(group(choice!(seq!(NEWLINE, INDENT))), &)),
        seq!(invalid_double_type_comments),
        seq!(TYPE_COMMENT));
    let invalid_arguments = choice!(seq!(group(choice!(seq!(group(choice!(seq!(gather(group(choice!(seq!(starred_expression), seq!(group(choice!(seq!(assignment_expression), seq!(expression, lookahead(eat_char_choice(":="), !)))), lookahead(eat_char_choice("="), !)))), eat_char_choice(",")), eat_char_choice(","), kwargs)))), seq!(kwargs))), eat_char_choice(","), gather(group(choice!(seq!(starred_expression, lookahead(eat_char_choice("="), !)))), eat_char_choice(","))),
        seq!(expression, for_if_clauses, eat_char_choice(","), opt(choice!(seq!(args), seq!(expression, for_if_clauses)))),
        seq!(NAME, eat_char_choice("="), expression, for_if_clauses),
        seq!(opt(group(choice!(seq!(args, eat_char_choice(","))))), NAME, eat_char_choice("="), lookahead(group(choice!(seq!(eat_char_choice(",")), seq!(eat_char_choice(")")))), &)),
        seq!(args, for_if_clauses),
        seq!(args, eat_char_choice(","), expression, for_if_clauses),
        seq!(args, eat_char_choice(","), args));
    let invalid_kwarg = choice!(seq!(group(choice!(seq!(eat_char_choice("True")), seq!(eat_char_choice("False")), seq!(eat_char_choice("None")))), eat_char_choice("=")),
        seq!(NAME, eat_char_choice("="), expression, for_if_clauses),
        seq!(lookahead(group(choice!(seq!(NAME, eat_char_choice("=")))), !), expression, eat_char_choice("=")),
        seq!(eat_char_choice("**"), expression, eat_char_choice("="), expression));
    let expression_without_invalid = choice!(seq!(disjunction, eat_char_choice("if"), disjunction, eat_char_choice("else"), expression),
        seq!(disjunction),
        seq!(lambdef));
    let invalid_legacy_expression = choice!(seq!(NAME, lookahead(eat_char_choice("("), !), star_expressions));
    let invalid_type_param = choice!(seq!(eat_char_choice("*"), NAME, eat_char_choice(":"), expression),
        seq!(eat_char_choice("**"), NAME, eat_char_choice(":"), expression));
    let invalid_expression = choice!(seq!(lookahead(group(choice!(seq!(NAME, STRING), seq!(SOFT_KEYWORD))), !), disjunction, expression_without_invalid),
        seq!(disjunction, eat_char_choice("if"), disjunction, lookahead(group(choice!(seq!(eat_char_choice("else")), seq!(eat_char_choice(":")))), !)),
        seq!(eat_char_choice("lambda"), opt(choice!(seq!(lambda_params))), eat_char_choice(":"), lookahead(FSTRING_MIDDLE, &)));
    let invalid_named_expression = choice!(seq!(expression, eat_char_choice(":="), expression),
        seq!(NAME, eat_char_choice("="), bitwise_or, lookahead(group(choice!(seq!(eat_char_choice("=")), seq!(eat_char_choice(":=")))), !)),
        seq!(lookahead(group(choice!(seq!(list), seq!(tuple), seq!(genexp), seq!(eat_char_choice("True")), seq!(eat_char_choice("None")), seq!(eat_char_choice("False")))), !), bitwise_or, eat_char_choice("="), bitwise_or, lookahead(group(choice!(seq!(eat_char_choice("=")), seq!(eat_char_choice(":=")))), !)));
    let invalid_assignment = choice!(seq!(invalid_ann_assign_target, eat_char_choice(":"), expression),
        seq!(star_named_expression, eat_char_choice(","), repeat(star_named_expressions), eat_char_choice(":"), expression),
        seq!(expression, eat_char_choice(":"), expression),
        seq!(repeat(group(choice!(seq!(star_targets, eat_char_choice("="))))), star_expressions, eat_char_choice("=")),
        seq!(repeat(group(choice!(seq!(star_targets, eat_char_choice("="))))), yield_expr, eat_char_choice("=")),
        seq!(star_expressions, augassign, annotated_rhs));
    let invalid_ann_assign_target = choice!(seq!(list),
        seq!(tuple),
        seq!(eat_char_choice("("), invalid_ann_assign_target, eat_char_choice(")")));
    let invalid_del_stmt = choice!(seq!(eat_char_choice("del"), star_expressions));
    let invalid_block = choice!(seq!(NEWLINE, lookahead(INDENT, !)));
    let invalid_comprehension = choice!(seq!(group(choice!(seq!(eat_char_choice("[")), seq!(eat_char_choice("(")), seq!(eat_char_choice("{")))), starred_expression, for_if_clauses),
        seq!(group(choice!(seq!(eat_char_choice("[")), seq!(eat_char_choice("{")))), star_named_expression, eat_char_choice(","), star_named_expressions, for_if_clauses),
        seq!(group(choice!(seq!(eat_char_choice("[")), seq!(eat_char_choice("{")))), star_named_expression, eat_char_choice(","), for_if_clauses));
    let invalid_dict_comprehension = choice!(seq!(eat_char_choice("{"), eat_char_choice("**"), bitwise_or, for_if_clauses, eat_char_choice("}")));
    let invalid_parameters = choice!(seq!("/", eat_char_choice(",")),
        seq!(group(choice!(seq!(slash_no_default), seq!(slash_with_default))), repeat(param_maybe_default), eat_char_choice("/")),
        seq!(opt(slash_no_default), repeat(param_no_default), invalid_parameters_helper, param_no_default),
        seq!(repeat(param_no_default), eat_char_choice("("), repeat(param_no_default), opt(eat_char_choice(",")), eat_char_choice(")")),
        seq!(opt(group(choice!(seq!(slash_no_default), seq!(slash_with_default)))), repeat(param_maybe_default), eat_char_choice("*"), group(choice!(seq!(eat_char_choice(",")), seq!(param_no_default))), repeat(param_maybe_default), eat_char_choice("/")),
        seq!(repeat(param_maybe_default), eat_char_choice("/"), eat_char_choice("*")));
    let invalid_default = choice!(seq!(eat_char_choice("="), lookahead(group(choice!(seq!(eat_char_choice(")")), seq!(eat_char_choice(",")))), &)));
    let invalid_star_etc = choice!(seq!(eat_char_choice("*"), group(choice!(seq!(eat_char_choice(")")), seq!(eat_char_choice(","), group(choice!(seq!(eat_char_choice(")")), seq!(eat_char_choice("**")))))))),
        seq!(eat_char_choice("*"), eat_char_choice(","), TYPE_COMMENT),
        seq!(eat_char_choice("*"), param, eat_char_choice("=")),
        seq!(eat_char_choice("*"), group(choice!(seq!(param_no_default), seq!(eat_char_choice(",")))), repeat(param_maybe_default), eat_char_choice("*"), group(choice!(seq!(param_no_default), seq!(eat_char_choice(","))))));
    let invalid_kwds = choice!(seq!(eat_char_choice("**"), param, eat_char_choice("=")),
        seq!(eat_char_choice("**"), param, eat_char_choice(","), param),
        seq!(eat_char_choice("**"), param, eat_char_choice(","), group(choice!(seq!(eat_char_choice("*")), seq!(eat_char_choice("**")), seq!(eat_char_choice("/"))))));
    let invalid_parameters_helper = choice!(seq!(slash_with_default),
        seq!(repeat(param_with_default)));
    let invalid_lambda_parameters = choice!(seq!("/", eat_char_choice(",")),
        seq!(group(choice!(seq!(lambda_slash_no_default), seq!(lambda_slash_with_default))), repeat(lambda_param_maybe_default), eat_char_choice("/")),
        seq!(opt(lambda_slash_no_default), repeat(lambda_param_no_default), invalid_lambda_parameters_helper, lambda_param_no_default),
        seq!(repeat(lambda_param_no_default), eat_char_choice("("), gather(lambda_param, eat_char_choice(",")), opt(eat_char_choice(",")), eat_char_choice(")")),
        seq!(opt(group(choice!(seq!(lambda_slash_no_default), seq!(lambda_slash_with_default)))), repeat(lambda_param_maybe_default), eat_char_choice("*"), group(choice!(seq!(eat_char_choice(",")), seq!(lambda_param_no_default))), repeat(lambda_param_maybe_default), eat_char_choice("/")),
        seq!(repeat(lambda_param_maybe_default), eat_char_choice("/"), eat_char_choice("*")));
    let invalid_lambda_parameters_helper = choice!(seq!(lambda_slash_with_default),
        seq!(repeat(lambda_param_with_default)));
    let invalid_lambda_star_etc = choice!(seq!(eat_char_choice("*"), group(choice!(seq!(eat_char_choice(":")), seq!(eat_char_choice(","), group(choice!(seq!(eat_char_choice(":")), seq!(eat_char_choice("**")))))))),
        seq!(eat_char_choice("*"), lambda_param, eat_char_choice("=")),
        seq!(eat_char_choice("*"), group(choice!(seq!(lambda_param_no_default), seq!(eat_char_choice(",")))), repeat(lambda_param_maybe_default), eat_char_choice("*"), group(choice!(seq!(lambda_param_no_default), seq!(eat_char_choice(","))))));
    let invalid_lambda_kwds = choice!(seq!(eat_char_choice("**"), lambda_param, eat_char_choice("=")),
        seq!(eat_char_choice("**"), lambda_param, eat_char_choice(","), lambda_param),
        seq!(eat_char_choice("**"), lambda_param, eat_char_choice(","), group(choice!(seq!(eat_char_choice("*")), seq!(eat_char_choice("**")), seq!(eat_char_choice("/"))))));
    let invalid_double_type_comments = choice!(seq!(TYPE_COMMENT, NEWLINE, TYPE_COMMENT, NEWLINE, INDENT));
    let invalid_with_item = choice!(seq!(expression, eat_char_choice("as"), expression, lookahead(group(choice!(seq!(eat_char_choice(",")), seq!(eat_char_choice(")")), seq!(eat_char_choice(":")))), &)));
    let invalid_for_if_clause = choice!(seq!(opt(eat_char_choice("async")), eat_char_choice("for"), group(choice!(seq!(bitwise_or, repeat(group(choice!(seq!(eat_char_choice(","), bitwise_or)))), opt(choice!(seq!(eat_char_choice(","))))))), lookahead(eat_char_choice("in"), !)));
    let invalid_for_target = choice!(seq!(opt(eat_char_choice("async")), eat_char_choice("for"), star_expressions));
    let invalid_group = choice!(seq!(eat_char_choice("("), starred_expression, eat_char_choice(")")),
        seq!(eat_char_choice("("), eat_char_choice("**"), expression, eat_char_choice(")")));
    let invalid_import = choice!(seq!(eat_char_choice("import"), gather(dotted_name, eat_char_choice(",")), eat_char_choice("from"), dotted_name),
        seq!(eat_char_choice("import"), NEWLINE));
    let invalid_import_from_targets = choice!(seq!(import_from_as_names, eat_char_choice(","), NEWLINE),
        seq!(NEWLINE));
    let invalid_with_stmt = choice!(seq!(opt(choice!(seq!(eat_char_choice("async")))), eat_char_choice("with"), gather(group(choice!(seq!(expression, opt(choice!(seq!(eat_char_choice("as"), star_target)))))), eat_char_choice(",")), NEWLINE),
        seq!(opt(choice!(seq!(eat_char_choice("async")))), eat_char_choice("with"), eat_char_choice("("), gather(group(choice!(seq!(expressions, opt(choice!(seq!(eat_char_choice("as"), star_target)))))), eat_char_choice(",")), opt(eat_char_choice(",")), eat_char_choice(")"), NEWLINE));
    let invalid_with_stmt_indent = choice!(seq!(opt(choice!(seq!(eat_char_choice("async")))), eat_char_choice("with"), gather(group(choice!(seq!(expression, opt(choice!(seq!(eat_char_choice("as"), star_target)))))), eat_char_choice(",")), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)),
        seq!(opt(choice!(seq!(eat_char_choice("async")))), eat_char_choice("with"), eat_char_choice("("), gather(group(choice!(seq!(expressions, opt(choice!(seq!(eat_char_choice("as"), star_target)))))), eat_char_choice(",")), opt(eat_char_choice(",")), eat_char_choice(")"), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_try_stmt = choice!(seq!(eat_char_choice("try"), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)),
        seq!(eat_char_choice("try"), eat_char_choice(":"), block, lookahead(group(choice!(seq!(eat_char_choice("except")), seq!(eat_char_choice("finally")))), !)),
        seq!(eat_char_choice("try"), eat_char_choice(":"), repeat(block), repeat(except_block), eat_char_choice("except"), eat_char_choice("*"), expression, opt(choice!(seq!(eat_char_choice("as"), NAME))), eat_char_choice(":")),
        seq!(eat_char_choice("try"), eat_char_choice(":"), repeat(block), repeat(except_star_block), eat_char_choice("except"), opt(choice!(seq!(expression, opt(choice!(seq!(eat_char_choice("as"), NAME)))))), eat_char_choice(":")));
    let invalid_except_stmt = choice!(seq!(eat_char_choice("except"), opt(eat_char_choice("*")), expression, eat_char_choice(","), expressions, opt(choice!(seq!(eat_char_choice("as"), NAME))), eat_char_choice(":")),
        seq!(eat_char_choice("except"), opt(eat_char_choice("*")), expression, opt(choice!(seq!(eat_char_choice("as"), NAME))), NEWLINE),
        seq!(eat_char_choice("except"), NEWLINE),
        seq!(eat_char_choice("except"), eat_char_choice("*"), group(choice!(seq!(NEWLINE), seq!(eat_char_choice(":"))))));
    let invalid_finally_stmt = choice!(seq!(eat_char_choice("finally"), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_except_stmt_indent = choice!(seq!(eat_char_choice("except"), expression, opt(choice!(seq!(eat_char_choice("as"), NAME))), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)),
        seq!(eat_char_choice("except"), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_except_star_stmt_indent = choice!(seq!(eat_char_choice("except"), eat_char_choice("*"), expression, opt(choice!(seq!(eat_char_choice("as"), NAME))), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_match_stmt = choice!(seq!("match", subject_expr, NEWLINE),
        seq!("match", subject_expr, eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_case_block = choice!(seq!("case", patterns, opt(guard), NEWLINE),
        seq!("case", patterns, opt(guard), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_as_pattern = choice!(seq!(or_pattern, eat_char_choice("as"), "_"),
        seq!(or_pattern, eat_char_choice("as"), lookahead(NAME, !), expression));
    let invalid_class_pattern = choice!(seq!(name_or_attr, eat_char_choice("("), invalid_class_argument_pattern));
    let invalid_class_argument_pattern = choice!(seq!(opt(choice!(seq!(positional_patterns, eat_char_choice(",")))), keyword_patterns, eat_char_choice(","), positional_patterns));
    let invalid_if_stmt = choice!(seq!(eat_char_choice("if"), named_expression, NEWLINE),
        seq!(eat_char_choice("if"), named_expression, eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_elif_stmt = choice!(seq!(eat_char_choice("elif"), named_expression, NEWLINE),
        seq!(eat_char_choice("elif"), named_expression, eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_else_stmt = choice!(seq!(eat_char_choice("else"), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_while_stmt = choice!(seq!(eat_char_choice("while"), named_expression, NEWLINE),
        seq!(eat_char_choice("while"), named_expression, eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_for_stmt = choice!(seq!(opt(choice!(seq!(eat_char_choice("async")))), eat_char_choice("for"), star_targets, eat_char_choice("in"), star_expressions, NEWLINE),
        seq!(opt(choice!(seq!(eat_char_choice("async")))), eat_char_choice("for"), star_targets, eat_char_choice("in"), star_expressions, eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_def_raw = choice!(seq!(opt(choice!(seq!(eat_char_choice("async")))), eat_char_choice("def"), NAME, opt(choice!(seq!(type_params))), eat_char_choice("("), opt(choice!(seq!(params))), eat_char_choice(")"), opt(choice!(seq!(eat_char_choice("->"), expression))), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)),
        seq!(opt(choice!(seq!(eat_char_choice("async")))), eat_char_choice("def"), NAME, opt(choice!(seq!(type_params))), forced(eat_char_choice("(")), opt(choice!(seq!(params))), eat_char_choice(")"), opt(choice!(seq!(eat_char_choice("->"), expression))), forced(eat_char_choice(":")), opt(choice!(seq!(func_type_comment))), block));
    let invalid_class_def_raw = choice!(seq!(eat_char_choice("class"), NAME, opt(choice!(seq!(type_params))), opt(choice!(seq!(eat_char_choice("("), opt(choice!(seq!(arguments))), eat_char_choice(")")))), NEWLINE),
        seq!(eat_char_choice("class"), NAME, opt(choice!(seq!(type_params))), opt(choice!(seq!(eat_char_choice("("), opt(choice!(seq!(arguments))), eat_char_choice(")")))), eat_char_choice(":"), NEWLINE, lookahead(INDENT, !)));
    let invalid_double_starred_kvpairs = choice!(seq!(gather(double_starred_kvpair, eat_char_choice(",")), eat_char_choice(","), invalid_kvpair),
        seq!(expression, eat_char_choice(":"), eat_char_choice("*"), bitwise_or),
        seq!(expression, eat_char_choice(":"), lookahead(group(choice!(seq!(eat_char_choice("}")), seq!(eat_char_choice(",")))), &)));
    let invalid_kvpair = choice!(seq!(expression, lookahead(group(choice!(seq!(eat_char_choice(":")))), !)),
        seq!(expression, eat_char_choice(":"), eat_char_choice("*"), bitwise_or),
        seq!(expression, eat_char_choice(":"), lookahead(group(choice!(seq!(eat_char_choice("}")), seq!(eat_char_choice(",")))), &)));
    let invalid_starred_expression_unpacking = choice!(seq!(eat_char_choice("*"), expression, eat_char_choice("="), expression));
    let invalid_starred_expression = choice!(seq!(eat_char_choice("*")));
    let invalid_replacement_field = choice!(seq!(eat_char_choice("{"), eat_char_choice("=")),
        seq!(eat_char_choice("{"), eat_char_choice("!")),
        seq!(eat_char_choice("{"), eat_char_choice(":")),
        seq!(eat_char_choice("{"), eat_char_choice("}")),
        seq!(eat_char_choice("{"), lookahead(annotated_rhs, !)),
        seq!(eat_char_choice("{"), annotated_rhs, lookahead(group(choice!(seq!(eat_char_choice("=")), seq!(eat_char_choice("!")), seq!(eat_char_choice(":")), seq!(eat_char_choice("}")))), !)),
        seq!(eat_char_choice("{"), annotated_rhs, eat_char_choice("="), lookahead(group(choice!(seq!(eat_char_choice("!")), seq!(eat_char_choice(":")), seq!(eat_char_choice("}")))), !)),
        seq!(eat_char_choice("{"), annotated_rhs, opt(eat_char_choice("=")), invalid_conversion_character),
        seq!(eat_char_choice("{"), annotated_rhs, opt(eat_char_choice("=")), opt(choice!(seq!(eat_char_choice("!"), NAME))), lookahead(group(choice!(seq!(eat_char_choice(":")), seq!(eat_char_choice("}")))), !)),
        seq!(eat_char_choice("{"), annotated_rhs, opt(eat_char_choice("=")), opt(choice!(seq!(eat_char_choice("!"), NAME))), eat_char_choice(":"), repeat(fstring_format_spec), lookahead(eat_char_choice("}"), !)),
        seq!(eat_char_choice("{"), annotated_rhs, opt(eat_char_choice("=")), opt(choice!(seq!(eat_char_choice("!"), NAME))), lookahead(eat_char_choice("}"), !)));
    let invalid_conversion_character = choice!(seq!(eat_char_choice("!"), lookahead(group(choice!(seq!(eat_char_choice(":")), seq!(eat_char_choice("}")))), &)),
        seq!(eat_char_choice("!"), lookahead(NAME, !)));
    let invalid_arithmetic = choice!(seq!(sum, group(choice!(seq!(eat_char_choice("+")), seq!(eat_char_choice("-")), seq!(eat_char_choice("*")), seq!(eat_char_choice("/")), seq!(eat_char_choice("%")), seq!(eat_char_choice("//")), seq!(eat_char_choice("@")))), eat_char_choice("not"), inversion));
    let invalid_factor = choice!(seq!(group(choice!(seq!(eat_char_choice("+")), seq!(eat_char_choice("-")), seq!(eat_char_choice("~")))), eat_char_choice("not"), factor));
    let invalid_type_params = choice!(seq!(eat_char_choice("["), eat_char_choice("]")));    file_copy.set(file);
    interactive_copy.set(interactive);
    eval_copy.set(eval);
    func_type_copy.set(func_type);
    statements_copy.set(statements);
    statement_copy.set(statement);
    statement_newline_copy.set(statement_newline);
    simple_stmts_copy.set(simple_stmts);
    simple_stmt_copy.set(simple_stmt);
    compound_stmt_copy.set(compound_stmt);
    assignment_copy.set(assignment);
    annotated_rhs_copy.set(annotated_rhs);
    augassign_copy.set(augassign);
    return_stmt_copy.set(return_stmt);
    raise_stmt_copy.set(raise_stmt);
    global_stmt_copy.set(global_stmt);
    nonlocal_stmt_copy.set(nonlocal_stmt);
    del_stmt_copy.set(del_stmt);
    yield_stmt_copy.set(yield_stmt);
    assert_stmt_copy.set(assert_stmt);
    import_stmt_copy.set(import_stmt);
    import_name_copy.set(import_name);
    import_from_copy.set(import_from);
    import_from_targets_copy.set(import_from_targets);
    import_from_as_names_copy.set(import_from_as_names);
    import_from_as_name_copy.set(import_from_as_name);
    dotted_as_names_copy.set(dotted_as_names);
    dotted_as_name_copy.set(dotted_as_name);
    dotted_name_copy.set(dotted_name);
    block_copy.set(block);
    decorators_copy.set(decorators);
    class_def_copy.set(class_def);
    class_def_raw_copy.set(class_def_raw);
    function_def_copy.set(function_def);
    function_def_raw_copy.set(function_def_raw);
    params_copy.set(params);
    parameters_copy.set(parameters);
    slash_no_default_copy.set(slash_no_default);
    slash_with_default_copy.set(slash_with_default);
    star_etc_copy.set(star_etc);
    kwds_copy.set(kwds);
    param_no_default_copy.set(param_no_default);
    param_no_default_star_annotation_copy.set(param_no_default_star_annotation);
    param_with_default_copy.set(param_with_default);
    param_maybe_default_copy.set(param_maybe_default);
    param_copy.set(param);
    param_star_annotation_copy.set(param_star_annotation);
    annotation_copy.set(annotation);
    star_annotation_copy.set(star_annotation);
    default_copy.set(default);
    if_stmt_copy.set(if_stmt);
    elif_stmt_copy.set(elif_stmt);
    else_block_copy.set(else_block);
    while_stmt_copy.set(while_stmt);
    for_stmt_copy.set(for_stmt);
    with_stmt_copy.set(with_stmt);
    with_item_copy.set(with_item);
    try_stmt_copy.set(try_stmt);
    except_block_copy.set(except_block);
    except_star_block_copy.set(except_star_block);
    finally_block_copy.set(finally_block);
    match_stmt_copy.set(match_stmt);
    subject_expr_copy.set(subject_expr);
    case_block_copy.set(case_block);
    guard_copy.set(guard);
    patterns_copy.set(patterns);
    pattern_copy.set(pattern);
    as_pattern_copy.set(as_pattern);
    or_pattern_copy.set(or_pattern);
    closed_pattern_copy.set(closed_pattern);
    literal_pattern_copy.set(literal_pattern);
    literal_expr_copy.set(literal_expr);
    complex_number_copy.set(complex_number);
    signed_number_copy.set(signed_number);
    signed_real_number_copy.set(signed_real_number);
    real_number_copy.set(real_number);
    imaginary_number_copy.set(imaginary_number);
    capture_pattern_copy.set(capture_pattern);
    pattern_capture_target_copy.set(pattern_capture_target);
    wildcard_pattern_copy.set(wildcard_pattern);
    value_pattern_copy.set(value_pattern);
    attr_copy.set(attr);
    name_or_attr_copy.set(name_or_attr);
    group_pattern_copy.set(group_pattern);
    sequence_pattern_copy.set(sequence_pattern);
    open_sequence_pattern_copy.set(open_sequence_pattern);
    maybe_sequence_pattern_copy.set(maybe_sequence_pattern);
    maybe_star_pattern_copy.set(maybe_star_pattern);
    star_pattern_copy.set(star_pattern);
    mapping_pattern_copy.set(mapping_pattern);
    items_pattern_copy.set(items_pattern);
    key_value_pattern_copy.set(key_value_pattern);
    double_star_pattern_copy.set(double_star_pattern);
    class_pattern_copy.set(class_pattern);
    positional_patterns_copy.set(positional_patterns);
    keyword_patterns_copy.set(keyword_patterns);
    keyword_pattern_copy.set(keyword_pattern);
    type_alias_copy.set(type_alias);
    type_params_copy.set(type_params);
    type_param_seq_copy.set(type_param_seq);
    type_param_copy.set(type_param);
    type_param_bound_copy.set(type_param_bound);
    type_param_default_copy.set(type_param_default);
    type_param_starred_default_copy.set(type_param_starred_default);
    expressions_copy.set(expressions);
    expression_copy.set(expression);
    yield_expr_copy.set(yield_expr);
    star_expressions_copy.set(star_expressions);
    star_expression_copy.set(star_expression);
    star_named_expressions_copy.set(star_named_expressions);
    star_named_expression_copy.set(star_named_expression);
    assignment_expression_copy.set(assignment_expression);
    named_expression_copy.set(named_expression);
    disjunction_copy.set(disjunction);
    conjunction_copy.set(conjunction);
    inversion_copy.set(inversion);
    comparison_copy.set(comparison);
    compare_op_bitwise_or_pair_copy.set(compare_op_bitwise_or_pair);
    eq_bitwise_or_copy.set(eq_bitwise_or);
    noteq_bitwise_or_copy.set(noteq_bitwise_or);
    lte_bitwise_or_copy.set(lte_bitwise_or);
    lt_bitwise_or_copy.set(lt_bitwise_or);
    gte_bitwise_or_copy.set(gte_bitwise_or);
    gt_bitwise_or_copy.set(gt_bitwise_or);
    notin_bitwise_or_copy.set(notin_bitwise_or);
    in_bitwise_or_copy.set(in_bitwise_or);
    isnot_bitwise_or_copy.set(isnot_bitwise_or);
    is_bitwise_or_copy.set(is_bitwise_or);
    bitwise_or_copy.set(bitwise_or);
    bitwise_xor_copy.set(bitwise_xor);
    bitwise_and_copy.set(bitwise_and);
    shift_expr_copy.set(shift_expr);
    sum_copy.set(sum);
    term_copy.set(term);
    factor_copy.set(factor);
    power_copy.set(power);
    await_primary_copy.set(await_primary);
    primary_copy.set(primary);
    slices_copy.set(slices);
    slice_copy.set(slice);
    atom_copy.set(atom);
    group_copy.set(group);
    lambdef_copy.set(lambdef);
    lambda_params_copy.set(lambda_params);
    lambda_parameters_copy.set(lambda_parameters);
    lambda_slash_no_default_copy.set(lambda_slash_no_default);
    lambda_slash_with_default_copy.set(lambda_slash_with_default);
    lambda_star_etc_copy.set(lambda_star_etc);
    lambda_kwds_copy.set(lambda_kwds);
    lambda_param_no_default_copy.set(lambda_param_no_default);
    lambda_param_with_default_copy.set(lambda_param_with_default);
    lambda_param_maybe_default_copy.set(lambda_param_maybe_default);
    lambda_param_copy.set(lambda_param);
    fstring_middle_copy.set(fstring_middle);
    fstring_replacement_field_copy.set(fstring_replacement_field);
    fstring_conversion_copy.set(fstring_conversion);
    fstring_full_format_spec_copy.set(fstring_full_format_spec);
    fstring_format_spec_copy.set(fstring_format_spec);
    fstring_copy.set(fstring);
    string_copy.set(string);
    strings_copy.set(strings);
    list_copy.set(list);
    tuple_copy.set(tuple);
    set_copy.set(set);
    dict_copy.set(dict);
    double_starred_kvpairs_copy.set(double_starred_kvpairs);
    double_starred_kvpair_copy.set(double_starred_kvpair);
    kvpair_copy.set(kvpair);
    for_if_clauses_copy.set(for_if_clauses);
    for_if_clause_copy.set(for_if_clause);
    listcomp_copy.set(listcomp);
    setcomp_copy.set(setcomp);
    genexp_copy.set(genexp);
    dictcomp_copy.set(dictcomp);
    arguments_copy.set(arguments);
    args_copy.set(args);
    kwargs_copy.set(kwargs);
    starred_expression_copy.set(starred_expression);
    kwarg_or_starred_copy.set(kwarg_or_starred);
    kwarg_or_double_starred_copy.set(kwarg_or_double_starred);
    star_targets_copy.set(star_targets);
    star_targets_list_seq_copy.set(star_targets_list_seq);
    star_targets_tuple_seq_copy.set(star_targets_tuple_seq);
    star_target_copy.set(star_target);
    target_with_star_atom_copy.set(target_with_star_atom);
    star_atom_copy.set(star_atom);
    single_target_copy.set(single_target);
    single_subscript_attribute_target_copy.set(single_subscript_attribute_target);
    t_primary_copy.set(t_primary);
    t_lookahead_copy.set(t_lookahead);
    del_targets_copy.set(del_targets);
    del_target_copy.set(del_target);
    del_t_atom_copy.set(del_t_atom);
    type_expressions_copy.set(type_expressions);
    func_type_comment_copy.set(func_type_comment);
    invalid_arguments_copy.set(invalid_arguments);
    invalid_kwarg_copy.set(invalid_kwarg);
    expression_without_invalid_copy.set(expression_without_invalid);
    invalid_legacy_expression_copy.set(invalid_legacy_expression);
    invalid_type_param_copy.set(invalid_type_param);
    invalid_expression_copy.set(invalid_expression);
    invalid_named_expression_copy.set(invalid_named_expression);
    invalid_assignment_copy.set(invalid_assignment);
    invalid_ann_assign_target_copy.set(invalid_ann_assign_target);
    invalid_del_stmt_copy.set(invalid_del_stmt);
    invalid_block_copy.set(invalid_block);
    invalid_comprehension_copy.set(invalid_comprehension);
    invalid_dict_comprehension_copy.set(invalid_dict_comprehension);
    invalid_parameters_copy.set(invalid_parameters);
    invalid_default_copy.set(invalid_default);
    invalid_star_etc_copy.set(invalid_star_etc);
    invalid_kwds_copy.set(invalid_kwds);
    invalid_parameters_helper_copy.set(invalid_parameters_helper);
    invalid_lambda_parameters_copy.set(invalid_lambda_parameters);
    invalid_lambda_parameters_helper_copy.set(invalid_lambda_parameters_helper);
    invalid_lambda_star_etc_copy.set(invalid_lambda_star_etc);
    invalid_lambda_kwds_copy.set(invalid_lambda_kwds);
    invalid_double_type_comments_copy.set(invalid_double_type_comments);
    invalid_with_item_copy.set(invalid_with_item);
    invalid_for_if_clause_copy.set(invalid_for_if_clause);
    invalid_for_target_copy.set(invalid_for_target);
    invalid_group_copy.set(invalid_group);
    invalid_import_copy.set(invalid_import);
    invalid_import_from_targets_copy.set(invalid_import_from_targets);
    invalid_with_stmt_copy.set(invalid_with_stmt);
    invalid_with_stmt_indent_copy.set(invalid_with_stmt_indent);
    invalid_try_stmt_copy.set(invalid_try_stmt);
    invalid_except_stmt_copy.set(invalid_except_stmt);
    invalid_finally_stmt_copy.set(invalid_finally_stmt);
    invalid_except_stmt_indent_copy.set(invalid_except_stmt_indent);
    invalid_except_star_stmt_indent_copy.set(invalid_except_star_stmt_indent);
    invalid_match_stmt_copy.set(invalid_match_stmt);
    invalid_case_block_copy.set(invalid_case_block);
    invalid_as_pattern_copy.set(invalid_as_pattern);
    invalid_class_pattern_copy.set(invalid_class_pattern);
    invalid_class_argument_pattern_copy.set(invalid_class_argument_pattern);
    invalid_if_stmt_copy.set(invalid_if_stmt);
    invalid_elif_stmt_copy.set(invalid_elif_stmt);
    invalid_else_stmt_copy.set(invalid_else_stmt);
    invalid_while_stmt_copy.set(invalid_while_stmt);
    invalid_for_stmt_copy.set(invalid_for_stmt);
    invalid_def_raw_copy.set(invalid_def_raw);
    invalid_class_def_raw_copy.set(invalid_class_def_raw);
    invalid_double_starred_kvpairs_copy.set(invalid_double_starred_kvpairs);
    invalid_kvpair_copy.set(invalid_kvpair);
    invalid_starred_expression_unpacking_copy.set(invalid_starred_expression_unpacking);
    invalid_starred_expression_copy.set(invalid_starred_expression);
    invalid_replacement_field_copy.set(invalid_replacement_field);
    invalid_conversion_character_copy.set(invalid_conversion_character);
    invalid_arithmetic_copy.set(invalid_arithmetic);
    invalid_factor_copy.set(invalid_factor);
    invalid_type_params_copy.set(invalid_type_params);}
