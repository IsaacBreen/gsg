use std::rc::Rc;
use crate::{choice, seq, repeat, repeat as repeat0, repeat1, opt, eat_char_choice, eat_string, eat_char_range, forward_ref, eps, python_newline, indent, dedent, DynCombinator, CombinatorTrait, symbol};
use super::python_tokenizer::{NAME, TYPE_COMMENT, FSTRING_START, FSTRING_MIDDLE, FSTRING_END, NUMBER, STRING};

pub fn python_file() -> Rc<DynCombinator> {
    let NAME = symbol(NAME());
    let TYPE_COMMENT = symbol(TYPE_COMMENT());
    let FSTRING_START = symbol(FSTRING_START());
    let FSTRING_MIDDLE = symbol(FSTRING_MIDDLE());
    let FSTRING_END = symbol(FSTRING_END());
    let NUMBER = symbol(NUMBER());
    let STRING = symbol(STRING());
    let NEWLINE = symbol(python_newline());
    let INDENT = symbol(indent());
    let DEDENT = symbol(dedent());
    let ENDMARKER = symbol(eps());

    let mut invalid_type_params = forward_ref();
    let mut invalid_factor = forward_ref();
    let mut invalid_arithmetic = forward_ref();
    let mut invalid_conversion_character = forward_ref();
    let mut invalid_replacement_field = forward_ref();
    let mut invalid_starred_expression = forward_ref();
    let mut invalid_starred_expression_unpacking = forward_ref();
    let mut invalid_kvpair = forward_ref();
    let mut invalid_double_starred_kvpairs = forward_ref();
    let mut invalid_class_def_raw = forward_ref();
    let mut invalid_def_raw = forward_ref();
    let mut invalid_for_stmt = forward_ref();
    let mut invalid_while_stmt = forward_ref();
    let mut invalid_else_stmt = forward_ref();
    let mut invalid_elif_stmt = forward_ref();
    let mut invalid_if_stmt = forward_ref();
    let mut invalid_class_argument_pattern = forward_ref();
    let mut invalid_class_pattern = forward_ref();
    let mut invalid_as_pattern = forward_ref();
    let mut invalid_case_block = forward_ref();
    let mut invalid_match_stmt = forward_ref();
    let mut invalid_except_star_stmt_indent = forward_ref();
    let mut invalid_except_stmt_indent = forward_ref();
    let mut invalid_finally_stmt = forward_ref();
    let mut invalid_except_stmt = forward_ref();
    let mut invalid_try_stmt = forward_ref();
    let mut invalid_with_stmt_indent = forward_ref();
    let mut invalid_with_stmt = forward_ref();
    let mut invalid_import_from_targets = forward_ref();
    let mut invalid_import = forward_ref();
    let mut invalid_group = forward_ref();
    let mut invalid_for_target = forward_ref();
    let mut invalid_for_if_clause = forward_ref();
    let mut invalid_with_item = forward_ref();
    let mut invalid_double_type_comments = forward_ref();
    let mut invalid_lambda_kwds = forward_ref();
    let mut invalid_lambda_star_etc = forward_ref();
    let mut invalid_lambda_parameters_helper = forward_ref();
    let mut invalid_lambda_parameters = forward_ref();
    let mut invalid_parameters_helper = forward_ref();
    let mut invalid_kwds = forward_ref();
    let mut invalid_star_etc = forward_ref();
    let mut invalid_default = forward_ref();
    let mut invalid_parameters = forward_ref();
    let mut invalid_dict_comprehension = forward_ref();
    let mut invalid_comprehension = forward_ref();
    let mut invalid_block = forward_ref();
    let mut invalid_del_stmt = forward_ref();
    let mut invalid_ann_assign_target = forward_ref();
    let mut invalid_assignment = forward_ref();
    let mut invalid_named_expression = forward_ref();
    let mut invalid_expression = forward_ref();
    let mut invalid_type_param = forward_ref();
    let mut invalid_legacy_expression = forward_ref();
    let mut expression_without_invalid = forward_ref();
    let mut invalid_kwarg = forward_ref();
    let mut invalid_arguments = forward_ref();
    let mut func_type_comment = forward_ref();
    let mut type_expressions = forward_ref();
    let mut del_t_atom = forward_ref();
    let mut del_target = forward_ref();
    let mut del_targets = forward_ref();
    let mut t_lookahead = forward_ref();
    let mut t_primary = forward_ref();
    let mut single_subscript_attribute_target = forward_ref();
    let mut single_target = forward_ref();
    let mut star_atom = forward_ref();
    let mut target_with_star_atom = forward_ref();
    let mut star_target = forward_ref();
    let mut star_targets_tuple_seq = forward_ref();
    let mut star_targets_list_seq = forward_ref();
    let mut star_targets = forward_ref();
    let mut kwarg_or_double_starred = forward_ref();
    let mut kwarg_or_starred = forward_ref();
    let mut starred_expression = forward_ref();
    let mut kwargs = forward_ref();
    let mut args = forward_ref();
    let mut arguments = forward_ref();
    let mut dictcomp = forward_ref();
    let mut genexp = forward_ref();
    let mut setcomp = forward_ref();
    let mut listcomp = forward_ref();
    let mut for_if_clause = forward_ref();
    let mut for_if_clauses = forward_ref();
    let mut kvpair = forward_ref();
    let mut double_starred_kvpair = forward_ref();
    let mut double_starred_kvpairs = forward_ref();
    let mut dict = forward_ref();
    let mut set = forward_ref();
    let mut tuple = forward_ref();
    let mut list = forward_ref();
    let mut strings = forward_ref();
    let mut string = forward_ref();
    let mut fstring = forward_ref();
    let mut fstring_format_spec = forward_ref();
    let mut fstring_full_format_spec = forward_ref();
    let mut fstring_conversion = forward_ref();
    let mut fstring_replacement_field = forward_ref();
    let mut fstring_middle = forward_ref();
    let mut lambda_param = forward_ref();
    let mut lambda_param_maybe_default = forward_ref();
    let mut lambda_param_with_default = forward_ref();
    let mut lambda_param_no_default = forward_ref();
    let mut lambda_kwds = forward_ref();
    let mut lambda_star_etc = forward_ref();
    let mut lambda_slash_with_default = forward_ref();
    let mut lambda_slash_no_default = forward_ref();
    let mut lambda_parameters = forward_ref();
    let mut lambda_params = forward_ref();
    let mut lambdef = forward_ref();
    let mut group = forward_ref();
    let mut atom = forward_ref();
    let mut slice = forward_ref();
    let mut slices = forward_ref();
    let mut primary = forward_ref();
    let mut await_primary = forward_ref();
    let mut power = forward_ref();
    let mut factor = forward_ref();
    let mut term = forward_ref();
    let mut sum = forward_ref();
    let mut shift_expr = forward_ref();
    let mut bitwise_and = forward_ref();
    let mut bitwise_xor = forward_ref();
    let mut bitwise_or = forward_ref();
    let mut is_bitwise_or = forward_ref();
    let mut isnot_bitwise_or = forward_ref();
    let mut in_bitwise_or = forward_ref();
    let mut notin_bitwise_or = forward_ref();
    let mut gt_bitwise_or = forward_ref();
    let mut gte_bitwise_or = forward_ref();
    let mut lt_bitwise_or = forward_ref();
    let mut lte_bitwise_or = forward_ref();
    let mut noteq_bitwise_or = forward_ref();
    let mut eq_bitwise_or = forward_ref();
    let mut compare_op_bitwise_or_pair = forward_ref();
    let mut comparison = forward_ref();
    let mut inversion = forward_ref();
    let mut conjunction = forward_ref();
    let mut disjunction = forward_ref();
    let mut named_expression = forward_ref();
    let mut assignment_expression = forward_ref();
    let mut star_named_expression = forward_ref();
    let mut star_named_expressions = forward_ref();
    let mut star_expression = forward_ref();
    let mut star_expressions = forward_ref();
    let mut yield_expr = forward_ref();
    let mut expression = forward_ref();
    let mut expressions = forward_ref();
    let mut type_param_starred_default = forward_ref();
    let mut type_param_default = forward_ref();
    let mut type_param_bound = forward_ref();
    let mut type_param = forward_ref();
    let mut type_param_seq = forward_ref();
    let mut type_params = forward_ref();
    let mut type_alias = forward_ref();
    let mut keyword_pattern = forward_ref();
    let mut keyword_patterns = forward_ref();
    let mut positional_patterns = forward_ref();
    let mut class_pattern = forward_ref();
    let mut double_star_pattern = forward_ref();
    let mut key_value_pattern = forward_ref();
    let mut items_pattern = forward_ref();
    let mut mapping_pattern = forward_ref();
    let mut star_pattern = forward_ref();
    let mut maybe_star_pattern = forward_ref();
    let mut maybe_sequence_pattern = forward_ref();
    let mut open_sequence_pattern = forward_ref();
    let mut sequence_pattern = forward_ref();
    let mut group_pattern = forward_ref();
    let mut name_or_attr = forward_ref();
    let mut attr = forward_ref();
    let mut value_pattern = forward_ref();
    let mut wildcard_pattern = forward_ref();
    let mut pattern_capture_target = forward_ref();
    let mut capture_pattern = forward_ref();
    let mut imaginary_number = forward_ref();
    let mut real_number = forward_ref();
    let mut signed_real_number = forward_ref();
    let mut signed_number = forward_ref();
    let mut complex_number = forward_ref();
    let mut literal_expr = forward_ref();
    let mut literal_pattern = forward_ref();
    let mut closed_pattern = forward_ref();
    let mut or_pattern = forward_ref();
    let mut as_pattern = forward_ref();
    let mut pattern = forward_ref();
    let mut patterns = forward_ref();
    let mut guard = forward_ref();
    let mut case_block = forward_ref();
    let mut subject_expr = forward_ref();
    let mut match_stmt = forward_ref();
    let mut finally_block = forward_ref();
    let mut except_star_block = forward_ref();
    let mut except_block = forward_ref();
    let mut try_stmt = forward_ref();
    let mut with_item = forward_ref();
    let mut with_stmt = forward_ref();
    let mut for_stmt = forward_ref();
    let mut while_stmt = forward_ref();
    let mut else_block = forward_ref();
    let mut elif_stmt = forward_ref();
    let mut if_stmt = forward_ref();
    let mut default = forward_ref();
    let mut star_annotation = forward_ref();
    let mut annotation = forward_ref();
    let mut param_star_annotation = forward_ref();
    let mut param = forward_ref();
    let mut param_maybe_default = forward_ref();
    let mut param_with_default = forward_ref();
    let mut param_no_default_star_annotation = forward_ref();
    let mut param_no_default = forward_ref();
    let mut kwds = forward_ref();
    let mut star_etc = forward_ref();
    let mut slash_with_default = forward_ref();
    let mut slash_no_default = forward_ref();
    let mut parameters = forward_ref();
    let mut params = forward_ref();
    let mut function_def_raw = forward_ref();
    let mut function_def = forward_ref();
    let mut class_def_raw = forward_ref();
    let mut class_def = forward_ref();
    let mut decorators = forward_ref();
    let mut block = forward_ref();
    let mut dotted_name = forward_ref();
    let mut dotted_as_name = forward_ref();
    let mut dotted_as_names = forward_ref();
    let mut import_from_as_name = forward_ref();
    let mut import_from_as_names = forward_ref();
    let mut import_from_targets = forward_ref();
    let mut import_from = forward_ref();
    let mut import_name = forward_ref();
    let mut import_stmt = forward_ref();
    let mut assert_stmt = forward_ref();
    let mut yield_stmt = forward_ref();
    let mut del_stmt = forward_ref();
    let mut nonlocal_stmt = forward_ref();
    let mut global_stmt = forward_ref();
    let mut raise_stmt = forward_ref();
    let mut return_stmt = forward_ref();
    let mut augassign = forward_ref();
    let mut annotated_rhs = forward_ref();
    let mut assignment = forward_ref();
    let mut compound_stmt = forward_ref();
    let mut simple_stmt = forward_ref();
    let mut simple_stmts = forward_ref();
    let mut statement_newline = forward_ref();
    let mut statement = forward_ref();
    let mut statements = forward_ref();
    let mut func_type = forward_ref();
    let mut eval = forward_ref();
    let mut interactive = forward_ref();
    let mut file = forward_ref();
    let invalid_type_params = invalid_type_params.set(choice!(seq!(eat_string("["), eat_string("]"))));
    let invalid_factor = invalid_factor.set(choice!(seq!(choice!(seq!(eat_string("+")), seq!(eat_string("-")), seq!(eat_string("~"))), eat_string("not"), &factor)));
    let invalid_arithmetic = invalid_arithmetic.set(choice!(seq!(&term, opt(choice!(seq!(repeat(choice!(seq!(eat_string("+"), &term), seq!(eat_string("-"), &term)))))), choice!(seq!(eat_string("+")), seq!(eat_string("-")), seq!(eat_string("*")), seq!(eat_string("/")), seq!(eat_string("%")), seq!(eat_string("//")), seq!(eat_string("@"))), eat_string("not"), &inversion)));
    let invalid_conversion_character = invalid_conversion_character.set(choice!(seq!(eat_string("!")),
        seq!(eat_string("!"))));
    let invalid_replacement_field = invalid_replacement_field.set(choice!(seq!(eat_string("{"), eat_string("=")),
        seq!(eat_string("{"), eat_string("!")),
        seq!(eat_string("{"), eat_string(":")),
        seq!(eat_string("{"), eat_string("}")),
        seq!(eat_string("{")),
        seq!(eat_string("{"), &annotated_rhs),
        seq!(eat_string("{"), &annotated_rhs, eat_string("=")),
        seq!(eat_string("{"), &annotated_rhs, opt(choice!(seq!(eat_string("=")))), &invalid_conversion_character),
        seq!(eat_string("{"), &annotated_rhs, opt(choice!(seq!(eat_string("=")))), opt(choice!(seq!(eat_string("!"), &NAME)))),
        seq!(eat_string("{"), &annotated_rhs, opt(choice!(seq!(eat_string("=")))), opt(choice!(seq!(eat_string("!"), &NAME))), eat_string(":"), repeat(&fstring_format_spec)),
        seq!(eat_string("{"), &annotated_rhs, opt(choice!(seq!(eat_string("=")))), opt(choice!(seq!(eat_string("!"), &NAME))))));
    let invalid_starred_expression = invalid_starred_expression.set(choice!(seq!(eat_string("*"))));
    let invalid_starred_expression_unpacking = invalid_starred_expression_unpacking.set(choice!(seq!(eat_string("*"), &expression, eat_string("="), &expression)));
    let invalid_kvpair = invalid_kvpair.set(choice!(seq!(&invalid_expression),
        seq!(&invalid_legacy_expression),
        seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(&disjunction),
        seq!(&lambdef),
        seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), eat_string(":"), eat_string("*"), &bitwise_or),
        seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), eat_string(":"))));
    let invalid_double_starred_kvpairs = invalid_double_starred_kvpairs.set(choice!(seq!(choice!(seq!(eat_string("**"), &bitwise_or), seq!(&kvpair)), repeat(eat_string(",")), &invalid_kvpair),
        seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), eat_string(":"), eat_string("*"), &bitwise_or),
        seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), eat_string(":"))));
    let invalid_class_def_raw = invalid_class_def_raw.set(choice!(seq!(eat_string("class"), &NAME, opt(choice!(seq!(&type_params))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(&arguments))), eat_string(")")))), &NEWLINE),
        seq!(eat_string("class"), &NAME, opt(choice!(seq!(&type_params))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(&arguments))), eat_string(")")))), eat_string(":"), &NEWLINE)));
    let invalid_def_raw = invalid_def_raw.set(choice!(seq!(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("def"), &NAME, opt(choice!(seq!(&type_params))), eat_string("("), opt(choice!(seq!(&params))), eat_string(")"), opt(choice!(seq!(eat_string("->"), &expression))), eat_string(":"), &NEWLINE), seq!(opt(choice!(seq!(eat_string("async")))), eat_string("def"), &NAME, opt(choice!(seq!(&type_params))), eat_string("("), opt(choice!(seq!(&params))), eat_string(")"), opt(choice!(seq!(eat_string("->"), &expression))), eat_string(":"), opt(choice!(seq!(&func_type_comment))), &block)), opt(choice!(seq!(repeat(choice!(seq!(eat_string("def"), &NAME, opt(choice!(seq!(&type_params))), eat_string("("), opt(choice!(seq!(&params))), eat_string(")"), opt(choice!(seq!(eat_string("->"), &expression))), eat_string(":"), &NEWLINE), seq!(eat_string("def"), &NAME, opt(choice!(seq!(&type_params))), eat_string("("), opt(choice!(seq!(&params))), eat_string(")"), opt(choice!(seq!(eat_string("->"), &expression))), eat_string(":"), opt(choice!(seq!(&func_type_comment))), &block)))))))));
    let invalid_for_stmt = invalid_for_stmt.set(choice!(seq!(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("for"), &star_targets, eat_string("in"), &star_expressions, &NEWLINE), seq!(opt(choice!(seq!(eat_string("async")))), eat_string("for"), &star_targets, eat_string("in"), &star_expressions, eat_string(":"), &NEWLINE)), opt(choice!(seq!(repeat(choice!(seq!(eat_string("for"), &star_targets, eat_string("in"), &star_expressions, &NEWLINE), seq!(eat_string("for"), &star_targets, eat_string("in"), &star_expressions, eat_string(":"), &NEWLINE)))))))));
    let invalid_while_stmt = invalid_while_stmt.set(choice!(seq!(eat_string("while"), &named_expression, &NEWLINE),
        seq!(eat_string("while"), &named_expression, eat_string(":"), &NEWLINE)));
    let invalid_else_stmt = invalid_else_stmt.set(choice!(seq!(eat_string("else"), eat_string(":"), &NEWLINE)));
    let invalid_elif_stmt = invalid_elif_stmt.set(choice!(seq!(eat_string("elif"), &named_expression, &NEWLINE),
        seq!(eat_string("elif"), &named_expression, eat_string(":"), &NEWLINE)));
    let invalid_if_stmt = invalid_if_stmt.set(choice!(seq!(eat_string("if"), &named_expression, &NEWLINE),
        seq!(eat_string("if"), &named_expression, eat_string(":"), &NEWLINE)));
    let invalid_class_argument_pattern = invalid_class_argument_pattern.set(choice!(seq!(opt(choice!(seq!(choice!(seq!(&as_pattern), seq!(&or_pattern)), repeat(eat_string(","))))), &keyword_patterns, eat_string(","), &positional_patterns, opt(choice!(seq!(repeat(choice!(seq!(&keyword_patterns, eat_string(","), &positional_patterns)))))))));
    let invalid_class_pattern = invalid_class_pattern.set(choice!(seq!(&NAME, opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME)))))), eat_string("("), &invalid_class_argument_pattern)));
    let invalid_as_pattern = invalid_as_pattern.set(choice!(seq!(&closed_pattern, repeat(eat_string("|")), eat_string("as"), eat_string("")),
        seq!(&closed_pattern, repeat(eat_string("|")), eat_string("as"), &expression)));
    let invalid_case_block = invalid_case_block.set(choice!(seq!(eat_string("as"), &patterns, opt(choice!(seq!(&guard))), &NEWLINE),
        seq!(eat_string("as"), &patterns, opt(choice!(seq!(&guard))), eat_string(":"), &NEWLINE)));
    let invalid_match_stmt = invalid_match_stmt.set(choice!(seq!(eat_string("atc"), &subject_expr, &NEWLINE),
        seq!(eat_string("atc"), &subject_expr, eat_string(":"), &NEWLINE)));
    let invalid_except_star_stmt_indent = invalid_except_star_stmt_indent.set(choice!(seq!(eat_string("except"), eat_string("*"), &expression, opt(choice!(seq!(eat_string("as"), &NAME))), eat_string(":"), &NEWLINE)));
    let invalid_except_stmt_indent = invalid_except_stmt_indent.set(choice!(seq!(eat_string("except"), &expression, opt(choice!(seq!(eat_string("as"), &NAME))), eat_string(":"), &NEWLINE),
        seq!(eat_string("except"), eat_string(":"), &NEWLINE)));
    let invalid_finally_stmt = invalid_finally_stmt.set(choice!(seq!(eat_string("finally"), eat_string(":"), &NEWLINE)));
    let invalid_except_stmt = invalid_except_stmt.set(choice!(seq!(eat_string("except"), opt(choice!(seq!(eat_string("*")))), &expression, eat_string(","), &expressions, opt(choice!(seq!(eat_string("as"), &NAME))), eat_string(":")),
        seq!(eat_string("except"), opt(choice!(seq!(eat_string("*")))), &expression, opt(choice!(seq!(eat_string("as"), &NAME))), &NEWLINE),
        seq!(eat_string("except"), &NEWLINE),
        seq!(eat_string("except"), eat_string("*"), choice!(seq!(&NEWLINE), seq!(eat_string(":"))))));
    let invalid_try_stmt = invalid_try_stmt.set(choice!(seq!(eat_string("try"), eat_string(":"), &NEWLINE),
        seq!(eat_string("try"), eat_string(":"), &block),
        seq!(eat_string("try"), eat_string(":"), repeat(&block), repeat(&except_block), eat_string("except"), eat_string("*"), &expression, opt(choice!(seq!(eat_string("as"), &NAME))), eat_string(":")),
        seq!(eat_string("try"), eat_string(":"), repeat(&block), repeat(&except_star_block), eat_string("except"), opt(choice!(seq!(&expression, opt(choice!(seq!(eat_string("as"), &NAME)))))), eat_string(":"))));
    let invalid_with_stmt_indent = invalid_with_stmt_indent.set(choice!(seq!(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), &expression, opt(choice!(seq!(eat_string("as"), &star_target))), repeat(eat_string(",")), eat_string(":"), &NEWLINE), seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), eat_string("("), &expressions, opt(choice!(seq!(eat_string("as"), &star_target))), repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))), eat_string(")"), eat_string(":"), &NEWLINE)), opt(choice!(seq!(repeat(choice!(seq!(eat_string("with"), &expression, opt(choice!(seq!(eat_string("as"), &star_target))), repeat(eat_string(",")), eat_string(":"), &NEWLINE), seq!(eat_string("with"), eat_string("("), &expressions, opt(choice!(seq!(eat_string("as"), &star_target))), repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))), eat_string(")"), eat_string(":"), &NEWLINE)))))))));
    let invalid_with_stmt = invalid_with_stmt.set(choice!(seq!(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), &expression, opt(choice!(seq!(eat_string("as"), &star_target))), repeat(eat_string(",")), &NEWLINE), seq!(opt(choice!(seq!(eat_string("async")))), eat_string("with"), eat_string("("), &expressions, opt(choice!(seq!(eat_string("as"), &star_target))), repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))), eat_string(")"), &NEWLINE)), opt(choice!(seq!(repeat(choice!(seq!(eat_string("with"), &expression, opt(choice!(seq!(eat_string("as"), &star_target))), repeat(eat_string(",")), &NEWLINE), seq!(eat_string("with"), eat_string("("), &expressions, opt(choice!(seq!(eat_string("as"), &star_target))), repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))), eat_string(")"), &NEWLINE)))))))));
    let invalid_import_from_targets = invalid_import_from_targets.set(choice!(seq!(&import_from_as_name, repeat(eat_string(",")), &NEWLINE),
        seq!(&NEWLINE)));
    let invalid_import = invalid_import.set(choice!(seq!(eat_string("import"), &dotted_name, repeat(eat_string(",")), eat_string("from"), &dotted_name),
        seq!(eat_string("import"), &NEWLINE)));
    let invalid_group = invalid_group.set(choice!(seq!(eat_string("("), &starred_expression, eat_string(")")),
        seq!(eat_string("("), eat_string("**"), &expression, eat_string(")"))));
    let invalid_for_target = invalid_for_target.set(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("for"), &star_expressions, opt(choice!(seq!(repeat(choice!(seq!(eat_string("for"), &star_expressions)))))))));
    let invalid_for_if_clause = invalid_for_if_clause.set(choice!(seq!(opt(choice!(seq!(eat_string("async")))), eat_string("for"), &bitwise_or, repeat(choice!(seq!(eat_string(","), &bitwise_or))), opt(choice!(seq!(eat_string(",")))), opt(choice!(seq!(repeat(choice!(seq!(eat_string("for"), &bitwise_or, repeat(choice!(seq!(eat_string(","), &bitwise_or))), opt(choice!(seq!(eat_string(",")))))))))))));
    let invalid_with_item = invalid_with_item.set(choice!(seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), eat_string("as"), &expression)));
    let invalid_double_type_comments = invalid_double_type_comments.set(choice!(seq!(&TYPE_COMMENT, &NEWLINE, &TYPE_COMMENT, &NEWLINE, &INDENT)));
    let invalid_lambda_kwds = invalid_lambda_kwds.set(choice!(seq!(eat_string("**"), &lambda_param, eat_string("=")),
        seq!(eat_string("**"), &lambda_param, eat_string(","), &lambda_param),
        seq!(eat_string("**"), &lambda_param, eat_string(","), choice!(seq!(eat_string("*")), seq!(eat_string("**")), seq!(eat_string("/"))))));
    let invalid_lambda_star_etc = invalid_lambda_star_etc.set(choice!(seq!(eat_string("*"), choice!(seq!(eat_string(":")), seq!(eat_string(","), choice!(seq!(eat_string(":")), seq!(eat_string("**")))))),
        seq!(eat_string("*"), &lambda_param, eat_string("=")),
        seq!(eat_string("*"), choice!(seq!(&lambda_param_no_default), seq!(eat_string(","))), repeat(&lambda_param_maybe_default), eat_string("*"), choice!(seq!(&lambda_param_no_default), seq!(eat_string(","))))));
    let invalid_lambda_parameters_helper = invalid_lambda_parameters_helper.set(choice!(seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/"), eat_string(",")),
        seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/")),
        seq!(choice!(seq!(&lambda_param, &default, eat_string(",")), seq!(&lambda_param, &default)), opt(choice!(seq!(repeat(&lambda_param_with_default)))))));
    let invalid_lambda_parameters = invalid_lambda_parameters.set(choice!(seq!(choice!(seq!(eat_string(""), eat_string(",")), seq!(choice!(seq!(repeat(&lambda_param_no_default), eat_string("/"), eat_string(",")), seq!(repeat(&lambda_param_no_default), eat_string("/")), seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/"), eat_string(",")), seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/"))), repeat(&lambda_param_maybe_default), eat_string("/")), seq!(opt(choice!(seq!(repeat(&lambda_param_no_default), eat_string("/"), eat_string(",")), seq!(repeat(&lambda_param_no_default), eat_string("/")))), repeat(&lambda_param_no_default), &invalid_lambda_parameters_helper, &lambda_param_no_default), seq!(choice!(seq!(&lambda_param, eat_string(",")), seq!(&lambda_param)), opt(choice!(seq!(repeat(&lambda_param_no_default)))), eat_string("("), &lambda_param, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))), eat_string(")")), seq!(opt(choice!(seq!(repeat(&lambda_param_no_default), eat_string("/"), eat_string(",")), seq!(repeat(&lambda_param_no_default), eat_string("/")), seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/"), eat_string(",")), seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/")))), repeat(&lambda_param_maybe_default), eat_string("*"), choice!(seq!(eat_string(",")), seq!(&lambda_param_no_default)), repeat(&lambda_param_maybe_default), eat_string("/")), seq!(choice!(seq!(&lambda_param, opt(choice!(seq!(&default))), eat_string(",")), seq!(&lambda_param, opt(choice!(seq!(&default))))), opt(choice!(seq!(repeat(&lambda_param_maybe_default)))), eat_string("/"), eat_string("*"))), opt(choice!(seq!(repeat(choice!(seq!(repeat(&lambda_param_no_default), &invalid_lambda_parameters_helper, &lambda_param_no_default), seq!(repeat(&lambda_param_maybe_default), eat_string("*"), choice!(seq!(eat_string(",")), seq!(&lambda_param_no_default)), repeat(&lambda_param_maybe_default), eat_string("/"))))))))));
    let invalid_parameters_helper = invalid_parameters_helper.set(choice!(seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/"), eat_string(",")),
        seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/")),
        seq!(choice!(seq!(&param, &default, eat_string(","), opt(choice!(seq!(&TYPE_COMMENT)))), seq!(&param, &default, opt(choice!(seq!(&TYPE_COMMENT))))), opt(choice!(seq!(repeat(&param_with_default)))))));
    let invalid_kwds = invalid_kwds.set(choice!(seq!(eat_string("**"), &param, eat_string("=")),
        seq!(eat_string("**"), &param, eat_string(","), &param),
        seq!(eat_string("**"), &param, eat_string(","), choice!(seq!(eat_string("*")), seq!(eat_string("**")), seq!(eat_string("/"))))));
    let invalid_star_etc = invalid_star_etc.set(choice!(seq!(eat_string("*"), choice!(seq!(eat_string(")")), seq!(eat_string(","), choice!(seq!(eat_string(")")), seq!(eat_string("**")))))),
        seq!(eat_string("*"), eat_string(","), &TYPE_COMMENT),
        seq!(eat_string("*"), &param, eat_string("=")),
        seq!(eat_string("*"), choice!(seq!(&param_no_default), seq!(eat_string(","))), repeat(&param_maybe_default), eat_string("*"), choice!(seq!(&param_no_default), seq!(eat_string(","))))));
    let invalid_default = invalid_default.set(choice!(seq!(eat_string("="))));
    let invalid_parameters = invalid_parameters.set(choice!(seq!(choice!(seq!(eat_string(""), eat_string(",")), seq!(choice!(seq!(repeat(&param_no_default), eat_string("/"), eat_string(",")), seq!(repeat(&param_no_default), eat_string("/")), seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/"), eat_string(",")), seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/"))), repeat(&param_maybe_default), eat_string("/")), seq!(opt(choice!(seq!(repeat(&param_no_default), eat_string("/"), eat_string(",")), seq!(repeat(&param_no_default), eat_string("/")))), repeat(&param_no_default), &invalid_parameters_helper, &param_no_default), seq!(choice!(seq!(&param, eat_string(","), opt(choice!(seq!(&TYPE_COMMENT)))), seq!(&param, opt(choice!(seq!(&TYPE_COMMENT))))), opt(choice!(seq!(repeat(&param_no_default)))), eat_string("("), repeat(&param_no_default), opt(choice!(seq!(eat_string(",")))), eat_string(")")), seq!(opt(choice!(seq!(repeat(&param_no_default), eat_string("/"), eat_string(",")), seq!(repeat(&param_no_default), eat_string("/")), seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/"), eat_string(",")), seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/")))), repeat(&param_maybe_default), eat_string("*"), choice!(seq!(eat_string(",")), seq!(&param_no_default)), repeat(&param_maybe_default), eat_string("/")), seq!(choice!(seq!(&param, opt(choice!(seq!(&default))), eat_string(","), opt(choice!(seq!(&TYPE_COMMENT)))), seq!(&param, opt(choice!(seq!(&default))), opt(choice!(seq!(&TYPE_COMMENT))))), opt(choice!(seq!(repeat(&param_maybe_default)))), eat_string("/"), eat_string("*"))), opt(choice!(seq!(repeat(choice!(seq!(repeat(&param_no_default), &invalid_parameters_helper, &param_no_default), seq!(repeat(&param_maybe_default), eat_string("*"), choice!(seq!(eat_string(",")), seq!(&param_no_default)), repeat(&param_maybe_default), eat_string("/"))))))))));
    let invalid_dict_comprehension = invalid_dict_comprehension.set(choice!(seq!(eat_string("{"), eat_string("**"), &bitwise_or, &for_if_clauses, eat_string("}"))));
    let invalid_comprehension = invalid_comprehension.set(choice!(seq!(choice!(seq!(eat_string("[")), seq!(eat_string("(")), seq!(eat_string("{"))), &starred_expression, &for_if_clauses),
        seq!(choice!(seq!(eat_string("[")), seq!(eat_string("{"))), &star_named_expression, eat_string(","), &star_named_expressions, &for_if_clauses),
        seq!(choice!(seq!(eat_string("[")), seq!(eat_string("{"))), &star_named_expression, eat_string(","), &for_if_clauses)));
    let invalid_block = invalid_block.set(choice!(seq!(&NEWLINE)));
    let invalid_del_stmt = invalid_del_stmt.set(choice!(seq!(eat_string("del"), &star_expressions)));
    let invalid_ann_assign_target = invalid_ann_assign_target.set(choice!(seq!(eat_string("["), opt(choice!(seq!(&star_named_expressions))), eat_string("]")),
        seq!(eat_string("("), opt(choice!(seq!(&star_named_expression, eat_string(","), opt(choice!(seq!(&star_named_expressions)))))), eat_string(")")),
        seq!(eat_string("("), &invalid_ann_assign_target, eat_string(")"))));
    let invalid_assignment = invalid_assignment.set(choice!(seq!(&invalid_ann_assign_target, eat_string(":"), &expression),
        seq!(choice!(seq!(eat_string("*"), &bitwise_or), seq!(&named_expression)), eat_string(","), repeat(&star_named_expressions), eat_string(":"), &expression),
        seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), eat_string(":"), &expression),
        seq!(choice!(seq!(&star_target), seq!(&star_target, repeat(choice!(seq!(eat_string(","), &star_target))), opt(choice!(seq!(eat_string(",")))))), eat_string("="), opt(choice!(seq!(repeat(choice!(seq!(choice!(seq!(&star_target), seq!(&star_target, repeat(choice!(seq!(eat_string(","), &star_target))), opt(choice!(seq!(eat_string(",")))))), eat_string("="))))))), &star_expressions, eat_string("=")),
        seq!(choice!(seq!(&star_target), seq!(&star_target, repeat(choice!(seq!(eat_string(","), &star_target))), opt(choice!(seq!(eat_string(",")))))), eat_string("="), opt(choice!(seq!(repeat(choice!(seq!(choice!(seq!(&star_target), seq!(&star_target, repeat(choice!(seq!(eat_string(","), &star_target))), opt(choice!(seq!(eat_string(",")))))), eat_string("="))))))), &yield_expr, eat_string("=")),
        seq!(choice!(seq!(&star_expression, repeat(choice!(seq!(eat_string(","), &star_expression))), opt(choice!(seq!(eat_string(","))))), seq!(&star_expression, eat_string(",")), seq!(&star_expression)), &augassign, &annotated_rhs)));
    let invalid_named_expression = invalid_named_expression.set(choice!(seq!(choice!(seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), eat_string(":="), &expression), seq!(&NAME, eat_string("="), &bitwise_or), seq!(&bitwise_or, eat_string("="), &bitwise_or)), opt(choice!(seq!(repeat(choice!(seq!(&bitwise_or, eat_string("="), &bitwise_or)))))))));
    let invalid_expression = invalid_expression.set(choice!(seq!(choice!(seq!(&disjunction, &expression_without_invalid), seq!(choice!(seq!(&conjunction, repeat(choice!(seq!(eat_string("or"), &conjunction)))), seq!(&conjunction)), eat_string("if"), &disjunction), seq!(eat_string("lambda"), opt(choice!(seq!(&lambda_params))), eat_string(":"))), opt(choice!(seq!(repeat(choice!(seq!(&disjunction, &expression_without_invalid)))))))));
    let invalid_type_param = invalid_type_param.set(choice!(seq!(eat_string("*"), &NAME, eat_string(":"), &expression),
        seq!(eat_string("**"), &NAME, eat_string(":"), &expression)));
    let invalid_legacy_expression = invalid_legacy_expression.set(choice!(seq!(&NAME, &star_expressions)));
    let expression_without_invalid = expression_without_invalid.set(choice!(seq!(choice!(seq!(&conjunction, repeat(choice!(seq!(eat_string("or"), &conjunction)))), seq!(&conjunction)), eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(&conjunction, repeat(choice!(seq!(eat_string("or"), &conjunction)))),
        seq!(&conjunction),
        seq!(eat_string("lambda"), opt(choice!(seq!(&lambda_params))), eat_string(":"), &expression)));
    let invalid_kwarg = invalid_kwarg.set(choice!(seq!(choice!(seq!(choice!(seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None"))), eat_string("=")), seq!(&NAME, eat_string("="), &expression, &for_if_clauses), seq!(&expression, eat_string("=")), seq!(eat_string("**"), &expression, eat_string("="), &expression)), opt(choice!(seq!(repeat(choice!(seq!(&expression, eat_string("="))))))))));
    let invalid_arguments = invalid_arguments.set(choice!(seq!(choice!(seq!(choice!(seq!(choice!(seq!(&invalid_starred_expression_unpacking), seq!(eat_string("*"), &expression), seq!(&invalid_starred_expression), seq!(&NAME, eat_string(":="), &expression), seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), &kwargs), seq!(&kwarg_or_starred, repeat(eat_string(",")), &kwarg_or_double_starred, repeat(eat_string(","))), seq!(&kwarg_or_starred, repeat(eat_string(","))), seq!(&kwarg_or_double_starred, repeat(eat_string(",")))), eat_string(","), &starred_expression, repeat(eat_string(","))), seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), &for_if_clauses, eat_string(","), opt(choice!(seq!(&args), seq!(&expression, &for_if_clauses)))), seq!(&NAME, eat_string("="), &expression, &for_if_clauses), seq!(opt(choice!(seq!(choice!(seq!(choice!(seq!(&starred_expression), seq!(&NAME, eat_string(":="), &expression), seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), opt(choice!(seq!(eat_string(","), &kwargs)))), seq!(&kwargs)), eat_string(",")))), &NAME, eat_string("=")), seq!(choice!(seq!(choice!(seq!(&starred_expression), seq!(&NAME, eat_string(":="), &expression), seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), opt(choice!(seq!(eat_string(","), &kwargs)))), seq!(&kwargs)), &for_if_clauses), seq!(choice!(seq!(choice!(seq!(&starred_expression), seq!(&NAME, eat_string(":="), &expression), seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), opt(choice!(seq!(eat_string(","), &kwargs)))), seq!(&kwargs)), eat_string(","), &expression, &for_if_clauses), seq!(choice!(seq!(choice!(seq!(&starred_expression), seq!(&NAME, eat_string(":="), &expression), seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), opt(choice!(seq!(eat_string(","), &kwargs)))), seq!(&kwargs)), eat_string(","), &args)), opt(choice!(seq!(repeat(choice!(seq!(&NAME, eat_string("="))))))))));
    let func_type_comment = func_type_comment.set(choice!(seq!(&NEWLINE, &TYPE_COMMENT),
        seq!(&invalid_double_type_comments),
        seq!(&TYPE_COMMENT)));
    let type_expressions = type_expressions.set(choice!(seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), eat_string("*"), &expression, eat_string(","), eat_string("**"), &expression),
        seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), eat_string("*"), &expression),
        seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), eat_string("**"), &expression),
        seq!(eat_string("*"), &expression, eat_string(","), eat_string("**"), &expression),
        seq!(eat_string("*"), &expression),
        seq!(eat_string("**"), &expression),
        seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")))));
    let del_t_atom = del_t_atom.set(choice!(seq!(&NAME),
        seq!(eat_string("("), &del_target, eat_string(")")),
        seq!(eat_string("("), opt(choice!(seq!(&del_targets))), eat_string(")")),
        seq!(eat_string("["), opt(choice!(seq!(&del_targets))), eat_string("]"))));
    let del_target = del_target.set(choice!(seq!(choice!(seq!(&NAME), seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None")), seq!(&strings), seq!(&NUMBER), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp), seq!(eat_string("..."))), opt(choice!(seq!(repeat(choice!(seq!(&strings), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp)))))), opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME), seq!(eat_string("["), &slices, eat_string("]")), seq!(&genexp), seq!(eat_string("("), opt(choice!(seq!(&arguments))), eat_string(")"))))))), eat_string("."), &NAME),
        seq!(choice!(seq!(&NAME), seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None")), seq!(&strings), seq!(&NUMBER), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp), seq!(eat_string("..."))), opt(choice!(seq!(repeat(choice!(seq!(&strings), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp)))))), opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME), seq!(eat_string("["), &slices, eat_string("]")), seq!(&genexp), seq!(eat_string("("), opt(choice!(seq!(&arguments))), eat_string(")"))))))), eat_string("["), &slices, eat_string("]")),
        seq!(&del_t_atom)));
    let del_targets = del_targets.set(choice!(seq!(&del_target, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let t_lookahead = t_lookahead.set(choice!(seq!(eat_string("(")),
        seq!(eat_string("[")),
        seq!(eat_string("."))));
    let t_primary = t_primary.set(choice!(seq!(choice!(seq!(&NAME), seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None")), seq!(&strings), seq!(&NUMBER), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp), seq!(eat_string("..."))), opt(choice!(seq!(repeat(choice!(seq!(&strings), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp)))))), opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME), seq!(eat_string("["), &slices, eat_string("]")), seq!(&genexp), seq!(eat_string("("), opt(choice!(seq!(&arguments))), eat_string(")"))))))))));
    let single_subscript_attribute_target = single_subscript_attribute_target.set(choice!(seq!(&t_primary, eat_string("."), &NAME),
        seq!(&t_primary, eat_string("["), &slices, eat_string("]"))));
    let single_target = single_target.set(choice!(seq!(&single_subscript_attribute_target),
        seq!(&NAME),
        seq!(eat_string("("), &single_target, eat_string(")"))));
    let star_atom = star_atom.set(choice!(seq!(&NAME),
        seq!(eat_string("("), &target_with_star_atom, eat_string(")")),
        seq!(eat_string("("), opt(choice!(seq!(&star_targets_tuple_seq))), eat_string(")")),
        seq!(eat_string("["), opt(choice!(seq!(&star_targets_list_seq))), eat_string("]"))));
    let target_with_star_atom = target_with_star_atom.set(choice!(seq!(&t_primary, eat_string("."), &NAME),
        seq!(&t_primary, eat_string("["), &slices, eat_string("]")),
        seq!(&star_atom)));
    let star_target = star_target.set(choice!(seq!(eat_string("*"), &star_target),
        seq!(&target_with_star_atom)));
    let star_targets_tuple_seq = star_targets_tuple_seq.set(choice!(seq!(&star_target, repeat(choice!(seq!(eat_string(","), &star_target))), opt(choice!(seq!(eat_string(","))))),
        seq!(&star_target, eat_string(","))));
    let star_targets_list_seq = star_targets_list_seq.set(choice!(seq!(&star_target, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let star_targets = star_targets.set(choice!(seq!(&star_target),
        seq!(&star_target, repeat(choice!(seq!(eat_string(","), &star_target))), opt(choice!(seq!(eat_string(",")))))));
    let kwarg_or_double_starred = kwarg_or_double_starred.set(choice!(seq!(&invalid_kwarg),
        seq!(&NAME, eat_string("="), &expression),
        seq!(eat_string("**"), &expression)));
    let kwarg_or_starred = kwarg_or_starred.set(choice!(seq!(&invalid_kwarg),
        seq!(&NAME, eat_string("="), &expression),
        seq!(&invalid_starred_expression_unpacking),
        seq!(eat_string("*"), &expression),
        seq!(&invalid_starred_expression)));
    let starred_expression = starred_expression.set(choice!(seq!(&invalid_starred_expression_unpacking),
        seq!(eat_string("*"), &expression),
        seq!(&invalid_starred_expression)));
    let kwargs = kwargs.set(choice!(seq!(&kwarg_or_starred, repeat(eat_string(",")), &kwarg_or_double_starred, repeat(eat_string(","))),
        seq!(&kwarg_or_starred, repeat(eat_string(","))),
        seq!(&kwarg_or_double_starred, repeat(eat_string(",")))));
    let args = args.set(choice!(seq!(choice!(seq!(&starred_expression), seq!(&NAME, eat_string(":="), &expression), seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), repeat(eat_string(",")), opt(choice!(seq!(eat_string(","), &kwargs)))),
        seq!(&kwargs)));
    let arguments = arguments.set(choice!(seq!(&args, opt(choice!(seq!(eat_string(","))))),
        seq!(&invalid_arguments)));
    let dictcomp = dictcomp.set(choice!(seq!(eat_string("{"), &kvpair, &for_if_clauses, eat_string("}")),
        seq!(&invalid_dict_comprehension)));
    let genexp = genexp.set(choice!(seq!(eat_string("("), choice!(seq!(&assignment_expression), seq!(&expression)), &for_if_clauses, eat_string(")")),
        seq!(&invalid_comprehension)));
    let setcomp = setcomp.set(choice!(seq!(eat_string("{"), &named_expression, &for_if_clauses, eat_string("}")),
        seq!(&invalid_comprehension)));
    let listcomp = listcomp.set(choice!(seq!(eat_string("["), &named_expression, &for_if_clauses, eat_string("]")),
        seq!(&invalid_comprehension)));
    let for_if_clause = for_if_clause.set(choice!(seq!(eat_string("async"), eat_string("for"), &star_targets, eat_string("in"), &disjunction, repeat(choice!(seq!(eat_string("if"), &disjunction)))),
        seq!(eat_string("for"), &star_targets, eat_string("in"), &disjunction, repeat(choice!(seq!(eat_string("if"), &disjunction)))),
        seq!(&invalid_for_if_clause),
        seq!(&invalid_for_target)));
    let for_if_clauses = for_if_clauses.set(choice!(seq!(repeat(&for_if_clause))));
    let kvpair = kvpair.set(choice!(seq!(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), eat_string(":"), &expression)));
    let double_starred_kvpair = double_starred_kvpair.set(choice!(seq!(eat_string("**"), &bitwise_or),
        seq!(&kvpair)));
    let double_starred_kvpairs = double_starred_kvpairs.set(choice!(seq!(&double_starred_kvpair, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let dict = dict.set(choice!(seq!(eat_string("{"), opt(choice!(seq!(&double_starred_kvpairs))), eat_string("}")),
        seq!(eat_string("{"), &invalid_double_starred_kvpairs, eat_string("}"))));
    let set = set.set(choice!(seq!(eat_string("{"), &star_named_expressions, eat_string("}"))));
    let tuple = tuple.set(choice!(seq!(eat_string("("), opt(choice!(seq!(&star_named_expression, eat_string(","), opt(choice!(seq!(&star_named_expressions)))))), eat_string(")"))));
    let list = list.set(choice!(seq!(eat_string("["), opt(choice!(seq!(&star_named_expressions))), eat_string("]"))));
    let strings = strings.set(choice!(seq!(repeat(choice!(seq!(&FSTRING_START, repeat(&fstring_middle), &FSTRING_END), seq!(&STRING))))));
    let string = string.set(choice!(seq!(&STRING)));
    let fstring = fstring.set(choice!(seq!(&FSTRING_START, repeat(&fstring_middle), &FSTRING_END)));
    let fstring_format_spec = fstring_format_spec.set(choice!(seq!(&FSTRING_MIDDLE),
        seq!(eat_string("{"), &annotated_rhs, opt(choice!(seq!(eat_string("=")))), opt(choice!(seq!(&fstring_conversion))), opt(choice!(seq!(&fstring_full_format_spec))), eat_string("}")),
        seq!(&invalid_replacement_field)));
    let fstring_full_format_spec = fstring_full_format_spec.set(choice!(seq!(eat_string(":"), repeat(&fstring_format_spec))));
    let fstring_conversion = fstring_conversion.set(choice!(seq!(eat_string(""), &NAME)));
    let fstring_replacement_field = fstring_replacement_field.set(choice!(seq!(eat_string("{"), &annotated_rhs, opt(choice!(seq!(eat_string("=")))), opt(choice!(seq!(&fstring_conversion))), opt(choice!(seq!(&fstring_full_format_spec))), eat_string("}")),
        seq!(&invalid_replacement_field)));
    let fstring_middle = fstring_middle.set(choice!(seq!(&fstring_replacement_field),
        seq!(&FSTRING_MIDDLE)));
    let lambda_param = lambda_param.set(choice!(seq!(&NAME)));
    let lambda_param_maybe_default = lambda_param_maybe_default.set(choice!(seq!(&lambda_param, opt(choice!(seq!(&default))), eat_string(",")),
        seq!(&lambda_param, opt(choice!(seq!(&default))))));
    let lambda_param_with_default = lambda_param_with_default.set(choice!(seq!(&lambda_param, &default, eat_string(",")),
        seq!(&lambda_param, &default)));
    let lambda_param_no_default = lambda_param_no_default.set(choice!(seq!(&lambda_param, eat_string(",")),
        seq!(&lambda_param)));
    let lambda_kwds = lambda_kwds.set(choice!(seq!(&invalid_lambda_kwds),
        seq!(eat_string("**"), &lambda_param_no_default)));
    let lambda_star_etc = lambda_star_etc.set(choice!(seq!(&invalid_lambda_star_etc),
        seq!(eat_string("*"), &lambda_param_no_default, repeat(&lambda_param_maybe_default), opt(choice!(seq!(&lambda_kwds)))),
        seq!(eat_string("*"), eat_string(","), repeat(&lambda_param_maybe_default), opt(choice!(seq!(&lambda_kwds)))),
        seq!(&lambda_kwds)));
    let lambda_slash_with_default = lambda_slash_with_default.set(choice!(seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/"), eat_string(",")),
        seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/"))));
    let lambda_slash_no_default = lambda_slash_no_default.set(choice!(seq!(repeat(&lambda_param_no_default), eat_string("/"), eat_string(",")),
        seq!(repeat(&lambda_param_no_default), eat_string("/"))));
    let lambda_parameters = lambda_parameters.set(choice!(seq!(&lambda_slash_no_default, repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), opt(choice!(seq!(&lambda_star_etc)))),
        seq!(&lambda_slash_with_default, repeat(&lambda_param_with_default), opt(choice!(seq!(&lambda_star_etc)))),
        seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), opt(choice!(seq!(&lambda_star_etc)))),
        seq!(repeat(&lambda_param_with_default), opt(choice!(seq!(&lambda_star_etc)))),
        seq!(&lambda_star_etc)));
    let lambda_params = lambda_params.set(choice!(seq!(&invalid_lambda_parameters),
        seq!(&lambda_parameters)));
    let lambdef = lambdef.set(choice!(seq!(eat_string("lambda"), opt(choice!(seq!(&lambda_params))), eat_string(":"), &expression)));
    let group = group.set(choice!(seq!(eat_string("("), choice!(seq!(&yield_expr), seq!(&named_expression)), eat_string(")")),
        seq!(&invalid_group)));
    let atom = atom.set(choice!(seq!(choice!(seq!(&NAME), seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None")), seq!(&strings), seq!(&NUMBER), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp), seq!(eat_string("..."))), opt(choice!(seq!(repeat(choice!(seq!(&strings), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp)))))))));
    let slice = slice.set(choice!(seq!(choice!(seq!(opt(choice!(seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef))), eat_string(":"), opt(choice!(seq!(&expression))), opt(choice!(seq!(eat_string(":"), opt(choice!(seq!(&expression))))))), seq!(&NAME, eat_string(":="), &expression), seq!(&invalid_named_expression), seq!(&invalid_expression), seq!(&invalid_legacy_expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&disjunction), seq!(&lambdef)), opt(choice!(seq!(repeat(choice!(seq!(eat_string(":"), opt(choice!(seq!(&expression))), opt(choice!(seq!(eat_string(":"), opt(choice!(seq!(&expression)))))))))))))));
    let slices = slices.set(choice!(seq!(&slice),
        seq!(choice!(seq!(&slice), seq!(&starred_expression)), repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let primary = primary.set(choice!(seq!(&atom, opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME), seq!(&genexp), seq!(eat_string("("), opt(choice!(seq!(&arguments))), eat_string(")")), seq!(eat_string("["), &slices, eat_string("]"))))))))));
    let await_primary = await_primary.set(choice!(seq!(eat_string("await"), &primary),
        seq!(&primary)));
    let power = power.set(choice!(seq!(&await_primary, eat_string("**"), &factor),
        seq!(&await_primary)));
    let factor = factor.set(choice!(seq!(eat_string("+"), &factor),
        seq!(eat_string("-"), &factor),
        seq!(eat_string("~"), &factor),
        seq!(&power)));
    let term = term.set(choice!(seq!(choice!(seq!(&invalid_factor), seq!(&factor)), opt(choice!(seq!(repeat(choice!(seq!(eat_string("*"), &factor), seq!(eat_string("/"), &factor), seq!(eat_string("//"), &factor), seq!(eat_string("%"), &factor), seq!(eat_string("@"), &factor)))))))));
    let sum = sum.set(choice!(seq!(&term, opt(choice!(seq!(repeat(choice!(seq!(eat_string("+"), &term), seq!(eat_string("-"), &term)))))))));
    let shift_expr = shift_expr.set(choice!(seq!(choice!(seq!(&invalid_arithmetic), seq!(&sum)), opt(choice!(seq!(repeat(choice!(seq!(eat_string("<<"), &sum), seq!(eat_string(">>"), &sum)))))))));
    let bitwise_and = bitwise_and.set(choice!(seq!(&shift_expr, opt(choice!(seq!(repeat(choice!(seq!(eat_string("&"), &shift_expr)))))))));
    let bitwise_xor = bitwise_xor.set(choice!(seq!(&bitwise_and, opt(choice!(seq!(repeat(choice!(seq!(eat_string("^"), &bitwise_and)))))))));
    let bitwise_or = bitwise_or.set(choice!(seq!(&bitwise_xor, opt(choice!(seq!(repeat(choice!(seq!(eat_string("|"), &bitwise_xor)))))))));
    let is_bitwise_or = is_bitwise_or.set(choice!(seq!(eat_string("is"), &bitwise_or)));
    let isnot_bitwise_or = isnot_bitwise_or.set(choice!(seq!(eat_string("is"), eat_string("not"), &bitwise_or)));
    let in_bitwise_or = in_bitwise_or.set(choice!(seq!(eat_string("in"), &bitwise_or)));
    let notin_bitwise_or = notin_bitwise_or.set(choice!(seq!(eat_string("not"), eat_string("in"), &bitwise_or)));
    let gt_bitwise_or = gt_bitwise_or.set(choice!(seq!(eat_string(">"), &bitwise_or)));
    let gte_bitwise_or = gte_bitwise_or.set(choice!(seq!(eat_string(">="), &bitwise_or)));
    let lt_bitwise_or = lt_bitwise_or.set(choice!(seq!(eat_string("<"), &bitwise_or)));
    let lte_bitwise_or = lte_bitwise_or.set(choice!(seq!(eat_string("<="), &bitwise_or)));
    let noteq_bitwise_or = noteq_bitwise_or.set(choice!(seq!(eat_string("!="), &bitwise_or)));
    let eq_bitwise_or = eq_bitwise_or.set(choice!(seq!(eat_string("=="), &bitwise_or)));
    let compare_op_bitwise_or_pair = compare_op_bitwise_or_pair.set(choice!(seq!(&eq_bitwise_or),
        seq!(&noteq_bitwise_or),
        seq!(&lte_bitwise_or),
        seq!(&lt_bitwise_or),
        seq!(&gte_bitwise_or),
        seq!(&gt_bitwise_or),
        seq!(&notin_bitwise_or),
        seq!(&in_bitwise_or),
        seq!(&isnot_bitwise_or),
        seq!(&is_bitwise_or)));
    let comparison = comparison.set(choice!(seq!(&bitwise_or, repeat(&compare_op_bitwise_or_pair)),
        seq!(&bitwise_or)));
    let inversion = inversion.set(choice!(seq!(eat_string("not"), &inversion),
        seq!(&comparison)));
    let conjunction = conjunction.set(choice!(seq!(&inversion, repeat(choice!(seq!(eat_string("and"), &inversion)))),
        seq!(&inversion)));
    let disjunction = disjunction.set(choice!(seq!(&conjunction, repeat(choice!(seq!(eat_string("or"), &conjunction)))),
        seq!(&conjunction)));
    let named_expression = named_expression.set(choice!(seq!(&NAME, eat_string(":="), &expression),
        seq!(&invalid_named_expression),
        seq!(&invalid_expression),
        seq!(&invalid_legacy_expression),
        seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(&disjunction),
        seq!(&lambdef)));
    let assignment_expression = assignment_expression.set(choice!(seq!(&NAME, eat_string(":="), &expression)));
    let star_named_expression = star_named_expression.set(choice!(seq!(eat_string("*"), &bitwise_or),
        seq!(&named_expression)));
    let star_named_expressions = star_named_expressions.set(choice!(seq!(&star_named_expression, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let star_expression = star_expression.set(choice!(seq!(eat_string("*"), &bitwise_or),
        seq!(&invalid_expression),
        seq!(&invalid_legacy_expression),
        seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(&disjunction),
        seq!(&lambdef)));
    let star_expressions = star_expressions.set(choice!(seq!(&star_expression, repeat(choice!(seq!(eat_string(","), &star_expression))), opt(choice!(seq!(eat_string(","))))),
        seq!(&star_expression, eat_string(",")),
        seq!(&star_expression)));
    let yield_expr = yield_expr.set(choice!(seq!(eat_string("yield"), eat_string("from"), &expression),
        seq!(eat_string("yield"), opt(choice!(seq!(&star_expressions))))));
    let expression = expression.set(choice!(seq!(&invalid_expression),
        seq!(&invalid_legacy_expression),
        seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(&disjunction),
        seq!(&lambdef)));
    let expressions = expressions.set(choice!(seq!(&expression, repeat(choice!(seq!(eat_string(","), &expression))), opt(choice!(seq!(eat_string(","))))),
        seq!(&expression, eat_string(",")),
        seq!(&expression)));
    let type_param_starred_default = type_param_starred_default.set(choice!(seq!(eat_string("="), &star_expression)));
    let type_param_default = type_param_default.set(choice!(seq!(eat_string("="), &expression)));
    let type_param_bound = type_param_bound.set(choice!(seq!(eat_string(":"), &expression)));
    let type_param = type_param.set(choice!(seq!(&NAME, opt(choice!(seq!(&type_param_bound))), opt(choice!(seq!(&type_param_default)))),
        seq!(&invalid_type_param),
        seq!(eat_string("*"), &NAME, opt(choice!(seq!(&type_param_starred_default)))),
        seq!(eat_string("**"), &NAME, opt(choice!(seq!(&type_param_default))))));
    let type_param_seq = type_param_seq.set(choice!(seq!(&type_param, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let type_params = type_params.set(choice!(seq!(&invalid_type_params),
        seq!(eat_string("["), &type_param_seq, eat_string("]"))));
    let type_alias = type_alias.set(choice!(seq!(eat_string("yp"), &NAME, opt(choice!(seq!(&type_params))), eat_string("="), &expression)));
    let keyword_pattern = keyword_pattern.set(choice!(seq!(&NAME, eat_string("="), &pattern)));
    let keyword_patterns = keyword_patterns.set(choice!(seq!(&keyword_pattern, repeat(eat_string(",")))));
    let positional_patterns = positional_patterns.set(choice!(seq!(choice!(seq!(&as_pattern), seq!(&or_pattern)), repeat(eat_string(",")))));
    let class_pattern = class_pattern.set(choice!(seq!(&NAME, opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME)))))), eat_string("("), eat_string(")")),
        seq!(&NAME, opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME)))))), eat_string("("), &positional_patterns, opt(choice!(seq!(eat_string(",")))), eat_string(")")),
        seq!(&NAME, opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME)))))), eat_string("("), &keyword_patterns, opt(choice!(seq!(eat_string(",")))), eat_string(")")),
        seq!(&NAME, opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME)))))), eat_string("("), &positional_patterns, eat_string(","), &keyword_patterns, opt(choice!(seq!(eat_string(",")))), eat_string(")")),
        seq!(&invalid_class_pattern)));
    let double_star_pattern = double_star_pattern.set(choice!(seq!(eat_string("**"), &pattern_capture_target)));
    let key_value_pattern = key_value_pattern.set(choice!(seq!(choice!(seq!(&signed_number), seq!(&complex_number), seq!(&strings), seq!(eat_string("None")), seq!(eat_string("True")), seq!(eat_string("False")), seq!(&name_or_attr, eat_string("."), &NAME)), eat_string(":"), &pattern)));
    let items_pattern = items_pattern.set(choice!(seq!(&key_value_pattern, repeat(eat_string(",")))));
    let mapping_pattern = mapping_pattern.set(choice!(seq!(eat_string("{"), eat_string("}")),
        seq!(eat_string("{"), &double_star_pattern, opt(choice!(seq!(eat_string(",")))), eat_string("}")),
        seq!(eat_string("{"), &items_pattern, eat_string(","), &double_star_pattern, opt(choice!(seq!(eat_string(",")))), eat_string("}")),
        seq!(eat_string("{"), &items_pattern, opt(choice!(seq!(eat_string(",")))), eat_string("}"))));
    let star_pattern = star_pattern.set(choice!(seq!(eat_string("*"), &pattern_capture_target),
        seq!(eat_string("*"), &wildcard_pattern)));
    let maybe_star_pattern = maybe_star_pattern.set(choice!(seq!(&star_pattern),
        seq!(&as_pattern),
        seq!(&or_pattern)));
    let maybe_sequence_pattern = maybe_sequence_pattern.set(choice!(seq!(&maybe_star_pattern, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))))));
    let open_sequence_pattern = open_sequence_pattern.set(choice!(seq!(&maybe_star_pattern, eat_string(","), opt(choice!(seq!(&maybe_sequence_pattern))))));
    let sequence_pattern = sequence_pattern.set(choice!(seq!(eat_string("["), opt(choice!(seq!(&maybe_sequence_pattern))), eat_string("]")),
        seq!(eat_string("("), opt(choice!(seq!(&open_sequence_pattern))), eat_string(")"))));
    let group_pattern = group_pattern.set(choice!(seq!(eat_string("("), &pattern, eat_string(")"))));
    let name_or_attr = name_or_attr.set(choice!(seq!(&NAME, opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME)))))))));
    let attr = attr.set(choice!(seq!(&name_or_attr, eat_string("."), &NAME)));
    let value_pattern = value_pattern.set(choice!(seq!(&attr)));
    let wildcard_pattern = wildcard_pattern.set(choice!(seq!(eat_string(""))));
    let pattern_capture_target = pattern_capture_target.set(choice!(seq!(repeat(&NAME))));
    let capture_pattern = capture_pattern.set(choice!(seq!(&pattern_capture_target)));
    let imaginary_number = imaginary_number.set(choice!(seq!(&NUMBER)));
    let real_number = real_number.set(choice!(seq!(&NUMBER)));
    let signed_real_number = signed_real_number.set(choice!(seq!(&real_number),
        seq!(eat_string("-"), &real_number)));
    let signed_number = signed_number.set(choice!(seq!(&NUMBER),
        seq!(eat_string("-"), &NUMBER)));
    let complex_number = complex_number.set(choice!(seq!(&signed_real_number, eat_string("+"), &imaginary_number),
        seq!(&signed_real_number, eat_string("-"), &imaginary_number)));
    let literal_expr = literal_expr.set(choice!(seq!(&signed_number),
        seq!(&complex_number),
        seq!(&strings),
        seq!(eat_string("None")),
        seq!(eat_string("True")),
        seq!(eat_string("False"))));
    let literal_pattern = literal_pattern.set(choice!(seq!(&signed_number),
        seq!(&complex_number),
        seq!(&strings),
        seq!(eat_string("None")),
        seq!(eat_string("True")),
        seq!(eat_string("False"))));
    let closed_pattern = closed_pattern.set(choice!(seq!(&literal_pattern),
        seq!(&capture_pattern),
        seq!(&wildcard_pattern),
        seq!(&value_pattern),
        seq!(&group_pattern),
        seq!(&sequence_pattern),
        seq!(&mapping_pattern),
        seq!(&class_pattern)));
    let or_pattern = or_pattern.set(choice!(seq!(&closed_pattern, repeat(eat_string("|")))));
    let as_pattern = as_pattern.set(choice!(seq!(&or_pattern, eat_string("as"), &pattern_capture_target),
        seq!(&invalid_as_pattern)));
    let pattern = pattern.set(choice!(seq!(&as_pattern),
        seq!(&or_pattern)));
    let patterns = patterns.set(choice!(seq!(&open_sequence_pattern),
        seq!(&pattern)));
    let guard = guard.set(choice!(seq!(eat_string("if"), &named_expression)));
    let case_block = case_block.set(choice!(seq!(&invalid_case_block),
        seq!(eat_string("as"), &patterns, opt(choice!(seq!(&guard))), eat_string(":"), &block)));
    let subject_expr = subject_expr.set(choice!(seq!(&star_named_expression, eat_string(","), opt(choice!(seq!(&star_named_expressions)))),
        seq!(&named_expression)));
    let match_stmt = match_stmt.set(choice!(seq!(eat_string("atc"), &subject_expr, eat_string(":"), &NEWLINE, &INDENT, repeat(&case_block), &DEDENT),
        seq!(&invalid_match_stmt)));
    let finally_block = finally_block.set(choice!(seq!(&invalid_finally_stmt),
        seq!(eat_string("finally"), eat_string(":"), &block)));
    let except_star_block = except_star_block.set(choice!(seq!(&invalid_except_star_stmt_indent),
        seq!(eat_string("except"), eat_string("*"), &expression, opt(choice!(seq!(eat_string("as"), &NAME))), eat_string(":"), &block),
        seq!(&invalid_except_stmt)));
    let except_block = except_block.set(choice!(seq!(&invalid_except_stmt_indent),
        seq!(eat_string("except"), &expression, opt(choice!(seq!(eat_string("as"), &NAME))), eat_string(":"), &block),
        seq!(eat_string("except"), eat_string(":"), &block),
        seq!(&invalid_except_stmt)));
    let try_stmt = try_stmt.set(choice!(seq!(&invalid_try_stmt),
        seq!(eat_string("try"), eat_string(":"), &block, &finally_block),
        seq!(eat_string("try"), eat_string(":"), &block, repeat(&except_block), opt(choice!(seq!(&else_block))), opt(choice!(seq!(&finally_block)))),
        seq!(eat_string("try"), eat_string(":"), &block, repeat(&except_star_block), opt(choice!(seq!(&else_block))), opt(choice!(seq!(&finally_block))))));
    let with_item = with_item.set(choice!(seq!(&expression, eat_string("as"), &star_target),
        seq!(&invalid_with_item),
        seq!(&expression)));
    let with_stmt = with_stmt.set(choice!(seq!(&invalid_with_stmt_indent),
        seq!(eat_string("with"), eat_string("("), &with_item, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))), eat_string(")"), eat_string(":"), opt(choice!(seq!(&TYPE_COMMENT))), &block),
        seq!(eat_string("with"), &with_item, repeat(eat_string(",")), eat_string(":"), opt(choice!(seq!(&TYPE_COMMENT))), &block),
        seq!(eat_string("async"), eat_string("with"), eat_string("("), &with_item, repeat(eat_string(",")), opt(choice!(seq!(eat_string(",")))), eat_string(")"), eat_string(":"), &block),
        seq!(eat_string("async"), eat_string("with"), &with_item, repeat(eat_string(",")), eat_string(":"), opt(choice!(seq!(&TYPE_COMMENT))), &block),
        seq!(&invalid_with_stmt)));
    let for_stmt = for_stmt.set(choice!(seq!(&invalid_for_stmt),
        seq!(eat_string("for"), &star_targets, eat_string("in"), &star_expressions, eat_string(":"), opt(choice!(seq!(&TYPE_COMMENT))), &block, opt(choice!(seq!(&else_block)))),
        seq!(eat_string("async"), eat_string("for"), &star_targets, eat_string("in"), &star_expressions, eat_string(":"), opt(choice!(seq!(&TYPE_COMMENT))), &block, opt(choice!(seq!(&else_block)))),
        seq!(&invalid_for_target)));
    let while_stmt = while_stmt.set(choice!(seq!(&invalid_while_stmt),
        seq!(eat_string("while"), &named_expression, eat_string(":"), &block, opt(choice!(seq!(&else_block))))));
    let else_block = else_block.set(choice!(seq!(&invalid_else_stmt),
        seq!(eat_string("else"), eat_string(":"), &block)));
    let elif_stmt = elif_stmt.set(choice!(seq!(&invalid_elif_stmt),
        seq!(eat_string("elif"), &named_expression, eat_string(":"), &block, &elif_stmt),
        seq!(eat_string("elif"), &named_expression, eat_string(":"), &block, opt(choice!(seq!(&else_block))))));
    let if_stmt = if_stmt.set(choice!(seq!(&invalid_if_stmt),
        seq!(eat_string("if"), &named_expression, eat_string(":"), &block, &elif_stmt),
        seq!(eat_string("if"), &named_expression, eat_string(":"), &block, opt(choice!(seq!(&else_block))))));
    let default = default.set(choice!(seq!(eat_string("="), &expression),
        seq!(&invalid_default)));
    let star_annotation = star_annotation.set(choice!(seq!(eat_string(":"), &star_expression)));
    let annotation = annotation.set(choice!(seq!(eat_string(":"), &expression)));
    let param_star_annotation = param_star_annotation.set(choice!(seq!(&NAME, &star_annotation)));
    let param = param.set(choice!(seq!(&NAME, opt(choice!(seq!(&annotation))))));
    let param_maybe_default = param_maybe_default.set(choice!(seq!(&param, opt(choice!(seq!(&default))), eat_string(","), opt(choice!(seq!(&TYPE_COMMENT)))),
        seq!(&param, opt(choice!(seq!(&default))), opt(choice!(seq!(&TYPE_COMMENT))))));
    let param_with_default = param_with_default.set(choice!(seq!(&param, &default, eat_string(","), opt(choice!(seq!(&TYPE_COMMENT)))),
        seq!(&param, &default, opt(choice!(seq!(&TYPE_COMMENT))))));
    let param_no_default_star_annotation = param_no_default_star_annotation.set(choice!(seq!(&param_star_annotation, eat_string(","), opt(choice!(seq!(&TYPE_COMMENT)))),
        seq!(&param_star_annotation, opt(choice!(seq!(&TYPE_COMMENT))))));
    let param_no_default = param_no_default.set(choice!(seq!(&param, eat_string(","), opt(choice!(seq!(&TYPE_COMMENT)))),
        seq!(&param, opt(choice!(seq!(&TYPE_COMMENT))))));
    let kwds = kwds.set(choice!(seq!(&invalid_kwds),
        seq!(eat_string("**"), &param_no_default)));
    let star_etc = star_etc.set(choice!(seq!(&invalid_star_etc),
        seq!(eat_string("*"), &param_no_default, repeat(&param_maybe_default), opt(choice!(seq!(&kwds)))),
        seq!(eat_string("*"), &param_no_default_star_annotation, repeat(&param_maybe_default), opt(choice!(seq!(&kwds)))),
        seq!(eat_string("*"), eat_string(","), repeat(&param_maybe_default), opt(choice!(seq!(&kwds)))),
        seq!(&kwds)));
    let slash_with_default = slash_with_default.set(choice!(seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/"), eat_string(",")),
        seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/"))));
    let slash_no_default = slash_no_default.set(choice!(seq!(repeat(&param_no_default), eat_string("/"), eat_string(",")),
        seq!(repeat(&param_no_default), eat_string("/"))));
    let parameters = parameters.set(choice!(seq!(&slash_no_default, repeat(&param_no_default), repeat(&param_with_default), opt(choice!(seq!(&star_etc)))),
        seq!(&slash_with_default, repeat(&param_with_default), opt(choice!(seq!(&star_etc)))),
        seq!(repeat(&param_no_default), repeat(&param_with_default), opt(choice!(seq!(&star_etc)))),
        seq!(repeat(&param_with_default), opt(choice!(seq!(&star_etc)))),
        seq!(&star_etc)));
    let params = params.set(choice!(seq!(&invalid_parameters),
        seq!(&parameters)));
    let function_def_raw = function_def_raw.set(choice!(seq!(&invalid_def_raw),
        seq!(eat_string("def"), &NAME, opt(choice!(seq!(&type_params))), eat_string("("), opt(choice!(seq!(&params))), eat_string(")"), opt(choice!(seq!(eat_string("->"), &expression))), eat_string(":"), opt(choice!(seq!(&func_type_comment))), &block),
        seq!(eat_string("async"), eat_string("def"), &NAME, opt(choice!(seq!(&type_params))), eat_string("("), opt(choice!(seq!(&params))), eat_string(")"), opt(choice!(seq!(eat_string("->"), &expression))), eat_string(":"), opt(choice!(seq!(&func_type_comment))), &block)));
    let function_def = function_def.set(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE, opt(choice!(seq!(repeat(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE)))))), &function_def_raw),
        seq!(&function_def_raw)));
    let class_def_raw = class_def_raw.set(choice!(seq!(&invalid_class_def_raw),
        seq!(eat_string("class"), &NAME, opt(choice!(seq!(&type_params))), opt(choice!(seq!(eat_string("("), opt(choice!(seq!(&arguments))), eat_string(")")))), eat_string(":"), &block)));
    let class_def = class_def.set(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE, opt(choice!(seq!(repeat(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE)))))), &class_def_raw),
        seq!(&class_def_raw)));
    let decorators = decorators.set(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE, opt(choice!(seq!(repeat(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE)))))))));
    let block = block.set(choice!(seq!(&NEWLINE, &INDENT, &statements, &DEDENT),
        seq!(&simple_stmt, &NEWLINE),
        seq!(&simple_stmt, repeat(eat_string(";")), opt(choice!(seq!(eat_string(";")))), &NEWLINE),
        seq!(&invalid_block)));
    let dotted_name = dotted_name.set(choice!(seq!(&NAME, opt(choice!(seq!(repeat(choice!(seq!(eat_string("."), &NAME)))))))));
    let dotted_as_name = dotted_as_name.set(choice!(seq!(&dotted_name, opt(choice!(seq!(eat_string("as"), &NAME))))));
    let dotted_as_names = dotted_as_names.set(choice!(seq!(&dotted_as_name, repeat(eat_string(",")))));
    let import_from_as_name = import_from_as_name.set(choice!(seq!(&NAME, opt(choice!(seq!(eat_string("as"), &NAME))))));
    let import_from_as_names = import_from_as_names.set(choice!(seq!(&import_from_as_name, repeat(eat_string(",")))));
    let import_from_targets = import_from_targets.set(choice!(seq!(eat_string("("), &import_from_as_names, opt(choice!(seq!(eat_string(",")))), eat_string(")")),
        seq!(&import_from_as_names),
        seq!(eat_string("*")),
        seq!(&invalid_import_from_targets)));
    let import_from = import_from.set(choice!(seq!(eat_string("from"), repeat(choice!(seq!(eat_string(".")), seq!(eat_string("...")))), &dotted_name, eat_string("import"), &import_from_targets),
        seq!(eat_string("from"), repeat(choice!(seq!(eat_string(".")), seq!(eat_string("...")))), eat_string("import"), &import_from_targets)));
    let import_name = import_name.set(choice!(seq!(eat_string("import"), &dotted_as_names)));
    let import_stmt = import_stmt.set(choice!(seq!(&invalid_import),
        seq!(&import_name),
        seq!(&import_from)));
    let assert_stmt = assert_stmt.set(choice!(seq!(eat_string("assert"), &expression, opt(choice!(seq!(eat_string(","), &expression))))));
    let yield_stmt = yield_stmt.set(choice!(seq!(&yield_expr)));
    let del_stmt = del_stmt.set(choice!(seq!(eat_string("del"), &del_targets),
        seq!(&invalid_del_stmt)));
    let nonlocal_stmt = nonlocal_stmt.set(choice!(seq!(eat_string("nonlocal"), &NAME, repeat(eat_string(",")))));
    let global_stmt = global_stmt.set(choice!(seq!(eat_string("global"), &NAME, repeat(eat_string(",")))));
    let raise_stmt = raise_stmt.set(choice!(seq!(eat_string("raise"), &expression, opt(choice!(seq!(eat_string("from"), &expression)))),
        seq!(eat_string("raise"))));
    let return_stmt = return_stmt.set(choice!(seq!(eat_string("return"), opt(choice!(seq!(&star_expressions))))));
    let augassign = augassign.set(choice!(seq!(eat_string("+=")),
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
    let annotated_rhs = annotated_rhs.set(choice!(seq!(&yield_expr),
        seq!(&star_expressions)));
    let assignment = assignment.set(choice!(seq!(&NAME, eat_string(":"), &expression, opt(choice!(seq!(eat_string("="), &annotated_rhs)))),
        seq!(choice!(seq!(eat_string("("), &single_target, eat_string(")")), seq!(&single_subscript_attribute_target)), eat_string(":"), &expression, opt(choice!(seq!(eat_string("="), &annotated_rhs)))),
        seq!(&star_targets, eat_string("="), opt(choice!(seq!(repeat(choice!(seq!(&star_targets, eat_string("="))))))), choice!(seq!(&yield_expr), seq!(&star_expressions)), opt(choice!(seq!(&TYPE_COMMENT)))),
        seq!(&single_target, &augassign, choice!(seq!(&yield_expr), seq!(&star_expressions))),
        seq!(&invalid_assignment)));
    let compound_stmt = compound_stmt.set(choice!(seq!(choice!(seq!(&function_def), seq!(&if_stmt), seq!(&class_def), seq!(&with_stmt), seq!(&for_stmt), seq!(&try_stmt), seq!(&while_stmt), seq!(&match_stmt)), opt(choice!(seq!(repeat(choice!(seq!(&function_def), seq!(&if_stmt), seq!(&class_def), seq!(&with_stmt), seq!(&for_stmt), seq!(&try_stmt), seq!(&while_stmt)))))))));
    let simple_stmt = simple_stmt.set(choice!(seq!(choice!(seq!(&assignment), seq!(&type_alias), seq!(&star_expressions), seq!(&return_stmt), seq!(&import_stmt), seq!(&raise_stmt), seq!(eat_string("pass")), seq!(&del_stmt), seq!(&yield_stmt), seq!(&assert_stmt), seq!(eat_string("break")), seq!(eat_string("continue")), seq!(&global_stmt), seq!(&nonlocal_stmt)), opt(choice!(seq!(repeat(choice!(seq!(&type_alias), seq!(&return_stmt), seq!(&import_stmt), seq!(&raise_stmt), seq!(&del_stmt), seq!(&yield_stmt), seq!(&assert_stmt), seq!(&global_stmt), seq!(&nonlocal_stmt)))))))));
    let simple_stmts = simple_stmts.set(choice!(seq!(&simple_stmt, &NEWLINE),
        seq!(&simple_stmt, repeat(eat_string(";")), opt(choice!(seq!(eat_string(";")))), &NEWLINE)));
    let statement_newline = statement_newline.set(choice!(seq!(&compound_stmt, &NEWLINE),
        seq!(&simple_stmts),
        seq!(&NEWLINE),
        seq!(&ENDMARKER)));
    let statement = statement.set(choice!(seq!(&compound_stmt),
        seq!(&simple_stmts)));
    let statements = statements.set(choice!(seq!(repeat(&statement))));
    let func_type = func_type.set(choice!(seq!(eat_string("("), opt(choice!(seq!(&type_expressions))), eat_string(")"), eat_string("->"), &expression, repeat(&NEWLINE), &ENDMARKER)));
    let eval = eval.set(choice!(seq!(&expressions, repeat(&NEWLINE), &ENDMARKER)));
    let interactive = interactive.set(choice!(seq!(&statement_newline)));
    let file = file.set(choice!(seq!(opt(choice!(seq!(&statements))), repeat(&ENDMARKER))));
    file.into_boxed().into()
}
