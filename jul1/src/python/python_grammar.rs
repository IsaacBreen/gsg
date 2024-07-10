use std::rc::Rc;
use crate::{choice, seq, repeat, repeat as repeat0, repeat1, opt, eat_char_choice, eat_string, eat_char_range, forward_ref, eps, python_newline, indent, dedent};
use super::python_tokenizer::{NAME, TYPE_COMMENT, FSTRING_START, FSTRING_MIDDLE, FSTRING_END, NUMBER, STRING};

fn main() {
    let NAME = Rc::new(NAME());
    let TYPE_COMMENT = Rc::new(TYPE_COMMENT());
    let FSTRING_START = Rc::new(FSTRING_START());
    let FSTRING_MIDDLE = Rc::new(FSTRING_MIDDLE());
    let FSTRING_END = Rc::new(FSTRING_END());
    let NUMBER = Rc::new(NUMBER());
    let STRING = Rc::new(STRING());
    let NEWLINE = Rc::new(python_newline());
    let INDENT = Rc::new(indent());
    let DEDENT = Rc::new(dedent());
    let ENDMARKER = eps();
    let mut file = forward_ref();
    let mut interactive = forward_ref();
    let mut eval = forward_ref();
    let mut func_type = forward_ref();
    let mut statements = forward_ref();
    let mut statement = forward_ref();
    let mut statement_newline = forward_ref();
    let mut simple_stmts = forward_ref();
    let mut simple_stmt = forward_ref();
    let mut compound_stmt = forward_ref();
    let mut assignment = forward_ref();
    let mut annotated_rhs = forward_ref();
    let mut augassign = forward_ref();
    let mut return_stmt = forward_ref();
    let mut raise_stmt = forward_ref();
    let mut global_stmt = forward_ref();
    let mut nonlocal_stmt = forward_ref();
    let mut del_stmt = forward_ref();
    let mut yield_stmt = forward_ref();
    let mut assert_stmt = forward_ref();
    let mut import_stmt = forward_ref();
    let mut import_name = forward_ref();
    let mut import_from = forward_ref();
    let mut import_from_targets = forward_ref();
    let mut import_from_as_names = forward_ref();
    let mut import_from_as_name = forward_ref();
    let mut dotted_as_names = forward_ref();
    let mut dotted_as_name = forward_ref();
    let mut dotted_name = forward_ref();
    let mut block = forward_ref();
    let mut decorators = forward_ref();
    let mut class_def = forward_ref();
    let mut class_def_raw = forward_ref();
    let mut function_def = forward_ref();
    let mut function_def_raw = forward_ref();
    let mut params = forward_ref();
    let mut parameters = forward_ref();
    let mut slash_no_default = forward_ref();
    let mut slash_with_default = forward_ref();
    let mut star_etc = forward_ref();
    let mut kwds = forward_ref();
    let mut param_no_default = forward_ref();
    let mut param_no_default_star_annotation = forward_ref();
    let mut param_with_default = forward_ref();
    let mut param_maybe_default = forward_ref();
    let mut param = forward_ref();
    let mut param_star_annotation = forward_ref();
    let mut annotation = forward_ref();
    let mut star_annotation = forward_ref();
    let mut default = forward_ref();
    let mut if_stmt = forward_ref();
    let mut elif_stmt = forward_ref();
    let mut else_block = forward_ref();
    let mut while_stmt = forward_ref();
    let mut for_stmt = forward_ref();
    let mut with_stmt = forward_ref();
    let mut with_item = forward_ref();
    let mut try_stmt = forward_ref();
    let mut except_block = forward_ref();
    let mut except_star_block = forward_ref();
    let mut finally_block = forward_ref();
    let mut match_stmt = forward_ref();
    let mut subject_expr = forward_ref();
    let mut case_block = forward_ref();
    let mut guard = forward_ref();
    let mut patterns = forward_ref();
    let mut pattern = forward_ref();
    let mut as_pattern = forward_ref();
    let mut or_pattern = forward_ref();
    let mut closed_pattern = forward_ref();
    let mut literal_pattern = forward_ref();
    let mut literal_expr = forward_ref();
    let mut complex_number = forward_ref();
    let mut signed_number = forward_ref();
    let mut signed_real_number = forward_ref();
    let mut real_number = forward_ref();
    let mut imaginary_number = forward_ref();
    let mut capture_pattern = forward_ref();
    let mut pattern_capture_target = forward_ref();
    let mut wildcard_pattern = forward_ref();
    let mut value_pattern = forward_ref();
    let mut attr = forward_ref();
    let mut name_or_attr = forward_ref();
    let mut group_pattern = forward_ref();
    let mut sequence_pattern = forward_ref();
    let mut open_sequence_pattern = forward_ref();
    let mut maybe_sequence_pattern = forward_ref();
    let mut maybe_star_pattern = forward_ref();
    let mut star_pattern = forward_ref();
    let mut mapping_pattern = forward_ref();
    let mut items_pattern = forward_ref();
    let mut key_value_pattern = forward_ref();
    let mut double_star_pattern = forward_ref();
    let mut class_pattern = forward_ref();
    let mut positional_patterns = forward_ref();
    let mut keyword_patterns = forward_ref();
    let mut keyword_pattern = forward_ref();
    let mut type_alias = forward_ref();
    let mut type_params = forward_ref();
    let mut type_param_seq = forward_ref();
    let mut type_param = forward_ref();
    let mut type_param_bound = forward_ref();
    let mut type_param_default = forward_ref();
    let mut type_param_starred_default = forward_ref();
    let mut expressions = forward_ref();
    let mut expression = forward_ref();
    let mut yield_expr = forward_ref();
    let mut star_expressions = forward_ref();
    let mut star_expression = forward_ref();
    let mut star_named_expressions = forward_ref();
    let mut star_named_expression = forward_ref();
    let mut assignment_expression = forward_ref();
    let mut named_expression = forward_ref();
    let mut disjunction = forward_ref();
    let mut conjunction = forward_ref();
    let mut inversion = forward_ref();
    let mut comparison = forward_ref();
    let mut compare_op_bitwise_or_pair = forward_ref();
    let mut eq_bitwise_or = forward_ref();
    let mut noteq_bitwise_or = forward_ref();
    let mut lte_bitwise_or = forward_ref();
    let mut lt_bitwise_or = forward_ref();
    let mut gte_bitwise_or = forward_ref();
    let mut gt_bitwise_or = forward_ref();
    let mut notin_bitwise_or = forward_ref();
    let mut in_bitwise_or = forward_ref();
    let mut isnot_bitwise_or = forward_ref();
    let mut is_bitwise_or = forward_ref();
    let mut bitwise_or = forward_ref();
    let mut bitwise_xor = forward_ref();
    let mut bitwise_and = forward_ref();
    let mut shift_expr = forward_ref();
    let mut sum = forward_ref();
    let mut term = forward_ref();
    let mut factor = forward_ref();
    let mut power = forward_ref();
    let mut await_primary = forward_ref();
    let mut primary = forward_ref();
    let mut slices = forward_ref();
    let mut slice = forward_ref();
    let mut atom = forward_ref();
    let mut group = forward_ref();
    let mut lambdef = forward_ref();
    let mut lambda_params = forward_ref();
    let mut lambda_parameters = forward_ref();
    let mut lambda_slash_no_default = forward_ref();
    let mut lambda_slash_with_default = forward_ref();
    let mut lambda_star_etc = forward_ref();
    let mut lambda_kwds = forward_ref();
    let mut lambda_param_no_default = forward_ref();
    let mut lambda_param_with_default = forward_ref();
    let mut lambda_param_maybe_default = forward_ref();
    let mut lambda_param = forward_ref();
    let mut fstring_middle = forward_ref();
    let mut fstring_replacement_field = forward_ref();
    let mut fstring_conversion = forward_ref();
    let mut fstring_full_format_spec = forward_ref();
    let mut fstring_format_spec = forward_ref();
    let mut fstring = forward_ref();
    let mut string = forward_ref();
    let mut strings = forward_ref();
    let mut list = forward_ref();
    let mut tuple = forward_ref();
    let mut set = forward_ref();
    let mut dict = forward_ref();
    let mut double_starred_kvpairs = forward_ref();
    let mut double_starred_kvpair = forward_ref();
    let mut kvpair = forward_ref();
    let mut for_if_clauses = forward_ref();
    let mut for_if_clause = forward_ref();
    let mut listcomp = forward_ref();
    let mut setcomp = forward_ref();
    let mut genexp = forward_ref();
    let mut dictcomp = forward_ref();
    let mut arguments = forward_ref();
    let mut args = forward_ref();
    let mut kwargs = forward_ref();
    let mut starred_expression = forward_ref();
    let mut kwarg_or_starred = forward_ref();
    let mut kwarg_or_double_starred = forward_ref();
    let mut star_targets = forward_ref();
    let mut star_targets_list_seq = forward_ref();
    let mut star_targets_tuple_seq = forward_ref();
    let mut star_target = forward_ref();
    let mut target_with_star_atom = forward_ref();
    let mut star_atom = forward_ref();
    let mut single_target = forward_ref();
    let mut single_subscript_attribute_target = forward_ref();
    let mut t_primary = forward_ref();
    let mut t_lookahead = forward_ref();
    let mut del_targets = forward_ref();
    let mut del_target = forward_ref();
    let mut del_t_atom = forward_ref();
    let mut type_expressions = forward_ref();
    let mut func_type_comment = forward_ref();
    let mut invalid_arguments = forward_ref();
    let mut invalid_kwarg = forward_ref();
    let mut expression_without_invalid = forward_ref();
    let mut invalid_legacy_expression = forward_ref();
    let mut invalid_type_param = forward_ref();
    let mut invalid_expression = forward_ref();
    let mut invalid_named_expression = forward_ref();
    let mut invalid_assignment = forward_ref();
    let mut invalid_ann_assign_target = forward_ref();
    let mut invalid_del_stmt = forward_ref();
    let mut invalid_block = forward_ref();
    let mut invalid_comprehension = forward_ref();
    let mut invalid_dict_comprehension = forward_ref();
    let mut invalid_parameters = forward_ref();
    let mut invalid_default = forward_ref();
    let mut invalid_star_etc = forward_ref();
    let mut invalid_kwds = forward_ref();
    let mut invalid_parameters_helper = forward_ref();
    let mut invalid_lambda_parameters = forward_ref();
    let mut invalid_lambda_parameters_helper = forward_ref();
    let mut invalid_lambda_star_etc = forward_ref();
    let mut invalid_lambda_kwds = forward_ref();
    let mut invalid_double_type_comments = forward_ref();
    let mut invalid_with_item = forward_ref();
    let mut invalid_for_if_clause = forward_ref();
    let mut invalid_for_target = forward_ref();
    let mut invalid_group = forward_ref();
    let mut invalid_import = forward_ref();
    let mut invalid_import_from_targets = forward_ref();
    let mut invalid_with_stmt = forward_ref();
    let mut invalid_with_stmt_indent = forward_ref();
    let mut invalid_try_stmt = forward_ref();
    let mut invalid_except_stmt = forward_ref();
    let mut invalid_finally_stmt = forward_ref();
    let mut invalid_except_stmt_indent = forward_ref();
    let mut invalid_except_star_stmt_indent = forward_ref();
    let mut invalid_match_stmt = forward_ref();
    let mut invalid_case_block = forward_ref();
    let mut invalid_as_pattern = forward_ref();
    let mut invalid_class_pattern = forward_ref();
    let mut invalid_class_argument_pattern = forward_ref();
    let mut invalid_if_stmt = forward_ref();
    let mut invalid_elif_stmt = forward_ref();
    let mut invalid_else_stmt = forward_ref();
    let mut invalid_while_stmt = forward_ref();
    let mut invalid_for_stmt = forward_ref();
    let mut invalid_def_raw = forward_ref();
    let mut invalid_class_def_raw = forward_ref();
    let mut invalid_double_starred_kvpairs = forward_ref();
    let mut invalid_kvpair = forward_ref();
    let mut invalid_starred_expression_unpacking = forward_ref();
    let mut invalid_starred_expression = forward_ref();
    let mut invalid_replacement_field = forward_ref();
    let mut invalid_conversion_character = forward_ref();
    let mut invalid_arithmetic = forward_ref();
    let mut invalid_factor = forward_ref();
    let mut invalid_type_params = forward_ref();
    let mut file_copy = file.clone();
    let mut interactive_copy = interactive.clone();
    let mut eval_copy = eval.clone();
    let mut func_type_copy = func_type.clone();
    let mut statements_copy = statements.clone();
    let mut statement_copy = statement.clone();
    let mut statement_newline_copy = statement_newline.clone();
    let mut simple_stmts_copy = simple_stmts.clone();
    let mut simple_stmt_copy = simple_stmt.clone();
    let mut compound_stmt_copy = compound_stmt.clone();
    let mut assignment_copy = assignment.clone();
    let mut annotated_rhs_copy = annotated_rhs.clone();
    let mut augassign_copy = augassign.clone();
    let mut return_stmt_copy = return_stmt.clone();
    let mut raise_stmt_copy = raise_stmt.clone();
    let mut global_stmt_copy = global_stmt.clone();
    let mut nonlocal_stmt_copy = nonlocal_stmt.clone();
    let mut del_stmt_copy = del_stmt.clone();
    let mut yield_stmt_copy = yield_stmt.clone();
    let mut assert_stmt_copy = assert_stmt.clone();
    let mut import_stmt_copy = import_stmt.clone();
    let mut import_name_copy = import_name.clone();
    let mut import_from_copy = import_from.clone();
    let mut import_from_targets_copy = import_from_targets.clone();
    let mut import_from_as_names_copy = import_from_as_names.clone();
    let mut import_from_as_name_copy = import_from_as_name.clone();
    let mut dotted_as_names_copy = dotted_as_names.clone();
    let mut dotted_as_name_copy = dotted_as_name.clone();
    let mut dotted_name_copy = dotted_name.clone();
    let mut block_copy = block.clone();
    let mut decorators_copy = decorators.clone();
    let mut class_def_copy = class_def.clone();
    let mut class_def_raw_copy = class_def_raw.clone();
    let mut function_def_copy = function_def.clone();
    let mut function_def_raw_copy = function_def_raw.clone();
    let mut params_copy = params.clone();
    let mut parameters_copy = parameters.clone();
    let mut slash_no_default_copy = slash_no_default.clone();
    let mut slash_with_default_copy = slash_with_default.clone();
    let mut star_etc_copy = star_etc.clone();
    let mut kwds_copy = kwds.clone();
    let mut param_no_default_copy = param_no_default.clone();
    let mut param_no_default_star_annotation_copy = param_no_default_star_annotation.clone();
    let mut param_with_default_copy = param_with_default.clone();
    let mut param_maybe_default_copy = param_maybe_default.clone();
    let mut param_copy = param.clone();
    let mut param_star_annotation_copy = param_star_annotation.clone();
    let mut annotation_copy = annotation.clone();
    let mut star_annotation_copy = star_annotation.clone();
    let mut default_copy = default.clone();
    let mut if_stmt_copy = if_stmt.clone();
    let mut elif_stmt_copy = elif_stmt.clone();
    let mut else_block_copy = else_block.clone();
    let mut while_stmt_copy = while_stmt.clone();
    let mut for_stmt_copy = for_stmt.clone();
    let mut with_stmt_copy = with_stmt.clone();
    let mut with_item_copy = with_item.clone();
    let mut try_stmt_copy = try_stmt.clone();
    let mut except_block_copy = except_block.clone();
    let mut except_star_block_copy = except_star_block.clone();
    let mut finally_block_copy = finally_block.clone();
    let mut match_stmt_copy = match_stmt.clone();
    let mut subject_expr_copy = subject_expr.clone();
    let mut case_block_copy = case_block.clone();
    let mut guard_copy = guard.clone();
    let mut patterns_copy = patterns.clone();
    let mut pattern_copy = pattern.clone();
    let mut as_pattern_copy = as_pattern.clone();
    let mut or_pattern_copy = or_pattern.clone();
    let mut closed_pattern_copy = closed_pattern.clone();
    let mut literal_pattern_copy = literal_pattern.clone();
    let mut literal_expr_copy = literal_expr.clone();
    let mut complex_number_copy = complex_number.clone();
    let mut signed_number_copy = signed_number.clone();
    let mut signed_real_number_copy = signed_real_number.clone();
    let mut real_number_copy = real_number.clone();
    let mut imaginary_number_copy = imaginary_number.clone();
    let mut capture_pattern_copy = capture_pattern.clone();
    let mut pattern_capture_target_copy = pattern_capture_target.clone();
    let mut wildcard_pattern_copy = wildcard_pattern.clone();
    let mut value_pattern_copy = value_pattern.clone();
    let mut attr_copy = attr.clone();
    let mut name_or_attr_copy = name_or_attr.clone();
    let mut group_pattern_copy = group_pattern.clone();
    let mut sequence_pattern_copy = sequence_pattern.clone();
    let mut open_sequence_pattern_copy = open_sequence_pattern.clone();
    let mut maybe_sequence_pattern_copy = maybe_sequence_pattern.clone();
    let mut maybe_star_pattern_copy = maybe_star_pattern.clone();
    let mut star_pattern_copy = star_pattern.clone();
    let mut mapping_pattern_copy = mapping_pattern.clone();
    let mut items_pattern_copy = items_pattern.clone();
    let mut key_value_pattern_copy = key_value_pattern.clone();
    let mut double_star_pattern_copy = double_star_pattern.clone();
    let mut class_pattern_copy = class_pattern.clone();
    let mut positional_patterns_copy = positional_patterns.clone();
    let mut keyword_patterns_copy = keyword_patterns.clone();
    let mut keyword_pattern_copy = keyword_pattern.clone();
    let mut type_alias_copy = type_alias.clone();
    let mut type_params_copy = type_params.clone();
    let mut type_param_seq_copy = type_param_seq.clone();
    let mut type_param_copy = type_param.clone();
    let mut type_param_bound_copy = type_param_bound.clone();
    let mut type_param_default_copy = type_param_default.clone();
    let mut type_param_starred_default_copy = type_param_starred_default.clone();
    let mut expressions_copy = expressions.clone();
    let mut expression_copy = expression.clone();
    let mut yield_expr_copy = yield_expr.clone();
    let mut star_expressions_copy = star_expressions.clone();
    let mut star_expression_copy = star_expression.clone();
    let mut star_named_expressions_copy = star_named_expressions.clone();
    let mut star_named_expression_copy = star_named_expression.clone();
    let mut assignment_expression_copy = assignment_expression.clone();
    let mut named_expression_copy = named_expression.clone();
    let mut disjunction_copy = disjunction.clone();
    let mut conjunction_copy = conjunction.clone();
    let mut inversion_copy = inversion.clone();
    let mut comparison_copy = comparison.clone();
    let mut compare_op_bitwise_or_pair_copy = compare_op_bitwise_or_pair.clone();
    let mut eq_bitwise_or_copy = eq_bitwise_or.clone();
    let mut noteq_bitwise_or_copy = noteq_bitwise_or.clone();
    let mut lte_bitwise_or_copy = lte_bitwise_or.clone();
    let mut lt_bitwise_or_copy = lt_bitwise_or.clone();
    let mut gte_bitwise_or_copy = gte_bitwise_or.clone();
    let mut gt_bitwise_or_copy = gt_bitwise_or.clone();
    let mut notin_bitwise_or_copy = notin_bitwise_or.clone();
    let mut in_bitwise_or_copy = in_bitwise_or.clone();
    let mut isnot_bitwise_or_copy = isnot_bitwise_or.clone();
    let mut is_bitwise_or_copy = is_bitwise_or.clone();
    let mut bitwise_or_copy = bitwise_or.clone();
    let mut bitwise_xor_copy = bitwise_xor.clone();
    let mut bitwise_and_copy = bitwise_and.clone();
    let mut shift_expr_copy = shift_expr.clone();
    let mut sum_copy = sum.clone();
    let mut term_copy = term.clone();
    let mut factor_copy = factor.clone();
    let mut power_copy = power.clone();
    let mut await_primary_copy = await_primary.clone();
    let mut primary_copy = primary.clone();
    let mut slices_copy = slices.clone();
    let mut slice_copy = slice.clone();
    let mut atom_copy = atom.clone();
    let mut group_copy = group.clone();
    let mut lambdef_copy = lambdef.clone();
    let mut lambda_params_copy = lambda_params.clone();
    let mut lambda_parameters_copy = lambda_parameters.clone();
    let mut lambda_slash_no_default_copy = lambda_slash_no_default.clone();
    let mut lambda_slash_with_default_copy = lambda_slash_with_default.clone();
    let mut lambda_star_etc_copy = lambda_star_etc.clone();
    let mut lambda_kwds_copy = lambda_kwds.clone();
    let mut lambda_param_no_default_copy = lambda_param_no_default.clone();
    let mut lambda_param_with_default_copy = lambda_param_with_default.clone();
    let mut lambda_param_maybe_default_copy = lambda_param_maybe_default.clone();
    let mut lambda_param_copy = lambda_param.clone();
    let mut fstring_middle_copy = fstring_middle.clone();
    let mut fstring_replacement_field_copy = fstring_replacement_field.clone();
    let mut fstring_conversion_copy = fstring_conversion.clone();
    let mut fstring_full_format_spec_copy = fstring_full_format_spec.clone();
    let mut fstring_format_spec_copy = fstring_format_spec.clone();
    let mut fstring_copy = fstring.clone();
    let mut string_copy = string.clone();
    let mut strings_copy = strings.clone();
    let mut list_copy = list.clone();
    let mut tuple_copy = tuple.clone();
    let mut set_copy = set.clone();
    let mut dict_copy = dict.clone();
    let mut double_starred_kvpairs_copy = double_starred_kvpairs.clone();
    let mut double_starred_kvpair_copy = double_starred_kvpair.clone();
    let mut kvpair_copy = kvpair.clone();
    let mut for_if_clauses_copy = for_if_clauses.clone();
    let mut for_if_clause_copy = for_if_clause.clone();
    let mut listcomp_copy = listcomp.clone();
    let mut setcomp_copy = setcomp.clone();
    let mut genexp_copy = genexp.clone();
    let mut dictcomp_copy = dictcomp.clone();
    let mut arguments_copy = arguments.clone();
    let mut args_copy = args.clone();
    let mut kwargs_copy = kwargs.clone();
    let mut starred_expression_copy = starred_expression.clone();
    let mut kwarg_or_starred_copy = kwarg_or_starred.clone();
    let mut kwarg_or_double_starred_copy = kwarg_or_double_starred.clone();
    let mut star_targets_copy = star_targets.clone();
    let mut star_targets_list_seq_copy = star_targets_list_seq.clone();
    let mut star_targets_tuple_seq_copy = star_targets_tuple_seq.clone();
    let mut star_target_copy = star_target.clone();
    let mut target_with_star_atom_copy = target_with_star_atom.clone();
    let mut star_atom_copy = star_atom.clone();
    let mut single_target_copy = single_target.clone();
    let mut single_subscript_attribute_target_copy = single_subscript_attribute_target.clone();
    let mut t_primary_copy = t_primary.clone();
    let mut t_lookahead_copy = t_lookahead.clone();
    let mut del_targets_copy = del_targets.clone();
    let mut del_target_copy = del_target.clone();
    let mut del_t_atom_copy = del_t_atom.clone();
    let mut type_expressions_copy = type_expressions.clone();
    let mut func_type_comment_copy = func_type_comment.clone();
    let mut invalid_arguments_copy = invalid_arguments.clone();
    let mut invalid_kwarg_copy = invalid_kwarg.clone();
    let mut expression_without_invalid_copy = expression_without_invalid.clone();
    let mut invalid_legacy_expression_copy = invalid_legacy_expression.clone();
    let mut invalid_type_param_copy = invalid_type_param.clone();
    let mut invalid_expression_copy = invalid_expression.clone();
    let mut invalid_named_expression_copy = invalid_named_expression.clone();
    let mut invalid_assignment_copy = invalid_assignment.clone();
    let mut invalid_ann_assign_target_copy = invalid_ann_assign_target.clone();
    let mut invalid_del_stmt_copy = invalid_del_stmt.clone();
    let mut invalid_block_copy = invalid_block.clone();
    let mut invalid_comprehension_copy = invalid_comprehension.clone();
    let mut invalid_dict_comprehension_copy = invalid_dict_comprehension.clone();
    let mut invalid_parameters_copy = invalid_parameters.clone();
    let mut invalid_default_copy = invalid_default.clone();
    let mut invalid_star_etc_copy = invalid_star_etc.clone();
    let mut invalid_kwds_copy = invalid_kwds.clone();
    let mut invalid_parameters_helper_copy = invalid_parameters_helper.clone();
    let mut invalid_lambda_parameters_copy = invalid_lambda_parameters.clone();
    let mut invalid_lambda_parameters_helper_copy = invalid_lambda_parameters_helper.clone();
    let mut invalid_lambda_star_etc_copy = invalid_lambda_star_etc.clone();
    let mut invalid_lambda_kwds_copy = invalid_lambda_kwds.clone();
    let mut invalid_double_type_comments_copy = invalid_double_type_comments.clone();
    let mut invalid_with_item_copy = invalid_with_item.clone();
    let mut invalid_for_if_clause_copy = invalid_for_if_clause.clone();
    let mut invalid_for_target_copy = invalid_for_target.clone();
    let mut invalid_group_copy = invalid_group.clone();
    let mut invalid_import_copy = invalid_import.clone();
    let mut invalid_import_from_targets_copy = invalid_import_from_targets.clone();
    let mut invalid_with_stmt_copy = invalid_with_stmt.clone();
    let mut invalid_with_stmt_indent_copy = invalid_with_stmt_indent.clone();
    let mut invalid_try_stmt_copy = invalid_try_stmt.clone();
    let mut invalid_except_stmt_copy = invalid_except_stmt.clone();
    let mut invalid_finally_stmt_copy = invalid_finally_stmt.clone();
    let mut invalid_except_stmt_indent_copy = invalid_except_stmt_indent.clone();
    let mut invalid_except_star_stmt_indent_copy = invalid_except_star_stmt_indent.clone();
    let mut invalid_match_stmt_copy = invalid_match_stmt.clone();
    let mut invalid_case_block_copy = invalid_case_block.clone();
    let mut invalid_as_pattern_copy = invalid_as_pattern.clone();
    let mut invalid_class_pattern_copy = invalid_class_pattern.clone();
    let mut invalid_class_argument_pattern_copy = invalid_class_argument_pattern.clone();
    let mut invalid_if_stmt_copy = invalid_if_stmt.clone();
    let mut invalid_elif_stmt_copy = invalid_elif_stmt.clone();
    let mut invalid_else_stmt_copy = invalid_else_stmt.clone();
    let mut invalid_while_stmt_copy = invalid_while_stmt.clone();
    let mut invalid_for_stmt_copy = invalid_for_stmt.clone();
    let mut invalid_def_raw_copy = invalid_def_raw.clone();
    let mut invalid_class_def_raw_copy = invalid_class_def_raw.clone();
    let mut invalid_double_starred_kvpairs_copy = invalid_double_starred_kvpairs.clone();
    let mut invalid_kvpair_copy = invalid_kvpair.clone();
    let mut invalid_starred_expression_unpacking_copy = invalid_starred_expression_unpacking.clone();
    let mut invalid_starred_expression_copy = invalid_starred_expression.clone();
    let mut invalid_replacement_field_copy = invalid_replacement_field.clone();
    let mut invalid_conversion_character_copy = invalid_conversion_character.clone();
    let mut invalid_arithmetic_copy = invalid_arithmetic.clone();
    let mut invalid_factor_copy = invalid_factor.clone();
    let mut invalid_type_params_copy = invalid_type_params.clone();
    let file = Rc::new(choice!(seq!(opt(choice!(seq!(statements.clone()))), ENDMARKER.clone())));
    let interactive = Rc::new(choice!(seq!(statement_newline.clone())));
    let eval = Rc::new(choice!(seq!(expressions.clone(), repeat(NEWLINE.clone()), ENDMARKER.clone())));
    let func_type = Rc::new(choice!(seq!(eat_string("("), opt(choice!(seq!(type_expressions.clone()))), eat_string(")"), eat_string("->"), expression.clone(), repeat(NEWLINE.clone()), ENDMARKER.clone())));
    let statements = Rc::new(choice!(seq!(repeat(statement.clone()))));
    let statement = Rc::new(choice!(seq!(compound_stmt.clone()),
        seq!(simple_stmts.clone())));
    let statement_newline = Rc::new(choice!(seq!(compound_stmt.clone(), NEWLINE.clone()),
        seq!(simple_stmts.clone()),
        seq!(NEWLINE.clone()),
        seq!(ENDMARKER.clone())));
    let simple_stmts = Rc::new(choice!(seq!(simple_stmt.clone(), eps(), NEWLINE.clone()),
        seq!(seq!(simple_stmt.clone(), eat_string(";")), opt(choice!(seq!(eat_string(";")))), NEWLINE.clone())));
    let simple_stmt = Rc::new(choice!(seq!(assignment.clone()),
        seq!(eps(), type_alias.clone()),
        seq!(star_expressions.clone()),
        seq!(eps(), return_stmt.clone()),
        seq!(eps(), import_stmt.clone()),
        seq!(eps(), raise_stmt.clone()),
        seq!(eat_string("pass")),
        seq!(eps(), del_stmt.clone()),
        seq!(eps(), yield_stmt.clone()),
        seq!(eps(), assert_stmt.clone()),
        seq!(eat_string("break")),
        seq!(eat_string("continue")),
        seq!(eps(), global_stmt.clone()),
        seq!(eps(), nonlocal_stmt.clone())));
    let compound_stmt = Rc::new(choice!(seq!(eps(), function_def.clone()),
        seq!(eps(), if_stmt.clone()),
        seq!(eps(), class_def.clone()),
        seq!(eps(), with_stmt.clone()),
        seq!(eps(), for_stmt.clone()),
        seq!(eps(), try_stmt.clone()),
        seq!(eps(), while_stmt.clone()),
        seq!(match_stmt.clone())));
    let assignment = Rc::new(choice!(seq!(NAME.clone(), eat_string(":"), expression.clone(), opt(choice!(seq!(eat_string("="), annotated_rhs.clone())))),
        seq!(choice!(seq!(eat_string("("), single_target.clone(), eat_string(")")), seq!(single_subscript_attribute_target.clone())), eat_string(":"), expression.clone(), opt(choice!(seq!(eat_string("="), annotated_rhs.clone())))),
        seq!(repeat(choice!(seq!(star_targets.clone(), eat_string("=")))), choice!(seq!(yield_expr.clone()), seq!(star_expressions.clone())), eps(), opt(choice!(seq!(TYPE_COMMENT.clone())))),
        seq!(single_target.clone(), augassign.clone(), eps(), choice!(seq!(yield_expr.clone()), seq!(star_expressions.clone()))),
        seq!(invalid_assignment.clone())));
    let annotated_rhs = Rc::new(choice!(seq!(yield_expr.clone()),
        seq!(star_expressions.clone())));
    let augassign = Rc::new(choice!(seq!(eat_string("+=")),
        seq!(eat_string("-=")),
        seq!(eat_string("*=")),
        seq!(eat_string("@=")),
        seq!(eat_string("/=")),
        seq!(eat_string("%=")),
        seq!(eat_string("&=")),
        seq!(eat_string("|=")),
        seq!(eat_string("^=")),
        seq!(eat_string("<<=")),
        seq!(eat_string(">>=")),
        seq!(eat_string("**=")),
        seq!(eat_string("//="))));
    let return_stmt = Rc::new(choice!(seq!(eat_string("return"), opt(choice!(seq!(star_expressions.clone()))))));
    let raise_stmt = Rc::new(choice!(seq!(eat_string("raise"), expression.clone(), opt(choice!(seq!(eat_string("from"), expression.clone())))),
        seq!(eat_string("raise"))));
    let global_stmt = Rc::new(choice!(seq!(eat_string("global"), seq!(NAME.clone(), eat_string(",")))));
    let nonlocal_stmt = Rc::new(choice!(seq!(eat_string("nonlocal"), seq!(NAME.clone(), eat_string(",")))));
    let del_stmt = Rc::new(choice!(seq!(eat_string("del"), del_targets.clone(), eps()),
        seq!(invalid_del_stmt.clone())));
    let yield_stmt = Rc::new(choice!(seq!(yield_expr.clone())));
    let assert_stmt = Rc::new(choice!(seq!(eat_string("assert"), expression.clone(), opt(choice!(seq!(eat_string(","), expression.clone()))))));
    let import_stmt = Rc::new(choice!(seq!(invalid_import.clone()),
        seq!(import_name.clone()),
        seq!(import_from.clone())));
    let import_name = Rc::new(choice!(seq!(eat_string("import"), dotted_as_names.clone())));
    let import_from = Rc::new(choice!(seq!(eat_string("from"), repeat(choice!(seq!(eat_string(".")), seq!(eat_string("...")))), dotted_name.clone(), eat_string("import"), import_from_targets.clone()),
        seq!(eat_string("from"), repeat(choice!(seq!(eat_string(".")), seq!(eat_string("...")))), eat_string("import"), import_from_targets.clone())));
    let import_from_targets = Rc::new(choice!(seq!(eat_string("("), import_from_as_names.clone(), opt(choice!(seq!(eat_string(",")))), eat_string(")")),
        seq!(import_from_as_names.clone(), eps()),
        seq!(eat_string("*")),
        seq!(invalid_import_from_targets.clone())));
    let import_from_as_names = Rc::new(choice!(seq!(seq!(import_from_as_name.clone(), eat_string(",")))));
    let import_from_as_name = Rc::new(choice!(seq!(NAME.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))))));
    let dotted_as_names = Rc::new(choice!(seq!(seq!(dotted_as_name.clone(), eat_string(",")))));
    let dotted_as_name = Rc::new(choice!(seq!(dotted_name.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))))));
    let dotted_name = Rc::new(choice!(seq!(dotted_name.clone(), eat_string("."), NAME.clone()),
        seq!(NAME.clone())));
    let block = Rc::new(choice!(seq!(NEWLINE.clone(), INDENT.clone(), statements.clone(), DEDENT.clone()),
        seq!(simple_stmts.clone()),
        seq!(invalid_block.clone())));
    let decorators = Rc::new(choice!(seq!(repeat(choice!(seq!(eat_string("@"), named_expression.clone(), NEWLINE.clone()))))));
    let class_def = Rc::new(choice!(seq!(decorators.clone(), class_def_raw.clone()),
        seq!(class_def_raw.clone())));
    let class_def_raw = Rc::new(choice!(seq!(invalid_class_def_raw.clone()),
        seq!(eat_string("class"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")")))), eat_string(":"), block.clone())));
    let function_def = Rc::new(choice!(seq!(decorators.clone(), function_def_raw.clone()),
        seq!(function_def_raw.clone())));
    let function_def_raw = Rc::new(choice!(seq!(invalid_def_raw.clone()),
        seq!(eat_string("def"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("("), opt(choice!(seq!(params.clone()))), eat_string(")"), opt(choice!(seq!(eat_string("->"), expression.clone()))), eat_string(":"), opt(choice!(seq!(func_type_comment.clone()))), block.clone()),
        seq!(eat_string("async"), eat_string("def"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("("), opt(choice!(seq!(params.clone()))), eat_string(")"), opt(choice!(seq!(eat_string("->"), expression.clone()))), eat_string(":"), opt(choice!(seq!(func_type_comment.clone()))), block.clone())));
    let params = Rc::new(choice!(seq!(invalid_parameters.clone()),
        seq!(parameters.clone())));
    let parameters = Rc::new(choice!(seq!(slash_no_default.clone(), repeat(param_no_default.clone()), repeat(param_with_default.clone()), opt(choice!(seq!(star_etc.clone())))),
        seq!(slash_with_default.clone(), repeat(param_with_default.clone()), opt(choice!(seq!(star_etc.clone())))),
        seq!(repeat(param_no_default.clone()), repeat(param_with_default.clone()), opt(choice!(seq!(star_etc.clone())))),
        seq!(repeat(param_with_default.clone()), opt(choice!(seq!(star_etc.clone())))),
        seq!(star_etc.clone())));
    let slash_no_default = Rc::new(choice!(seq!(repeat(param_no_default.clone()), eat_string("/"), eat_string(",")),
        seq!(repeat(param_no_default.clone()), eat_string("/"), eps())));
    let slash_with_default = Rc::new(choice!(seq!(repeat(param_no_default.clone()), repeat(param_with_default.clone()), eat_string("/"), eat_string(",")),
        seq!(repeat(param_no_default.clone()), repeat(param_with_default.clone()), eat_string("/"), eps())));
    let star_etc = Rc::new(choice!(seq!(invalid_star_etc.clone()),
        seq!(eat_string("*"), param_no_default.clone(), repeat(param_maybe_default.clone()), opt(choice!(seq!(kwds.clone())))),
        seq!(eat_string("*"), param_no_default_star_annotation.clone(), repeat(param_maybe_default.clone()), opt(choice!(seq!(kwds.clone())))),
        seq!(eat_string("*"), eat_string(","), repeat(param_maybe_default.clone()), opt(choice!(seq!(kwds.clone())))),
        seq!(kwds.clone())));
    let kwds = Rc::new(choice!(seq!(invalid_kwds.clone()),
        seq!(eat_string("**"), param_no_default.clone())));
    let param_no_default = Rc::new(choice!(seq!(param.clone(), eat_string(","), opt(TYPE_COMMENT.clone())),
        seq!(param.clone(), opt(TYPE_COMMENT.clone()), eps())));
    let param_no_default_star_annotation = Rc::new(choice!(seq!(param_star_annotation.clone(), eat_string(","), opt(TYPE_COMMENT.clone())),
        seq!(param_star_annotation.clone(), opt(TYPE_COMMENT.clone()), eps())));
    let param_with_default = Rc::new(choice!(seq!(param.clone(), default.clone(), eat_string(","), opt(TYPE_COMMENT.clone())),
        seq!(param.clone(), default.clone(), opt(TYPE_COMMENT.clone()), eps())));
    let param_maybe_default = Rc::new(choice!(seq!(param.clone(), opt(default.clone()), eat_string(","), opt(TYPE_COMMENT.clone())),
        seq!(param.clone(), opt(default.clone()), opt(TYPE_COMMENT.clone()), eps())));
    let param = Rc::new(choice!(seq!(NAME.clone(), opt(annotation.clone()))));
    let param_star_annotation = Rc::new(choice!(seq!(NAME.clone(), star_annotation.clone())));
    let annotation = Rc::new(choice!(seq!(eat_string(":"), expression.clone())));
    let star_annotation = Rc::new(choice!(seq!(eat_string(":"), star_expression.clone())));
    let default = Rc::new(choice!(seq!(eat_string("="), expression.clone()),
        seq!(invalid_default.clone())));
    let if_stmt = Rc::new(choice!(seq!(invalid_if_stmt.clone()),
        seq!(eat_string("if"), named_expression.clone(), eat_string(":"), block.clone(), elif_stmt.clone()),
        seq!(eat_string("if"), named_expression.clone(), eat_string(":"), block.clone(), opt(choice!(seq!(else_block.clone()))))));
    let elif_stmt = Rc::new(choice!(seq!(invalid_elif_stmt.clone()),
        seq!(eat_string("elif"), named_expression.clone(), eat_string(":"), block.clone(), elif_stmt.clone()),
        seq!(eat_string("elif"), named_expression.clone(), eat_string(":"), block.clone(), opt(choice!(seq!(else_block.clone()))))));
    let else_block = Rc::new(choice!(seq!(invalid_else_stmt.clone()),
        seq!(eat_string("else"), eat_string(":"), block.clone())));
    let while_stmt = Rc::new(choice!(seq!(invalid_while_stmt.clone()),
        seq!(eat_string("while"), named_expression.clone(), eat_string(":"), block.clone(), opt(choice!(seq!(else_block.clone()))))));
    let for_stmt = Rc::new(choice!(seq!(invalid_for_stmt.clone()),
        seq!(eat_string("for"), star_targets.clone(), eat_string("in"), eps(), star_expressions.clone(), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone(), opt(choice!(seq!(else_block.clone())))),
        seq!(eat_string("async"), eat_string("for"), star_targets.clone(), eat_string("in"), eps(), star_expressions.clone(), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone(), opt(choice!(seq!(else_block.clone())))),
        seq!(invalid_for_target.clone())));
    let with_stmt = Rc::new(choice!(seq!(invalid_with_stmt_indent.clone()),
        seq!(eat_string("with"), eat_string("("), seq!(with_item.clone(), eat_string(",")), opt(eat_string(",")), eat_string(")"), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone()),
        seq!(eat_string("with"), seq!(with_item.clone(), eat_string(",")), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone()),
        seq!(eat_string("async"), eat_string("with"), eat_string("("), seq!(with_item.clone(), eat_string(",")), opt(eat_string(",")), eat_string(")"), eat_string(":"), block.clone()),
        seq!(eat_string("async"), eat_string("with"), seq!(with_item.clone(), eat_string(",")), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone()),
        seq!(invalid_with_stmt.clone())));
    let with_item = Rc::new(choice!(seq!(expression.clone(), eat_string("as"), star_target.clone(), eps()),
        seq!(invalid_with_item.clone()),
        seq!(expression.clone())));
    let try_stmt = Rc::new(choice!(seq!(invalid_try_stmt.clone()),
        seq!(eat_string("try"), eat_string(":"), block.clone(), finally_block.clone()),
        seq!(eat_string("try"), eat_string(":"), block.clone(), repeat(except_block.clone()), opt(choice!(seq!(else_block.clone()))), opt(choice!(seq!(finally_block.clone())))),
        seq!(eat_string("try"), eat_string(":"), block.clone(), repeat(except_star_block.clone()), opt(choice!(seq!(else_block.clone()))), opt(choice!(seq!(finally_block.clone()))))));
    let except_block = Rc::new(choice!(seq!(invalid_except_stmt_indent.clone()),
        seq!(eat_string("except"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":"), block.clone()),
        seq!(eat_string("except"), eat_string(":"), block.clone()),
        seq!(invalid_except_stmt.clone())));
    let except_star_block = Rc::new(choice!(seq!(invalid_except_star_stmt_indent.clone()),
        seq!(eat_string("except"), eat_string("*"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":"), block.clone()),
        seq!(invalid_except_stmt.clone())));
    let finally_block = Rc::new(choice!(seq!(invalid_finally_stmt.clone()),
        seq!(eat_string("finally"), eat_string(":"), block.clone())));
    let match_stmt = Rc::new(choice!(seq!(eat_string("match"), subject_expr.clone(), eat_string(":"), NEWLINE.clone(), INDENT.clone(), repeat(case_block.clone()), DEDENT.clone()),
        seq!(invalid_match_stmt.clone())));
    let subject_expr = Rc::new(choice!(seq!(star_named_expression.clone(), eat_string(","), opt(star_named_expressions.clone())),
        seq!(named_expression.clone())));
    let case_block = Rc::new(choice!(seq!(invalid_case_block.clone()),
        seq!(eat_string("case"), patterns.clone(), opt(guard.clone()), eat_string(":"), block.clone())));
    let guard = Rc::new(choice!(seq!(eat_string("if"), named_expression.clone())));
    let patterns = Rc::new(choice!(seq!(open_sequence_pattern.clone()),
        seq!(pattern.clone())));
    let pattern = Rc::new(choice!(seq!(as_pattern.clone()),
        seq!(or_pattern.clone())));
    let as_pattern = Rc::new(choice!(seq!(or_pattern.clone(), eat_string("as"), pattern_capture_target.clone()),
        seq!(invalid_as_pattern.clone())));
    let or_pattern = Rc::new(choice!(seq!(seq!(closed_pattern.clone(), eat_string("|")))));
    let closed_pattern = Rc::new(choice!(seq!(literal_pattern.clone()),
        seq!(capture_pattern.clone()),
        seq!(wildcard_pattern.clone()),
        seq!(value_pattern.clone()),
        seq!(group_pattern.clone()),
        seq!(sequence_pattern.clone()),
        seq!(mapping_pattern.clone()),
        seq!(class_pattern.clone())));
    let literal_pattern = Rc::new(choice!(seq!(signed_number.clone(), eps()),
        seq!(complex_number.clone()),
        seq!(strings.clone()),
        seq!(eat_string("None")),
        seq!(eat_string("True")),
        seq!(eat_string("False"))));
    let literal_expr = Rc::new(choice!(seq!(signed_number.clone(), eps()),
        seq!(complex_number.clone()),
        seq!(strings.clone()),
        seq!(eat_string("None")),
        seq!(eat_string("True")),
        seq!(eat_string("False"))));
    let complex_number = Rc::new(choice!(seq!(signed_real_number.clone(), eat_string("+"), imaginary_number.clone()),
        seq!(signed_real_number.clone(), eat_string("-"), imaginary_number.clone())));
    let signed_number = Rc::new(choice!(seq!(NUMBER.clone()),
        seq!(eat_string("-"), NUMBER.clone())));
    let signed_real_number = Rc::new(choice!(seq!(real_number.clone()),
        seq!(eat_string("-"), real_number.clone())));
    let real_number = Rc::new(choice!(seq!(NUMBER.clone())));
    let imaginary_number = Rc::new(choice!(seq!(NUMBER.clone())));
    let capture_pattern = Rc::new(choice!(seq!(pattern_capture_target.clone())));
    let pattern_capture_target = Rc::new(choice!(seq!(eps(), NAME.clone(), eps())));
    let wildcard_pattern = Rc::new(choice!(seq!(eat_string("_"))));
    let value_pattern = Rc::new(choice!(seq!(attr.clone(), eps())));
    let attr = Rc::new(choice!(seq!(name_or_attr.clone(), eat_string("."), NAME.clone())));
    let name_or_attr = Rc::new(choice!(seq!(attr.clone()),
        seq!(NAME.clone())));
    let group_pattern = Rc::new(choice!(seq!(eat_string("("), pattern.clone(), eat_string(")"))));
    let sequence_pattern = Rc::new(choice!(seq!(eat_string("["), opt(maybe_sequence_pattern.clone()), eat_string("]")),
        seq!(eat_string("("), opt(open_sequence_pattern.clone()), eat_string(")"))));
    let open_sequence_pattern = Rc::new(choice!(seq!(maybe_star_pattern.clone(), eat_string(","), opt(maybe_sequence_pattern.clone()))));
    let maybe_sequence_pattern = Rc::new(choice!(seq!(seq!(maybe_star_pattern.clone(), eat_string(",")), opt(eat_string(",")))));
    let maybe_star_pattern = Rc::new(choice!(seq!(star_pattern.clone()),
        seq!(pattern.clone())));
    let star_pattern = Rc::new(choice!(seq!(eat_string("*"), pattern_capture_target.clone()),
        seq!(eat_string("*"), wildcard_pattern.clone())));
    let mapping_pattern = Rc::new(choice!(seq!(eat_string("{"), eat_string("}")),
        seq!(eat_string("{"), double_star_pattern.clone(), opt(eat_string(",")), eat_string("}")),
        seq!(eat_string("{"), items_pattern.clone(), eat_string(","), double_star_pattern.clone(), opt(eat_string(",")), eat_string("}")),
        seq!(eat_string("{"), items_pattern.clone(), opt(eat_string(",")), eat_string("}"))));
    let items_pattern = Rc::new(choice!(seq!(seq!(key_value_pattern.clone(), eat_string(",")))));
    let key_value_pattern = Rc::new(choice!(seq!(choice!(seq!(literal_expr.clone()), seq!(attr.clone())), eat_string(":"), pattern.clone())));
    let double_star_pattern = Rc::new(choice!(seq!(eat_string("**"), pattern_capture_target.clone())));
    let class_pattern = Rc::new(choice!(seq!(name_or_attr.clone(), eat_string("("), eat_string(")")),
        seq!(name_or_attr.clone(), eat_string("("), positional_patterns.clone(), opt(eat_string(",")), eat_string(")")),
        seq!(name_or_attr.clone(), eat_string("("), keyword_patterns.clone(), opt(eat_string(",")), eat_string(")")),
        seq!(name_or_attr.clone(), eat_string("("), positional_patterns.clone(), eat_string(","), keyword_patterns.clone(), opt(eat_string(",")), eat_string(")")),
        seq!(invalid_class_pattern.clone())));
    let positional_patterns = Rc::new(choice!(seq!(seq!(pattern.clone(), eat_string(",")))));
    let keyword_patterns = Rc::new(choice!(seq!(seq!(keyword_pattern.clone(), eat_string(",")))));
    let keyword_pattern = Rc::new(choice!(seq!(NAME.clone(), eat_string("="), pattern.clone())));
    let type_alias = Rc::new(choice!(seq!(eat_string("type"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("="), expression.clone())));
    let type_params = Rc::new(choice!(seq!(invalid_type_params.clone()),
        seq!(eat_string("["), type_param_seq.clone(), eat_string("]"))));
    let type_param_seq = Rc::new(choice!(seq!(seq!(type_param.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let type_param = Rc::new(choice!(seq!(NAME.clone(), opt(choice!(seq!(type_param_bound.clone()))), opt(choice!(seq!(type_param_default.clone())))),
        seq!(invalid_type_param.clone()),
        seq!(eat_string("*"), NAME.clone(), opt(choice!(seq!(type_param_starred_default.clone())))),
        seq!(eat_string("**"), NAME.clone(), opt(choice!(seq!(type_param_default.clone()))))));
    let type_param_bound = Rc::new(choice!(seq!(eat_string(":"), expression.clone())));
    let type_param_default = Rc::new(choice!(seq!(eat_string("="), expression.clone())));
    let type_param_starred_default = Rc::new(choice!(seq!(eat_string("="), star_expression.clone())));
    let expressions = Rc::new(choice!(seq!(expression.clone(), repeat(choice!(seq!(eat_string(","), expression.clone()))), opt(choice!(seq!(eat_string(","))))),
        seq!(expression.clone(), eat_string(",")),
        seq!(expression.clone())));
    let expression = Rc::new(choice!(seq!(invalid_expression.clone()),
        seq!(invalid_legacy_expression.clone()),
        seq!(disjunction.clone(), eat_string("if"), disjunction.clone(), eat_string("else"), expression.clone()),
        seq!(disjunction.clone()),
        seq!(lambdef.clone())));
    let yield_expr = Rc::new(choice!(seq!(eat_string("yield"), eat_string("from"), expression.clone()),
        seq!(eat_string("yield"), opt(choice!(seq!(star_expressions.clone()))))));
    let star_expressions = Rc::new(choice!(seq!(star_expression.clone(), repeat(choice!(seq!(eat_string(","), star_expression.clone()))), opt(choice!(seq!(eat_string(","))))),
        seq!(star_expression.clone(), eat_string(",")),
        seq!(star_expression.clone())));
    let star_expression = Rc::new(choice!(seq!(eat_string("*"), bitwise_or.clone()),
        seq!(expression.clone())));
    let star_named_expressions = Rc::new(choice!(seq!(seq!(star_named_expression.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let star_named_expression = Rc::new(choice!(seq!(eat_string("*"), bitwise_or.clone()),
        seq!(named_expression.clone())));
    let assignment_expression = Rc::new(choice!(seq!(NAME.clone(), eat_string(":="), eps(), expression.clone())));
    let named_expression = Rc::new(choice!(seq!(assignment_expression.clone()),
        seq!(invalid_named_expression.clone()),
        seq!(expression.clone(), eps())));
    let disjunction = Rc::new(choice!(seq!(conjunction.clone(), repeat(choice!(seq!(eat_string("or"), conjunction.clone())))),
        seq!(conjunction.clone())));
    let conjunction = Rc::new(choice!(seq!(inversion.clone(), repeat(choice!(seq!(eat_string("and"), inversion.clone())))),
        seq!(inversion.clone())));
    let inversion = Rc::new(choice!(seq!(eat_string("not"), inversion.clone()),
        seq!(comparison.clone())));
    let comparison = Rc::new(choice!(seq!(bitwise_or.clone(), repeat(compare_op_bitwise_or_pair.clone())),
        seq!(bitwise_or.clone())));
    let compare_op_bitwise_or_pair = Rc::new(choice!(seq!(eq_bitwise_or.clone()),
        seq!(noteq_bitwise_or.clone()),
        seq!(lte_bitwise_or.clone()),
        seq!(lt_bitwise_or.clone()),
        seq!(gte_bitwise_or.clone()),
        seq!(gt_bitwise_or.clone()),
        seq!(notin_bitwise_or.clone()),
        seq!(in_bitwise_or.clone()),
        seq!(isnot_bitwise_or.clone()),
        seq!(is_bitwise_or.clone())));
    let eq_bitwise_or = Rc::new(choice!(seq!(eat_string("=="), bitwise_or.clone())));
    let noteq_bitwise_or = Rc::new(choice!(seq!(choice!(seq!(eat_string("!="))), bitwise_or.clone())));
    let lte_bitwise_or = Rc::new(choice!(seq!(eat_string("<="), bitwise_or.clone())));
    let lt_bitwise_or = Rc::new(choice!(seq!(eat_string("<"), bitwise_or.clone())));
    let gte_bitwise_or = Rc::new(choice!(seq!(eat_string(">="), bitwise_or.clone())));
    let gt_bitwise_or = Rc::new(choice!(seq!(eat_string(">"), bitwise_or.clone())));
    let notin_bitwise_or = Rc::new(choice!(seq!(eat_string("not"), eat_string("in"), bitwise_or.clone())));
    let in_bitwise_or = Rc::new(choice!(seq!(eat_string("in"), bitwise_or.clone())));
    let isnot_bitwise_or = Rc::new(choice!(seq!(eat_string("is"), eat_string("not"), bitwise_or.clone())));
    let is_bitwise_or = Rc::new(choice!(seq!(eat_string("is"), bitwise_or.clone())));
    let bitwise_or = Rc::new(choice!(seq!(bitwise_or.clone(), eat_string("|"), bitwise_xor.clone()),
        seq!(bitwise_xor.clone())));
    let bitwise_xor = Rc::new(choice!(seq!(bitwise_xor.clone(), eat_string("^"), bitwise_and.clone()),
        seq!(bitwise_and.clone())));
    let bitwise_and = Rc::new(choice!(seq!(bitwise_and.clone(), eat_string("&"), shift_expr.clone()),
        seq!(shift_expr.clone())));
    let shift_expr = Rc::new(choice!(seq!(shift_expr.clone(), eat_string("<<"), sum.clone()),
        seq!(shift_expr.clone(), eat_string(">>"), sum.clone()),
        seq!(invalid_arithmetic.clone()),
        seq!(sum.clone())));
    let sum = Rc::new(choice!(seq!(sum.clone(), eat_string("+"), term.clone()),
        seq!(sum.clone(), eat_string("-"), term.clone()),
        seq!(term.clone())));
    let term = Rc::new(choice!(seq!(term.clone(), eat_string("*"), factor.clone()),
        seq!(term.clone(), eat_string("/"), factor.clone()),
        seq!(term.clone(), eat_string("//"), factor.clone()),
        seq!(term.clone(), eat_string("%"), factor.clone()),
        seq!(term.clone(), eat_string("@"), factor.clone()),
        seq!(invalid_factor.clone()),
        seq!(factor.clone())));
    let factor = Rc::new(choice!(seq!(eat_string("+"), factor.clone()),
        seq!(eat_string("-"), factor.clone()),
        seq!(eat_string("~"), factor.clone()),
        seq!(power.clone())));
    let power = Rc::new(choice!(seq!(await_primary.clone(), eat_string("**"), factor.clone()),
        seq!(await_primary.clone())));
    let await_primary = Rc::new(choice!(seq!(eat_string("await"), primary.clone()),
        seq!(primary.clone())));
    let primary = Rc::new(choice!(seq!(primary.clone(), eat_string("."), NAME.clone()),
        seq!(primary.clone(), genexp.clone()),
        seq!(primary.clone(), eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")")),
        seq!(primary.clone(), eat_string("["), slices.clone(), eat_string("]")),
        seq!(atom.clone())));
    let slices = Rc::new(choice!(seq!(slice.clone(), eps()),
        seq!(seq!(choice!(seq!(slice.clone()), seq!(starred_expression.clone())), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let slice = Rc::new(choice!(seq!(opt(choice!(seq!(expression.clone()))), eat_string(":"), opt(choice!(seq!(expression.clone()))), opt(choice!(seq!(eat_string(":"), opt(choice!(seq!(expression.clone()))))))),
        seq!(named_expression.clone())));
    let atom = Rc::new(choice!(seq!(NAME.clone()),
        seq!(eat_string("True")),
        seq!(eat_string("False")),
        seq!(eat_string("None")),
        seq!(eps(), strings.clone()),
        seq!(NUMBER.clone()),
        seq!(eps(), choice!(seq!(tuple.clone()), seq!(group.clone()), seq!(genexp.clone()))),
        seq!(eps(), choice!(seq!(list.clone()), seq!(listcomp.clone()))),
        seq!(eps(), choice!(seq!(dict.clone()), seq!(set.clone()), seq!(dictcomp.clone()), seq!(setcomp.clone()))),
        seq!(eat_string("..."))));
    let group = Rc::new(choice!(seq!(eat_string("("), choice!(seq!(yield_expr.clone()), seq!(named_expression.clone())), eat_string(")")),
        seq!(invalid_group.clone())));
    let lambdef = Rc::new(choice!(seq!(eat_string("lambda"), opt(choice!(seq!(lambda_params.clone()))), eat_string(":"), expression.clone())));
    let lambda_params = Rc::new(choice!(seq!(invalid_lambda_parameters.clone()),
        seq!(lambda_parameters.clone())));
    let lambda_parameters = Rc::new(choice!(seq!(lambda_slash_no_default.clone(), repeat(lambda_param_no_default.clone()), repeat(lambda_param_with_default.clone()), opt(choice!(seq!(lambda_star_etc.clone())))),
        seq!(lambda_slash_with_default.clone(), repeat(lambda_param_with_default.clone()), opt(choice!(seq!(lambda_star_etc.clone())))),
        seq!(repeat(lambda_param_no_default.clone()), repeat(lambda_param_with_default.clone()), opt(choice!(seq!(lambda_star_etc.clone())))),
        seq!(repeat(lambda_param_with_default.clone()), opt(choice!(seq!(lambda_star_etc.clone())))),
        seq!(lambda_star_etc.clone())));
    let lambda_slash_no_default = Rc::new(choice!(seq!(repeat(lambda_param_no_default.clone()), eat_string("/"), eat_string(",")),
        seq!(repeat(lambda_param_no_default.clone()), eat_string("/"), eps())));
    let lambda_slash_with_default = Rc::new(choice!(seq!(repeat(lambda_param_no_default.clone()), repeat(lambda_param_with_default.clone()), eat_string("/"), eat_string(",")),
        seq!(repeat(lambda_param_no_default.clone()), repeat(lambda_param_with_default.clone()), eat_string("/"), eps())));
    let lambda_star_etc = Rc::new(choice!(seq!(invalid_lambda_star_etc.clone()),
        seq!(eat_string("*"), lambda_param_no_default.clone(), repeat(lambda_param_maybe_default.clone()), opt(choice!(seq!(lambda_kwds.clone())))),
        seq!(eat_string("*"), eat_string(","), repeat(lambda_param_maybe_default.clone()), opt(choice!(seq!(lambda_kwds.clone())))),
        seq!(lambda_kwds.clone())));
    let lambda_kwds = Rc::new(choice!(seq!(invalid_lambda_kwds.clone()),
        seq!(eat_string("**"), lambda_param_no_default.clone())));
    let lambda_param_no_default = Rc::new(choice!(seq!(lambda_param.clone(), eat_string(",")),
        seq!(lambda_param.clone(), eps())));
    let lambda_param_with_default = Rc::new(choice!(seq!(lambda_param.clone(), default.clone(), eat_string(",")),
        seq!(lambda_param.clone(), default.clone(), eps())));
    let lambda_param_maybe_default = Rc::new(choice!(seq!(lambda_param.clone(), opt(default.clone()), eat_string(",")),
        seq!(lambda_param.clone(), opt(default.clone()), eps())));
    let lambda_param = Rc::new(choice!(seq!(NAME.clone())));
    let fstring_middle = Rc::new(choice!(seq!(fstring_replacement_field.clone()),
        seq!(FSTRING_MIDDLE.clone())));
    let fstring_replacement_field = Rc::new(choice!(seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), opt(choice!(seq!(fstring_conversion.clone()))), opt(choice!(seq!(fstring_full_format_spec.clone()))), eat_string("}")),
        seq!(invalid_replacement_field.clone())));
    let fstring_conversion = Rc::new(choice!(seq!(eat_string("!"), NAME.clone())));
    let fstring_full_format_spec = Rc::new(choice!(seq!(eat_string(":"), repeat(fstring_format_spec.clone()))));
    let fstring_format_spec = Rc::new(choice!(seq!(FSTRING_MIDDLE.clone()),
        seq!(fstring_replacement_field.clone())));
    let fstring = Rc::new(choice!(seq!(FSTRING_START.clone(), repeat(fstring_middle.clone()), FSTRING_END.clone())));
    let string = Rc::new(choice!(seq!(STRING.clone())));
    let strings = Rc::new(choice!(seq!(repeat(choice!(seq!(fstring.clone()), seq!(string.clone()))))));
    let list = Rc::new(choice!(seq!(eat_string("["), opt(choice!(seq!(star_named_expressions.clone()))), eat_string("]"))));
    let tuple = Rc::new(choice!(seq!(eat_string("("), opt(choice!(seq!(star_named_expression.clone(), eat_string(","), opt(choice!(seq!(star_named_expressions.clone())))))), eat_string(")"))));
    let set = Rc::new(choice!(seq!(eat_string("{"), star_named_expressions.clone(), eat_string("}"))));
    let dict = Rc::new(choice!(seq!(eat_string("{"), opt(choice!(seq!(double_starred_kvpairs.clone()))), eat_string("}")),
        seq!(eat_string("{"), invalid_double_starred_kvpairs.clone(), eat_string("}"))));
    let double_starred_kvpairs = Rc::new(choice!(seq!(seq!(double_starred_kvpair.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let double_starred_kvpair = Rc::new(choice!(seq!(eat_string("**"), bitwise_or.clone()),
        seq!(kvpair.clone())));
    let kvpair = Rc::new(choice!(seq!(expression.clone(), eat_string(":"), expression.clone())));
    let for_if_clauses = Rc::new(choice!(seq!(repeat(for_if_clause.clone()))));
    let for_if_clause = Rc::new(choice!(seq!(eat_string("async"), eat_string("for"), star_targets.clone(), eat_string("in"), eps(), disjunction.clone(), repeat(choice!(seq!(eat_string("if"), disjunction.clone())))),
        seq!(eat_string("for"), star_targets.clone(), eat_string("in"), eps(), disjunction.clone(), repeat(choice!(seq!(eat_string("if"), disjunction.clone())))),
        seq!(invalid_for_if_clause.clone()),
        seq!(invalid_for_target.clone())));
    let listcomp = Rc::new(choice!(seq!(eat_string("["), named_expression.clone(), for_if_clauses.clone(), eat_string("]")),
        seq!(invalid_comprehension.clone())));
    let setcomp = Rc::new(choice!(seq!(eat_string("{"), named_expression.clone(), for_if_clauses.clone(), eat_string("}")),
        seq!(invalid_comprehension.clone())));
    let genexp = Rc::new(choice!(seq!(eat_string("("), choice!(seq!(assignment_expression.clone()), seq!(expression.clone(), eps())), for_if_clauses.clone(), eat_string(")")),
        seq!(invalid_comprehension.clone())));
    let dictcomp = Rc::new(choice!(seq!(eat_string("{"), kvpair.clone(), for_if_clauses.clone(), eat_string("}")),
        seq!(invalid_dict_comprehension.clone())));
    let arguments = Rc::new(choice!(seq!(args.clone(), opt(choice!(seq!(eat_string(",")))), eps()),
        seq!(invalid_arguments.clone())));
    let args = Rc::new(choice!(seq!(seq!(choice!(seq!(starred_expression.clone()), seq!(choice!(seq!(assignment_expression.clone()), seq!(expression.clone(), eps())), eps())), eat_string(",")), opt(choice!(seq!(eat_string(","), kwargs.clone())))),
        seq!(kwargs.clone())));
    let kwargs = Rc::new(choice!(seq!(seq!(kwarg_or_starred.clone(), eat_string(",")), eat_string(","), seq!(kwarg_or_double_starred.clone(), eat_string(","))),
        seq!(seq!(kwarg_or_starred.clone(), eat_string(","))),
        seq!(seq!(kwarg_or_double_starred.clone(), eat_string(",")))));
    let starred_expression = Rc::new(choice!(seq!(invalid_starred_expression_unpacking.clone()),
        seq!(eat_string("*"), expression.clone()),
        seq!(invalid_starred_expression.clone())));
    let kwarg_or_starred = Rc::new(choice!(seq!(invalid_kwarg.clone()),
        seq!(NAME.clone(), eat_string("="), expression.clone()),
        seq!(starred_expression.clone())));
    let kwarg_or_double_starred = Rc::new(choice!(seq!(invalid_kwarg.clone()),
        seq!(NAME.clone(), eat_string("="), expression.clone()),
        seq!(eat_string("**"), expression.clone())));
    let star_targets = Rc::new(choice!(seq!(star_target.clone(), eps()),
        seq!(star_target.clone(), repeat(choice!(seq!(eat_string(","), star_target.clone()))), opt(choice!(seq!(eat_string(",")))))));
    let star_targets_list_seq = Rc::new(choice!(seq!(seq!(star_target.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let star_targets_tuple_seq = Rc::new(choice!(seq!(star_target.clone(), repeat(choice!(seq!(eat_string(","), star_target.clone()))), opt(choice!(seq!(eat_string(","))))),
        seq!(star_target.clone(), eat_string(","))));
    let star_target = Rc::new(choice!(seq!(eat_string("*"), choice!(seq!(eps(), star_target.clone()))),
        seq!(target_with_star_atom.clone())));
    let target_with_star_atom = Rc::new(choice!(seq!(t_primary.clone(), eat_string("."), NAME.clone(), eps()),
        seq!(t_primary.clone(), eat_string("["), slices.clone(), eat_string("]"), eps()),
        seq!(star_atom.clone())));
    let star_atom = Rc::new(choice!(seq!(NAME.clone()),
        seq!(eat_string("("), target_with_star_atom.clone(), eat_string(")")),
        seq!(eat_string("("), opt(choice!(seq!(star_targets_tuple_seq.clone()))), eat_string(")")),
        seq!(eat_string("["), opt(choice!(seq!(star_targets_list_seq.clone()))), eat_string("]"))));
    let single_target = Rc::new(choice!(seq!(single_subscript_attribute_target.clone()),
        seq!(NAME.clone()),
        seq!(eat_string("("), single_target.clone(), eat_string(")"))));
    let single_subscript_attribute_target = Rc::new(choice!(seq!(t_primary.clone(), eat_string("."), NAME.clone(), eps()),
        seq!(t_primary.clone(), eat_string("["), slices.clone(), eat_string("]"), eps())));
    let t_primary = Rc::new(choice!(seq!(t_primary.clone(), eat_string("."), NAME.clone(), eps()),
        seq!(t_primary.clone(), eat_string("["), slices.clone(), eat_string("]"), eps()),
        seq!(t_primary.clone(), genexp.clone(), eps()),
        seq!(t_primary.clone(), eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")"), eps()),
        seq!(atom.clone(), eps())));
    let t_lookahead = Rc::new(choice!(seq!(eat_string("(")),
        seq!(eat_string("[")),
        seq!(eat_string("."))));
    let del_targets = Rc::new(choice!(seq!(seq!(del_target.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let del_target = Rc::new(choice!(seq!(t_primary.clone(), eat_string("."), NAME.clone(), eps()),
        seq!(t_primary.clone(), eat_string("["), slices.clone(), eat_string("]"), eps()),
        seq!(del_t_atom.clone())));
    let del_t_atom = Rc::new(choice!(seq!(NAME.clone()),
        seq!(eat_string("("), del_target.clone(), eat_string(")")),
        seq!(eat_string("("), opt(choice!(seq!(del_targets.clone()))), eat_string(")")),
        seq!(eat_string("["), opt(choice!(seq!(del_targets.clone()))), eat_string("]"))));
    let type_expressions = Rc::new(choice!(seq!(seq!(expression.clone(), eat_string(",")), eat_string(","), eat_string("*"), expression.clone(), eat_string(","), eat_string("**"), expression.clone()),
        seq!(seq!(expression.clone(), eat_string(",")), eat_string(","), eat_string("*"), expression.clone()),
        seq!(seq!(expression.clone(), eat_string(",")), eat_string(","), eat_string("**"), expression.clone()),
        seq!(eat_string("*"), expression.clone(), eat_string(","), eat_string("**"), expression.clone()),
        seq!(eat_string("*"), expression.clone()),
        seq!(eat_string("**"), expression.clone()),
        seq!(seq!(expression.clone(), eat_string(",")))));
    let func_type_comment = Rc::new(choice!(seq!(NEWLINE.clone(), TYPE_COMMENT.clone(), eps()),
        seq!(invalid_double_type_comments.clone()),
        seq!(TYPE_COMMENT.clone())));
    let invalid_arguments = Rc::new(choice!(seq!(choice!(seq!(choice!(seq!(seq!(choice!(seq!(starred_expression.clone()), seq!(choice!(seq!(assignment_expression.clone()), seq!(expression.clone(), eps())), eps())), eat_string(",")), eat_string(","), kwargs.clone()))), seq!(kwargs.clone())), eat_string(","), seq!(choice!(seq!(starred_expression.clone(), eps())), eat_string(","))),
        seq!(expression.clone(), for_if_clauses.clone(), eat_string(","), opt(choice!(seq!(args.clone()), seq!(expression.clone(), for_if_clauses.clone())))),
        seq!(NAME.clone(), eat_string("="), expression.clone(), for_if_clauses.clone()),
        seq!(opt(choice!(seq!(args.clone(), eat_string(",")))), NAME.clone(), eat_string("="), eps()),
        seq!(args.clone(), for_if_clauses.clone()),
        seq!(args.clone(), eat_string(","), expression.clone(), for_if_clauses.clone()),
        seq!(args.clone(), eat_string(","), args.clone())));
    let invalid_kwarg = Rc::new(choice!(seq!(choice!(seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None"))), eat_string("=")),
        seq!(NAME.clone(), eat_string("="), expression.clone(), for_if_clauses.clone()),
        seq!(eps(), expression.clone(), eat_string("=")),
        seq!(eat_string("**"), expression.clone(), eat_string("="), expression.clone())));
    let expression_without_invalid = Rc::new(choice!(seq!(disjunction.clone(), eat_string("if"), disjunction.clone(), eat_string("else"), expression.clone()),
        seq!(disjunction.clone()),
        seq!(lambdef.clone())));
    let invalid_legacy_expression = Rc::new(choice!(seq!(NAME.clone(), eps(), star_expressions.clone())));
    let invalid_type_param = Rc::new(choice!(seq!(eat_string("*"), NAME.clone(), eat_string(":"), expression.clone()),
        seq!(eat_string("**"), NAME.clone(), eat_string(":"), expression.clone())));
    let invalid_expression = Rc::new(choice!(seq!(eps(), disjunction.clone(), expression_without_invalid.clone()),
        seq!(disjunction.clone(), eat_string("if"), disjunction.clone(), eps()),
        seq!(eat_string("lambda"), opt(choice!(seq!(lambda_params.clone()))), eat_string(":"), eps())));
    let invalid_named_expression = Rc::new(choice!(seq!(expression.clone(), eat_string(":="), expression.clone()),
        seq!(NAME.clone(), eat_string("="), bitwise_or.clone(), eps()),
        seq!(eps(), bitwise_or.clone(), eat_string("="), bitwise_or.clone(), eps())));
    let invalid_assignment = Rc::new(choice!(seq!(invalid_ann_assign_target.clone(), eat_string(":"), expression.clone()),
        seq!(star_named_expression.clone(), eat_string(","), repeat(star_named_expressions.clone()), eat_string(":"), expression.clone()),
        seq!(expression.clone(), eat_string(":"), expression.clone()),
        seq!(repeat(choice!(seq!(star_targets.clone(), eat_string("=")))), star_expressions.clone(), eat_string("=")),
        seq!(repeat(choice!(seq!(star_targets.clone(), eat_string("=")))), yield_expr.clone(), eat_string("=")),
        seq!(star_expressions.clone(), augassign.clone(), annotated_rhs.clone())));
    let invalid_ann_assign_target = Rc::new(choice!(seq!(list.clone()),
        seq!(tuple.clone()),
        seq!(eat_string("("), invalid_ann_assign_target.clone(), eat_string(")"))));
    let invalid_del_stmt = Rc::new(choice!(seq!(eat_string("del"), star_expressions.clone())));
    let invalid_block = Rc::new(choice!(seq!(NEWLINE.clone(), eps())));
    let invalid_comprehension = Rc::new(choice!(seq!(choice!(seq!(eat_string("[")), seq!(eat_string("(")), seq!(eat_string("{"))), starred_expression.clone(), for_if_clauses.clone()),
        seq!(choice!(seq!(eat_string("[")), seq!(eat_string("{"))), star_named_expression.clone(), eat_string(","), star_named_expressions.clone(), for_if_clauses.clone()),
        seq!(choice!(seq!(eat_string("[")), seq!(eat_string("{"))), star_named_expression.clone(), eat_string(","), for_if_clauses.clone())));
    let invalid_dict_comprehension = Rc::new(choice!(seq!(eat_string("{"), eat_string("**"), bitwise_or.clone(), for_if_clauses.clone(), eat_string("}"))));
    let invalid_parameters = Rc::new(choice!(seq!(eat_string("/"), eat_string(",")),
        seq!(choice!(seq!(slash_no_default.clone()), seq!(slash_with_default.clone())), repeat(param_maybe_default.clone()), eat_string("/")),
        seq!(opt(slash_no_default.clone()), repeat(param_no_default.clone()), invalid_parameters_helper.clone(), param_no_default.clone()),
        seq!(repeat(param_no_default.clone()), eat_string("("), repeat(param_no_default.clone()), opt(eat_string(",")), eat_string(")")),
        seq!(opt(choice!(seq!(slash_no_default.clone()), seq!(slash_with_default.clone()))), repeat(param_maybe_default.clone()), eat_string("*"), choice!(seq!(eat_string(",")), seq!(param_no_default.clone())), repeat(param_maybe_default.clone()), eat_string("/")),
        seq!(repeat(param_maybe_default.clone()), eat_string("/"), eat_string("*"))));
    let invalid_default = Rc::new(choice!(seq!(eat_string("="), eps())));
    let invalid_star_etc = Rc::new(choice!(seq!(eat_string("*"), choice!(seq!(eat_string(")")), seq!(eat_string(","), choice!(seq!(eat_string(")")), seq!(eat_string("**")))))),
        seq!(eat_string("*"), eat_string(","), TYPE_COMMENT.clone()),
        seq!(eat_string("*"), param.clone(), eat_string("=")),
        seq!(eat_string("*"), choice!(seq!(param_no_default.clone()), seq!(eat_string(","))), repeat(param_maybe_default.clone()), eat_string("*"), choice!(seq!(param_no_default.clone()), seq!(eat_string(","))))));
    let invalid_kwds = Rc::new(choice!(seq!(eat_string("**"), param.clone(), eat_string("=")),
        seq!(eat_string("**"), param.clone(), eat_string(","), param.clone()),
        seq!(eat_string("**"), param.clone(), eat_string(","), choice!(seq!(eat_string("*")), seq!(eat_string("**")), seq!(eat_string("/"))))));
    let invalid_parameters_helper = Rc::new(choice!(seq!(slash_with_default.clone()),
        seq!(repeat(param_with_default.clone()))));
    let invalid_lambda_parameters = Rc::new(choice!(seq!(eat_string("/"), eat_string(",")),
        seq!(choice!(seq!(lambda_slash_no_default.clone()), seq!(lambda_slash_with_default.clone())), repeat(lambda_param_maybe_default.clone()), eat_string("/")),
        seq!(opt(lambda_slash_no_default.clone()), repeat(lambda_param_no_default.clone()), invalid_lambda_parameters_helper.clone(), lambda_param_no_default.clone()),
        seq!(repeat(lambda_param_no_default.clone()), eat_string("("), seq!(lambda_param.clone(), eat_string(",")), opt(eat_string(",")), eat_string(")")),
        seq!(opt(choice!(seq!(lambda_slash_no_default.clone()), seq!(lambda_slash_with_default.clone()))), repeat(lambda_param_maybe_default.clone()), eat_string("*"), choice!(seq!(eat_string(",")), seq!(lambda_param_no_default.clone())), repeat(lambda_param_maybe_default.clone()), eat_string("/")),
        seq!(repeat(lambda_param_maybe_default.clone()), eat_string("/"), eat_string("*"))));
    let invalid_lambda_parameters_helper = Rc::new(choice!(seq!(lambda_slash_with_default.clone()),
        seq!(repeat(lambda_param_with_default.clone()))));
    let invalid_lambda_star_etc = Rc::new(choice!(seq!(eat_string("*"), choice!(seq!(eat_string(":")), seq!(eat_string(","), choice!(seq!(eat_string(":")), seq!(eat_string("**")))))),
        seq!(eat_string("*"), lambda_param.clone(), eat_string("=")),
        seq!(eat_string("*"), choice!(seq!(lambda_param_no_default.clone()), seq!(eat_string(","))), repeat(lambda_param_maybe_default.clone()), eat_string("*"), choice!(seq!(lambda_param_no_default.clone()), seq!(eat_string(","))))));
    let invalid_lambda_kwds = Rc::new(choice!(seq!(eat_string("**"), lambda_param.clone(), eat_string("=")),
        seq!(eat_string("**"), lambda_param.clone(), eat_string(","), lambda_param.clone()),
        seq!(eat_string("**"), lambda_param.clone(), eat_string(","), choice!(seq!(eat_string("*")), seq!(eat_string("**")), seq!(eat_string("/"))))));
    let invalid_double_type_comments = Rc::new(choice!(seq!(TYPE_COMMENT.clone(), NEWLINE.clone(), TYPE_COMMENT.clone(), NEWLINE.clone(), INDENT.clone())));
    let invalid_with_item = Rc::new(choice!(seq!(expression.clone(), eat_string("as"), expression.clone(), eps())));
    let invalid_for_if_clause = Rc::new(choice!(seq!(opt(eat_string("async")), eat_string("for"), choice!(seq!(bitwise_or.clone(), repeat(choice!(seq!(eat_string(","), bitwise_or.clone()))), opt(choice!(seq!(eat_string(",")))))), eps())));
    let invalid_for_target = Rc::new(choice!(seq!(opt(eat_string("async")), eat_string("for"), star_expressions.clone())));
    let invalid_group = Rc::new(choice!(seq!(eat_string("("), starred_expression.clone(), eat_string(")")),
        seq!(eat_string("("), eat_string("**"), expression.clone(), eat_string(")"))));
    let invalid_import = Rc::new(choice!(seq!(eat_string("import"), seq!(dotted_name.clone(), eat_string(",")), eat_string("from"), dotted_name.clone()),
        seq!(eat_string("import"), NEWLINE.clone())));
    let invalid_import_from_targets = Rc::new(choice!(seq!(import_from_as_names.clone(), eat_string(","), NEWLINE.clone()),
        seq!(NEWLINE.clone())));
    let invalid_with_stmt = Rc::new(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), seq!(choice!(seq!(expression.clone(), opt(choice!(seq!(eat_string("as"), star_target.clone()))))), eat_string(",")), NEWLINE.clone()),
        seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), eat_string("("), seq!(choice!(seq!(expressions.clone(), opt(choice!(seq!(eat_string("as"), star_target.clone()))))), eat_string(",")), opt(eat_string(",")), eat_string(")"), NEWLINE.clone())));
    let invalid_with_stmt_indent = Rc::new(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), seq!(choice!(seq!(expression.clone(), opt(choice!(seq!(eat_string("as"), star_target.clone()))))), eat_string(",")), eat_string(":"), NEWLINE.clone(), eps()),
        seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), eat_string("("), seq!(choice!(seq!(expressions.clone(), opt(choice!(seq!(eat_string("as"), star_target.clone()))))), eat_string(",")), opt(eat_string(",")), eat_string(")"), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_try_stmt = Rc::new(choice!(seq!(eat_string("try"), eat_string(":"), NEWLINE.clone(), eps()),
        seq!(eat_string("try"), eat_string(":"), block.clone(), eps()),
        seq!(eat_string("try"), eat_string(":"), repeat(block.clone()), repeat(except_block.clone()), eat_string("except"), eat_string("*"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":")),
        seq!(eat_string("try"), eat_string(":"), repeat(block.clone()), repeat(except_star_block.clone()), eat_string("except"), opt(choice!(seq!(expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone())))))), eat_string(":"))));
    let invalid_except_stmt = Rc::new(choice!(seq!(eat_string("except"), opt(eat_string("*")), expression.clone(), eat_string(","), expressions.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":")),
        seq!(eat_string("except"), opt(eat_string("*")), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), NEWLINE.clone()),
        seq!(eat_string("except"), NEWLINE.clone()),
        seq!(eat_string("except"), eat_string("*"), choice!(seq!(NEWLINE.clone()), seq!(eat_string(":"))))));
    let invalid_finally_stmt = Rc::new(choice!(seq!(eat_string("finally"), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_except_stmt_indent = Rc::new(choice!(seq!(eat_string("except"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":"), NEWLINE.clone(), eps()),
        seq!(eat_string("except"), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_except_star_stmt_indent = Rc::new(choice!(seq!(eat_string("except"), eat_string("*"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_match_stmt = Rc::new(choice!(seq!(eat_string("match"), subject_expr.clone(), NEWLINE.clone()),
        seq!(eat_string("match"), subject_expr.clone(), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_case_block = Rc::new(choice!(seq!(eat_string("case"), patterns.clone(), opt(guard.clone()), NEWLINE.clone()),
        seq!(eat_string("case"), patterns.clone(), opt(guard.clone()), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_as_pattern = Rc::new(choice!(seq!(or_pattern.clone(), eat_string("as"), eat_string("_")),
        seq!(or_pattern.clone(), eat_string("as"), eps(), expression.clone())));
    let invalid_class_pattern = Rc::new(choice!(seq!(name_or_attr.clone(), eat_string("("), invalid_class_argument_pattern.clone())));
    let invalid_class_argument_pattern = Rc::new(choice!(seq!(opt(choice!(seq!(positional_patterns.clone(), eat_string(",")))), keyword_patterns.clone(), eat_string(","), positional_patterns.clone())));
    let invalid_if_stmt = Rc::new(choice!(seq!(eat_string("if"), named_expression.clone(), NEWLINE.clone()),
        seq!(eat_string("if"), named_expression.clone(), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_elif_stmt = Rc::new(choice!(seq!(eat_string("elif"), named_expression.clone(), NEWLINE.clone()),
        seq!(eat_string("elif"), named_expression.clone(), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_else_stmt = Rc::new(choice!(seq!(eat_string("else"), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_while_stmt = Rc::new(choice!(seq!(eat_string("while"), named_expression.clone(), NEWLINE.clone()),
        seq!(eat_string("while"), named_expression.clone(), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_for_stmt = Rc::new(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("for"), star_targets.clone(), eat_string("in"), star_expressions.clone(), NEWLINE.clone()),
        seq!(opt(choice!(seq!(eat_string("async")))), eat_string("for"), star_targets.clone(), eat_string("in"), star_expressions.clone(), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_def_raw = Rc::new(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("def"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("("), opt(choice!(seq!(params.clone()))), eat_string(")"), opt(choice!(seq!(eat_string("->"), expression.clone()))), eat_string(":"), NEWLINE.clone(), eps()),
        seq!(opt(choice!(seq!(eat_string("async")))), eat_string("def"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("("), opt(choice!(seq!(params.clone()))), eat_string(")"), opt(choice!(seq!(eat_string("->"), expression.clone()))), eat_string(":"), opt(choice!(seq!(func_type_comment.clone()))), block.clone())));
    let invalid_class_def_raw = Rc::new(choice!(seq!(eat_string("class"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")")))), NEWLINE.clone()),
        seq!(eat_string("class"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")")))), eat_string(":"), NEWLINE.clone(), eps())));
    let invalid_double_starred_kvpairs = Rc::new(choice!(seq!(seq!(double_starred_kvpair.clone(), eat_string(",")), eat_string(","), invalid_kvpair.clone()),
        seq!(expression.clone(), eat_string(":"), eat_string("*"), bitwise_or.clone()),
        seq!(expression.clone(), eat_string(":"), eps())));
    let invalid_kvpair = Rc::new(choice!(seq!(expression.clone(), eps()),
        seq!(expression.clone(), eat_string(":"), eat_string("*"), bitwise_or.clone()),
        seq!(expression.clone(), eat_string(":"), eps())));
    let invalid_starred_expression_unpacking = Rc::new(choice!(seq!(eat_string("*"), expression.clone(), eat_string("="), expression.clone())));
    let invalid_starred_expression = Rc::new(choice!(seq!(eat_string("*"))));
    let invalid_replacement_field = Rc::new(choice!(seq!(eat_string("{"), eat_string("=")),
        seq!(eat_string("{"), eat_string("!")),
        seq!(eat_string("{"), eat_string(":")),
        seq!(eat_string("{"), eat_string("}")),
        seq!(eat_string("{"), eps()),
        seq!(eat_string("{"), annotated_rhs.clone(), eps()),
        seq!(eat_string("{"), annotated_rhs.clone(), eat_string("="), eps()),
        seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), invalid_conversion_character.clone()),
        seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), opt(choice!(seq!(eat_string("!"), NAME.clone()))), eps()),
        seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), opt(choice!(seq!(eat_string("!"), NAME.clone()))), eat_string(":"), repeat(fstring_format_spec.clone()), eps()),
        seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), opt(choice!(seq!(eat_string("!"), NAME.clone()))), eps())));
    let invalid_conversion_character = Rc::new(choice!(seq!(eat_string("!"), eps()),
        seq!(eat_string("!"), eps())));
    let invalid_arithmetic = Rc::new(choice!(seq!(sum.clone(), choice!(seq!(eat_string("+")), seq!(eat_string("-")), seq!(eat_string("*")), seq!(eat_string("/")), seq!(eat_string("%")), seq!(eat_string("//")), seq!(eat_string("@"))), eat_string("not"), inversion.clone())));
    let invalid_factor = Rc::new(choice!(seq!(choice!(seq!(eat_string("+")), seq!(eat_string("-")), seq!(eat_string("~"))), eat_string("not"), factor.clone())));
    let invalid_type_params = Rc::new(choice!(seq!(eat_string("["), eat_string("]"))));
    file_copy.set(file);
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
