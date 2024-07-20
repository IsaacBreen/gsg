use std::rc::Rc;
use crate::{choice, seq, repeat0, repeat1, opt, eat_char_choice, eat_string, eat_char_range, forward_ref, eps, cut, DynCombinator, CombinatorTrait};
use super::python_tokenizer::{NAME, TYPE_COMMENT, FSTRING_START, FSTRING_MIDDLE, FSTRING_END, NUMBER, STRING, NEWLINE, INDENT, DEDENT, ENDMARKER};
use super::python_tokenizer::python_literal;

pub fn python_file() -> Rc<DynCombinator> {
    let NAME = NAME();
    let TYPE_COMMENT = TYPE_COMMENT();
    let FSTRING_START = FSTRING_START();
    let FSTRING_MIDDLE = FSTRING_MIDDLE();
    let FSTRING_END = FSTRING_END();
    let NUMBER = NUMBER();
    let STRING = STRING();
    let NEWLINE = NEWLINE();
    let INDENT = INDENT();
    let DEDENT = DEDENT();
    let ENDMARKER = ENDMARKER();

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
        seq!(&conjunction, opt(repeat1(seq!(python_literal("or"), &conjunction))), opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), &expression)
    )).into_boxed());
    let func_type_comment = Rc::new(func_type_comment.set(choice!(
        seq!(&NEWLINE, &TYPE_COMMENT),
        &TYPE_COMMENT
    )).into_boxed());
    let type_expressions = Rc::new(type_expressions.set(choice!(
        seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), opt(repeat1(seq!(python_literal(","), &expression))), opt(seq!(python_literal(","), choice!(seq!(python_literal("*"), &expression, opt(seq!(python_literal(","), python_literal("**"), &expression))), seq!(python_literal("**"), &expression))))),
        seq!(python_literal("*"), &expression, opt(seq!(python_literal(","), python_literal("**"), &expression))),
        seq!(python_literal("**"), &expression)
    )).into_boxed());
    let del_t_atom = Rc::new(del_t_atom.set(choice!(
        &NAME,
        seq!(python_literal("("), choice!(seq!(&del_target, python_literal(")")), seq!(opt(&del_targets), python_literal(")")))),
        seq!(python_literal("["), opt(&del_targets), python_literal("]"))
    )).into_boxed());
    let del_target = Rc::new(del_target.set(choice!(
        seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), &strings, &NUMBER, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp, python_literal("...")), opt(repeat1(choice!(&strings, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp))), opt(repeat1(choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")), &genexp, seq!(python_literal("("), opt(&arguments), python_literal(")"))))), choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")))),
        &del_t_atom
    )).into_boxed());
    let del_targets = Rc::new(del_targets.set(seq!(&del_target, opt(repeat1(seq!(python_literal(","), &del_target))), opt(python_literal(",")))).into_boxed());
    let t_lookahead = Rc::new(t_lookahead.set(choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    )).into_boxed());
    let t_primary = Rc::new(t_primary.set(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), &strings, &NUMBER, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp, python_literal("...")), opt(repeat1(choice!(&strings, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp))), opt(repeat1(choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")), &genexp, seq!(python_literal("("), opt(&arguments), python_literal(")"))))))).into_boxed());
    let single_subscript_attribute_target = Rc::new(single_subscript_attribute_target.set(seq!(&t_primary, choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]"))))).into_boxed());
    let single_target = Rc::new(single_target.set(choice!(
        &single_subscript_attribute_target,
        &NAME,
        seq!(python_literal("("), &single_target, python_literal(")"))
    )).into_boxed());
    let star_atom = Rc::new(star_atom.set(choice!(
        &NAME,
        seq!(python_literal("("), choice!(seq!(&target_with_star_atom, python_literal(")")), seq!(opt(&star_targets_tuple_seq), python_literal(")")))),
        seq!(python_literal("["), opt(&star_targets_list_seq), python_literal("]"))
    )).into_boxed());
    let target_with_star_atom = Rc::new(target_with_star_atom.set(choice!(
        seq!(&t_primary, choice!(seq!(python_literal("."), &NAME), seq!(python_literal("["), &slices, python_literal("]")))),
        &star_atom
    )).into_boxed());
    let star_target = Rc::new(star_target.set(choice!(
        seq!(python_literal("*"), &star_target),
        &target_with_star_atom
    )).into_boxed());
    let star_targets_tuple_seq = Rc::new(star_targets_tuple_seq.set(seq!(&star_target, choice!(seq!(repeat1(seq!(python_literal(","), &star_target)), opt(python_literal(","))), python_literal(",")))).into_boxed());
    let star_targets_list_seq = Rc::new(star_targets_list_seq.set(seq!(&star_target, opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))).into_boxed());
    let star_targets = Rc::new(star_targets.set(seq!(&star_target, opt(seq!(opt(repeat1(seq!(python_literal(","), &star_target))), opt(python_literal(",")))))).into_boxed());
    let kwarg_or_double_starred = Rc::new(kwarg_or_double_starred.set(choice!(
        seq!(&NAME, python_literal("="), &expression),
        seq!(python_literal("**"), &expression)
    )).into_boxed());
    let kwarg_or_starred = Rc::new(kwarg_or_starred.set(choice!(
        seq!(&NAME, python_literal("="), &expression),
        seq!(python_literal("*"), &expression)
    )).into_boxed());
    let starred_expression = Rc::new(starred_expression.set(seq!(python_literal("*"), &expression)).into_boxed());
    let kwargs = Rc::new(kwargs.set(choice!(
        seq!(&kwarg_or_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_starred))), opt(seq!(python_literal(","), &kwarg_or_double_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_double_starred)))))),
        seq!(&kwarg_or_double_starred, opt(repeat1(seq!(python_literal(","), &kwarg_or_double_starred))))
    )).into_boxed());
    let args = Rc::new(args.set(choice!(
        seq!(choice!(&starred_expression, seq!(&NAME, python_literal(":="), cut(), &expression), seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), opt(repeat1(seq!(python_literal(","), choice!(&starred_expression, seq!(&NAME, python_literal(":="), cut(), &expression), seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)))), opt(seq!(python_literal(","), &kwargs))),
        &kwargs
    )).into_boxed());
    let arguments = Rc::new(arguments.set(seq!(&args, opt(python_literal(",")))).into_boxed());
    let dictcomp = Rc::new(dictcomp.set(seq!(python_literal("{"), &kvpair, &for_if_clauses, python_literal("}"))).into_boxed());
    let genexp = Rc::new(genexp.set(seq!(python_literal("("), choice!(&assignment_expression, &expression), &for_if_clauses, python_literal(")"))).into_boxed());
    let setcomp = Rc::new(setcomp.set(seq!(python_literal("{"), &named_expression, &for_if_clauses, python_literal("}"))).into_boxed());
    let listcomp = Rc::new(listcomp.set(seq!(python_literal("["), &named_expression, &for_if_clauses, python_literal("]"))).into_boxed());
    let for_if_clause = Rc::new(for_if_clause.set(choice!(
        seq!(python_literal("async"), python_literal("for"), &star_targets, python_literal("in"), cut(), &disjunction, opt(repeat1(seq!(python_literal("if"), &disjunction)))),
        seq!(python_literal("for"), &star_targets, python_literal("in"), cut(), &disjunction, opt(repeat1(seq!(python_literal("if"), &disjunction))))
    )).into_boxed());
    let for_if_clauses = Rc::new(for_if_clauses.set(repeat1(&for_if_clause)).into_boxed());
    let kvpair = Rc::new(kvpair.set(seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), python_literal(":"), &expression)).into_boxed());
    let double_starred_kvpair = Rc::new(double_starred_kvpair.set(choice!(
        seq!(python_literal("**"), &bitwise_or),
        &kvpair
    )).into_boxed());
    let double_starred_kvpairs = Rc::new(double_starred_kvpairs.set(seq!(&double_starred_kvpair, opt(repeat1(seq!(python_literal(","), &double_starred_kvpair))), opt(python_literal(",")))).into_boxed());
    let dict = Rc::new(dict.set(seq!(python_literal("{"), opt(&double_starred_kvpairs), python_literal("}"))).into_boxed());
    let set = Rc::new(set.set(seq!(python_literal("{"), &star_named_expressions, python_literal("}"))).into_boxed());
    let tuple = Rc::new(tuple.set(seq!(python_literal("("), opt(seq!(&star_named_expression, python_literal(","), opt(&star_named_expressions))), python_literal(")"))).into_boxed());
    let list = Rc::new(list.set(seq!(python_literal("["), opt(&star_named_expressions), python_literal("]"))).into_boxed());
    let strings = Rc::new(strings.set(repeat1(choice!(seq!(&FSTRING_START, opt(repeat1(&fstring_middle)), &FSTRING_END), &STRING))).into_boxed());
    let string = Rc::new(string.set(&STRING).into_boxed());
    let fstring = Rc::new(fstring.set(seq!(&FSTRING_START, opt(repeat1(&fstring_middle)), &FSTRING_END)).into_boxed());
    let fstring_format_spec = Rc::new(fstring_format_spec.set(choice!(
        &FSTRING_MIDDLE,
        seq!(python_literal("{"), &annotated_rhs, opt(python_literal("=")), opt(&fstring_conversion), opt(&fstring_full_format_spec), python_literal("}"))
    )).into_boxed());
    let fstring_full_format_spec = Rc::new(fstring_full_format_spec.set(seq!(python_literal(":"), opt(repeat1(&fstring_format_spec)))).into_boxed());
    let fstring_conversion = Rc::new(fstring_conversion.set(seq!(python_literal(""), &NAME)).into_boxed());
    let fstring_replacement_field = Rc::new(fstring_replacement_field.set(seq!(python_literal("{"), &annotated_rhs, opt(python_literal("=")), opt(&fstring_conversion), opt(&fstring_full_format_spec), python_literal("}"))).into_boxed());
    let fstring_middle = Rc::new(fstring_middle.set(choice!(
        &fstring_replacement_field,
        &FSTRING_MIDDLE
    )).into_boxed());
    let lambda_param = Rc::new(lambda_param.set(&NAME).into_boxed());
    let lambda_param_maybe_default = Rc::new(lambda_param_maybe_default.set(seq!(&lambda_param, opt(&default), opt(python_literal(",")))).into_boxed());
    let lambda_param_with_default = Rc::new(lambda_param_with_default.set(seq!(&lambda_param, &default, opt(python_literal(",")))).into_boxed());
    let lambda_param_no_default = Rc::new(lambda_param_no_default.set(seq!(&lambda_param, opt(python_literal(",")))).into_boxed());
    let lambda_kwds = Rc::new(lambda_kwds.set(seq!(python_literal("**"), &lambda_param_no_default)).into_boxed());
    let lambda_star_etc = Rc::new(lambda_star_etc.set(choice!(
        seq!(python_literal("*"), choice!(seq!(&lambda_param_no_default, opt(repeat1(&lambda_param_maybe_default)), opt(&lambda_kwds)), seq!(python_literal(","), repeat1(&lambda_param_maybe_default), opt(&lambda_kwds)))),
        &lambda_kwds
    )).into_boxed());
    let lambda_slash_with_default = Rc::new(lambda_slash_with_default.set(seq!(opt(repeat1(&lambda_param_no_default)), repeat1(&lambda_param_with_default), python_literal("/"), opt(python_literal(",")), opt(repeat1(seq!(repeat1(&lambda_param_with_default), python_literal("/"), opt(python_literal(","))))))).into_boxed());
    let lambda_slash_no_default = Rc::new(lambda_slash_no_default.set(seq!(repeat1(&lambda_param_no_default), python_literal("/"), opt(python_literal(",")))).into_boxed());
    let lambda_parameters = Rc::new(lambda_parameters.set(choice!(
        seq!(&lambda_slash_no_default, opt(repeat1(&lambda_param_no_default)), opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(&lambda_slash_with_default, opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(repeat1(&lambda_param_no_default), opt(repeat1(&lambda_param_with_default)), opt(&lambda_star_etc)),
        seq!(repeat1(&lambda_param_with_default), opt(&lambda_star_etc)),
        &lambda_star_etc
    )).into_boxed());
    let lambda_params = Rc::new(lambda_params.set(&lambda_parameters).into_boxed());
    let lambdef = Rc::new(lambdef.set(seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), &expression)).into_boxed());
    let group = Rc::new(group.set(seq!(python_literal("("), choice!(&yield_expr, &named_expression), python_literal(")"))).into_boxed());
    let atom = Rc::new(atom.set(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), &strings, &NUMBER, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp, python_literal("...")), opt(repeat1(choice!(&strings, &tuple, &group, &genexp, &list, &listcomp, &dict, &set, &dictcomp, &setcomp))))).into_boxed());
    let slice = Rc::new(slice.set(seq!(choice!(seq!(opt(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), python_literal(":"), opt(&expression), opt(seq!(python_literal(":"), opt(&expression)))), seq!(&NAME, python_literal(":="), cut(), &expression), seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), opt(repeat1(seq!(python_literal(":"), opt(&expression), opt(seq!(python_literal(":"), opt(&expression)))))))).into_boxed());
    let slices = Rc::new(slices.set(choice!(
        &slice,
        seq!(choice!(&slice, &starred_expression), opt(repeat1(seq!(python_literal(","), choice!(&slice, &starred_expression)))), opt(python_literal(",")))
    )).into_boxed());
    let primary = Rc::new(primary.set(seq!(&atom, opt(repeat1(choice!(seq!(python_literal("."), &NAME), &genexp, seq!(python_literal("("), opt(&arguments), python_literal(")")), seq!(python_literal("["), &slices, python_literal("]"))))))).into_boxed());
    let await_primary = Rc::new(await_primary.set(choice!(
        seq!(python_literal("await"), &primary),
        &primary
    )).into_boxed());
    let power = Rc::new(power.set(seq!(&await_primary, opt(seq!(python_literal("**"), &factor)))).into_boxed());
    let factor = Rc::new(factor.set(choice!(
        seq!(python_literal("+"), &factor),
        seq!(python_literal("-"), &factor),
        seq!(python_literal("~"), &factor),
        &power
    )).into_boxed());
    let term = Rc::new(term.set(seq!(&factor, opt(repeat1(choice!(seq!(python_literal("*"), &factor), seq!(python_literal("/"), &factor), seq!(python_literal("//"), &factor), seq!(python_literal("%"), &factor), seq!(python_literal("@"), &factor)))))).into_boxed());
    let sum = Rc::new(sum.set(seq!(&term, opt(repeat1(choice!(seq!(python_literal("+"), &term), seq!(python_literal("-"), &term)))))).into_boxed());
    let shift_expr = Rc::new(shift_expr.set(seq!(&sum, opt(repeat1(choice!(seq!(python_literal("<<"), &sum), seq!(python_literal(">>"), &sum)))))).into_boxed());
    let bitwise_and = Rc::new(bitwise_and.set(seq!(&shift_expr, opt(repeat1(seq!(python_literal("&"), &shift_expr))))).into_boxed());
    let bitwise_xor = Rc::new(bitwise_xor.set(seq!(&bitwise_and, opt(repeat1(seq!(python_literal("^"), &bitwise_and))))).into_boxed());
    let bitwise_or = Rc::new(bitwise_or.set(seq!(&bitwise_xor, opt(repeat1(seq!(python_literal("|"), &bitwise_xor))))).into_boxed());
    let is_bitwise_or = Rc::new(is_bitwise_or.set(seq!(python_literal("is"), &bitwise_or)).into_boxed());
    let isnot_bitwise_or = Rc::new(isnot_bitwise_or.set(seq!(python_literal("is"), python_literal("not"), &bitwise_or)).into_boxed());
    let in_bitwise_or = Rc::new(in_bitwise_or.set(seq!(python_literal("in"), &bitwise_or)).into_boxed());
    let notin_bitwise_or = Rc::new(notin_bitwise_or.set(seq!(python_literal("not"), python_literal("in"), &bitwise_or)).into_boxed());
    let gt_bitwise_or = Rc::new(gt_bitwise_or.set(seq!(python_literal(">"), &bitwise_or)).into_boxed());
    let gte_bitwise_or = Rc::new(gte_bitwise_or.set(seq!(python_literal(">="), &bitwise_or)).into_boxed());
    let lt_bitwise_or = Rc::new(lt_bitwise_or.set(seq!(python_literal("<"), &bitwise_or)).into_boxed());
    let lte_bitwise_or = Rc::new(lte_bitwise_or.set(seq!(python_literal("<="), &bitwise_or)).into_boxed());
    let noteq_bitwise_or = Rc::new(noteq_bitwise_or.set(seq!(python_literal("!="), &bitwise_or)).into_boxed());
    let eq_bitwise_or = Rc::new(eq_bitwise_or.set(seq!(python_literal("=="), &bitwise_or)).into_boxed());
    let compare_op_bitwise_or_pair = Rc::new(compare_op_bitwise_or_pair.set(choice!(
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
    )).into_boxed());
    let comparison = Rc::new(comparison.set(seq!(&bitwise_or, opt(repeat1(&compare_op_bitwise_or_pair)))).into_boxed());
    let inversion = Rc::new(inversion.set(choice!(
        seq!(python_literal("not"), &inversion),
        &comparison
    )).into_boxed());
    let conjunction = Rc::new(conjunction.set(seq!(&inversion, opt(repeat1(seq!(python_literal("and"), &inversion))))).into_boxed());
    let disjunction = Rc::new(disjunction.set(seq!(&conjunction, opt(repeat1(seq!(python_literal("or"), &conjunction))))).into_boxed());
    let named_expression = Rc::new(named_expression.set(choice!(
        seq!(&NAME, python_literal(":="), cut(), &expression),
        seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        &lambdef
    )).into_boxed());
    let assignment_expression = Rc::new(assignment_expression.set(seq!(&NAME, python_literal(":="), cut(), &expression)).into_boxed());
    let star_named_expression = Rc::new(star_named_expression.set(choice!(
        seq!(python_literal("*"), &bitwise_or),
        &named_expression
    )).into_boxed());
    let star_named_expressions = Rc::new(star_named_expressions.set(seq!(&star_named_expression, opt(repeat1(seq!(python_literal(","), &star_named_expression))), opt(python_literal(",")))).into_boxed());
    let star_expression = Rc::new(star_expression.set(choice!(
        seq!(python_literal("*"), &bitwise_or),
        seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        &lambdef
    )).into_boxed());
    let star_expressions = Rc::new(star_expressions.set(seq!(&star_expression, opt(choice!(seq!(repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), python_literal(","))))).into_boxed());
    let yield_expr = Rc::new(yield_expr.set(seq!(python_literal("yield"), opt(choice!(seq!(python_literal("from"), &expression), &star_expressions)))).into_boxed());
    let expression = Rc::new(expression.set(choice!(
        seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        &lambdef
    )).into_boxed());
    let expressions = Rc::new(expressions.set(seq!(&expression, opt(choice!(seq!(repeat1(seq!(python_literal(","), &expression)), opt(python_literal(","))), python_literal(","))))).into_boxed());
    let type_param_starred_default = Rc::new(type_param_starred_default.set(seq!(python_literal("="), &star_expression)).into_boxed());
    let type_param_default = Rc::new(type_param_default.set(seq!(python_literal("="), &expression)).into_boxed());
    let type_param_bound = Rc::new(type_param_bound.set(seq!(python_literal(":"), &expression)).into_boxed());
    let type_param = Rc::new(type_param.set(choice!(
        seq!(&NAME, opt(&type_param_bound), opt(&type_param_default)),
        seq!(python_literal("*"), &NAME, opt(&type_param_starred_default)),
        seq!(python_literal("**"), &NAME, opt(&type_param_default))
    )).into_boxed());
    let type_param_seq = Rc::new(type_param_seq.set(seq!(&type_param, opt(repeat1(seq!(python_literal(","), &type_param))), opt(python_literal(",")))).into_boxed());
    let type_params = Rc::new(type_params.set(seq!(python_literal("["), &type_param_seq, python_literal("]"))).into_boxed());
    let type_alias = Rc::new(type_alias.set(seq!(python_literal("yp"), &NAME, opt(&type_params), python_literal("="), &expression)).into_boxed());
    let keyword_pattern = Rc::new(keyword_pattern.set(seq!(&NAME, python_literal("="), &pattern)).into_boxed());
    let keyword_patterns = Rc::new(keyword_patterns.set(seq!(&keyword_pattern, opt(repeat1(seq!(python_literal(","), &keyword_pattern))))).into_boxed());
    let positional_patterns = Rc::new(positional_patterns.set(seq!(choice!(&as_pattern, &or_pattern), opt(repeat1(seq!(python_literal(","), &pattern))))).into_boxed());
    let class_pattern = Rc::new(class_pattern.set(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME))), python_literal("("), choice!(python_literal(")"), seq!(&positional_patterns, choice!(seq!(opt(python_literal(",")), python_literal(")")), seq!(python_literal(","), &keyword_patterns, opt(python_literal(",")), python_literal(")")))), seq!(&keyword_patterns, opt(python_literal(",")), python_literal(")"))))).into_boxed());
    let double_star_pattern = Rc::new(double_star_pattern.set(seq!(python_literal("**"), &pattern_capture_target)).into_boxed());
    let key_value_pattern = Rc::new(key_value_pattern.set(seq!(choice!(&signed_number, &complex_number, &strings, python_literal("None"), python_literal("True"), python_literal("False"), seq!(&name_or_attr, python_literal("."), &NAME)), python_literal(":"), &pattern)).into_boxed());
    let items_pattern = Rc::new(items_pattern.set(seq!(&key_value_pattern, opt(repeat1(seq!(python_literal(","), &key_value_pattern))))).into_boxed());
    let mapping_pattern = Rc::new(mapping_pattern.set(seq!(python_literal("{"), choice!(python_literal("}"), seq!(&double_star_pattern, opt(python_literal(",")), python_literal("}")), seq!(&items_pattern, choice!(seq!(python_literal(","), &double_star_pattern, opt(python_literal(",")), python_literal("}")), seq!(opt(python_literal(",")), python_literal("}"))))))).into_boxed());
    let star_pattern = Rc::new(star_pattern.set(seq!(python_literal("*"), choice!(&pattern_capture_target, &wildcard_pattern))).into_boxed());
    let maybe_star_pattern = Rc::new(maybe_star_pattern.set(choice!(
        &star_pattern,
        &as_pattern,
        &or_pattern
    )).into_boxed());
    let maybe_sequence_pattern = Rc::new(maybe_sequence_pattern.set(seq!(&maybe_star_pattern, opt(repeat1(seq!(python_literal(","), &maybe_star_pattern))), opt(python_literal(",")))).into_boxed());
    let open_sequence_pattern = Rc::new(open_sequence_pattern.set(seq!(&maybe_star_pattern, python_literal(","), opt(&maybe_sequence_pattern))).into_boxed());
    let sequence_pattern = Rc::new(sequence_pattern.set(choice!(
        seq!(python_literal("["), opt(&maybe_sequence_pattern), python_literal("]")),
        seq!(python_literal("("), opt(&open_sequence_pattern), python_literal(")"))
    )).into_boxed());
    let group_pattern = Rc::new(group_pattern.set(seq!(python_literal("("), &pattern, python_literal(")"))).into_boxed());
    let name_or_attr = Rc::new(name_or_attr.set(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME))))).into_boxed());
    let attr = Rc::new(attr.set(seq!(&name_or_attr, python_literal("."), &NAME)).into_boxed());
    let value_pattern = Rc::new(value_pattern.set(&attr).into_boxed());
    let wildcard_pattern = Rc::new(wildcard_pattern.set(python_literal("")).into_boxed());
    let pattern_capture_target = Rc::new(pattern_capture_target.set(repeat1(&NAME)).into_boxed());
    let capture_pattern = Rc::new(capture_pattern.set(&pattern_capture_target).into_boxed());
    let imaginary_number = Rc::new(imaginary_number.set(&NUMBER).into_boxed());
    let real_number = Rc::new(real_number.set(&NUMBER).into_boxed());
    let signed_real_number = Rc::new(signed_real_number.set(choice!(
        &real_number,
        seq!(python_literal("-"), &real_number)
    )).into_boxed());
    let signed_number = Rc::new(signed_number.set(choice!(
        &NUMBER,
        seq!(python_literal("-"), &NUMBER)
    )).into_boxed());
    let complex_number = Rc::new(complex_number.set(seq!(&signed_real_number, choice!(seq!(python_literal("+"), &imaginary_number), seq!(python_literal("-"), &imaginary_number)))).into_boxed());
    let literal_expr = Rc::new(literal_expr.set(choice!(
        &signed_number,
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).into_boxed());
    let literal_pattern = Rc::new(literal_pattern.set(choice!(
        &signed_number,
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).into_boxed());
    let closed_pattern = Rc::new(closed_pattern.set(choice!(
        &literal_pattern,
        &capture_pattern,
        &wildcard_pattern,
        &value_pattern,
        &group_pattern,
        &sequence_pattern,
        &mapping_pattern,
        &class_pattern
    )).into_boxed());
    let or_pattern = Rc::new(or_pattern.set(seq!(&closed_pattern, opt(repeat1(seq!(python_literal("|"), &closed_pattern))))).into_boxed());
    let as_pattern = Rc::new(as_pattern.set(seq!(&or_pattern, python_literal("as"), &pattern_capture_target)).into_boxed());
    let pattern = Rc::new(pattern.set(choice!(
        &as_pattern,
        &or_pattern
    )).into_boxed());
    let patterns = Rc::new(patterns.set(choice!(
        &open_sequence_pattern,
        &pattern
    )).into_boxed());
    let guard = Rc::new(guard.set(seq!(python_literal("if"), &named_expression)).into_boxed());
    let case_block = Rc::new(case_block.set(seq!(python_literal("as"), &patterns, opt(&guard), python_literal(":"), &block)).into_boxed());
    let subject_expr = Rc::new(subject_expr.set(choice!(
        seq!(&star_named_expression, python_literal(","), opt(&star_named_expressions)),
        &named_expression
    )).into_boxed());
    let match_stmt = Rc::new(match_stmt.set(seq!(python_literal("atc"), &subject_expr, python_literal(":"), &NEWLINE, &INDENT, repeat1(&case_block), &DEDENT)).into_boxed());
    let finally_block = Rc::new(finally_block.set(seq!(python_literal("finally"), python_literal(":"), &block)).into_boxed());
    let except_star_block = Rc::new(except_star_block.set(seq!(python_literal("except"), python_literal("*"), &expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), &block)).into_boxed());
    let except_block = Rc::new(except_block.set(seq!(python_literal("except"), choice!(seq!(&expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), &block), seq!(python_literal(":"), &block)))).into_boxed());
    let try_stmt = Rc::new(try_stmt.set(seq!(python_literal("try"), python_literal(":"), &block, choice!(&finally_block, seq!(repeat1(&except_block), opt(&else_block), opt(&finally_block)), seq!(repeat1(&except_star_block), opt(&else_block), opt(&finally_block))))).into_boxed());
    let with_item = Rc::new(with_item.set(seq!(&expression, opt(seq!(python_literal("as"), &star_target)))).into_boxed());
    let with_stmt = Rc::new(with_stmt.set(choice!(
        seq!(python_literal("with"), choice!(seq!(python_literal("("), &with_item, opt(repeat1(seq!(python_literal(","), &with_item))), opt(python_literal(",")), python_literal(")"), python_literal(":"), opt(&TYPE_COMMENT), &block), seq!(&with_item, opt(repeat1(seq!(python_literal(","), &with_item))), python_literal(":"), opt(&TYPE_COMMENT), &block))),
        seq!(python_literal("async"), python_literal("with"), choice!(seq!(python_literal("("), &with_item, opt(repeat1(seq!(python_literal(","), &with_item))), opt(python_literal(",")), python_literal(")"), python_literal(":"), &block), seq!(&with_item, opt(repeat1(seq!(python_literal(","), &with_item))), python_literal(":"), opt(&TYPE_COMMENT), &block)))
    )).into_boxed());
    let for_stmt = Rc::new(for_stmt.set(choice!(
        seq!(python_literal("for"), &star_targets, python_literal("in"), cut(), &star_expressions, python_literal(":"), opt(&TYPE_COMMENT), &block, opt(&else_block)),
        seq!(python_literal("async"), python_literal("for"), &star_targets, python_literal("in"), cut(), &star_expressions, python_literal(":"), opt(&TYPE_COMMENT), &block, opt(&else_block))
    )).into_boxed());
    let while_stmt = Rc::new(while_stmt.set(seq!(python_literal("while"), &named_expression, python_literal(":"), &block, opt(&else_block))).into_boxed());
    let else_block = Rc::new(else_block.set(seq!(python_literal("else"), python_literal(":"), &block)).into_boxed());
    let elif_stmt = Rc::new(elif_stmt.set(seq!(python_literal("elif"), &named_expression, python_literal(":"), &block, opt(choice!(&elif_stmt, &else_block)))).into_boxed());
    let if_stmt = Rc::new(if_stmt.set(seq!(python_literal("if"), &named_expression, python_literal(":"), &block, opt(choice!(&elif_stmt, &else_block)))).into_boxed());
    let default = Rc::new(default.set(seq!(python_literal("="), &expression)).into_boxed());
    let star_annotation = Rc::new(star_annotation.set(seq!(python_literal(":"), &star_expression)).into_boxed());
    let annotation = Rc::new(annotation.set(seq!(python_literal(":"), &expression)).into_boxed());
    let param_star_annotation = Rc::new(param_star_annotation.set(seq!(&NAME, &star_annotation)).into_boxed());
    let param = Rc::new(param.set(seq!(&NAME, opt(&annotation))).into_boxed());
    let param_maybe_default = Rc::new(param_maybe_default.set(seq!(&param, opt(&default), opt(choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), &TYPE_COMMENT)))).into_boxed());
    let param_with_default = Rc::new(param_with_default.set(seq!(&param, &default, opt(choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), &TYPE_COMMENT)))).into_boxed());
    let param_no_default_star_annotation = Rc::new(param_no_default_star_annotation.set(seq!(&param_star_annotation, opt(choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), &TYPE_COMMENT)))).into_boxed());
    let param_no_default = Rc::new(param_no_default.set(seq!(&param, opt(choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), &TYPE_COMMENT)))).into_boxed());
    let kwds = Rc::new(kwds.set(seq!(python_literal("**"), &param_no_default)).into_boxed());
    let star_etc = Rc::new(star_etc.set(choice!(
        seq!(python_literal("*"), choice!(seq!(&param_no_default, opt(repeat1(&param_maybe_default)), opt(&kwds)), seq!(&param_no_default_star_annotation, opt(repeat1(&param_maybe_default)), opt(&kwds)), seq!(python_literal(","), repeat1(&param_maybe_default), opt(&kwds)))),
        &kwds
    )).into_boxed());
    let slash_with_default = Rc::new(slash_with_default.set(seq!(opt(repeat1(&param_no_default)), repeat1(&param_with_default), python_literal("/"), opt(python_literal(",")), opt(repeat1(seq!(repeat1(&param_with_default), python_literal("/"), opt(python_literal(","))))))).into_boxed());
    let slash_no_default = Rc::new(slash_no_default.set(seq!(repeat1(&param_no_default), python_literal("/"), opt(python_literal(",")))).into_boxed());
    let parameters = Rc::new(parameters.set(choice!(
        seq!(&slash_no_default, opt(repeat1(&param_no_default)), opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(&slash_with_default, opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(repeat1(&param_no_default), opt(repeat1(&param_with_default)), opt(&star_etc)),
        seq!(repeat1(&param_with_default), opt(&star_etc)),
        &star_etc
    )).into_boxed());
    let params = Rc::new(params.set(&parameters).into_boxed());
    let function_def_raw = Rc::new(function_def_raw.set(choice!(
        seq!(python_literal("def"), &NAME, opt(&type_params), python_literal("("), opt(&params), python_literal(")"), opt(seq!(python_literal("->"), &expression)), python_literal(":"), opt(&func_type_comment), &block),
        seq!(python_literal("async"), python_literal("def"), &NAME, opt(&type_params), python_literal("("), opt(&params), python_literal(")"), opt(seq!(python_literal("->"), &expression)), python_literal(":"), opt(&func_type_comment), &block)
    )).into_boxed());
    let function_def = Rc::new(function_def.set(choice!(
        seq!(python_literal("@"), &named_expression, &NEWLINE, opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE))), &function_def_raw),
        &function_def_raw
    )).into_boxed());
    let class_def_raw = Rc::new(class_def_raw.set(seq!(python_literal("class"), &NAME, opt(&type_params), opt(seq!(python_literal("("), opt(&arguments), python_literal(")"))), python_literal(":"), &block)).into_boxed());
    let class_def = Rc::new(class_def.set(choice!(
        seq!(python_literal("@"), &named_expression, &NEWLINE, opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE))), &class_def_raw),
        &class_def_raw
    )).into_boxed());
    let decorators = Rc::new(decorators.set(seq!(python_literal("@"), &named_expression, &NEWLINE, opt(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE))))).into_boxed());
    let block = Rc::new(block.set(choice!(
        seq!(&NEWLINE, &INDENT, &statements, &DEDENT),
        seq!(&simple_stmt, choice!(&NEWLINE, seq!(opt(repeat1(seq!(python_literal(";"), &simple_stmt))), opt(python_literal(";")), &NEWLINE)))
    )).into_boxed());
    let dotted_name = Rc::new(dotted_name.set(seq!(&NAME, opt(repeat1(seq!(python_literal("."), &NAME))))).into_boxed());
    let dotted_as_name = Rc::new(dotted_as_name.set(seq!(&dotted_name, opt(seq!(python_literal("as"), &NAME)))).into_boxed());
    let dotted_as_names = Rc::new(dotted_as_names.set(seq!(&dotted_as_name, opt(repeat1(seq!(python_literal(","), &dotted_as_name))))).into_boxed());
    let import_from_as_name = Rc::new(import_from_as_name.set(seq!(&NAME, opt(seq!(python_literal("as"), &NAME)))).into_boxed());
    let import_from_as_names = Rc::new(import_from_as_names.set(seq!(&import_from_as_name, opt(repeat1(seq!(python_literal(","), &import_from_as_name))))).into_boxed());
    let import_from_targets = Rc::new(import_from_targets.set(choice!(
        seq!(python_literal("("), &import_from_as_names, opt(python_literal(",")), python_literal(")")),
        &import_from_as_names,
        python_literal("*")
    )).into_boxed());
    let import_from = Rc::new(import_from.set(seq!(python_literal("from"), choice!(seq!(opt(repeat1(choice!(python_literal("."), python_literal("...")))), &dotted_name, python_literal("import"), &import_from_targets), seq!(repeat1(choice!(python_literal("."), python_literal("..."))), python_literal("import"), &import_from_targets)))).into_boxed());
    let import_name = Rc::new(import_name.set(seq!(python_literal("import"), &dotted_as_names)).into_boxed());
    let import_stmt = Rc::new(import_stmt.set(choice!(
        &import_name,
        &import_from
    )).into_boxed());
    let assert_stmt = Rc::new(assert_stmt.set(seq!(python_literal("assert"), &expression, opt(seq!(python_literal(","), &expression)))).into_boxed());
    let yield_stmt = Rc::new(yield_stmt.set(&yield_expr).into_boxed());
    let del_stmt = Rc::new(del_stmt.set(seq!(python_literal("del"), &del_targets)).into_boxed());
    let nonlocal_stmt = Rc::new(nonlocal_stmt.set(seq!(python_literal("nonlocal"), &NAME, opt(repeat1(seq!(python_literal(","), &NAME))))).into_boxed());
    let global_stmt = Rc::new(global_stmt.set(seq!(python_literal("global"), &NAME, opt(repeat1(seq!(python_literal(","), &NAME))))).into_boxed());
    let raise_stmt = Rc::new(raise_stmt.set(seq!(python_literal("raise"), opt(seq!(&expression, opt(seq!(python_literal("from"), &expression)))))).into_boxed());
    let return_stmt = Rc::new(return_stmt.set(seq!(python_literal("return"), opt(&star_expressions))).into_boxed());
    let augassign = Rc::new(augassign.set(choice!(
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
    )).into_boxed());
    let annotated_rhs = Rc::new(annotated_rhs.set(choice!(
        &yield_expr,
        &star_expressions
    )).into_boxed());
    let assignment = Rc::new(assignment.set(choice!(
        seq!(&NAME, python_literal(":"), &expression, opt(seq!(python_literal("="), &annotated_rhs))),
        seq!(choice!(seq!(python_literal("("), &single_target, python_literal(")")), &single_subscript_attribute_target), python_literal(":"), &expression, opt(seq!(python_literal("="), &annotated_rhs))),
        seq!(&star_targets, python_literal("="), opt(repeat1(seq!(&star_targets, python_literal("=")))), choice!(&yield_expr, &star_expressions), opt(&TYPE_COMMENT)),
        seq!(&single_target, &augassign, cut(), choice!(&yield_expr, &star_expressions))
    )).into_boxed());
    let compound_stmt = Rc::new(compound_stmt.set(seq!(choice!(&function_def, &if_stmt, &class_def, &with_stmt, &for_stmt, &try_stmt, &while_stmt, &match_stmt), opt(repeat1(choice!(&function_def, &if_stmt, &class_def, &with_stmt, &for_stmt, &try_stmt, &while_stmt))))).into_boxed());
    let simple_stmt = Rc::new(simple_stmt.set(seq!(choice!(&assignment, &type_alias, &star_expressions, &return_stmt, &import_stmt, &raise_stmt, python_literal("pass"), &del_stmt, &yield_stmt, &assert_stmt, python_literal("break"), python_literal("continue"), &global_stmt, &nonlocal_stmt), opt(repeat1(choice!(&type_alias, &return_stmt, &import_stmt, &raise_stmt, &del_stmt, &yield_stmt, &assert_stmt, &global_stmt, &nonlocal_stmt))))).into_boxed());
    let simple_stmts = Rc::new(simple_stmts.set(seq!(&simple_stmt, choice!(&NEWLINE, seq!(opt(repeat1(seq!(python_literal(";"), &simple_stmt))), opt(python_literal(";")), &NEWLINE)))).into_boxed());
    let statement_newline = Rc::new(statement_newline.set(choice!(
        seq!(&compound_stmt, &NEWLINE),
        &simple_stmts,
        &NEWLINE,
        &ENDMARKER
    )).into_boxed());
    let statement = Rc::new(statement.set(choice!(
        &compound_stmt,
        &simple_stmts
    )).into_boxed());
    let statements = Rc::new(statements.set(repeat1(&statement)).into_boxed());
    let func_type = Rc::new(func_type.set(seq!(python_literal("("), opt(&type_expressions), python_literal(")"), python_literal("->"), &expression, opt(repeat1(&NEWLINE)), &ENDMARKER)).into_boxed());
    let eval = Rc::new(eval.set(seq!(&expressions, opt(repeat1(&NEWLINE)), &ENDMARKER)).into_boxed());
    let interactive = Rc::new(interactive.set(&statement_newline).into_boxed());
    let file = Rc::new(file.set(seq!(opt(&statements), repeat1(&ENDMARKER))).into_boxed());
    seq!(repeat0(NEWLINE), file).into_boxed().into()
}
