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

    let mut expression_without_invalid = forward_ref();
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
    let expression_without_invalid = Rc::new(expression_without_invalid.set(choice!(
        seq!(&conjunction, repeat(choice!(seq!(eat_string("or"), &conjunction))), eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(eat_string("lambda"), &lambda_params, eat_string(":"), &expression)
    )).into_boxed());
    let func_type_comment = Rc::new(func_type_comment.set(choice!(
        seq!(&NEWLINE, &TYPE_COMMENT),
        seq!(&TYPE_COMMENT)
    )).into_boxed());
    let type_expressions = Rc::new(type_expressions.set(choice!(
        seq!(choice!(seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&lambdef)), repeat(eat_string(",")), choice!(seq!(eat_string("*"), &expression, eat_string(","), eat_string("**"), &expression), seq!(eat_string("**"), &expression))),
        seq!(eat_string("*"), &expression, eat_string(","), eat_string("**"), &expression),
        seq!(eat_string("**"), &expression)
    )).into_boxed());
    let del_t_atom = Rc::new(del_t_atom.set(choice!(
        seq!(&NAME),
        seq!(eat_string("("), choice!(seq!(&del_target, eat_string(")")), seq!(&del_targets, eat_string(")")))),
        seq!(eat_string("["), &del_targets, eat_string("]"))
    )).into_boxed());
    let del_target = Rc::new(del_target.set(choice!(
        seq!(choice!(seq!(&NAME), seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None")), seq!(&strings), seq!(&NUMBER), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp), seq!(eat_string("..."))), repeat(choice!(seq!(&strings), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp))), repeat(choice!(seq!(eat_string("."), &NAME), seq!(eat_string("["), &slices, eat_string("]")), seq!(&genexp), seq!(eat_string("("), &arguments, eat_string(")")))), choice!(seq!(eat_string("."), &NAME), seq!(eat_string("["), &slices, eat_string("]")))),
        seq!(&del_t_atom)
    )).into_boxed());
    let del_targets = Rc::new(del_targets.set(choice!(
        seq!(&del_target, repeat(eat_string(",")))
    )).into_boxed());
    let t_lookahead = Rc::new(t_lookahead.set(choice!(
        seq!(eat_string("(")),
        seq!(eat_string("[")),
        seq!(eat_string("."))
    )).into_boxed());
    let t_primary = Rc::new(t_primary.set(choice!(
        seq!(choice!(seq!(&NAME), seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None")), seq!(&strings), seq!(&NUMBER), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp), seq!(eat_string("..."))), repeat(choice!(seq!(&strings), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp))), repeat(choice!(seq!(eat_string("."), &NAME), seq!(eat_string("["), &slices, eat_string("]")), seq!(&genexp), seq!(eat_string("("), &arguments, eat_string(")")))))
    )).into_boxed());
    let single_subscript_attribute_target = Rc::new(single_subscript_attribute_target.set(choice!(
        seq!(&t_primary, choice!(seq!(eat_string("."), &NAME), seq!(eat_string("["), &slices, eat_string("]"))))
    )).into_boxed());
    let single_target = Rc::new(single_target.set(choice!(
        seq!(&single_subscript_attribute_target),
        seq!(&NAME),
        seq!(eat_string("("), &single_target, eat_string(")"))
    )).into_boxed());
    let star_atom = Rc::new(star_atom.set(choice!(
        seq!(&NAME),
        seq!(eat_string("("), choice!(seq!(&target_with_star_atom, eat_string(")")), seq!(&star_targets_tuple_seq, eat_string(")")))),
        seq!(eat_string("["), &star_targets_list_seq, eat_string("]"))
    )).into_boxed());
    let target_with_star_atom = Rc::new(target_with_star_atom.set(choice!(
        seq!(&t_primary, choice!(seq!(eat_string("."), &NAME), seq!(eat_string("["), &slices, eat_string("]")))),
        seq!(&star_atom)
    )).into_boxed());
    let star_target = Rc::new(star_target.set(choice!(
        seq!(eat_string("*"), &star_target),
        seq!(&target_with_star_atom)
    )).into_boxed());
    let star_targets_tuple_seq = Rc::new(star_targets_tuple_seq.set(choice!(
        seq!(&star_target, choice!(seq!(repeat(choice!(seq!(eat_string(","), &star_target))), eat_string(",")), seq!(eat_string(","))))
    )).into_boxed());
    let star_targets_list_seq = Rc::new(star_targets_list_seq.set(choice!(
        seq!(&star_target, repeat(eat_string(",")))
    )).into_boxed());
    let star_targets = Rc::new(star_targets.set(choice!(
        seq!(&star_target, repeat(choice!(seq!(eat_string(","), &star_target))), eat_string(","))
    )).into_boxed());
    let kwarg_or_double_starred = Rc::new(kwarg_or_double_starred.set(choice!(
        seq!(&NAME, eat_string("="), &expression),
        seq!(eat_string("**"), &expression)
    )).into_boxed());
    let kwarg_or_starred = Rc::new(kwarg_or_starred.set(choice!(
        seq!(&NAME, eat_string("="), &expression),
        seq!(eat_string("*"), &expression)
    )).into_boxed());
    let starred_expression = Rc::new(starred_expression.set(choice!(
        seq!(eat_string("*"), &expression)
    )).into_boxed());
    let kwargs = Rc::new(kwargs.set(choice!(
        seq!(&kwarg_or_starred, repeat(eat_string(",")), &kwarg_or_double_starred, repeat(eat_string(","))),
        seq!(&kwarg_or_double_starred, repeat(eat_string(",")))
    )).into_boxed());
    let args = Rc::new(args.set(choice!(
        seq!(choice!(seq!(&starred_expression), seq!(&NAME, eat_string(":="), &expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&lambdef)), repeat(eat_string(",")), &kwargs),
        seq!(&kwargs)
    )).into_boxed());
    let arguments = Rc::new(arguments.set(choice!(
        seq!(&args, eat_string(","))
    )).into_boxed());
    let dictcomp = Rc::new(dictcomp.set(choice!(
        seq!(eat_string("{"), &kvpair, &for_if_clauses, eat_string("}"))
    )).into_boxed());
    let genexp = Rc::new(genexp.set(choice!(
        seq!(eat_string("("), choice!(seq!(&assignment_expression), seq!(&expression)), &for_if_clauses, eat_string(")"))
    )).into_boxed());
    let setcomp = Rc::new(setcomp.set(choice!(
        seq!(eat_string("{"), &named_expression, &for_if_clauses, eat_string("}"))
    )).into_boxed());
    let listcomp = Rc::new(listcomp.set(choice!(
        seq!(eat_string("["), &named_expression, &for_if_clauses, eat_string("]"))
    )).into_boxed());
    let for_if_clause = Rc::new(for_if_clause.set(choice!(
        seq!(eat_string("async"), eat_string("for"), &star_targets, eat_string("in"), &disjunction, repeat(choice!(seq!(eat_string("if"), &disjunction)))),
        seq!(eat_string("for"), &star_targets, eat_string("in"), &disjunction, repeat(choice!(seq!(eat_string("if"), &disjunction))))
    )).into_boxed());
    let for_if_clauses = Rc::new(for_if_clauses.set(choice!(
        seq!(repeat(&for_if_clause))
    )).into_boxed());
    let kvpair = Rc::new(kvpair.set(choice!(
        seq!(choice!(seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&lambdef)), eat_string(":"), &expression)
    )).into_boxed());
    let double_starred_kvpair = Rc::new(double_starred_kvpair.set(choice!(
        seq!(eat_string("**"), &bitwise_or),
        seq!(&kvpair)
    )).into_boxed());
    let double_starred_kvpairs = Rc::new(double_starred_kvpairs.set(choice!(
        seq!(&double_starred_kvpair, repeat(eat_string(",")))
    )).into_boxed());
    let dict = Rc::new(dict.set(choice!(
        seq!(eat_string("{"), &double_starred_kvpairs, eat_string("}"))
    )).into_boxed());
    let set = Rc::new(set.set(choice!(
        seq!(eat_string("{"), &star_named_expressions, eat_string("}"))
    )).into_boxed());
    let tuple = Rc::new(tuple.set(choice!(
        seq!(eat_string("("), &star_named_expression, eat_string(","), &star_named_expressions, eat_string(")"))
    )).into_boxed());
    let list = Rc::new(list.set(choice!(
        seq!(eat_string("["), &star_named_expressions, eat_string("]"))
    )).into_boxed());
    let strings = Rc::new(strings.set(choice!(
        seq!(repeat(choice!(seq!(&FSTRING_START, repeat(&fstring_middle), &FSTRING_END), seq!(&STRING))))
    )).into_boxed());
    let string = Rc::new(string.set(choice!(
        seq!(&STRING)
    )).into_boxed());
    let fstring = Rc::new(fstring.set(choice!(
        seq!(&FSTRING_START, repeat(&fstring_middle), &FSTRING_END)
    )).into_boxed());
    let fstring_format_spec = Rc::new(fstring_format_spec.set(choice!(
        seq!(&FSTRING_MIDDLE),
        seq!(eat_string("{"), &annotated_rhs, eat_string("="), &fstring_conversion, &fstring_full_format_spec, eat_string("}"))
    )).into_boxed());
    let fstring_full_format_spec = Rc::new(fstring_full_format_spec.set(choice!(
        seq!(eat_string(":"), repeat(&fstring_format_spec))
    )).into_boxed());
    let fstring_conversion = Rc::new(fstring_conversion.set(choice!(
        seq!(eat_string(""), &NAME)
    )).into_boxed());
    let fstring_replacement_field = Rc::new(fstring_replacement_field.set(choice!(
        seq!(eat_string("{"), &annotated_rhs, eat_string("="), &fstring_conversion, &fstring_full_format_spec, eat_string("}"))
    )).into_boxed());
    let fstring_middle = Rc::new(fstring_middle.set(choice!(
        seq!(&fstring_replacement_field),
        seq!(&FSTRING_MIDDLE)
    )).into_boxed());
    let lambda_param = Rc::new(lambda_param.set(choice!(
        seq!(&NAME)
    )).into_boxed());
    let lambda_param_maybe_default = Rc::new(lambda_param_maybe_default.set(choice!(
        seq!(&lambda_param, &default, eat_string(","))
    )).into_boxed());
    let lambda_param_with_default = Rc::new(lambda_param_with_default.set(choice!(
        seq!(&lambda_param, &default, eat_string(","))
    )).into_boxed());
    let lambda_param_no_default = Rc::new(lambda_param_no_default.set(choice!(
        seq!(&lambda_param, eat_string(","))
    )).into_boxed());
    let lambda_kwds = Rc::new(lambda_kwds.set(choice!(
        seq!(eat_string("**"), &lambda_param_no_default)
    )).into_boxed());
    let lambda_star_etc = Rc::new(lambda_star_etc.set(choice!(
        seq!(eat_string("*"), choice!(seq!(&lambda_param_no_default, repeat(&lambda_param_maybe_default), &lambda_kwds), seq!(eat_string(","), repeat(&lambda_param_maybe_default), &lambda_kwds))),
        seq!(&lambda_kwds)
    )).into_boxed());
    let lambda_slash_with_default = Rc::new(lambda_slash_with_default.set(choice!(
        seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), eat_string("/"), eat_string(","))
    )).into_boxed());
    let lambda_slash_no_default = Rc::new(lambda_slash_no_default.set(choice!(
        seq!(repeat(&lambda_param_no_default), eat_string("/"), eat_string(","))
    )).into_boxed());
    let lambda_parameters = Rc::new(lambda_parameters.set(choice!(
        seq!(&lambda_slash_no_default, repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), &lambda_star_etc),
        seq!(&lambda_slash_with_default, repeat(&lambda_param_with_default), &lambda_star_etc),
        seq!(repeat(&lambda_param_no_default), repeat(&lambda_param_with_default), &lambda_star_etc),
        seq!(repeat(&lambda_param_with_default), &lambda_star_etc),
        seq!(&lambda_star_etc)
    )).into_boxed());
    let lambda_params = Rc::new(lambda_params.set(choice!(
        seq!(&lambda_parameters)
    )).into_boxed());
    let lambdef = Rc::new(lambdef.set(choice!(
        seq!(eat_string("lambda"), &lambda_params, eat_string(":"), &expression)
    )).into_boxed());
    let group = Rc::new(group.set(choice!(
        seq!(eat_string("("), choice!(seq!(&yield_expr), seq!(&named_expression)), eat_string(")"))
    )).into_boxed());
    let atom = Rc::new(atom.set(choice!(
        seq!(choice!(seq!(&NAME), seq!(eat_string("True")), seq!(eat_string("False")), seq!(eat_string("None")), seq!(&strings), seq!(&NUMBER), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp), seq!(eat_string("..."))), repeat(choice!(seq!(&strings), seq!(&tuple), seq!(&group), seq!(&genexp), seq!(&list), seq!(&listcomp), seq!(&dict), seq!(&set), seq!(&dictcomp), seq!(&setcomp))))
    )).into_boxed());
    let slice = Rc::new(slice.set(choice!(
        seq!(choice!(seq!(choice!(seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&lambdef)), eat_string(":"), &expression, eat_string(":"), &expression), seq!(&NAME, eat_string(":="), &expression), seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression), seq!(&lambdef)), repeat(choice!(seq!(eat_string(":"), &expression, eat_string(":"), &expression))))
    )).into_boxed());
    let slices = Rc::new(slices.set(choice!(
        seq!(&slice),
        seq!(choice!(seq!(&slice), seq!(&starred_expression)), repeat(eat_string(",")))
    )).into_boxed());
    let primary = Rc::new(primary.set(choice!(
        seq!(&atom, repeat(choice!(seq!(eat_string("."), &NAME), seq!(&genexp), seq!(eat_string("("), &arguments, eat_string(")")), seq!(eat_string("["), &slices, eat_string("]")))))
    )).into_boxed());
    let await_primary = Rc::new(await_primary.set(choice!(
        seq!(eat_string("await"), &primary),
        seq!(&primary)
    )).into_boxed());
    let power = Rc::new(power.set(choice!(
        seq!(&await_primary, eat_string("**"), &factor)
    )).into_boxed());
    let factor = Rc::new(factor.set(choice!(
        seq!(eat_string("+"), &factor),
        seq!(eat_string("-"), &factor),
        seq!(eat_string("~"), &factor),
        seq!(&power)
    )).into_boxed());
    let term = Rc::new(term.set(choice!(
        seq!(&factor, repeat(choice!(seq!(eat_string("*"), &factor), seq!(eat_string("/"), &factor), seq!(eat_string("//"), &factor), seq!(eat_string("%"), &factor), seq!(eat_string("@"), &factor))))
    )).into_boxed());
    let sum = Rc::new(sum.set(choice!(
        seq!(&term, repeat(choice!(seq!(eat_string("+"), &term), seq!(eat_string("-"), &term))))
    )).into_boxed());
    let shift_expr = Rc::new(shift_expr.set(choice!(
        seq!(&sum, repeat(choice!(seq!(eat_string("<<"), &sum), seq!(eat_string(">>"), &sum))))
    )).into_boxed());
    let bitwise_and = Rc::new(bitwise_and.set(choice!(
        seq!(&shift_expr, repeat(choice!(seq!(eat_string("&"), &shift_expr))))
    )).into_boxed());
    let bitwise_xor = Rc::new(bitwise_xor.set(choice!(
        seq!(&bitwise_and, repeat(choice!(seq!(eat_string("^"), &bitwise_and))))
    )).into_boxed());
    let bitwise_or = Rc::new(bitwise_or.set(choice!(
        seq!(&bitwise_xor, repeat(choice!(seq!(eat_string("|"), &bitwise_xor))))
    )).into_boxed());
    let is_bitwise_or = Rc::new(is_bitwise_or.set(choice!(
        seq!(eat_string("is"), &bitwise_or)
    )).into_boxed());
    let isnot_bitwise_or = Rc::new(isnot_bitwise_or.set(choice!(
        seq!(eat_string("is"), eat_string("not"), &bitwise_or)
    )).into_boxed());
    let in_bitwise_or = Rc::new(in_bitwise_or.set(choice!(
        seq!(eat_string("in"), &bitwise_or)
    )).into_boxed());
    let notin_bitwise_or = Rc::new(notin_bitwise_or.set(choice!(
        seq!(eat_string("not"), eat_string("in"), &bitwise_or)
    )).into_boxed());
    let gt_bitwise_or = Rc::new(gt_bitwise_or.set(choice!(
        seq!(eat_string(">"), &bitwise_or)
    )).into_boxed());
    let gte_bitwise_or = Rc::new(gte_bitwise_or.set(choice!(
        seq!(eat_string(">="), &bitwise_or)
    )).into_boxed());
    let lt_bitwise_or = Rc::new(lt_bitwise_or.set(choice!(
        seq!(eat_string("<"), &bitwise_or)
    )).into_boxed());
    let lte_bitwise_or = Rc::new(lte_bitwise_or.set(choice!(
        seq!(eat_string("<="), &bitwise_or)
    )).into_boxed());
    let noteq_bitwise_or = Rc::new(noteq_bitwise_or.set(choice!(
        seq!(eat_string("!="), &bitwise_or)
    )).into_boxed());
    let eq_bitwise_or = Rc::new(eq_bitwise_or.set(choice!(
        seq!(eat_string("=="), &bitwise_or)
    )).into_boxed());
    let compare_op_bitwise_or_pair = Rc::new(compare_op_bitwise_or_pair.set(choice!(
        seq!(&eq_bitwise_or),
        seq!(&noteq_bitwise_or),
        seq!(&lte_bitwise_or),
        seq!(&lt_bitwise_or),
        seq!(&gte_bitwise_or),
        seq!(&gt_bitwise_or),
        seq!(&notin_bitwise_or),
        seq!(&in_bitwise_or),
        seq!(&isnot_bitwise_or),
        seq!(&is_bitwise_or)
    )).into_boxed());
    let comparison = Rc::new(comparison.set(choice!(
        seq!(&bitwise_or, repeat(&compare_op_bitwise_or_pair))
    )).into_boxed());
    let inversion = Rc::new(inversion.set(choice!(
        seq!(eat_string("not"), &inversion),
        seq!(&comparison)
    )).into_boxed());
    let conjunction = Rc::new(conjunction.set(choice!(
        seq!(&inversion, repeat(choice!(seq!(eat_string("and"), &inversion))))
    )).into_boxed());
    let disjunction = Rc::new(disjunction.set(choice!(
        seq!(&conjunction, repeat(choice!(seq!(eat_string("or"), &conjunction))))
    )).into_boxed());
    let named_expression = Rc::new(named_expression.set(choice!(
        seq!(&NAME, eat_string(":="), &expression),
        seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(&lambdef)
    )).into_boxed());
    let assignment_expression = Rc::new(assignment_expression.set(choice!(
        seq!(&NAME, eat_string(":="), &expression)
    )).into_boxed());
    let star_named_expression = Rc::new(star_named_expression.set(choice!(
        seq!(eat_string("*"), &bitwise_or),
        seq!(&named_expression)
    )).into_boxed());
    let star_named_expressions = Rc::new(star_named_expressions.set(choice!(
        seq!(&star_named_expression, repeat(eat_string(",")))
    )).into_boxed());
    let star_expression = Rc::new(star_expression.set(choice!(
        seq!(eat_string("*"), &bitwise_or),
        seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(&lambdef)
    )).into_boxed());
    let star_expressions = Rc::new(star_expressions.set(choice!(
        seq!(&star_expression, choice!(seq!(repeat(choice!(seq!(eat_string(","), &star_expression))), eat_string(",")), seq!(eat_string(","))))
    )).into_boxed());
    let yield_expr = Rc::new(yield_expr.set(choice!(
        seq!(eat_string("yield"), choice!(seq!(eat_string("from"), &expression), seq!(&star_expressions)))
    )).into_boxed());
    let expression = Rc::new(expression.set(choice!(
        seq!(&disjunction, eat_string("if"), &disjunction, eat_string("else"), &expression),
        seq!(&lambdef)
    )).into_boxed());
    let expressions = Rc::new(expressions.set(choice!(
        seq!(&expression, choice!(seq!(repeat(choice!(seq!(eat_string(","), &expression))), eat_string(",")), seq!(eat_string(","))))
    )).into_boxed());
    let type_param_starred_default = Rc::new(type_param_starred_default.set(choice!(
        seq!(eat_string("="), &star_expression)
    )).into_boxed());
    let type_param_default = Rc::new(type_param_default.set(choice!(
        seq!(eat_string("="), &expression)
    )).into_boxed());
    let type_param_bound = Rc::new(type_param_bound.set(choice!(
        seq!(eat_string(":"), &expression)
    )).into_boxed());
    let type_param = Rc::new(type_param.set(choice!(
        seq!(&NAME, &type_param_bound, &type_param_default),
        seq!(eat_string("*"), &NAME, &type_param_starred_default),
        seq!(eat_string("**"), &NAME, &type_param_default)
    )).into_boxed());
    let type_param_seq = Rc::new(type_param_seq.set(choice!(
        seq!(&type_param, repeat(eat_string(",")))
    )).into_boxed());
    let type_params = Rc::new(type_params.set(choice!(
        seq!(eat_string("["), &type_param_seq, eat_string("]"))
    )).into_boxed());
    let type_alias = Rc::new(type_alias.set(choice!(
        seq!(eat_string("yp"), &NAME, &type_params, eat_string("="), &expression)
    )).into_boxed());
    let keyword_pattern = Rc::new(keyword_pattern.set(choice!(
        seq!(&NAME, eat_string("="), &pattern)
    )).into_boxed());
    let keyword_patterns = Rc::new(keyword_patterns.set(choice!(
        seq!(&keyword_pattern, repeat(eat_string(",")))
    )).into_boxed());
    let positional_patterns = Rc::new(positional_patterns.set(choice!(
        seq!(choice!(seq!(&as_pattern), seq!(&or_pattern)), repeat(eat_string(",")))
    )).into_boxed());
    let class_pattern = Rc::new(class_pattern.set(choice!(
        seq!(&NAME, repeat(choice!(seq!(eat_string("."), &NAME))), eat_string("("), choice!(seq!(eat_string(")")), seq!(&positional_patterns, eat_string(","), choice!(seq!(eat_string(")")), seq!(&keyword_patterns, eat_string(","), eat_string(")")))), seq!(&keyword_patterns, eat_string(","), eat_string(")"))))
    )).into_boxed());
    let double_star_pattern = Rc::new(double_star_pattern.set(choice!(
        seq!(eat_string("**"), &pattern_capture_target)
    )).into_boxed());
    let key_value_pattern = Rc::new(key_value_pattern.set(choice!(
        seq!(choice!(seq!(&signed_number), seq!(&complex_number), seq!(&strings), seq!(eat_string("None")), seq!(eat_string("True")), seq!(eat_string("False")), seq!(&name_or_attr, eat_string("."), &NAME)), eat_string(":"), &pattern)
    )).into_boxed());
    let items_pattern = Rc::new(items_pattern.set(choice!(
        seq!(&key_value_pattern, repeat(eat_string(",")))
    )).into_boxed());
    let mapping_pattern = Rc::new(mapping_pattern.set(choice!(
        seq!(eat_string("{"), choice!(seq!(eat_string("}")), seq!(&double_star_pattern, eat_string(","), eat_string("}")), seq!(&items_pattern, eat_string(","), choice!(seq!(&double_star_pattern, eat_string(","), eat_string("}")), seq!(eat_string("}"))))))
    )).into_boxed());
    let star_pattern = Rc::new(star_pattern.set(choice!(
        seq!(eat_string("*"), choice!(seq!(&pattern_capture_target), seq!(&wildcard_pattern)))
    )).into_boxed());
    let maybe_star_pattern = Rc::new(maybe_star_pattern.set(choice!(
        seq!(&star_pattern),
        seq!(&as_pattern),
        seq!(&or_pattern)
    )).into_boxed());
    let maybe_sequence_pattern = Rc::new(maybe_sequence_pattern.set(choice!(
        seq!(&maybe_star_pattern, repeat(eat_string(",")))
    )).into_boxed());
    let open_sequence_pattern = Rc::new(open_sequence_pattern.set(choice!(
        seq!(&maybe_star_pattern, eat_string(","), &maybe_sequence_pattern)
    )).into_boxed());
    let sequence_pattern = Rc::new(sequence_pattern.set(choice!(
        seq!(eat_string("["), &maybe_sequence_pattern, eat_string("]")),
        seq!(eat_string("("), &open_sequence_pattern, eat_string(")"))
    )).into_boxed());
    let group_pattern = Rc::new(group_pattern.set(choice!(
        seq!(eat_string("("), &pattern, eat_string(")"))
    )).into_boxed());
    let name_or_attr = Rc::new(name_or_attr.set(choice!(
        seq!(&NAME, repeat(choice!(seq!(eat_string("."), &NAME))))
    )).into_boxed());
    let attr = Rc::new(attr.set(choice!(
        seq!(&name_or_attr, eat_string("."), &NAME)
    )).into_boxed());
    let value_pattern = Rc::new(value_pattern.set(choice!(
        seq!(&attr)
    )).into_boxed());
    let wildcard_pattern = Rc::new(wildcard_pattern.set(choice!(
        seq!(eat_string(""))
    )).into_boxed());
    let pattern_capture_target = Rc::new(pattern_capture_target.set(choice!(
        seq!(repeat(&NAME))
    )).into_boxed());
    let capture_pattern = Rc::new(capture_pattern.set(choice!(
        seq!(&pattern_capture_target)
    )).into_boxed());
    let imaginary_number = Rc::new(imaginary_number.set(choice!(
        seq!(&NUMBER)
    )).into_boxed());
    let real_number = Rc::new(real_number.set(choice!(
        seq!(&NUMBER)
    )).into_boxed());
    let signed_real_number = Rc::new(signed_real_number.set(choice!(
        seq!(&real_number),
        seq!(eat_string("-"), &real_number)
    )).into_boxed());
    let signed_number = Rc::new(signed_number.set(choice!(
        seq!(&NUMBER),
        seq!(eat_string("-"), &NUMBER)
    )).into_boxed());
    let complex_number = Rc::new(complex_number.set(choice!(
        seq!(&signed_real_number, choice!(seq!(eat_string("+"), &imaginary_number), seq!(eat_string("-"), &imaginary_number)))
    )).into_boxed());
    let literal_expr = Rc::new(literal_expr.set(choice!(
        seq!(&signed_number),
        seq!(&complex_number),
        seq!(&strings),
        seq!(eat_string("None")),
        seq!(eat_string("True")),
        seq!(eat_string("False"))
    )).into_boxed());
    let literal_pattern = Rc::new(literal_pattern.set(choice!(
        seq!(&signed_number),
        seq!(&complex_number),
        seq!(&strings),
        seq!(eat_string("None")),
        seq!(eat_string("True")),
        seq!(eat_string("False"))
    )).into_boxed());
    let closed_pattern = Rc::new(closed_pattern.set(choice!(
        seq!(&literal_pattern),
        seq!(&capture_pattern),
        seq!(&wildcard_pattern),
        seq!(&value_pattern),
        seq!(&group_pattern),
        seq!(&sequence_pattern),
        seq!(&mapping_pattern),
        seq!(&class_pattern)
    )).into_boxed());
    let or_pattern = Rc::new(or_pattern.set(choice!(
        seq!(&closed_pattern, repeat(eat_string("|")))
    )).into_boxed());
    let as_pattern = Rc::new(as_pattern.set(choice!(
        seq!(&or_pattern, eat_string("as"), &pattern_capture_target)
    )).into_boxed());
    let pattern = Rc::new(pattern.set(choice!(
        seq!(&as_pattern),
        seq!(&or_pattern)
    )).into_boxed());
    let patterns = Rc::new(patterns.set(choice!(
        seq!(&open_sequence_pattern),
        seq!(&pattern)
    )).into_boxed());
    let guard = Rc::new(guard.set(choice!(
        seq!(eat_string("if"), &named_expression)
    )).into_boxed());
    let case_block = Rc::new(case_block.set(choice!(
        seq!(eat_string("as"), &patterns, &guard, eat_string(":"), &block)
    )).into_boxed());
    let subject_expr = Rc::new(subject_expr.set(choice!(
        seq!(&star_named_expression, eat_string(","), &star_named_expressions),
        seq!(&named_expression)
    )).into_boxed());
    let match_stmt = Rc::new(match_stmt.set(choice!(
        seq!(eat_string("atc"), &subject_expr, eat_string(":"), &NEWLINE, &INDENT, repeat(&case_block), &DEDENT)
    )).into_boxed());
    let finally_block = Rc::new(finally_block.set(choice!(
        seq!(eat_string("finally"), eat_string(":"), &block)
    )).into_boxed());
    let except_star_block = Rc::new(except_star_block.set(choice!(
        seq!(eat_string("except"), eat_string("*"), &expression, eat_string("as"), &NAME, eat_string(":"), &block)
    )).into_boxed());
    let except_block = Rc::new(except_block.set(choice!(
        seq!(eat_string("except"), choice!(seq!(&expression, eat_string("as"), &NAME, eat_string(":"), &block), seq!(eat_string(":"), &block)))
    )).into_boxed());
    let try_stmt = Rc::new(try_stmt.set(choice!(
        seq!(eat_string("try"), eat_string(":"), &block, choice!(seq!(&finally_block), seq!(repeat(&except_block), &else_block, &finally_block), seq!(repeat(&except_star_block), &else_block, &finally_block)))
    )).into_boxed());
    let with_item = Rc::new(with_item.set(choice!(
        seq!(&expression, eat_string("as"), &star_target)
    )).into_boxed());
    let with_stmt = Rc::new(with_stmt.set(choice!(
        seq!(eat_string("with"), choice!(seq!(eat_string("("), &with_item, repeat(eat_string(",")), eat_string(")"), eat_string(":"), &TYPE_COMMENT, &block), seq!(&with_item, repeat(eat_string(",")), eat_string(":"), &TYPE_COMMENT, &block))),
        seq!(eat_string("async"), eat_string("with"), choice!(seq!(eat_string("("), &with_item, repeat(eat_string(",")), eat_string(")"), eat_string(":"), &block), seq!(&with_item, repeat(eat_string(",")), eat_string(":"), &TYPE_COMMENT, &block)))
    )).into_boxed());
    let for_stmt = Rc::new(for_stmt.set(choice!(
        seq!(eat_string("for"), &star_targets, eat_string("in"), &star_expressions, eat_string(":"), &TYPE_COMMENT, &block, &else_block),
        seq!(eat_string("async"), eat_string("for"), &star_targets, eat_string("in"), &star_expressions, eat_string(":"), &TYPE_COMMENT, &block, &else_block)
    )).into_boxed());
    let while_stmt = Rc::new(while_stmt.set(choice!(
        seq!(eat_string("while"), &named_expression, eat_string(":"), &block, &else_block)
    )).into_boxed());
    let else_block = Rc::new(else_block.set(choice!(
        seq!(eat_string("else"), eat_string(":"), &block)
    )).into_boxed());
    let elif_stmt = Rc::new(elif_stmt.set(choice!(
        seq!(eat_string("elif"), &named_expression, eat_string(":"), &block, choice!(seq!(&elif_stmt), seq!(&else_block)))
    )).into_boxed());
    let if_stmt = Rc::new(if_stmt.set(choice!(
        seq!(eat_string("if"), &named_expression, eat_string(":"), &block, choice!(seq!(&elif_stmt), seq!(&else_block)))
    )).into_boxed());
    let default = Rc::new(default.set(choice!(
        seq!(eat_string("="), &expression)
    )).into_boxed());
    let star_annotation = Rc::new(star_annotation.set(choice!(
        seq!(eat_string(":"), &star_expression)
    )).into_boxed());
    let annotation = Rc::new(annotation.set(choice!(
        seq!(eat_string(":"), &expression)
    )).into_boxed());
    let param_star_annotation = Rc::new(param_star_annotation.set(choice!(
        seq!(&NAME, &star_annotation)
    )).into_boxed());
    let param = Rc::new(param.set(choice!(
        seq!(&NAME, &annotation)
    )).into_boxed());
    let param_maybe_default = Rc::new(param_maybe_default.set(choice!(
        seq!(&param, &default, choice!(seq!(eat_string(","), &TYPE_COMMENT), seq!(&TYPE_COMMENT)))
    )).into_boxed());
    let param_with_default = Rc::new(param_with_default.set(choice!(
        seq!(&param, &default, choice!(seq!(eat_string(","), &TYPE_COMMENT), seq!(&TYPE_COMMENT)))
    )).into_boxed());
    let param_no_default_star_annotation = Rc::new(param_no_default_star_annotation.set(choice!(
        seq!(&param_star_annotation, choice!(seq!(eat_string(","), &TYPE_COMMENT), seq!(&TYPE_COMMENT)))
    )).into_boxed());
    let param_no_default = Rc::new(param_no_default.set(choice!(
        seq!(&param, choice!(seq!(eat_string(","), &TYPE_COMMENT), seq!(&TYPE_COMMENT)))
    )).into_boxed());
    let kwds = Rc::new(kwds.set(choice!(
        seq!(eat_string("**"), &param_no_default)
    )).into_boxed());
    let star_etc = Rc::new(star_etc.set(choice!(
        seq!(eat_string("*"), choice!(seq!(&param_no_default, repeat(&param_maybe_default), &kwds), seq!(&param_no_default_star_annotation, repeat(&param_maybe_default), &kwds), seq!(eat_string(","), repeat(&param_maybe_default), &kwds))),
        seq!(&kwds)
    )).into_boxed());
    let slash_with_default = Rc::new(slash_with_default.set(choice!(
        seq!(repeat(&param_no_default), repeat(&param_with_default), eat_string("/"), eat_string(","))
    )).into_boxed());
    let slash_no_default = Rc::new(slash_no_default.set(choice!(
        seq!(repeat(&param_no_default), eat_string("/"), eat_string(","))
    )).into_boxed());
    let parameters = Rc::new(parameters.set(choice!(
        seq!(&slash_no_default, repeat(&param_no_default), repeat(&param_with_default), &star_etc),
        seq!(&slash_with_default, repeat(&param_with_default), &star_etc),
        seq!(repeat(&param_no_default), repeat(&param_with_default), &star_etc),
        seq!(repeat(&param_with_default), &star_etc),
        seq!(&star_etc)
    )).into_boxed());
    let params = Rc::new(params.set(choice!(
        seq!(&parameters)
    )).into_boxed());
    let function_def_raw = Rc::new(function_def_raw.set(choice!(
        seq!(eat_string("def"), &NAME, &type_params, eat_string("("), &params, eat_string(")"), eat_string("->"), &expression, eat_string(":"), &func_type_comment, &block),
        seq!(eat_string("async"), eat_string("def"), &NAME, &type_params, eat_string("("), &params, eat_string(")"), eat_string("->"), &expression, eat_string(":"), &func_type_comment, &block)
    )).into_boxed());
    let function_def = Rc::new(function_def.set(choice!(
        seq!(eat_string("@"), &named_expression, &NEWLINE, repeat(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE))), &function_def_raw),
        seq!(&function_def_raw)
    )).into_boxed());
    let class_def_raw = Rc::new(class_def_raw.set(choice!(
        seq!(eat_string("class"), &NAME, &type_params, eat_string("("), &arguments, eat_string(")"), eat_string(":"), &block)
    )).into_boxed());
    let class_def = Rc::new(class_def.set(choice!(
        seq!(eat_string("@"), &named_expression, &NEWLINE, repeat(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE))), &class_def_raw),
        seq!(&class_def_raw)
    )).into_boxed());
    let decorators = Rc::new(decorators.set(choice!(
        seq!(eat_string("@"), &named_expression, &NEWLINE, repeat(choice!(seq!(eat_string("@"), &named_expression, &NEWLINE))))
    )).into_boxed());
    let block = Rc::new(block.set(choice!(
        seq!(&NEWLINE, &INDENT, &statements, &DEDENT),
        seq!(&simple_stmt, choice!(seq!(&NEWLINE), seq!(repeat(eat_string(";")), &NEWLINE)))
    )).into_boxed());
    let dotted_name = Rc::new(dotted_name.set(choice!(
        seq!(&NAME, repeat(choice!(seq!(eat_string("."), &NAME))))
    )).into_boxed());
    let dotted_as_name = Rc::new(dotted_as_name.set(choice!(
        seq!(&dotted_name, eat_string("as"), &NAME)
    )).into_boxed());
    let dotted_as_names = Rc::new(dotted_as_names.set(choice!(
        seq!(&dotted_as_name, repeat(eat_string(",")))
    )).into_boxed());
    let import_from_as_name = Rc::new(import_from_as_name.set(choice!(
        seq!(&NAME, eat_string("as"), &NAME)
    )).into_boxed());
    let import_from_as_names = Rc::new(import_from_as_names.set(choice!(
        seq!(&import_from_as_name, repeat(eat_string(",")))
    )).into_boxed());
    let import_from_targets = Rc::new(import_from_targets.set(choice!(
        seq!(eat_string("("), &import_from_as_names, eat_string(","), eat_string(")")),
        seq!(&import_from_as_names),
        seq!(eat_string("*"))
    )).into_boxed());
    let import_from = Rc::new(import_from.set(choice!(
        seq!(eat_string("from"), repeat(choice!(seq!(eat_string(".")), seq!(eat_string("...")))), choice!(seq!(&dotted_name, eat_string("import"), &import_from_targets), seq!(eat_string("import"), &import_from_targets)))
    )).into_boxed());
    let import_name = Rc::new(import_name.set(choice!(
        seq!(eat_string("import"), &dotted_as_names)
    )).into_boxed());
    let import_stmt = Rc::new(import_stmt.set(choice!(
        seq!(&import_name),
        seq!(&import_from)
    )).into_boxed());
    let assert_stmt = Rc::new(assert_stmt.set(choice!(
        seq!(eat_string("assert"), &expression, eat_string(","), &expression)
    )).into_boxed());
    let yield_stmt = Rc::new(yield_stmt.set(choice!(
        seq!(&yield_expr)
    )).into_boxed());
    let del_stmt = Rc::new(del_stmt.set(choice!(
        seq!(eat_string("del"), &del_targets)
    )).into_boxed());
    let nonlocal_stmt = Rc::new(nonlocal_stmt.set(choice!(
        seq!(eat_string("nonlocal"), &NAME, repeat(eat_string(",")))
    )).into_boxed());
    let global_stmt = Rc::new(global_stmt.set(choice!(
        seq!(eat_string("global"), &NAME, repeat(eat_string(",")))
    )).into_boxed());
    let raise_stmt = Rc::new(raise_stmt.set(choice!(
        seq!(eat_string("raise"), &expression, eat_string("from"), &expression)
    )).into_boxed());
    let return_stmt = Rc::new(return_stmt.set(choice!(
        seq!(eat_string("return"), &star_expressions)
    )).into_boxed());
    let augassign = Rc::new(augassign.set(choice!(
        seq!(eat_string("+=")),
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
        seq!(eat_string("//="))
    )).into_boxed());
    let annotated_rhs = Rc::new(annotated_rhs.set(choice!(
        seq!(&yield_expr),
        seq!(&star_expressions)
    )).into_boxed());
    let assignment = Rc::new(assignment.set(choice!(
        seq!(&NAME, eat_string(":"), &expression, eat_string("="), &annotated_rhs),
        seq!(choice!(seq!(eat_string("("), &single_target, eat_string(")")), seq!(&single_subscript_attribute_target)), eat_string(":"), &expression, eat_string("="), &annotated_rhs),
        seq!(&star_targets, eat_string("="), repeat(choice!(seq!(&star_targets, eat_string("=")))), choice!(seq!(&yield_expr), seq!(&star_expressions)), &TYPE_COMMENT),
        seq!(&single_target, &augassign, choice!(seq!(&yield_expr), seq!(&star_expressions)))
    )).into_boxed());
    let compound_stmt = Rc::new(compound_stmt.set(choice!(
        seq!(choice!(seq!(&function_def), seq!(&if_stmt), seq!(&class_def), seq!(&with_stmt), seq!(&for_stmt), seq!(&try_stmt), seq!(&while_stmt), seq!(&match_stmt)), repeat(choice!(seq!(&function_def), seq!(&if_stmt), seq!(&class_def), seq!(&with_stmt), seq!(&for_stmt), seq!(&try_stmt), seq!(&while_stmt))))
    )).into_boxed());
    let simple_stmt = Rc::new(simple_stmt.set(choice!(
        seq!(choice!(seq!(&assignment), seq!(&type_alias), seq!(&star_expressions), seq!(&return_stmt), seq!(&import_stmt), seq!(&raise_stmt), seq!(eat_string("pass")), seq!(&del_stmt), seq!(&yield_stmt), seq!(&assert_stmt), seq!(eat_string("break")), seq!(eat_string("continue")), seq!(&global_stmt), seq!(&nonlocal_stmt)), repeat(choice!(seq!(&type_alias), seq!(&return_stmt), seq!(&import_stmt), seq!(&raise_stmt), seq!(&del_stmt), seq!(&yield_stmt), seq!(&assert_stmt), seq!(&global_stmt), seq!(&nonlocal_stmt))))
    )).into_boxed());
    let simple_stmts = Rc::new(simple_stmts.set(choice!(
        seq!(&simple_stmt, choice!(seq!(&NEWLINE), seq!(repeat(eat_string(";")), &NEWLINE)))
    )).into_boxed());
    let statement_newline = Rc::new(statement_newline.set(choice!(
        seq!(&compound_stmt, &NEWLINE),
        seq!(&simple_stmts),
        seq!(&NEWLINE),
        seq!(&ENDMARKER)
    )).into_boxed());
    let statement = Rc::new(statement.set(choice!(
        seq!(&compound_stmt),
        seq!(&simple_stmts)
    )).into_boxed());
    let statements = Rc::new(statements.set(choice!(
        seq!(repeat(&statement))
    )).into_boxed());
    let func_type = Rc::new(func_type.set(choice!(
        seq!(eat_string("("), &type_expressions, eat_string(")"), eat_string("->"), &expression, repeat(&NEWLINE), &ENDMARKER)
    )).into_boxed());
    let eval = Rc::new(eval.set(choice!(
        seq!(&expressions, repeat(&NEWLINE), &ENDMARKER)
    )).into_boxed());
    let interactive = Rc::new(interactive.set(choice!(
        seq!(&statement_newline)
    )).into_boxed());
    let file = Rc::new(file.set(choice!(
        seq!(&statements, repeat(&ENDMARKER))
    )).into_boxed());
    file.into_boxed().into()
}
