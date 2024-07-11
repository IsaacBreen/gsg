// use std::rc::Rc;
// use crate::{choice, seq, repeat, repeat as repeat0, repeat1, opt, eat_char_choice, eat_string, eat_char_range, forward_ref, eps, python_newline, indent, dedent, DynCombinator, CombinatorTrait};
// use super::python_tokenizer::{NAME, TYPE_COMMENT, FSTRING_START, FSTRING_MIDDLE, FSTRING_END, NUMBER, STRING};
//
// pub fn python_file() -> Rc<DynCombinator> {
//     let NAME = Rc::new(NAME());
//     let TYPE_COMMENT = Rc::new(TYPE_COMMENT());
//     let FSTRING_START = Rc::new(FSTRING_START());
//     let FSTRING_MIDDLE = Rc::new(FSTRING_MIDDLE());
//     let FSTRING_END = Rc::new(FSTRING_END());
//     let NUMBER = Rc::new(NUMBER());
//     let STRING = Rc::new(STRING());
//     let NEWLINE = Rc::new(python_newline());
//     let INDENT = Rc::new(indent());
//     let DEDENT = Rc::new(dedent());
//     let ENDMARKER = eps();
//     let mut file = forward_ref();
//     let mut interactive = forward_ref();
//     let mut eval = forward_ref();
//     let mut func_type = forward_ref();
//     let mut statements = forward_ref();
//     let mut statement = forward_ref();
//     let mut statement_newline = forward_ref();
//     let mut simple_stmts = forward_ref();
//     let mut simple_stmt = forward_ref();
//     let mut compound_stmt = forward_ref();
//     let mut assignment = forward_ref();
//     let mut annotated_rhs = forward_ref();
//     let mut augassign = forward_ref();
//     let mut return_stmt = forward_ref();
//     let mut raise_stmt = forward_ref();
//     let mut global_stmt = forward_ref();
//     let mut nonlocal_stmt = forward_ref();
//     let mut del_stmt = forward_ref();
//     let mut yield_stmt = forward_ref();
//     let mut assert_stmt = forward_ref();
//     let mut import_stmt = forward_ref();
//     let mut import_name = forward_ref();
//     let mut import_from = forward_ref();
//     let mut import_from_targets = forward_ref();
//     let mut import_from_as_names = forward_ref();
//     let mut import_from_as_name = forward_ref();
//     let mut dotted_as_names = forward_ref();
//     let mut dotted_as_name = forward_ref();
//     let mut dotted_name = forward_ref();
//     let mut block = forward_ref();
//     let mut decorators = forward_ref();
//     let mut class_def = forward_ref();
//     let mut class_def_raw = forward_ref();
//     let mut function_def = forward_ref();
//     let mut function_def_raw = forward_ref();
//     let mut params = forward_ref();
//     let mut parameters = forward_ref();
//     let mut slash_no_default = forward_ref();
//     let mut slash_with_default = forward_ref();
//     let mut star_etc = forward_ref();
//     let mut kwds = forward_ref();
//     let mut param_no_default = forward_ref();
//     let mut param_no_default_star_annotation = forward_ref();
//     let mut param_with_default = forward_ref();
//     let mut param_maybe_default = forward_ref();
//     let mut param = forward_ref();
//     let mut param_star_annotation = forward_ref();
//     let mut annotation = forward_ref();
//     let mut star_annotation = forward_ref();
//     let mut default = forward_ref();
//     let mut if_stmt = forward_ref();
//     let mut elif_stmt = forward_ref();
//     let mut else_block = forward_ref();
//     let mut while_stmt = forward_ref();
//     let mut for_stmt = forward_ref();
//     let mut with_stmt = forward_ref();
//     let mut with_item = forward_ref();
//     let mut try_stmt = forward_ref();
//     let mut except_block = forward_ref();
//     let mut except_star_block = forward_ref();
//     let mut finally_block = forward_ref();
//     let mut match_stmt = forward_ref();
//     let mut subject_expr = forward_ref();
//     let mut case_block = forward_ref();
//     let mut guard = forward_ref();
//     let mut patterns = forward_ref();
//     let mut pattern = forward_ref();
//     let mut as_pattern = forward_ref();
//     let mut or_pattern = forward_ref();
//     let mut closed_pattern = forward_ref();
//     let mut literal_pattern = forward_ref();
//     let mut literal_expr = forward_ref();
//     let mut complex_number = forward_ref();
//     let mut signed_number = forward_ref();
//     let mut signed_real_number = forward_ref();
//     let mut real_number = forward_ref();
//     let mut imaginary_number = forward_ref();
//     let mut capture_pattern = forward_ref();
//     let mut pattern_capture_target = forward_ref();
//     let mut wildcard_pattern = forward_ref();
//     let mut value_pattern = forward_ref();
//     let mut attr = forward_ref();
//     let mut name_or_attr = forward_ref();
//     let mut group_pattern = forward_ref();
//     let mut sequence_pattern = forward_ref();
//     let mut open_sequence_pattern = forward_ref();
//     let mut maybe_sequence_pattern = forward_ref();
//     let mut maybe_star_pattern = forward_ref();
//     let mut star_pattern = forward_ref();
//     let mut mapping_pattern = forward_ref();
//     let mut items_pattern = forward_ref();
//     let mut key_value_pattern = forward_ref();
//     let mut double_star_pattern = forward_ref();
//     let mut class_pattern = forward_ref();
//     let mut positional_patterns = forward_ref();
//     let mut keyword_patterns = forward_ref();
//     let mut keyword_pattern = forward_ref();
//     let mut type_alias = forward_ref();
//     let mut type_params = forward_ref();
//     let mut type_param_seq = forward_ref();
//     let mut type_param = forward_ref();
//     let mut type_param_bound = forward_ref();
//     let mut type_param_default = forward_ref();
//     let mut type_param_starred_default = forward_ref();
//     let mut expressions = forward_ref();
//     let mut expression = forward_ref();
//     let mut yield_expr = forward_ref();
//     let mut star_expressions = forward_ref();
//     let mut star_expression = forward_ref();
//     let mut star_named_expressions = forward_ref();
//     let mut star_named_expression = forward_ref();
//     let mut assignment_expression = forward_ref();
//     let mut named_expression = forward_ref();
//     let mut disjunction = forward_ref();
//     let mut conjunction = forward_ref();
//     let mut inversion = forward_ref();
//     let mut comparison = forward_ref();
//     let mut compare_op_bitwise_or_pair = forward_ref();
//     let mut eq_bitwise_or = forward_ref();
//     let mut noteq_bitwise_or = forward_ref();
//     let mut lte_bitwise_or = forward_ref();
//     let mut lt_bitwise_or = forward_ref();
//     let mut gte_bitwise_or = forward_ref();
//     let mut gt_bitwise_or = forward_ref();
//     let mut notin_bitwise_or = forward_ref();
//     let mut in_bitwise_or = forward_ref();
//     let mut isnot_bitwise_or = forward_ref();
//     let mut is_bitwise_or = forward_ref();
//     let mut bitwise_or = forward_ref();
//     let mut bitwise_xor = forward_ref();
//     let mut bitwise_and = forward_ref();
//     let mut shift_expr = forward_ref();
//     let mut sum = forward_ref();
//     let mut term = forward_ref();
//     let mut factor = forward_ref();
//     let mut power = forward_ref();
//     let mut await_primary = forward_ref();
//     let mut primary = forward_ref();
//     let mut slices = forward_ref();
//     let mut slice = forward_ref();
//     let mut atom = forward_ref();
//     let mut group = forward_ref();
//     let mut lambdef = forward_ref();
//     let mut lambda_params = forward_ref();
//     let mut lambda_parameters = forward_ref();
//     let mut lambda_slash_no_default = forward_ref();
//     let mut lambda_slash_with_default = forward_ref();
//     let mut lambda_star_etc = forward_ref();
//     let mut lambda_kwds = forward_ref();
//     let mut lambda_param_no_default = forward_ref();
//     let mut lambda_param_with_default = forward_ref();
//     let mut lambda_param_maybe_default = forward_ref();
//     let mut lambda_param = forward_ref();
//     let mut fstring_middle = forward_ref();
//     let mut fstring_replacement_field = forward_ref();
//     let mut fstring_conversion = forward_ref();
//     let mut fstring_full_format_spec = forward_ref();
//     let mut fstring_format_spec = forward_ref();
//     let mut fstring = forward_ref();
//     let mut string = forward_ref();
//     let mut strings = forward_ref();
//     let mut list = forward_ref();
//     let mut tuple = forward_ref();
//     let mut set = forward_ref();
//     let mut dict = forward_ref();
//     let mut double_starred_kvpairs = forward_ref();
//     let mut double_starred_kvpair = forward_ref();
//     let mut kvpair = forward_ref();
//     let mut for_if_clauses = forward_ref();
//     let mut for_if_clause = forward_ref();
//     let mut listcomp = forward_ref();
//     let mut setcomp = forward_ref();
//     let mut genexp = forward_ref();
//     let mut dictcomp = forward_ref();
//     let mut arguments = forward_ref();
//     let mut args = forward_ref();
//     let mut kwargs = forward_ref();
//     let mut starred_expression = forward_ref();
//     let mut kwarg_or_starred = forward_ref();
//     let mut kwarg_or_double_starred = forward_ref();
//     let mut star_targets = forward_ref();
//     let mut star_targets_list_seq = forward_ref();
//     let mut star_targets_tuple_seq = forward_ref();
//     let mut star_target = forward_ref();
//     let mut target_with_star_atom = forward_ref();
//     let mut star_atom = forward_ref();
//     let mut single_target = forward_ref();
//     let mut single_subscript_attribute_target = forward_ref();
//     let mut t_primary = forward_ref();
//     let mut t_lookahead = forward_ref();
//     let mut del_targets = forward_ref();
//     let mut del_target = forward_ref();
//     let mut del_t_atom = forward_ref();
//     let mut type_expressions = forward_ref();
//     let mut func_type_comment = forward_ref();
//     let mut invalid_arguments = forward_ref();
//     let mut invalid_kwarg = forward_ref();
//     let mut expression_without_invalid = forward_ref();
//     let mut invalid_legacy_expression = forward_ref();
//     let mut invalid_type_param = forward_ref();
//     let mut invalid_expression = forward_ref();
//     let mut invalid_named_expression = forward_ref();
//     let mut invalid_assignment = forward_ref();
//     let mut invalid_ann_assign_target = forward_ref();
//     let mut invalid_del_stmt = forward_ref();
//     let mut invalid_block = forward_ref();
//     let mut invalid_comprehension = forward_ref();
//     let mut invalid_dict_comprehension = forward_ref();
//     let mut invalid_parameters = forward_ref();
//     let mut invalid_default = forward_ref();
//     let mut invalid_star_etc = forward_ref();
//     let mut invalid_kwds = forward_ref();
//     let mut invalid_parameters_helper = forward_ref();
//     let mut invalid_lambda_parameters = forward_ref();
//     let mut invalid_lambda_parameters_helper = forward_ref();
//     let mut invalid_lambda_star_etc = forward_ref();
//     let mut invalid_lambda_kwds = forward_ref();
//     let mut invalid_double_type_comments = forward_ref();
//     let mut invalid_with_item = forward_ref();
//     let mut invalid_for_if_clause = forward_ref();
//     let mut invalid_for_target = forward_ref();
//     let mut invalid_group = forward_ref();
//     let mut invalid_import = forward_ref();
//     let mut invalid_import_from_targets = forward_ref();
//     let mut invalid_with_stmt = forward_ref();
//     let mut invalid_with_stmt_indent = forward_ref();
//     let mut invalid_try_stmt = forward_ref();
//     let mut invalid_except_stmt = forward_ref();
//     let mut invalid_finally_stmt = forward_ref();
//     let mut invalid_except_stmt_indent = forward_ref();
//     let mut invalid_except_star_stmt_indent = forward_ref();
//     let mut invalid_match_stmt = forward_ref();
//     let mut invalid_case_block = forward_ref();
//     let mut invalid_as_pattern = forward_ref();
//     let mut invalid_class_pattern = forward_ref();
//     let mut invalid_class_argument_pattern = forward_ref();
//     let mut invalid_if_stmt = forward_ref();
//     let mut invalid_elif_stmt = forward_ref();
//     let mut invalid_else_stmt = forward_ref();
//     let mut invalid_while_stmt = forward_ref();
//     let mut invalid_for_stmt = forward_ref();
//     let mut invalid_def_raw = forward_ref();
//     let mut invalid_class_def_raw = forward_ref();
//     let mut invalid_double_starred_kvpairs = forward_ref();
//     let mut invalid_kvpair = forward_ref();
//     let mut invalid_starred_expression_unpacking = forward_ref();
//     let mut invalid_starred_expression = forward_ref();
//     let mut invalid_replacement_field = forward_ref();
//     let mut invalid_conversion_character = forward_ref();
//     let mut invalid_arithmetic = forward_ref();
//     let mut invalid_factor = forward_ref();
//     let mut invalid_type_params = forward_ref();
//     let file = file.set(choice!(seq!(opt(choice!(seq!(statements.clone()))), ENDMARKER.clone())));
//     let interactive = interactive.set(choice!(seq!(statement_newline.clone())));
//     let eval = eval.set(choice!(seq!(expressions.clone(), repeat(NEWLINE.clone()), ENDMARKER.clone())));
//     let func_type = func_type.set(choice!(seq!(eat_string("("), opt(choice!(seq!(type_expressions.clone()))), eat_string(")"), eat_string("->"), expression.clone(), repeat(NEWLINE.clone()), ENDMARKER.clone())));
//     let statements = statements.set(choice!(seq!(repeat(statement.clone()))));
//     let statement = statement.set(choice!(seq!(compound_stmt.clone()),
//         seq!(simple_stmts.clone())));
//     let statement_newline = statement_newline.set(choice!(seq!(compound_stmt.clone(), NEWLINE.clone()),
//         seq!(simple_stmts.clone()),
//         seq!(NEWLINE.clone()),
//         seq!(ENDMARKER.clone())));
//     let simple_stmts = simple_stmts.set(choice!(seq!(simple_stmt.clone(), eps(), NEWLINE.clone()),
//         seq!(seq!(simple_stmt.clone(), eat_string(";")), opt(choice!(seq!(eat_string(";")))), NEWLINE.clone())));
//     let simple_stmt = simple_stmt.set(choice!(seq!(assignment.clone()),
//         seq!(eps(), type_alias.clone()),
//         seq!(star_expressions.clone()),
//         seq!(eps(), return_stmt.clone()),
//         seq!(eps(), import_stmt.clone()),
//         seq!(eps(), raise_stmt.clone()),
//         seq!(eat_string("pass")),
//         seq!(eps(), del_stmt.clone()),
//         seq!(eps(), yield_stmt.clone()),
//         seq!(eps(), assert_stmt.clone()),
//         seq!(eat_string("break")),
//         seq!(eat_string("continue")),
//         seq!(eps(), global_stmt.clone()),
//         seq!(eps(), nonlocal_stmt.clone())));
//     let compound_stmt = compound_stmt.set(choice!(seq!(eps(), function_def.clone()),
//         seq!(eps(), if_stmt.clone()),
//         seq!(eps(), class_def.clone()),
//         seq!(eps(), with_stmt.clone()),
//         seq!(eps(), for_stmt.clone()),
//         seq!(eps(), try_stmt.clone()),
//         seq!(eps(), while_stmt.clone()),
//         seq!(match_stmt.clone())));
//     let assignment = assignment.set(choice!(seq!(NAME.clone(), eat_string(":"), expression.clone(), opt(choice!(seq!(eat_string("="), annotated_rhs.clone())))),
//         seq!(choice!(seq!(eat_string("("), single_target.clone(), eat_string(")")), seq!(single_subscript_attribute_target.clone())), eat_string(":"), expression.clone(), opt(choice!(seq!(eat_string("="), annotated_rhs.clone())))),
//         seq!(repeat(choice!(seq!(star_targets.clone(), eat_string("=")))), choice!(seq!(yield_expr.clone()), seq!(star_expressions.clone())), eps(), opt(choice!(seq!(TYPE_COMMENT.clone())))),
//         seq!(single_target.clone(), augassign.clone(), eps(), choice!(seq!(yield_expr.clone()), seq!(star_expressions.clone()))),
//         seq!(invalid_assignment.clone())));
//     let annotated_rhs = annotated_rhs.set(choice!(seq!(yield_expr.clone()),
//         seq!(star_expressions.clone())));
//     let augassign = augassign.set(choice!(seq!(eat_string("+=")),
//         seq!(eat_string("-=")),
//         seq!(eat_string("*=")),
//         seq!(eat_string("@=")),
//         seq!(eat_string("/=")),
//         seq!(eat_string("%=")),
//         seq!(eat_string("&=")),
//         seq!(eat_string("|=")),
//         seq!(eat_string("^=")),
//         seq!(eat_string("<<=")),
//         seq!(eat_string(">>=")),
//         seq!(eat_string("**=")),
//         seq!(eat_string("//="))));
//     let return_stmt = return_stmt.set(choice!(seq!(eat_string("return"), opt(choice!(seq!(star_expressions.clone()))))));
//     let raise_stmt = raise_stmt.set(choice!(seq!(eat_string("raise"), expression.clone(), opt(choice!(seq!(eat_string("from"), expression.clone())))),
//         seq!(eat_string("raise"))));
//     let global_stmt = global_stmt.set(choice!(seq!(eat_string("global"), seq!(NAME.clone(), eat_string(",")))));
//     let nonlocal_stmt = nonlocal_stmt.set(choice!(seq!(eat_string("nonlocal"), seq!(NAME.clone(), eat_string(",")))));
//     let del_stmt = del_stmt.set(choice!(seq!(eat_string("del"), del_targets.clone(), eps()),
//         seq!(invalid_del_stmt.clone())));
//     let yield_stmt = yield_stmt.set(choice!(seq!(yield_expr.clone())));
//     let assert_stmt = assert_stmt.set(choice!(seq!(eat_string("assert"), expression.clone(), opt(choice!(seq!(eat_string(","), expression.clone()))))));
//     let import_stmt = import_stmt.set(choice!(seq!(invalid_import.clone()),
//         seq!(import_name.clone()),
//         seq!(import_from.clone())));
//     let import_name = import_name.set(choice!(seq!(eat_string("import"), dotted_as_names.clone())));
//     let import_from = import_from.set(choice!(seq!(eat_string("from"), repeat(choice!(seq!(eat_string(".")), seq!(eat_string("...")))), dotted_name.clone(), eat_string("import"), import_from_targets.clone()),
//         seq!(eat_string("from"), repeat(choice!(seq!(eat_string(".")), seq!(eat_string("...")))), eat_string("import"), import_from_targets.clone())));
//     let import_from_targets = import_from_targets.set(choice!(seq!(eat_string("("), import_from_as_names.clone(), opt(choice!(seq!(eat_string(",")))), eat_string(")")),
//         seq!(import_from_as_names.clone(), eps()),
//         seq!(eat_string("*")),
//         seq!(invalid_import_from_targets.clone())));
//     let import_from_as_names = import_from_as_names.set(choice!(seq!(seq!(import_from_as_name.clone(), eat_string(",")))));
//     let import_from_as_name = import_from_as_name.set(choice!(seq!(NAME.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))))));
//     let dotted_as_names = dotted_as_names.set(choice!(seq!(seq!(dotted_as_name.clone(), eat_string(",")))));
//     let dotted_as_name = dotted_as_name.set(choice!(seq!(dotted_name.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))))));
//     let dotted_name = dotted_name.set(choice!(seq!(dotted_name.clone(), eat_string("."), NAME.clone()),
//         seq!(NAME.clone())));
//     let block = block.set(choice!(seq!(NEWLINE.clone(), INDENT.clone(), statements.clone(), DEDENT.clone()),
//         seq!(simple_stmts.clone()),
//         seq!(invalid_block.clone())));
//     let decorators = decorators.set(choice!(seq!(repeat(choice!(seq!(eat_string("@"), named_expression.clone(), NEWLINE.clone()))))));
//     let class_def = class_def.set(choice!(seq!(decorators.clone(), class_def_raw.clone()),
//         seq!(class_def_raw.clone())));
//     let class_def_raw = class_def_raw.set(choice!(seq!(invalid_class_def_raw.clone()),
//         seq!(eat_string("class"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")")))), eat_string(":"), block.clone())));
//     let function_def = function_def.set(choice!(seq!(decorators.clone(), function_def_raw.clone()),
//         seq!(function_def_raw.clone())));
//     let function_def_raw = function_def_raw.set(choice!(seq!(invalid_def_raw.clone()),
//         seq!(eat_string("def"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("("), opt(choice!(seq!(params.clone()))), eat_string(")"), opt(choice!(seq!(eat_string("->"), expression.clone()))), eat_string(":"), opt(choice!(seq!(func_type_comment.clone()))), block.clone()),
//         seq!(eat_string("async"), eat_string("def"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("("), opt(choice!(seq!(params.clone()))), eat_string(")"), opt(choice!(seq!(eat_string("->"), expression.clone()))), eat_string(":"), opt(choice!(seq!(func_type_comment.clone()))), block.clone())));
//     let params = params.set(choice!(seq!(invalid_parameters.clone()),
//         seq!(parameters.clone())));
//     let parameters = parameters.set(choice!(seq!(slash_no_default.clone(), repeat(param_no_default.clone()), repeat(param_with_default.clone()), opt(choice!(seq!(star_etc.clone())))),
//         seq!(slash_with_default.clone(), repeat(param_with_default.clone()), opt(choice!(seq!(star_etc.clone())))),
//         seq!(repeat(param_no_default.clone()), repeat(param_with_default.clone()), opt(choice!(seq!(star_etc.clone())))),
//         seq!(repeat(param_with_default.clone()), opt(choice!(seq!(star_etc.clone())))),
//         seq!(star_etc.clone())));
//     let slash_no_default = slash_no_default.set(choice!(seq!(repeat(param_no_default.clone()), eat_string("/"), eat_string(",")),
//         seq!(repeat(param_no_default.clone()), eat_string("/"), eps())));
//     let slash_with_default = slash_with_default.set(choice!(seq!(repeat(param_no_default.clone()), repeat(param_with_default.clone()), eat_string("/"), eat_string(",")),
//         seq!(repeat(param_no_default.clone()), repeat(param_with_default.clone()), eat_string("/"), eps())));
//     let star_etc = star_etc.set(choice!(seq!(invalid_star_etc.clone()),
//         seq!(eat_string("*"), param_no_default.clone(), repeat(param_maybe_default.clone()), opt(choice!(seq!(kwds.clone())))),
//         seq!(eat_string("*"), param_no_default_star_annotation.clone(), repeat(param_maybe_default.clone()), opt(choice!(seq!(kwds.clone())))),
//         seq!(eat_string("*"), eat_string(","), repeat(param_maybe_default.clone()), opt(choice!(seq!(kwds.clone())))),
//         seq!(kwds.clone())));
//     let kwds = kwds.set(choice!(seq!(invalid_kwds.clone()),
//         seq!(eat_string("**"), param_no_default.clone())));
//     let param_no_default = param_no_default.set(choice!(seq!(param.clone(), eat_string(","), opt(TYPE_COMMENT.clone())),
//         seq!(param.clone(), opt(TYPE_COMMENT.clone()), eps())));
//     let param_no_default_star_annotation = param_no_default_star_annotation.set(choice!(seq!(param_star_annotation.clone(), eat_string(","), opt(TYPE_COMMENT.clone())),
//         seq!(param_star_annotation.clone(), opt(TYPE_COMMENT.clone()), eps())));
//     let param_with_default = param_with_default.set(choice!(seq!(param.clone(), default.clone(), eat_string(","), opt(TYPE_COMMENT.clone())),
//         seq!(param.clone(), default.clone(), opt(TYPE_COMMENT.clone()), eps())));
//     let param_maybe_default = param_maybe_default.set(choice!(seq!(param.clone(), opt(default.clone()), eat_string(","), opt(TYPE_COMMENT.clone())),
//         seq!(param.clone(), opt(default.clone()), opt(TYPE_COMMENT.clone()), eps())));
//     let param = param.set(choice!(seq!(NAME.clone(), opt(annotation.clone()))));
//     let param_star_annotation = param_star_annotation.set(choice!(seq!(NAME.clone(), star_annotation.clone())));
//     let annotation = annotation.set(choice!(seq!(eat_string(":"), expression.clone())));
//     let star_annotation = star_annotation.set(choice!(seq!(eat_string(":"), star_expression.clone())));
//     let default = default.set(choice!(seq!(eat_string("="), expression.clone()),
//         seq!(invalid_default.clone())));
//     let if_stmt = if_stmt.set(choice!(seq!(invalid_if_stmt.clone()),
//         seq!(eat_string("if"), named_expression.clone(), eat_string(":"), block.clone(), elif_stmt.clone()),
//         seq!(eat_string("if"), named_expression.clone(), eat_string(":"), block.clone(), opt(choice!(seq!(else_block.clone()))))));
//     let elif_stmt = elif_stmt.set(choice!(seq!(invalid_elif_stmt.clone()),
//         seq!(eat_string("elif"), named_expression.clone(), eat_string(":"), block.clone(), elif_stmt.clone()),
//         seq!(eat_string("elif"), named_expression.clone(), eat_string(":"), block.clone(), opt(choice!(seq!(else_block.clone()))))));
//     let else_block = else_block.set(choice!(seq!(invalid_else_stmt.clone()),
//         seq!(eat_string("else"), eat_string(":"), block.clone())));
//     let while_stmt = while_stmt.set(choice!(seq!(invalid_while_stmt.clone()),
//         seq!(eat_string("while"), named_expression.clone(), eat_string(":"), block.clone(), opt(choice!(seq!(else_block.clone()))))));
//     let for_stmt = for_stmt.set(choice!(seq!(invalid_for_stmt.clone()),
//         seq!(eat_string("for"), star_targets.clone(), eat_string("in"), eps(), star_expressions.clone(), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone(), opt(choice!(seq!(else_block.clone())))),
//         seq!(eat_string("async"), eat_string("for"), star_targets.clone(), eat_string("in"), eps(), star_expressions.clone(), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone(), opt(choice!(seq!(else_block.clone())))),
//         seq!(invalid_for_target.clone())));
//     let with_stmt = with_stmt.set(choice!(seq!(invalid_with_stmt_indent.clone()),
//         seq!(eat_string("with"), eat_string("("), seq!(with_item.clone(), eat_string(",")), opt(eat_string(",")), eat_string(")"), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone()),
//         seq!(eat_string("with"), seq!(with_item.clone(), eat_string(",")), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone()),
//         seq!(eat_string("async"), eat_string("with"), eat_string("("), seq!(with_item.clone(), eat_string(",")), opt(eat_string(",")), eat_string(")"), eat_string(":"), block.clone()),
//         seq!(eat_string("async"), eat_string("with"), seq!(with_item.clone(), eat_string(",")), eat_string(":"), opt(choice!(seq!(TYPE_COMMENT.clone()))), block.clone()),
//         seq!(invalid_with_stmt.clone())));
//     let with_item = with_item.set(choice!(seq!(expression.clone(), eat_string("as"), star_target.clone(), eps()),
//         seq!(invalid_with_item.clone()),
//         seq!(expression.clone())));
//     let try_stmt = try_stmt.set(choice!(seq!(invalid_try_stmt.clone()),
//         seq!(eat_string("try"), eat_string(":"), block.clone(), finally_block.clone()),
//         seq!(eat_string("try"), eat_string(":"), block.clone(), repeat(except_block.clone()), opt(choice!(seq!(else_block.clone()))), opt(choice!(seq!(finally_block.clone())))),
//         seq!(eat_string("try"), eat_string(":"), block.clone(), repeat(except_star_block.clone()), opt(choice!(seq!(else_block.clone()))), opt(choice!(seq!(finally_block.clone()))))));
//     let except_block = except_block.set(choice!(seq!(invalid_except_stmt_indent.clone()),
//         seq!(eat_string("except"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":"), block.clone()),
//         seq!(eat_string("except"), eat_string(":"), block.clone()),
//         seq!(invalid_except_stmt.clone())));
//     let except_star_block = except_star_block.set(choice!(seq!(invalid_except_star_stmt_indent.clone()),
//         seq!(eat_string("except"), eat_string("*"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":"), block.clone()),
//         seq!(invalid_except_stmt.clone())));
//     let finally_block = finally_block.set(choice!(seq!(invalid_finally_stmt.clone()),
//         seq!(eat_string("finally"), eat_string(":"), block.clone())));
//     let match_stmt = match_stmt.set(choice!(seq!(eat_string("match"), subject_expr.clone(), eat_string(":"), NEWLINE.clone(), INDENT.clone(), repeat(case_block.clone()), DEDENT.clone()),
//         seq!(invalid_match_stmt.clone())));
//     let subject_expr = subject_expr.set(choice!(seq!(star_named_expression.clone(), eat_string(","), opt(star_named_expressions.clone())),
//         seq!(named_expression.clone())));
//     let case_block = case_block.set(choice!(seq!(invalid_case_block.clone()),
//         seq!(eat_string("case"), patterns.clone(), opt(guard.clone()), eat_string(":"), block.clone())));
//     let guard = guard.set(choice!(seq!(eat_string("if"), named_expression.clone())));
//     let patterns = patterns.set(choice!(seq!(open_sequence_pattern.clone()),
//         seq!(pattern.clone())));
//     let pattern = pattern.set(choice!(seq!(as_pattern.clone()),
//         seq!(or_pattern.clone())));
//     let as_pattern = as_pattern.set(choice!(seq!(or_pattern.clone(), eat_string("as"), pattern_capture_target.clone()),
//         seq!(invalid_as_pattern.clone())));
//     let or_pattern = or_pattern.set(choice!(seq!(seq!(closed_pattern.clone(), eat_string("|")))));
//     let closed_pattern = closed_pattern.set(choice!(seq!(literal_pattern.clone()),
//         seq!(capture_pattern.clone()),
//         seq!(wildcard_pattern.clone()),
//         seq!(value_pattern.clone()),
//         seq!(group_pattern.clone()),
//         seq!(sequence_pattern.clone()),
//         seq!(mapping_pattern.clone()),
//         seq!(class_pattern.clone())));
//     let literal_pattern = literal_pattern.set(choice!(seq!(signed_number.clone(), eps()),
//         seq!(complex_number.clone()),
//         seq!(strings.clone()),
//         seq!(eat_string("None")),
//         seq!(eat_string("True")),
//         seq!(eat_string("False"))));
//     let literal_expr = literal_expr.set(choice!(seq!(signed_number.clone(), eps()),
//         seq!(complex_number.clone()),
//         seq!(strings.clone()),
//         seq!(eat_string("None")),
//         seq!(eat_string("True")),
//         seq!(eat_string("False"))));
//     let complex_number = complex_number.set(choice!(seq!(signed_real_number.clone(), eat_string("+"), imaginary_number.clone()),
//         seq!(signed_real_number.clone(), eat_string("-"), imaginary_number.clone())));
//     let signed_number = signed_number.set(choice!(seq!(NUMBER.clone()),
//         seq!(eat_string("-"), NUMBER.clone())));
//     let signed_real_number = signed_real_number.set(choice!(seq!(real_number.clone()),
//         seq!(eat_string("-"), real_number.clone())));
//     let real_number = real_number.set(choice!(seq!(NUMBER.clone())));
//     let imaginary_number = imaginary_number.set(choice!(seq!(NUMBER.clone())));
//     let capture_pattern = capture_pattern.set(choice!(seq!(pattern_capture_target.clone())));
//     let pattern_capture_target = pattern_capture_target.set(choice!(seq!(eps(), NAME.clone(), eps())));
//     let wildcard_pattern = wildcard_pattern.set(choice!(seq!(eat_string("_"))));
//     let value_pattern = value_pattern.set(choice!(seq!(attr.clone(), eps())));
//     let attr = attr.set(choice!(seq!(name_or_attr.clone(), eat_string("."), NAME.clone())));
//     let name_or_attr = name_or_attr.set(choice!(seq!(attr.clone()),
//         seq!(NAME.clone())));
//     let group_pattern = group_pattern.set(choice!(seq!(eat_string("("), pattern.clone(), eat_string(")"))));
//     let sequence_pattern = sequence_pattern.set(choice!(seq!(eat_string("["), opt(maybe_sequence_pattern.clone()), eat_string("]")),
//         seq!(eat_string("("), opt(open_sequence_pattern.clone()), eat_string(")"))));
//     let open_sequence_pattern = open_sequence_pattern.set(choice!(seq!(maybe_star_pattern.clone(), eat_string(","), opt(maybe_sequence_pattern.clone()))));
//     let maybe_sequence_pattern = maybe_sequence_pattern.set(choice!(seq!(seq!(maybe_star_pattern.clone(), eat_string(",")), opt(eat_string(",")))));
//     let maybe_star_pattern = maybe_star_pattern.set(choice!(seq!(star_pattern.clone()),
//         seq!(pattern.clone())));
//     let star_pattern = star_pattern.set(choice!(seq!(eat_string("*"), pattern_capture_target.clone()),
//         seq!(eat_string("*"), wildcard_pattern.clone())));
//     let mapping_pattern = mapping_pattern.set(choice!(seq!(eat_string("{"), eat_string("}")),
//         seq!(eat_string("{"), double_star_pattern.clone(), opt(eat_string(",")), eat_string("}")),
//         seq!(eat_string("{"), items_pattern.clone(), eat_string(","), double_star_pattern.clone(), opt(eat_string(",")), eat_string("}")),
//         seq!(eat_string("{"), items_pattern.clone(), opt(eat_string(",")), eat_string("}"))));
//     let items_pattern = items_pattern.set(choice!(seq!(seq!(key_value_pattern.clone(), eat_string(",")))));
//     let key_value_pattern = key_value_pattern.set(choice!(seq!(choice!(seq!(literal_expr.clone()), seq!(attr.clone())), eat_string(":"), pattern.clone())));
//     let double_star_pattern = double_star_pattern.set(choice!(seq!(eat_string("**"), pattern_capture_target.clone())));
//     let class_pattern = class_pattern.set(choice!(seq!(name_or_attr.clone(), eat_string("("), eat_string(")")),
//         seq!(name_or_attr.clone(), eat_string("("), positional_patterns.clone(), opt(eat_string(",")), eat_string(")")),
//         seq!(name_or_attr.clone(), eat_string("("), keyword_patterns.clone(), opt(eat_string(",")), eat_string(")")),
//         seq!(name_or_attr.clone(), eat_string("("), positional_patterns.clone(), eat_string(","), keyword_patterns.clone(), opt(eat_string(",")), eat_string(")")),
//         seq!(invalid_class_pattern.clone())));
//     let positional_patterns = positional_patterns.set(choice!(seq!(seq!(pattern.clone(), eat_string(",")))));
//     let keyword_patterns = keyword_patterns.set(choice!(seq!(seq!(keyword_pattern.clone(), eat_string(",")))));
//     let keyword_pattern = keyword_pattern.set(choice!(seq!(NAME.clone(), eat_string("="), pattern.clone())));
//     let type_alias = type_alias.set(choice!(seq!(eat_string("type"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("="), expression.clone())));
//     let type_params = type_params.set(choice!(seq!(invalid_type_params.clone()),
//         seq!(eat_string("["), type_param_seq.clone(), eat_string("]"))));
//     let type_param_seq = type_param_seq.set(choice!(seq!(seq!(type_param.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
//     let type_param = type_param.set(choice!(seq!(NAME.clone(), opt(choice!(seq!(type_param_bound.clone()))), opt(choice!(seq!(type_param_default.clone())))),
//         seq!(invalid_type_param.clone()),
//         seq!(eat_string("*"), NAME.clone(), opt(choice!(seq!(type_param_starred_default.clone())))),
//         seq!(eat_string("**"), NAME.clone(), opt(choice!(seq!(type_param_default.clone()))))));
//     let type_param_bound = type_param_bound.set(choice!(seq!(eat_string(":"), expression.clone())));
//     let type_param_default = type_param_default.set(choice!(seq!(eat_string("="), expression.clone())));
//     let type_param_starred_default = type_param_starred_default.set(choice!(seq!(eat_string("="), star_expression.clone())));
//     let expressions = expressions.set(choice!(seq!(expression.clone(), repeat(choice!(seq!(eat_string(","), expression.clone()))), opt(choice!(seq!(eat_string(","))))),
//         seq!(expression.clone(), eat_string(",")),
//         seq!(expression.clone())));
//     let expression = expression.set(choice!(seq!(invalid_expression.clone()),
//         seq!(invalid_legacy_expression.clone()),
//         seq!(disjunction.clone(), eat_string("if"), disjunction.clone(), eat_string("else"), expression.clone()),
//         seq!(disjunction.clone()),
//         seq!(lambdef.clone())));
//     let yield_expr = yield_expr.set(choice!(seq!(eat_string("yield"), eat_string("from"), expression.clone()),
//         seq!(eat_string("yield"), opt(choice!(seq!(star_expressions.clone()))))));
//     let star_expressions = star_expressions.set(choice!(seq!(star_expression.clone(), repeat(choice!(seq!(eat_string(","), star_expression.clone()))), opt(choice!(seq!(eat_string(","))))),
//         seq!(star_expression.clone(), eat_string(",")),
//         seq!(star_expression.clone())));
//     let star_expression = star_expression.set(choice!(seq!(eat_string("*"), bitwise_or.clone()),
//         seq!(expression.clone())));
//     let star_named_expressions = star_named_expressions.set(choice!(seq!(seq!(star_named_expression.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
//     let star_named_expression = star_named_expression.set(choice!(seq!(eat_string("*"), bitwise_or.clone()),
//         seq!(named_expression.clone())));
//     let assignment_expression = assignment_expression.set(choice!(seq!(NAME.clone(), eat_string(":="), eps(), expression.clone())));
//     let named_expression = named_expression.set(choice!(seq!(assignment_expression.clone()),
//         seq!(invalid_named_expression.clone()),
//         seq!(expression.clone(), eps())));
//     let disjunction = disjunction.set(choice!(seq!(conjunction.clone(), repeat(choice!(seq!(eat_string("or"), conjunction.clone())))),
//         seq!(conjunction.clone())));
//     let conjunction = conjunction.set(choice!(seq!(inversion.clone(), repeat(choice!(seq!(eat_string("and"), inversion.clone())))),
//         seq!(inversion.clone())));
//     let inversion = inversion.set(choice!(seq!(eat_string("not"), inversion.clone()),
//         seq!(comparison.clone())));
//     let comparison = comparison.set(choice!(seq!(bitwise_or.clone(), repeat(compare_op_bitwise_or_pair.clone())),
//         seq!(bitwise_or.clone())));
//     let compare_op_bitwise_or_pair = compare_op_bitwise_or_pair.set(choice!(seq!(eq_bitwise_or.clone()),
//         seq!(noteq_bitwise_or.clone()),
//         seq!(lte_bitwise_or.clone()),
//         seq!(lt_bitwise_or.clone()),
//         seq!(gte_bitwise_or.clone()),
//         seq!(gt_bitwise_or.clone()),
//         seq!(notin_bitwise_or.clone()),
//         seq!(in_bitwise_or.clone()),
//         seq!(isnot_bitwise_or.clone()),
//         seq!(is_bitwise_or.clone())));
//     let eq_bitwise_or = eq_bitwise_or.set(choice!(seq!(eat_string("=="), bitwise_or.clone())));
//     let noteq_bitwise_or = noteq_bitwise_or.set(choice!(seq!(choice!(seq!(eat_string("!="))), bitwise_or.clone())));
//     let lte_bitwise_or = lte_bitwise_or.set(choice!(seq!(eat_string("<="), bitwise_or.clone())));
//     let lt_bitwise_or = lt_bitwise_or.set(choice!(seq!(eat_string("<"), bitwise_or.clone())));
//     let gte_bitwise_or = gte_bitwise_or.set(choice!(seq!(eat_string(">="), bitwise_or.clone())));
//     let gt_bitwise_or = gt_bitwise_or.set(choice!(seq!(eat_string(">"), bitwise_or.clone())));
//     let notin_bitwise_or = notin_bitwise_or.set(choice!(seq!(eat_string("not"), eat_string("in"), bitwise_or.clone())));
//     let in_bitwise_or = in_bitwise_or.set(choice!(seq!(eat_string("in"), bitwise_or.clone())));
//     let isnot_bitwise_or = isnot_bitwise_or.set(choice!(seq!(eat_string("is"), eat_string("not"), bitwise_or.clone())));
//     let is_bitwise_or = is_bitwise_or.set(choice!(seq!(eat_string("is"), bitwise_or.clone())));
//     let bitwise_or = bitwise_or.set(choice!(seq!(bitwise_or.clone(), eat_string("|"), bitwise_xor.clone()),
//         seq!(bitwise_xor.clone())));
//     let bitwise_xor = bitwise_xor.set(choice!(seq!(bitwise_xor.clone(), eat_string("^"), bitwise_and.clone()),
//         seq!(bitwise_and.clone())));
//     let bitwise_and = bitwise_and.set(choice!(seq!(bitwise_and.clone(), eat_string("&"), shift_expr.clone()),
//         seq!(shift_expr.clone())));
//     let shift_expr = shift_expr.set(choice!(seq!(shift_expr.clone(), eat_string("<<"), sum.clone()),
//         seq!(shift_expr.clone(), eat_string(">>"), sum.clone()),
//         seq!(invalid_arithmetic.clone()),
//         seq!(sum.clone())));
//     let sum = sum.set(choice!(seq!(sum.clone(), eat_string("+"), term.clone()),
//         seq!(sum.clone(), eat_string("-"), term.clone()),
//         seq!(term.clone())));
//     let term = term.set(choice!(seq!(term.clone(), eat_string("*"), factor.clone()),
//         seq!(term.clone(), eat_string("/"), factor.clone()),
//         seq!(term.clone(), eat_string("//"), factor.clone()),
//         seq!(term.clone(), eat_string("%"), factor.clone()),
//         seq!(term.clone(), eat_string("@"), factor.clone()),
//         seq!(invalid_factor.clone()),
//         seq!(factor.clone())));
//     let factor = factor.set(choice!(seq!(eat_string("+"), factor.clone()),
//         seq!(eat_string("-"), factor.clone()),
//         seq!(eat_string("~"), factor.clone()),
//         seq!(power.clone())));
//     let power = power.set(choice!(seq!(await_primary.clone(), eat_string("**"), factor.clone()),
//         seq!(await_primary.clone())));
//     let await_primary = await_primary.set(choice!(seq!(eat_string("await"), primary.clone()),
//         seq!(primary.clone())));
//     let primary = primary.set(choice!(seq!(primary.clone(), eat_string("."), NAME.clone()),
//         seq!(primary.clone(), genexp.clone()),
//         seq!(primary.clone(), eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")")),
//         seq!(primary.clone(), eat_string("["), slices.clone(), eat_string("]")),
//         seq!(atom.clone())));
//     let slices = slices.set(choice!(seq!(slice.clone(), eps()),
//         seq!(seq!(choice!(seq!(slice.clone()), seq!(starred_expression.clone())), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
//     let slice = slice.set(choice!(seq!(opt(choice!(seq!(expression.clone()))), eat_string(":"), opt(choice!(seq!(expression.clone()))), opt(choice!(seq!(eat_string(":"), opt(choice!(seq!(expression.clone()))))))),
//         seq!(named_expression.clone())));
//     let atom = atom.set(choice!(seq!(NAME.clone()),
//         seq!(eat_string("True")),
//         seq!(eat_string("False")),
//         seq!(eat_string("None")),
//         seq!(eps(), strings.clone()),
//         seq!(NUMBER.clone()),
//         seq!(eps(), choice!(seq!(tuple.clone()), seq!(group.clone()), seq!(genexp.clone()))),
//         seq!(eps(), choice!(seq!(list.clone()), seq!(listcomp.clone()))),
//         seq!(eps(), choice!(seq!(dict.clone()), seq!(set.clone()), seq!(dictcomp.clone()), seq!(setcomp.clone()))),
//         seq!(eat_string("..."))));
//     let group = group.set(choice!(seq!(eat_string("("), choice!(seq!(yield_expr.clone()), seq!(named_expression.clone())), eat_string(")")),
//         seq!(invalid_group.clone())));
//     let lambdef = lambdef.set(choice!(seq!(eat_string("lambda"), opt(choice!(seq!(lambda_params.clone()))), eat_string(":"), expression.clone())));
//     let lambda_params = lambda_params.set(choice!(seq!(invalid_lambda_parameters.clone()),
//         seq!(lambda_parameters.clone())));
//     let lambda_parameters = lambda_parameters.set(choice!(seq!(lambda_slash_no_default.clone(), repeat(lambda_param_no_default.clone()), repeat(lambda_param_with_default.clone()), opt(choice!(seq!(lambda_star_etc.clone())))),
//         seq!(lambda_slash_with_default.clone(), repeat(lambda_param_with_default.clone()), opt(choice!(seq!(lambda_star_etc.clone())))),
//         seq!(repeat(lambda_param_no_default.clone()), repeat(lambda_param_with_default.clone()), opt(choice!(seq!(lambda_star_etc.clone())))),
//         seq!(repeat(lambda_param_with_default.clone()), opt(choice!(seq!(lambda_star_etc.clone())))),
//         seq!(lambda_star_etc.clone())));
//     let lambda_slash_no_default = lambda_slash_no_default.set(choice!(seq!(repeat(lambda_param_no_default.clone()), eat_string("/"), eat_string(",")),
//         seq!(repeat(lambda_param_no_default.clone()), eat_string("/"), eps())));
//     let lambda_slash_with_default = lambda_slash_with_default.set(choice!(seq!(repeat(lambda_param_no_default.clone()), repeat(lambda_param_with_default.clone()), eat_string("/"), eat_string(",")),
//         seq!(repeat(lambda_param_no_default.clone()), repeat(lambda_param_with_default.clone()), eat_string("/"), eps())));
//     let lambda_star_etc = lambda_star_etc.set(choice!(seq!(invalid_lambda_star_etc.clone()),
//         seq!(eat_string("*"), lambda_param_no_default.clone(), repeat(lambda_param_maybe_default.clone()), opt(choice!(seq!(lambda_kwds.clone())))),
//         seq!(eat_string("*"), eat_string(","), repeat(lambda_param_maybe_default.clone()), opt(choice!(seq!(lambda_kwds.clone())))),
//         seq!(lambda_kwds.clone())));
//     let lambda_kwds = lambda_kwds.set(choice!(seq!(invalid_lambda_kwds.clone()),
//         seq!(eat_string("**"), lambda_param_no_default.clone())));
//     let lambda_param_no_default = lambda_param_no_default.set(choice!(seq!(lambda_param.clone(), eat_string(",")),
//         seq!(lambda_param.clone(), eps())));
//     let lambda_param_with_default = lambda_param_with_default.set(choice!(seq!(lambda_param.clone(), default.clone(), eat_string(",")),
//         seq!(lambda_param.clone(), default.clone(), eps())));
//     let lambda_param_maybe_default = lambda_param_maybe_default.set(choice!(seq!(lambda_param.clone(), opt(default.clone()), eat_string(",")),
//         seq!(lambda_param.clone(), opt(default.clone()), eps())));
//     let lambda_param = lambda_param.set(choice!(seq!(NAME.clone())));
//     let fstring_middle = fstring_middle.set(choice!(seq!(fstring_replacement_field.clone()),
//         seq!(FSTRING_MIDDLE.clone())));
//     let fstring_replacement_field = fstring_replacement_field.set(choice!(seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), opt(choice!(seq!(fstring_conversion.clone()))), opt(choice!(seq!(fstring_full_format_spec.clone()))), eat_string("}")),
//         seq!(invalid_replacement_field.clone())));
//     let fstring_conversion = fstring_conversion.set(choice!(seq!(eat_string("!"), NAME.clone())));
//     let fstring_full_format_spec = fstring_full_format_spec.set(choice!(seq!(eat_string(":"), repeat(fstring_format_spec.clone()))));
//     let fstring_format_spec = fstring_format_spec.set(choice!(seq!(FSTRING_MIDDLE.clone()),
//         seq!(fstring_replacement_field.clone())));
//     let fstring = fstring.set(choice!(seq!(FSTRING_START.clone(), repeat(fstring_middle.clone()), FSTRING_END.clone())));
//     let string = string.set(choice!(seq!(STRING.clone())));
//     let strings = strings.set(choice!(seq!(repeat(choice!(seq!(fstring.clone()), seq!(string.clone()))))));
//     let list = list.set(choice!(seq!(eat_string("["), opt(choice!(seq!(star_named_expressions.clone()))), eat_string("]"))));
//     let tuple = tuple.set(choice!(seq!(eat_string("("), opt(choice!(seq!(star_named_expression.clone(), eat_string(","), opt(choice!(seq!(star_named_expressions.clone())))))), eat_string(")"))));
//     let set = set.set(choice!(seq!(eat_string("{"), star_named_expressions.clone(), eat_string("}"))));
//     let dict = dict.set(choice!(seq!(eat_string("{"), opt(choice!(seq!(double_starred_kvpairs.clone()))), eat_string("}")),
//         seq!(eat_string("{"), invalid_double_starred_kvpairs.clone(), eat_string("}"))));
//     let double_starred_kvpairs = double_starred_kvpairs.set(choice!(seq!(seq!(double_starred_kvpair.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
//     let double_starred_kvpair = double_starred_kvpair.set(choice!(seq!(eat_string("**"), bitwise_or.clone()),
//         seq!(kvpair.clone())));
//     let kvpair = kvpair.set(choice!(seq!(expression.clone(), eat_string(":"), expression.clone())));
//     let for_if_clauses = for_if_clauses.set(choice!(seq!(repeat(for_if_clause.clone()))));
//     let for_if_clause = for_if_clause.set(choice!(seq!(eat_string("async"), eat_string("for"), star_targets.clone(), eat_string("in"), eps(), disjunction.clone(), repeat(choice!(seq!(eat_string("if"), disjunction.clone())))),
//         seq!(eat_string("for"), star_targets.clone(), eat_string("in"), eps(), disjunction.clone(), repeat(choice!(seq!(eat_string("if"), disjunction.clone())))),
//         seq!(invalid_for_if_clause.clone()),
//         seq!(invalid_for_target.clone())));
//     let listcomp = listcomp.set(choice!(seq!(eat_string("["), named_expression.clone(), for_if_clauses.clone(), eat_string("]")),
//         seq!(invalid_comprehension.clone())));
//     let setcomp = setcomp.set(choice!(seq!(eat_string("{"), named_expression.clone(), for_if_clauses.clone(), eat_string("}")),
//         seq!(invalid_comprehension.clone())));
//     let genexp = genexp.set(choice!(seq!(eat_string("("), choice!(seq!(assignment_expression.clone()), seq!(expression.clone(), eps())), for_if_clauses.clone(), eat_string(")")),
//         seq!(invalid_comprehension.clone())));
//     let dictcomp = dictcomp.set(choice!(seq!(eat_string("{"), kvpair.clone(), for_if_clauses.clone(), eat_string("}")),
//         seq!(invalid_dict_comprehension.clone())));
//     let arguments = arguments.set(choice!(seq!(args.clone(), opt(choice!(seq!(eat_string(",")))), eps()),
//         seq!(invalid_arguments.clone())));
//     let args = args.set(choice!(seq!(seq!(choice!(seq!(starred_expression.clone()), seq!(choice!(seq!(assignment_expression.clone()), seq!(expression.clone(), eps())), eps())), eat_string(",")), opt(choice!(seq!(eat_string(","), kwargs.clone())))),
//         seq!(kwargs.clone())));
//     let kwargs = kwargs.set(choice!(seq!(seq!(kwarg_or_starred.clone(), eat_string(",")), eat_string(","), seq!(kwarg_or_double_starred.clone(), eat_string(","))),
//         seq!(seq!(kwarg_or_starred.clone(), eat_string(","))),
//         seq!(seq!(kwarg_or_double_starred.clone(), eat_string(",")))));
//     let starred_expression = starred_expression.set(choice!(seq!(invalid_starred_expression_unpacking.clone()),
//         seq!(eat_string("*"), expression.clone()),
//         seq!(invalid_starred_expression.clone())));
//     let kwarg_or_starred = kwarg_or_starred.set(choice!(seq!(invalid_kwarg.clone()),
//         seq!(NAME.clone(), eat_string("="), expression.clone()),
//         seq!(starred_expression.clone())));
//     let kwarg_or_double_starred = kwarg_or_double_starred.set(choice!(seq!(invalid_kwarg.clone()),
//         seq!(NAME.clone(), eat_string("="), expression.clone()),
//         seq!(eat_string("**"), expression.clone())));
//     let star_targets = star_targets.set(choice!(seq!(star_target.clone(), eps()),
//         seq!(star_target.clone(), repeat(choice!(seq!(eat_string(","), star_target.clone()))), opt(choice!(seq!(eat_string(",")))))));
//     let star_targets_list_seq = star_targets_list_seq.set(choice!(seq!(seq!(star_target.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
//     let star_targets_tuple_seq = star_targets_tuple_seq.set(choice!(seq!(star_target.clone(), repeat(choice!(seq!(eat_string(","), star_target.clone()))), opt(choice!(seq!(eat_string(","))))),
//         seq!(star_target.clone(), eat_string(","))));
//     let star_target = star_target.set(choice!(seq!(eat_string("*"), choice!(seq!(eps(), star_target.clone()))),
//         seq!(target_with_star_atom.clone())));
//     let target_with_star_atom = target_with_star_atom.set(choice!(seq!(t_primary.clone(), eat_string("."), NAME.clone(), eps()),
//         seq!(t_primary.clone(), eat_string("["), slices.clone(), eat_string("]"), eps()),
//         seq!(star_atom.clone())));
//     let star_atom = star_atom.set(choice!(seq!(NAME.clone()),
//         seq!(eat_string("("), target_with_star_atom.clone(), eat_string(")")),
//         seq!(eat_string("("), opt(choice!(seq!(star_targets_tuple_seq.clone()))), eat_string(")")),
//         seq!(eat_string("["), opt(choice!(seq!(star_targets_list_seq.clone()))), eat_string("]"))));
//     let single_target = single_target.set(choice!(seq!(single_subscript_attribute_target.clone()),
//         seq!(NAME.clone()),
//         seq!(eat_string("("), single_target.clone(), eat_string(")"))));
//     let single_subscript_attribute_target = single_subscript_attribute_target.set(choice!(seq!(t_primary.clone(), eat_string("."), NAME.clone(), eps()),
//         seq!(t_primary.clone(), eat_string("["), slices.clone(), eat_string("]"), eps())));
//     let t_primary = t_primary.set(choice!(seq!(t_primary.clone(), eat_string("."), NAME.clone(), eps()),
//         seq!(t_primary.clone(), eat_string("["), slices.clone(), eat_string("]"), eps()),
//         seq!(t_primary.clone(), genexp.clone(), eps()),
//         seq!(t_primary.clone(), eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")"), eps()),
//         seq!(atom.clone(), eps())));
//     let t_lookahead = t_lookahead.set(choice!(seq!(eat_string("(")),
//         seq!(eat_string("[")),
//         seq!(eat_string("."))));
//     let del_targets = del_targets.set(choice!(seq!(seq!(del_target.clone(), eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
//     let del_target = del_target.set(choice!(seq!(t_primary.clone(), eat_string("."), NAME.clone(), eps()),
//         seq!(t_primary.clone(), eat_string("["), slices.clone(), eat_string("]"), eps()),
//         seq!(del_t_atom.clone())));
//     let del_t_atom = del_t_atom.set(choice!(seq!(NAME.clone()),
//         seq!(eat_string("("), del_target.clone(), eat_string(")")),
//         seq!(eat_string("("), opt(choice!(seq!(del_targets.clone()))), eat_string(")")),
//         seq!(eat_string("["), opt(choice!(seq!(del_targets.clone()))), eat_string("]"))));
//     let type_expressions = type_expressions.set(choice!(seq!(seq!(expression.clone(), eat_string(",")), eat_string(","), eat_string("*"), expression.clone(), eat_string(","), eat_string("**"), expression.clone()),
//         seq!(seq!(expression.clone(), eat_string(",")), eat_string(","), eat_string("*"), expression.clone()),
//         seq!(seq!(expression.clone(), eat_string(",")), eat_string(","), eat_string("**"), expression.clone()),
//         seq!(eat_string("*"), expression.clone(), eat_string(","), eat_string("**"), expression.clone()),
//         seq!(eat_string("*"), expression.clone()),
//         seq!(eat_string("**"), expression.clone()),
//         seq!(seq!(expression.clone(), eat_string(",")))));
//     let func_type_comment = func_type_comment.set(choice!(seq!(NEWLINE.clone(), TYPE_COMMENT.clone(), eps()),
//         seq!(invalid_double_type_comments.clone()),
//         seq!(TYPE_COMMENT.clone())));
//     let invalid_arguments = invalid_arguments.set(choice!(seq!(choice!(seq!(choice!(seq!(seq!(choice!(seq!(starred_expression.clone()), seq!(choice!(seq!(assignment_expression.clone()), seq!(expression.clone(), eps())), eps())), eat_string(",")), eat_string(","), kwargs.clone()))), seq!(kwargs.clone())), eat_string(","), seq!(choice!(seq!(starred_expression.clone(), eps())), eat_string(","))),
//         seq!(expression.clone(), for_if_clauses.clone(), eat_string(","), opt(choice!(seq!(args.clone()), seq!(expression.clone(), for_if_clauses.clone())))),
//         seq!(NAME.clone(), eat_string("="), expression.clone(), for_if_clauses.clone()),
//         seq!(opt(choice!(seq!(args.clone(), eat_string(",")))), NAME.clone(), eat_string("="), eps()),
//         seq!(args.clone(), for_if_clauses.clone()),
//         seq!(args.clone(), eat_string(","), expression.clone(), for_if_clauses.clone()),
//         seq!(args.clone(), eat_string(","), args.clone())));
//     let invalid_kwarg = invalid_kwarg.set(choice!(seq!(choice!(seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None"))), eat_string("=")),
//         seq!(NAME.clone(), eat_string("="), expression.clone(), for_if_clauses.clone()),
//         seq!(eps(), expression.clone(), eat_string("=")),
//         seq!(eat_string("**"), expression.clone(), eat_string("="), expression.clone())));
//     let expression_without_invalid = expression_without_invalid.set(choice!(seq!(disjunction.clone(), eat_string("if"), disjunction.clone(), eat_string("else"), expression.clone()),
//         seq!(disjunction.clone()),
//         seq!(lambdef.clone())));
//     let invalid_legacy_expression = invalid_legacy_expression.set(choice!(seq!(NAME.clone(), eps(), star_expressions.clone())));
//     let invalid_type_param = invalid_type_param.set(choice!(seq!(eat_string("*"), NAME.clone(), eat_string(":"), expression.clone()),
//         seq!(eat_string("**"), NAME.clone(), eat_string(":"), expression.clone())));
//     let invalid_expression = invalid_expression.set(choice!(seq!(eps(), disjunction.clone(), expression_without_invalid.clone()),
//         seq!(disjunction.clone(), eat_string("if"), disjunction.clone(), eps()),
//         seq!(eat_string("lambda"), opt(choice!(seq!(lambda_params.clone()))), eat_string(":"), eps())));
//     let invalid_named_expression = invalid_named_expression.set(choice!(seq!(expression.clone(), eat_string(":="), expression.clone()),
//         seq!(NAME.clone(), eat_string("="), bitwise_or.clone(), eps()),
//         seq!(eps(), bitwise_or.clone(), eat_string("="), bitwise_or.clone(), eps())));
//     let invalid_assignment = invalid_assignment.set(choice!(seq!(invalid_ann_assign_target.clone(), eat_string(":"), expression.clone()),
//         seq!(star_named_expression.clone(), eat_string(","), repeat(star_named_expressions.clone()), eat_string(":"), expression.clone()),
//         seq!(expression.clone(), eat_string(":"), expression.clone()),
//         seq!(repeat(choice!(seq!(star_targets.clone(), eat_string("=")))), star_expressions.clone(), eat_string("=")),
//         seq!(repeat(choice!(seq!(star_targets.clone(), eat_string("=")))), yield_expr.clone(), eat_string("=")),
//         seq!(star_expressions.clone(), augassign.clone(), annotated_rhs.clone())));
//     let invalid_ann_assign_target = invalid_ann_assign_target.set(choice!(seq!(list.clone()),
//         seq!(tuple.clone()),
//         seq!(eat_string("("), invalid_ann_assign_target.clone(), eat_string(")"))));
//     let invalid_del_stmt = invalid_del_stmt.set(choice!(seq!(eat_string("del"), star_expressions.clone())));
//     let invalid_block = invalid_block.set(choice!(seq!(NEWLINE.clone(), eps())));
//     let invalid_comprehension = invalid_comprehension.set(choice!(seq!(choice!(seq!(eat_string("[")), seq!(eat_string("(")), seq!(eat_string("{"))), starred_expression.clone(), for_if_clauses.clone()),
//         seq!(choice!(seq!(eat_string("[")), seq!(eat_string("{"))), star_named_expression.clone(), eat_string(","), star_named_expressions.clone(), for_if_clauses.clone()),
//         seq!(choice!(seq!(eat_string("[")), seq!(eat_string("{"))), star_named_expression.clone(), eat_string(","), for_if_clauses.clone())));
//     let invalid_dict_comprehension = invalid_dict_comprehension.set(choice!(seq!(eat_string("{"), eat_string("**"), bitwise_or.clone(), for_if_clauses.clone(), eat_string("}"))));
//     let invalid_parameters = invalid_parameters.set(choice!(seq!(eat_string("/"), eat_string(",")),
//         seq!(choice!(seq!(slash_no_default.clone()), seq!(slash_with_default.clone())), repeat(param_maybe_default.clone()), eat_string("/")),
//         seq!(opt(slash_no_default.clone()), repeat(param_no_default.clone()), invalid_parameters_helper.clone(), param_no_default.clone()),
//         seq!(repeat(param_no_default.clone()), eat_string("("), repeat(param_no_default.clone()), opt(eat_string(",")), eat_string(")")),
//         seq!(opt(choice!(seq!(slash_no_default.clone()), seq!(slash_with_default.clone()))), repeat(param_maybe_default.clone()), eat_string("*"), choice!(seq!(eat_string(",")), seq!(param_no_default.clone())), repeat(param_maybe_default.clone()), eat_string("/")),
//         seq!(repeat(param_maybe_default.clone()), eat_string("/"), eat_string("*"))));
//     let invalid_default = invalid_default.set(choice!(seq!(eat_string("="), eps())));
//     let invalid_star_etc = invalid_star_etc.set(choice!(seq!(eat_string("*"), choice!(seq!(eat_string(")")), seq!(eat_string(","), choice!(seq!(eat_string(")")), seq!(eat_string("**")))))),
//         seq!(eat_string("*"), eat_string(","), TYPE_COMMENT.clone()),
//         seq!(eat_string("*"), param.clone(), eat_string("=")),
//         seq!(eat_string("*"), choice!(seq!(param_no_default.clone()), seq!(eat_string(","))), repeat(param_maybe_default.clone()), eat_string("*"), choice!(seq!(param_no_default.clone()), seq!(eat_string(","))))));
//     let invalid_kwds = invalid_kwds.set(choice!(seq!(eat_string("**"), param.clone(), eat_string("=")),
//         seq!(eat_string("**"), param.clone(), eat_string(","), param.clone()),
//         seq!(eat_string("**"), param.clone(), eat_string(","), choice!(seq!(eat_string("*")), seq!(eat_string("**")), seq!(eat_string("/"))))));
//     let invalid_parameters_helper = invalid_parameters_helper.set(choice!(seq!(slash_with_default.clone()),
//         seq!(repeat(param_with_default.clone()))));
//     let invalid_lambda_parameters = invalid_lambda_parameters.set(choice!(seq!(eat_string("/"), eat_string(",")),
//         seq!(choice!(seq!(lambda_slash_no_default.clone()), seq!(lambda_slash_with_default.clone())), repeat(lambda_param_maybe_default.clone()), eat_string("/")),
//         seq!(opt(lambda_slash_no_default.clone()), repeat(lambda_param_no_default.clone()), invalid_lambda_parameters_helper.clone(), lambda_param_no_default.clone()),
//         seq!(repeat(lambda_param_no_default.clone()), eat_string("("), seq!(lambda_param.clone(), eat_string(",")), opt(eat_string(",")), eat_string(")")),
//         seq!(opt(choice!(seq!(lambda_slash_no_default.clone()), seq!(lambda_slash_with_default.clone()))), repeat(lambda_param_maybe_default.clone()), eat_string("*"), choice!(seq!(eat_string(",")), seq!(lambda_param_no_default.clone())), repeat(lambda_param_maybe_default.clone()), eat_string("/")),
//         seq!(repeat(lambda_param_maybe_default.clone()), eat_string("/"), eat_string("*"))));
//     let invalid_lambda_parameters_helper = invalid_lambda_parameters_helper.set(choice!(seq!(lambda_slash_with_default.clone()),
//         seq!(repeat(lambda_param_with_default.clone()))));
//     let invalid_lambda_star_etc = invalid_lambda_star_etc.set(choice!(seq!(eat_string("*"), choice!(seq!(eat_string(":")), seq!(eat_string(","), choice!(seq!(eat_string(":")), seq!(eat_string("**")))))),
//         seq!(eat_string("*"), lambda_param.clone(), eat_string("=")),
//         seq!(eat_string("*"), choice!(seq!(lambda_param_no_default.clone()), seq!(eat_string(","))), repeat(lambda_param_maybe_default.clone()), eat_string("*"), choice!(seq!(lambda_param_no_default.clone()), seq!(eat_string(","))))));
//     let invalid_lambda_kwds = invalid_lambda_kwds.set(choice!(seq!(eat_string("**"), lambda_param.clone(), eat_string("=")),
//         seq!(eat_string("**"), lambda_param.clone(), eat_string(","), lambda_param.clone()),
//         seq!(eat_string("**"), lambda_param.clone(), eat_string(","), choice!(seq!(eat_string("*")), seq!(eat_string("**")), seq!(eat_string("/"))))));
//     let invalid_double_type_comments = invalid_double_type_comments.set(choice!(seq!(TYPE_COMMENT.clone(), NEWLINE.clone(), TYPE_COMMENT.clone(), NEWLINE.clone(), INDENT.clone())));
//     let invalid_with_item = invalid_with_item.set(choice!(seq!(expression.clone(), eat_string("as"), expression.clone(), eps())));
//     let invalid_for_if_clause = invalid_for_if_clause.set(choice!(seq!(opt(eat_string("async")), eat_string("for"), choice!(seq!(bitwise_or.clone(), repeat(choice!(seq!(eat_string(","), bitwise_or.clone()))), opt(choice!(seq!(eat_string(",")))))), eps())));
//     let invalid_for_target = invalid_for_target.set(choice!(seq!(opt(eat_string("async")), eat_string("for"), star_expressions.clone())));
//     let invalid_group = invalid_group.set(choice!(seq!(eat_string("("), starred_expression.clone(), eat_string(")")),
//         seq!(eat_string("("), eat_string("**"), expression.clone(), eat_string(")"))));
//     let invalid_import = invalid_import.set(choice!(seq!(eat_string("import"), seq!(dotted_name.clone(), eat_string(",")), eat_string("from"), dotted_name.clone()),
//         seq!(eat_string("import"), NEWLINE.clone())));
//     let invalid_import_from_targets = invalid_import_from_targets.set(choice!(seq!(import_from_as_names.clone(), eat_string(","), NEWLINE.clone()),
//         seq!(NEWLINE.clone())));
//     let invalid_with_stmt = invalid_with_stmt.set(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), seq!(choice!(seq!(expression.clone(), opt(choice!(seq!(eat_string("as"), star_target.clone()))))), eat_string(",")), NEWLINE.clone()),
//         seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), eat_string("("), seq!(choice!(seq!(expressions.clone(), opt(choice!(seq!(eat_string("as"), star_target.clone()))))), eat_string(",")), opt(eat_string(",")), eat_string(")"), NEWLINE.clone())));
//     let invalid_with_stmt_indent = invalid_with_stmt_indent.set(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), seq!(choice!(seq!(expression.clone(), opt(choice!(seq!(eat_string("as"), star_target.clone()))))), eat_string(",")), eat_string(":"), NEWLINE.clone(), eps()),
//         seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), eat_string("("), seq!(choice!(seq!(expressions.clone(), opt(choice!(seq!(eat_string("as"), star_target.clone()))))), eat_string(",")), opt(eat_string(",")), eat_string(")"), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_try_stmt = invalid_try_stmt.set(choice!(seq!(eat_string("try"), eat_string(":"), NEWLINE.clone(), eps()),
//         seq!(eat_string("try"), eat_string(":"), block.clone(), eps()),
//         seq!(eat_string("try"), eat_string(":"), repeat(block.clone()), repeat(except_block.clone()), eat_string("except"), eat_string("*"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":")),
//         seq!(eat_string("try"), eat_string(":"), repeat(block.clone()), repeat(except_star_block.clone()), eat_string("except"), opt(choice!(seq!(expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone())))))), eat_string(":"))));
//     let invalid_except_stmt = invalid_except_stmt.set(choice!(seq!(eat_string("except"), opt(eat_string("*")), expression.clone(), eat_string(","), expressions.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":")),
//         seq!(eat_string("except"), opt(eat_string("*")), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), NEWLINE.clone()),
//         seq!(eat_string("except"), NEWLINE.clone()),
//         seq!(eat_string("except"), eat_string("*"), choice!(seq!(NEWLINE.clone()), seq!(eat_string(":"))))));
//     let invalid_finally_stmt = invalid_finally_stmt.set(choice!(seq!(eat_string("finally"), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_except_stmt_indent = invalid_except_stmt_indent.set(choice!(seq!(eat_string("except"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":"), NEWLINE.clone(), eps()),
//         seq!(eat_string("except"), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_except_star_stmt_indent = invalid_except_star_stmt_indent.set(choice!(seq!(eat_string("except"), eat_string("*"), expression.clone(), opt(choice!(seq!(eat_string("as"), NAME.clone()))), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_match_stmt = invalid_match_stmt.set(choice!(seq!(eat_string("match"), subject_expr.clone(), NEWLINE.clone()),
//         seq!(eat_string("match"), subject_expr.clone(), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_case_block = invalid_case_block.set(choice!(seq!(eat_string("case"), patterns.clone(), opt(guard.clone()), NEWLINE.clone()),
//         seq!(eat_string("case"), patterns.clone(), opt(guard.clone()), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_as_pattern = invalid_as_pattern.set(choice!(seq!(or_pattern.clone(), eat_string("as"), eat_string("_")),
//         seq!(or_pattern.clone(), eat_string("as"), eps(), expression.clone())));
//     let invalid_class_pattern = invalid_class_pattern.set(choice!(seq!(name_or_attr.clone(), eat_string("("), invalid_class_argument_pattern.clone())));
//     let invalid_class_argument_pattern = invalid_class_argument_pattern.set(choice!(seq!(opt(choice!(seq!(positional_patterns.clone(), eat_string(",")))), keyword_patterns.clone(), eat_string(","), positional_patterns.clone())));
//     let invalid_if_stmt = invalid_if_stmt.set(choice!(seq!(eat_string("if"), named_expression.clone(), NEWLINE.clone()),
//         seq!(eat_string("if"), named_expression.clone(), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_elif_stmt = invalid_elif_stmt.set(choice!(seq!(eat_string("elif"), named_expression.clone(), NEWLINE.clone()),
//         seq!(eat_string("elif"), named_expression.clone(), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_else_stmt = invalid_else_stmt.set(choice!(seq!(eat_string("else"), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_while_stmt = invalid_while_stmt.set(choice!(seq!(eat_string("while"), named_expression.clone(), NEWLINE.clone()),
//         seq!(eat_string("while"), named_expression.clone(), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_for_stmt = invalid_for_stmt.set(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("for"), star_targets.clone(), eat_string("in"), star_expressions.clone(), NEWLINE.clone()),
//         seq!(opt(choice!(seq!(eat_string("async")))), eat_string("for"), star_targets.clone(), eat_string("in"), star_expressions.clone(), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_def_raw = invalid_def_raw.set(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("def"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("("), opt(choice!(seq!(params.clone()))), eat_string(")"), opt(choice!(seq!(eat_string("->"), expression.clone()))), eat_string(":"), NEWLINE.clone(), eps()),
//         seq!(opt(choice!(seq!(eat_string("async")))), eat_string("def"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), eat_string("("), opt(choice!(seq!(params.clone()))), eat_string(")"), opt(choice!(seq!(eat_string("->"), expression.clone()))), eat_string(":"), opt(choice!(seq!(func_type_comment.clone()))), block.clone())));
//     let invalid_class_def_raw = invalid_class_def_raw.set(choice!(seq!(eat_string("class"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")")))), NEWLINE.clone()),
//         seq!(eat_string("class"), NAME.clone(), opt(choice!(seq!(type_params.clone()))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(arguments.clone()))), eat_string(")")))), eat_string(":"), NEWLINE.clone(), eps())));
//     let invalid_double_starred_kvpairs = invalid_double_starred_kvpairs.set(choice!(seq!(seq!(double_starred_kvpair.clone(), eat_string(",")), eat_string(","), invalid_kvpair.clone()),
//         seq!(expression.clone(), eat_string(":"), eat_string("*"), bitwise_or.clone()),
//         seq!(expression.clone(), eat_string(":"), eps())));
//     let invalid_kvpair = invalid_kvpair.set(choice!(seq!(expression.clone(), eps()),
//         seq!(expression.clone(), eat_string(":"), eat_string("*"), bitwise_or.clone()),
//         seq!(expression.clone(), eat_string(":"), eps())));
//     let invalid_starred_expression_unpacking = invalid_starred_expression_unpacking.set(choice!(seq!(eat_string("*"), expression.clone(), eat_string("="), expression.clone())));
//     let invalid_starred_expression = invalid_starred_expression.set(choice!(seq!(eat_string("*"))));
//     let invalid_replacement_field = invalid_replacement_field.set(choice!(seq!(eat_string("{"), eat_string("=")),
//         seq!(eat_string("{"), eat_string("!")),
//         seq!(eat_string("{"), eat_string(":")),
//         seq!(eat_string("{"), eat_string("}")),
//         seq!(eat_string("{"), eps()),
//         seq!(eat_string("{"), annotated_rhs.clone(), eps()),
//         seq!(eat_string("{"), annotated_rhs.clone(), eat_string("="), eps()),
//         seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), invalid_conversion_character.clone()),
//         seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), opt(choice!(seq!(eat_string("!"), NAME.clone()))), eps()),
//         seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), opt(choice!(seq!(eat_string("!"), NAME.clone()))), eat_string(":"), repeat(fstring_format_spec.clone()), eps()),
//         seq!(eat_string("{"), annotated_rhs.clone(), opt(eat_string("=")), opt(choice!(seq!(eat_string("!"), NAME.clone()))), eps())));
//     let invalid_conversion_character = invalid_conversion_character.set(choice!(seq!(eat_string("!"), eps()),
//         seq!(eat_string("!"), eps())));
//     let invalid_arithmetic = invalid_arithmetic.set(choice!(seq!(sum.clone(), choice!(seq!(eat_string("+")), seq!(eat_string("-")), seq!(eat_string("*")), seq!(eat_string("/")), seq!(eat_string("%")), seq!(eat_string("//")), seq!(eat_string("@"))), eat_string("not"), inversion.clone())));
//     let invalid_factor = invalid_factor.set(choice!(seq!(choice!(seq!(eat_string("+")), seq!(eat_string("-")), seq!(eat_string("~"))), eat_string("not"), factor.clone())));
//     let invalid_type_params = invalid_type_params.set(choice!(seq!(eat_string("["), eat_string("]"))));
//     file.into_boxed().into()
// }
