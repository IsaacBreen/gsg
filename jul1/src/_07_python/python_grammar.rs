use crate::seq;
use crate::IntoDyn;
use crate::{cache_context, cached, deferred, eat_string, forbid_follows, forbid_follows_check_not, forbid_follows_clear, lookahead, mutate_right_data, negative_lookahead, tag, CombinatorTrait, RightData};
use crate::{choice_greedy as choice, opt_greedy as opt, repeat0_greedy as repeat0, repeat1_greedy as repeat1, seprep1_greedy as seprep1};
use crate::BaseCombinatorTrait;
use std::rc::Rc;

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

use super::python_tokenizer as token;

pub fn python_literal(s: &str) -> impl CombinatorTrait {
    let increment_scope_count = |right_data: &mut RightData| { Rc::make_mut(&mut right_data.right_data_inner).get_fields1_mut().scope_count += 1; true };
    let decrement_scope_count = |right_data: &mut RightData| { Rc::make_mut(&mut right_data.right_data_inner).get_fields1_mut().scope_count -= 1; true };

    match s {
        "(" | "[" | "{" => seq!(eat_string(s), mutate_right_data(increment_scope_count), forbid_follows_clear(), opt(deferred(WS))).into_dyn(),
        ")" | "]" | "}" => seq!(eat_string(s), mutate_right_data(decrement_scope_count), forbid_follows_clear(), opt(deferred(WS))).into_dyn(),
        _ => seq!(eat_string(s), forbid_follows_clear(), opt(deferred(WS))).into_dyn(),
    }
}
pub fn WS() -> impl CombinatorTrait { tag("WS", crate::profile("WS", seq!(forbid_follows_check_not(Forbidden::WS as usize), token::WS().compile(), forbid_follows(&[Forbidden::INDENT as usize, Forbidden::DEDENT as usize])))) }
pub fn NAME() -> impl CombinatorTrait { seq!(tag("NAME", crate::profile("NAME", seq!(forbid_follows_check_not(Forbidden::NAME as usize), token::NAME().compile(), forbid_follows(&[Forbidden::NAME as usize, Forbidden::NUMBER as usize])))), opt(deferred(WS))) }
pub fn TYPE_COMMENT() -> impl CombinatorTrait { seq!(tag("TYPE_COMMENT", crate::profile("TYPE_COMMENT", seq!(token::TYPE_COMMENT().compile(), forbid_follows_clear()))), opt(deferred(WS))) }
pub fn FSTRING_START() -> impl CombinatorTrait { tag("FSTRING_START", crate::profile("FSTRING_START", seq!(token::FSTRING_START().compile(), forbid_follows(&[Forbidden::WS as usize, Forbidden::NEWLINE as usize])))) }
pub fn FSTRING_MIDDLE() -> impl CombinatorTrait { tag("FSTRING_MIDDLE", crate::profile("FSTRING_MIDDLE", seq!(forbid_follows_check_not(Forbidden::FSTRING_MIDDLE as usize), token::FSTRING_MIDDLE().compile(), forbid_follows(&[Forbidden::FSTRING_MIDDLE as usize, Forbidden::WS as usize])))) }
pub fn FSTRING_END() -> impl CombinatorTrait { seq!(tag("FSTRING_END", crate::profile("FSTRING_END", seq!(token::FSTRING_END().compile(), forbid_follows_clear()))), opt(deferred(WS))) }
pub fn NUMBER() -> impl CombinatorTrait { seq!(tag("NUMBER", crate::profile("NUMBER", seq!(forbid_follows_check_not(Forbidden::NUMBER as usize), token::NUMBER().compile(), forbid_follows(&[Forbidden::NUMBER as usize])))), opt(deferred(WS))) }
pub fn STRING() -> impl CombinatorTrait { seq!(tag("STRING", crate::profile("STRING", seq!(token::STRING().compile(), forbid_follows_clear()))), opt(deferred(WS))) }
pub fn NEWLINE() -> impl CombinatorTrait { tag("NEWLINE", crate::profile("NEWLINE", seq!(forbid_follows_check_not(Forbidden::NEWLINE as usize), token::NEWLINE().compile(), forbid_follows(&[Forbidden::WS as usize])))) }
pub fn INDENT() -> impl CombinatorTrait { tag("INDENT", crate::profile("INDENT", seq!(forbid_follows_check_not(Forbidden::INDENT as usize), token::INDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))) }
pub fn DEDENT() -> impl CombinatorTrait { tag("DEDENT", crate::profile("DEDENT", seq!(forbid_follows_check_not(Forbidden::DEDENT as usize), token::DEDENT().compile(), forbid_follows(&[Forbidden::WS as usize])))) }
pub fn ENDMARKER() -> impl CombinatorTrait { seq!(tag("ENDMARKER", crate::profile("ENDMARKER", seq!(token::ENDMARKER().compile(), forbid_follows_clear()))), opt(deferred(WS))) }

pub fn expression_without_invalid() -> impl CombinatorTrait {
    tag("expression_without_invalid", choice!(
        seq!(deferred(disjunction).into_dyn(), opt(seq!(python_literal("if"), deferred(disjunction).into_dyn(), python_literal("else"), deferred(expression).into_dyn()))),
        deferred(lambdef).into_dyn()
    ))
}

pub fn func_type_comment() -> impl CombinatorTrait {
    tag("func_type_comment", choice!(
        seq!(deferred(NEWLINE).into_dyn(), deferred(TYPE_COMMENT).into_dyn(), lookahead(seq!(deferred(NEWLINE).into_dyn(), deferred(INDENT).into_dyn()))),
        deferred(TYPE_COMMENT).into_dyn()
    ))
}

pub fn type_expressions() -> impl CombinatorTrait {
    tag("type_expressions", choice!(
        seq!(seprep1(deferred(expression).into_dyn(), python_literal(",")), opt(seq!(python_literal(","), choice!(seq!(python_literal("*"), deferred(expression).into_dyn(), opt(seq!(python_literal(","), python_literal("**"), deferred(expression).into_dyn()))), seq!(python_literal("**"), deferred(expression).into_dyn()))))),
        seq!(python_literal("*"), deferred(expression).into_dyn(), opt(seq!(python_literal(","), python_literal("**"), deferred(expression).into_dyn()))),
        seq!(python_literal("**"), deferred(expression).into_dyn())
    ))
}

pub fn del_t_atom() -> impl CombinatorTrait {
    tag("del_t_atom", choice!(
        deferred(NAME).into_dyn(),
        seq!(python_literal("("), choice!(seq!(deferred(del_target).into_dyn(), python_literal(")")), seq!(opt(deferred(del_targets).into_dyn()), python_literal(")")))),
        seq!(python_literal("["), opt(deferred(del_targets).into_dyn()), python_literal("]"))
    ))
}

pub fn del_target() -> impl CombinatorTrait {
    cached(tag("del_target", choice!(
        seq!(deferred(t_primary).into_dyn(), choice!(seq!(python_literal("."), deferred(NAME).into_dyn(), negative_lookahead(deferred(t_lookahead).into_dyn())), seq!(python_literal("["), deferred(slices).into_dyn(), python_literal("]"), negative_lookahead(deferred(t_lookahead).into_dyn())))),
        del_t_atom()
    )))
}

pub fn del_targets() -> impl CombinatorTrait {
    tag("del_targets", seq!(
        seprep1(deferred(del_target).into_dyn(), python_literal(",")),
        opt(python_literal(","))
    ))
}

pub fn t_lookahead() -> impl CombinatorTrait {
    tag("t_lookahead", choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    ))
}

pub fn t_primary() -> impl CombinatorTrait {
    tag("t_primary", seq!(
        deferred(atom).into_dyn(),
        lookahead(deferred(t_lookahead).into_dyn()),
        repeat0(choice!(seq!(python_literal("."), deferred(NAME).into_dyn(), lookahead(deferred(t_lookahead).into_dyn())), seq!(python_literal("["), deferred(slices).into_dyn(), python_literal("]"), lookahead(deferred(t_lookahead).into_dyn())), seq!(deferred(genexp).into_dyn(), lookahead(deferred(t_lookahead).into_dyn())), seq!(python_literal("("), opt(deferred(arguments).into_dyn()), python_literal(")"), lookahead(deferred(t_lookahead).into_dyn()))))
    ))
}

pub fn single_subscript_attribute_target() -> impl CombinatorTrait {
    tag("single_subscript_attribute_target", seq!(
        deferred(t_primary).into_dyn(),
        choice!(seq!(python_literal("."), deferred(NAME).into_dyn(), negative_lookahead(deferred(t_lookahead).into_dyn())), seq!(python_literal("["), deferred(slices).into_dyn(), python_literal("]"), negative_lookahead(deferred(t_lookahead).into_dyn())))
    ))
}

pub fn single_target() -> impl CombinatorTrait {
    tag("single_target", choice!(
        deferred(single_subscript_attribute_target).into_dyn(),
        deferred(NAME).into_dyn(),
        seq!(python_literal("("), deferred(single_target).into_dyn(), python_literal(")"))
    ))
}

pub fn star_atom() -> impl CombinatorTrait {
    tag("star_atom", choice!(
        deferred(NAME).into_dyn(),
        seq!(python_literal("("), choice!(seq!(deferred(target_with_star_atom).into_dyn(), python_literal(")")), seq!(opt(star_targets_tuple_seq()), python_literal(")")))),
        seq!(python_literal("["), opt(star_targets_list_seq()), python_literal("]"))
    ))
}

pub fn target_with_star_atom() -> impl CombinatorTrait {
    cached(tag("target_with_star_atom", choice!(
        seq!(deferred(t_primary).into_dyn(), choice!(seq!(python_literal("."), deferred(NAME).into_dyn(), negative_lookahead(deferred(t_lookahead).into_dyn())), seq!(python_literal("["), deferred(slices).into_dyn(), python_literal("]"), negative_lookahead(deferred(t_lookahead).into_dyn())))),
        star_atom()
    )))
}

pub fn star_target() -> impl CombinatorTrait {
    cached(tag("star_target", choice!(
        seq!(python_literal("*"), negative_lookahead(python_literal("*")), deferred(star_target).into_dyn()),
        deferred(target_with_star_atom).into_dyn()
    )))
}

pub fn star_targets_tuple_seq() -> impl CombinatorTrait {
    tag("star_targets_tuple_seq", seq!(
        deferred(star_target).into_dyn(),
        choice!(seq!(repeat1(seq!(python_literal(","), deferred(star_target).into_dyn())), opt(python_literal(","))), python_literal(","))
    ))
}

pub fn star_targets_list_seq() -> impl CombinatorTrait {
    tag("star_targets_list_seq", seq!(
        seprep1(deferred(star_target).into_dyn(), python_literal(",")),
        opt(python_literal(","))
    ))
}

pub fn star_targets() -> impl CombinatorTrait {
    tag("star_targets", seq!(
        deferred(star_target).into_dyn(),
        choice!(negative_lookahead(python_literal(",")), seq!(repeat0(seq!(python_literal(","), deferred(star_target).into_dyn())), opt(python_literal(","))))
    ))
}

pub fn kwarg_or_double_starred() -> impl CombinatorTrait {
    tag("kwarg_or_double_starred", choice!(
        seq!(deferred(NAME).into_dyn(), python_literal("="), deferred(expression).into_dyn()),
        seq!(python_literal("**"), deferred(expression).into_dyn())
    ))
}

pub fn kwarg_or_starred() -> impl CombinatorTrait {
    tag("kwarg_or_starred", choice!(
        seq!(deferred(NAME).into_dyn(), python_literal("="), deferred(expression).into_dyn()),
        deferred(starred_expression).into_dyn()
    ))
}

pub fn starred_expression() -> impl CombinatorTrait {
    tag("starred_expression", seq!(python_literal("*"), deferred(expression).into_dyn()))
}

pub fn kwargs() -> impl CombinatorTrait {
    tag("kwargs", choice!(
        seq!(seprep1(kwarg_or_starred(), python_literal(",")), opt(seq!(python_literal(","), seprep1(deferred(kwarg_or_double_starred).into_dyn(), python_literal(","))))),
        seprep1(deferred(kwarg_or_double_starred).into_dyn(), python_literal(","))
    ))
}

pub fn args() -> impl CombinatorTrait {
    tag("args", choice!(
        seq!(seprep1(choice!(deferred(starred_expression).into_dyn(), seq!(choice!(deferred(assignment_expression).into_dyn(), seq!(deferred(expression).into_dyn(), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), python_literal(",")), opt(seq!(python_literal(","), deferred(kwargs).into_dyn()))),
        deferred(kwargs).into_dyn()
    ))
}

pub fn arguments() -> impl CombinatorTrait {
    cached(tag("arguments", seq!(args(), opt(python_literal(",")), lookahead(python_literal(")")))))
}

pub fn dictcomp() -> impl CombinatorTrait {
    tag("dictcomp", seq!(
        python_literal("{"),
        deferred(kvpair).into_dyn(),
        deferred(for_if_clauses).into_dyn(),
        python_literal("}")
    ))
}

pub fn genexp() -> impl CombinatorTrait {
    tag("genexp", seq!(
        python_literal("("),
        choice!(deferred(assignment_expression).into_dyn(), seq!(deferred(expression).into_dyn(), negative_lookahead(python_literal(":=")))),
        deferred(for_if_clauses).into_dyn(),
        python_literal(")")
    ))
}

pub fn setcomp() -> impl CombinatorTrait {
    tag("setcomp", seq!(
        python_literal("{"),
        deferred(named_expression).into_dyn(),
        deferred(for_if_clauses).into_dyn(),
        python_literal("}")
    ))
}

pub fn listcomp() -> impl CombinatorTrait {
    tag("listcomp", seq!(
        python_literal("["),
        deferred(named_expression).into_dyn(),
        deferred(for_if_clauses).into_dyn(),
        python_literal("]")
    ))
}

pub fn for_if_clause() -> impl CombinatorTrait {
    tag("for_if_clause", choice!(
        seq!(python_literal("async"), python_literal("for"), deferred(star_targets).into_dyn(), python_literal("in"), deferred(disjunction).into_dyn(), repeat0(seq!(python_literal("if"), deferred(disjunction).into_dyn()))),
        seq!(python_literal("for"), deferred(star_targets).into_dyn(), python_literal("in"), deferred(disjunction).into_dyn(), repeat0(seq!(python_literal("if"), deferred(disjunction).into_dyn())))
    ))
}

pub fn for_if_clauses() -> impl CombinatorTrait {
    tag("for_if_clauses", repeat1(for_if_clause()))
}

pub fn kvpair() -> impl CombinatorTrait {
    tag("kvpair", seq!(
        deferred(expression).into_dyn(),
        python_literal(":"),
        deferred(expression).into_dyn()
    ))
}

pub fn double_starred_kvpair() -> impl CombinatorTrait {
    tag("double_starred_kvpair", choice!(
        seq!(python_literal("**"), deferred(bitwise_or).into_dyn()),
        deferred(kvpair).into_dyn()
    ))
}

pub fn double_starred_kvpairs() -> impl CombinatorTrait {
    tag("double_starred_kvpairs", seq!(
        seprep1(double_starred_kvpair(), python_literal(",")),
        opt(python_literal(","))
    ))
}

pub fn dict() -> impl CombinatorTrait {
    tag("dict", seq!(python_literal("{"), opt(double_starred_kvpairs()), python_literal("}")))
}

pub fn set() -> impl CombinatorTrait {
    tag("set", seq!(
        python_literal("{"),
        deferred(star_named_expressions).into_dyn(),
        python_literal("}")
    ))
}

pub fn tuple() -> impl CombinatorTrait {
    tag("tuple", seq!(
        python_literal("("),
        opt(seq!(deferred(star_named_expression).into_dyn(), python_literal(","), opt(deferred(star_named_expressions).into_dyn()))),
        python_literal(")")
    ))
}

pub fn list() -> impl CombinatorTrait {
    tag("list", seq!(
        python_literal("["),
        opt(deferred(star_named_expressions).into_dyn()),
        python_literal("]")
    ))
}

pub fn strings() -> impl CombinatorTrait {
    cached(tag("strings", repeat1(choice!(fstring(), string()))))
}

pub fn string() -> impl CombinatorTrait {
    tag("string", deferred(STRING).into_dyn())
}

pub fn fstring() -> impl CombinatorTrait {
    tag("fstring", seq!(
        deferred(FSTRING_START).into_dyn(),
        repeat0(fstring_middle()),
        FSTRING_END()
    ))
}

pub fn fstring_format_spec() -> impl CombinatorTrait {
    tag("fstring_format_spec", choice!(
        deferred(FSTRING_MIDDLE).into_dyn(),
        deferred(fstring_replacement_field).into_dyn()
    ))
}

pub fn fstring_full_format_spec() -> impl CombinatorTrait {
    tag("fstring_full_format_spec", seq!(python_literal(":"), repeat0(fstring_format_spec())))
}

pub fn fstring_conversion() -> impl CombinatorTrait {
    tag("fstring_conversion", seq!(python_literal("!"), deferred(NAME).into_dyn()))
}

pub fn fstring_replacement_field() -> impl CombinatorTrait {
    tag("fstring_replacement_field", seq!(
        python_literal("{"),
        deferred(annotated_rhs).into_dyn(),
        opt(python_literal("=")),
        opt(fstring_conversion()),
        opt(fstring_full_format_spec()),
        python_literal("}")
    ))
}

pub fn fstring_middle() -> impl CombinatorTrait {
    tag("fstring_middle", choice!(
        deferred(fstring_replacement_field).into_dyn(),
        deferred(FSTRING_MIDDLE).into_dyn()
    ))
}

pub fn lambda_param() -> impl CombinatorTrait {
    tag("lambda_param", deferred(NAME).into_dyn())
}

pub fn lambda_param_maybe_default() -> impl CombinatorTrait {
    tag("lambda_param_maybe_default", seq!(
        deferred(lambda_param).into_dyn(),
        opt(deferred(default).into_dyn()),
        choice!(python_literal(","), lookahead(python_literal(":")))
    ))
}

pub fn lambda_param_with_default() -> impl CombinatorTrait {
    tag("lambda_param_with_default", seq!(
        deferred(lambda_param).into_dyn(),
        deferred(default).into_dyn(),
        choice!(python_literal(","), lookahead(python_literal(":")))
    ))
}

pub fn lambda_param_no_default() -> impl CombinatorTrait {
    tag("lambda_param_no_default", seq!(
        deferred(lambda_param).into_dyn(),
        choice!(python_literal(","), lookahead(python_literal(":")))
    ))
}

pub fn lambda_kwds() -> impl CombinatorTrait {
    tag("lambda_kwds", seq!(python_literal("**"), deferred(lambda_param_no_default).into_dyn()))
}

pub fn lambda_star_etc() -> impl CombinatorTrait {
    tag("lambda_star_etc", choice!(
        seq!(python_literal("*"), choice!(seq!(deferred(lambda_param_no_default).into_dyn(), repeat0(deferred(lambda_param_maybe_default).into_dyn()), opt(deferred(lambda_kwds).into_dyn())), seq!(python_literal(","), repeat1(deferred(lambda_param_maybe_default).into_dyn()), opt(deferred(lambda_kwds).into_dyn())))),
        deferred(lambda_kwds).into_dyn()
    ))
}

pub fn lambda_slash_with_default() -> impl CombinatorTrait {
    tag("lambda_slash_with_default", seq!(
        repeat0(deferred(lambda_param_no_default).into_dyn()),
        repeat1(deferred(lambda_param_with_default).into_dyn()),
        python_literal("/"),
        choice!(python_literal(","), lookahead(python_literal(":")))
    ))
}

pub fn lambda_slash_no_default() -> impl CombinatorTrait {
    tag("lambda_slash_no_default", seq!(
        repeat1(deferred(lambda_param_no_default).into_dyn()),
        python_literal("/"),
        choice!(python_literal(","), lookahead(python_literal(":")))
    ))
}

pub fn lambda_parameters() -> impl CombinatorTrait {
    tag("lambda_parameters", choice!(
        seq!(lambda_slash_no_default(), repeat0(deferred(lambda_param_no_default).into_dyn()), repeat0(deferred(lambda_param_with_default).into_dyn()), opt(deferred(lambda_star_etc).into_dyn())),
        seq!(lambda_slash_with_default(), repeat0(deferred(lambda_param_with_default).into_dyn()), opt(deferred(lambda_star_etc).into_dyn())),
        seq!(repeat1(deferred(lambda_param_no_default).into_dyn()), repeat0(deferred(lambda_param_with_default).into_dyn()), opt(deferred(lambda_star_etc).into_dyn())),
        seq!(repeat1(deferred(lambda_param_with_default).into_dyn()), opt(deferred(lambda_star_etc).into_dyn())),
        deferred(lambda_star_etc).into_dyn()
    ))
}

pub fn lambda_params() -> impl CombinatorTrait {
    tag("lambda_params", lambda_parameters())
}

pub fn lambdef() -> impl CombinatorTrait {
    tag("lambdef", seq!(
        python_literal("lambda"),
        opt(lambda_params()),
        python_literal(":"),
        deferred(expression).into_dyn()
    ))
}

pub fn group() -> impl CombinatorTrait {
    tag("group", seq!(
        python_literal("("),
        choice!(deferred(yield_expr).into_dyn(), deferred(named_expression).into_dyn()),
        python_literal(")")
    ))
}

pub fn atom() -> impl CombinatorTrait {
    tag("atom", choice!(
        deferred(NAME).into_dyn(),
        python_literal("True"),
        python_literal("False"),
        python_literal("None"),
        seq!(lookahead(choice!(deferred(STRING).into_dyn(), deferred(FSTRING_START).into_dyn())), deferred(strings).into_dyn()),
        deferred(NUMBER).into_dyn(),
        seq!(lookahead(python_literal("(")), choice!(tuple(), group(), deferred(genexp).into_dyn())),
        seq!(lookahead(python_literal("[")), choice!(list(), listcomp())),
        seq!(lookahead(python_literal("{")), choice!(dict(), set(), dictcomp(), setcomp())),
        python_literal("...")
    ))
}

pub fn slice() -> impl CombinatorTrait {
    tag("slice", choice!(
        seq!(opt(deferred(expression).into_dyn()), python_literal(":"), opt(deferred(expression).into_dyn()), opt(seq!(python_literal(":"), opt(deferred(expression).into_dyn())))),
        deferred(named_expression).into_dyn()
    ))
}

pub fn slices() -> impl CombinatorTrait {
    tag("slices", choice!(
        seq!(deferred(slice).into_dyn(), negative_lookahead(python_literal(","))),
        seq!(seprep1(choice!(deferred(slice).into_dyn(), deferred(starred_expression).into_dyn()), python_literal(",")), opt(python_literal(",")))
    ))
}

pub fn primary() -> impl CombinatorTrait {
    tag("primary", seq!(
        deferred(atom).into_dyn(),
        repeat0(choice!(seq!(python_literal("."), deferred(NAME).into_dyn()), deferred(genexp).into_dyn(), seq!(python_literal("("), opt(deferred(arguments).into_dyn()), python_literal(")")), seq!(python_literal("["), deferred(slices).into_dyn(), python_literal("]"))))
    ))
}

pub fn await_primary() -> impl CombinatorTrait {
    cached(tag("await_primary", choice!(
        seq!(python_literal("await"), deferred(primary).into_dyn()),
        deferred(primary).into_dyn()
    )))
}

pub fn power() -> impl CombinatorTrait {
    tag("power", seq!(
        await_primary(),
        opt(seq!(python_literal("**"), deferred(factor).into_dyn()))
    ))
}

pub fn factor() -> impl CombinatorTrait {
    cached(tag("factor", choice!(
        seq!(python_literal("+"), deferred(factor).into_dyn()),
        seq!(python_literal("-"), deferred(factor).into_dyn()),
        seq!(python_literal("~"), deferred(factor).into_dyn()),
        power()
    )))
}

pub fn term() -> impl CombinatorTrait {
    tag("term", seq!(
        deferred(factor).into_dyn(),
        repeat0(choice!(seq!(python_literal("*"), deferred(factor).into_dyn()), seq!(python_literal("/"), deferred(factor).into_dyn()), seq!(python_literal("//"), deferred(factor).into_dyn()), seq!(python_literal("%"), deferred(factor).into_dyn()), seq!(python_literal("@"), deferred(factor).into_dyn())))
    ))
}

pub fn sum() -> impl CombinatorTrait {
    tag("sum", seq!(
        deferred(term).into_dyn(),
        repeat0(choice!(seq!(python_literal("+"), deferred(term).into_dyn()), seq!(python_literal("-"), deferred(term).into_dyn())))
    ))
}

pub fn shift_expr() -> impl CombinatorTrait {
    tag("shift_expr", seq!(
        deferred(sum).into_dyn(),
        repeat0(choice!(seq!(python_literal("<<"), deferred(sum).into_dyn()), seq!(python_literal(">>"), deferred(sum).into_dyn())))
    ))
}

pub fn bitwise_and() -> impl CombinatorTrait {
    tag("bitwise_and", seq!(
        deferred(shift_expr).into_dyn(),
        repeat0(seq!(python_literal("&"), deferred(shift_expr).into_dyn()))
    ))
}

pub fn bitwise_xor() -> impl CombinatorTrait {
    tag("bitwise_xor", seq!(
        deferred(bitwise_and).into_dyn(),
        repeat0(seq!(python_literal("^"), deferred(bitwise_and).into_dyn()))
    ))
}

pub fn bitwise_or() -> impl CombinatorTrait {
    tag("bitwise_or", seq!(
        deferred(bitwise_xor).into_dyn(),
        repeat0(seq!(python_literal("|"), deferred(bitwise_xor).into_dyn()))
    ))
}

pub fn is_bitwise_or() -> impl CombinatorTrait {
    tag("is_bitwise_or", seq!(python_literal("is"), deferred(bitwise_or).into_dyn()))
}

pub fn isnot_bitwise_or() -> impl CombinatorTrait {
    tag("isnot_bitwise_or", seq!(
        python_literal("is"),
        python_literal("not"),
        deferred(bitwise_or).into_dyn()
    ))
}

pub fn in_bitwise_or() -> impl CombinatorTrait {
    tag("in_bitwise_or", seq!(python_literal("in"), deferred(bitwise_or).into_dyn()))
}

pub fn notin_bitwise_or() -> impl CombinatorTrait {
    tag("notin_bitwise_or", seq!(
        python_literal("not"),
        python_literal("in"),
        deferred(bitwise_or).into_dyn()
    ))
}

pub fn gt_bitwise_or() -> impl CombinatorTrait {
    tag("gt_bitwise_or", seq!(python_literal(">"), deferred(bitwise_or).into_dyn()))
}

pub fn gte_bitwise_or() -> impl CombinatorTrait {
    tag("gte_bitwise_or", seq!(python_literal(">="), deferred(bitwise_or).into_dyn()))
}

pub fn lt_bitwise_or() -> impl CombinatorTrait {
    tag("lt_bitwise_or", seq!(python_literal("<"), deferred(bitwise_or).into_dyn()))
}

pub fn lte_bitwise_or() -> impl CombinatorTrait {
    tag("lte_bitwise_or", seq!(python_literal("<="), deferred(bitwise_or).into_dyn()))
}

pub fn noteq_bitwise_or() -> impl CombinatorTrait {
    tag("noteq_bitwise_or", seq!(python_literal("!="), deferred(bitwise_or).into_dyn()))
}

pub fn eq_bitwise_or() -> impl CombinatorTrait {
    tag("eq_bitwise_or", seq!(python_literal("=="), deferred(bitwise_or).into_dyn()))
}

pub fn compare_op_bitwise_or_pair() -> impl CombinatorTrait {
    tag("compare_op_bitwise_or_pair", choice!(
        eq_bitwise_or(),
        noteq_bitwise_or(),
        lte_bitwise_or(),
        lt_bitwise_or(),
        gte_bitwise_or(),
        gt_bitwise_or(),
        notin_bitwise_or(),
        in_bitwise_or(),
        isnot_bitwise_or(),
        is_bitwise_or()
    ))
}

pub fn comparison() -> impl CombinatorTrait {
    tag("comparison", seq!(deferred(bitwise_or).into_dyn(), repeat0(compare_op_bitwise_or_pair())))
}

pub fn inversion() -> impl CombinatorTrait {
    cached(tag("inversion", choice!(
        seq!(python_literal("not"), deferred(inversion).into_dyn()),
        comparison()
    )))
}

pub fn conjunction() -> impl CombinatorTrait {
    cached(tag("conjunction", seq!(
        deferred(inversion).into_dyn(),
        repeat0(seq!(python_literal("and"), deferred(inversion).into_dyn()))
    )))
}

pub fn disjunction() -> impl CombinatorTrait {
    cached(tag("disjunction", seq!(
        deferred(conjunction).into_dyn(),
        repeat0(seq!(python_literal("or"), deferred(conjunction).into_dyn()))
    )))
}

pub fn named_expression() -> impl CombinatorTrait {
    tag("named_expression", choice!(
        deferred(assignment_expression).into_dyn(),
        seq!(deferred(expression).into_dyn(), negative_lookahead(python_literal(":=")))
    ))
}

pub fn assignment_expression() -> impl CombinatorTrait {
    tag("assignment_expression", seq!(
        deferred(NAME).into_dyn(),
        python_literal(":="),
        deferred(expression).into_dyn()
    ))
}

pub fn star_named_expression() -> impl CombinatorTrait {
    tag("star_named_expression", choice!(
        seq!(python_literal("*"), deferred(bitwise_or).into_dyn()),
        deferred(named_expression).into_dyn()
    ))
}

pub fn star_named_expressions() -> impl CombinatorTrait {
    tag("star_named_expressions", seq!(
        seprep1(deferred(star_named_expression).into_dyn(), python_literal(",")),
        opt(python_literal(","))
    ))
}

pub fn star_expression() -> impl CombinatorTrait {
    cached(tag("star_expression", choice!(
        seq!(python_literal("*"), deferred(bitwise_or).into_dyn()),
        deferred(expression).into_dyn()
    )))
}

pub fn star_expressions() -> impl CombinatorTrait {
    tag("star_expressions", seq!(
        deferred(star_expression).into_dyn(),
        opt(choice!(seq!(repeat1(seq!(python_literal(","), deferred(star_expression).into_dyn())), opt(python_literal(","))), python_literal(",")))
    ))
}

pub fn yield_expr() -> impl CombinatorTrait {
    tag("yield_expr", seq!(
        python_literal("yield"),
        choice!(seq!(python_literal("from"), deferred(expression).into_dyn()), opt(deferred(star_expressions).into_dyn()))
    ))
}

pub fn expression() -> impl CombinatorTrait {
    cached(tag("expression", choice!(
        seq!(deferred(disjunction).into_dyn(), opt(seq!(python_literal("if"), deferred(disjunction).into_dyn(), python_literal("else"), deferred(expression).into_dyn()))),
        deferred(lambdef).into_dyn()
    )))
}

pub fn expressions() -> impl CombinatorTrait {
    tag("expressions", seq!(
        deferred(expression).into_dyn(),
        opt(choice!(seq!(repeat1(seq!(python_literal(","), deferred(expression).into_dyn())), opt(python_literal(","))), python_literal(",")))
    ))
}

pub fn type_param_starred_default() -> impl CombinatorTrait {
    tag("type_param_starred_default", seq!(python_literal("="), deferred(star_expression).into_dyn()))
}

pub fn type_param_default() -> impl CombinatorTrait {
    tag("type_param_default", seq!(python_literal("="), deferred(expression).into_dyn()))
}

pub fn type_param_bound() -> impl CombinatorTrait {
    tag("type_param_bound", seq!(python_literal(":"), deferred(expression).into_dyn()))
}

pub fn type_param() -> impl CombinatorTrait {
    cached(tag("type_param", choice!(
        seq!(deferred(NAME).into_dyn(), opt(type_param_bound()), opt(deferred(type_param_default).into_dyn())),
        seq!(python_literal("*"), deferred(NAME).into_dyn(), opt(type_param_starred_default())),
        seq!(python_literal("**"), deferred(NAME).into_dyn(), opt(deferred(type_param_default).into_dyn()))
    )))
}

pub fn type_param_seq() -> impl CombinatorTrait {
    tag("type_param_seq", seq!(seprep1(type_param(), python_literal(",")), opt(python_literal(","))))
}

pub fn type_params() -> impl CombinatorTrait {
    tag("type_params", seq!(python_literal("["), type_param_seq(), python_literal("]")))
}

pub fn type_alias() -> impl CombinatorTrait {
    tag("type_alias", seq!(
        python_literal("type"),
        deferred(NAME).into_dyn(),
        opt(deferred(type_params).into_dyn()),
        python_literal("="),
        deferred(expression).into_dyn()
    ))
}

pub fn keyword_pattern() -> impl CombinatorTrait {
    tag("keyword_pattern", seq!(
        deferred(NAME).into_dyn(),
        python_literal("="),
        deferred(pattern).into_dyn()
    ))
}

pub fn keyword_patterns() -> impl CombinatorTrait {
    tag("keyword_patterns", seprep1(keyword_pattern(), python_literal(",")))
}

pub fn positional_patterns() -> impl CombinatorTrait {
    tag("positional_patterns", seprep1(deferred(pattern).into_dyn(), python_literal(",")))
}

pub fn class_pattern() -> impl CombinatorTrait {
    tag("class_pattern", seq!(
        deferred(name_or_attr).into_dyn(),
        python_literal("("),
        choice!(python_literal(")"), seq!(positional_patterns(), choice!(seq!(opt(python_literal(",")), python_literal(")")), seq!(python_literal(","), deferred(keyword_patterns).into_dyn(), opt(python_literal(",")), python_literal(")")))), seq!(deferred(keyword_patterns).into_dyn(), opt(python_literal(",")), python_literal(")")))
    ))
}

pub fn double_star_pattern() -> impl CombinatorTrait {
    tag("double_star_pattern", seq!(python_literal("**"), deferred(pattern_capture_target).into_dyn()))
}

pub fn key_value_pattern() -> impl CombinatorTrait {
    tag("key_value_pattern", seq!(
        choice!(literal_expr(), deferred(attr).into_dyn()),
        python_literal(":"),
        deferred(pattern).into_dyn()
    ))
}

pub fn items_pattern() -> impl CombinatorTrait {
    tag("items_pattern", seprep1(key_value_pattern(), python_literal(",")))
}

pub fn mapping_pattern() -> impl CombinatorTrait {
    tag("mapping_pattern", seq!(
        python_literal("{"),
        choice!(python_literal("}"), seq!(deferred(double_star_pattern).into_dyn(), opt(python_literal(",")), python_literal("}")), seq!(items_pattern(), choice!(seq!(python_literal(","), deferred(double_star_pattern).into_dyn(), opt(python_literal(",")), python_literal("}")), seq!(opt(python_literal(",")), python_literal("}")))))
    ))
}

pub fn star_pattern() -> impl CombinatorTrait {
    cached(tag("star_pattern", seq!(
        python_literal("*"),
        choice!(deferred(pattern_capture_target).into_dyn(), deferred(wildcard_pattern).into_dyn())
    )))
}

pub fn maybe_star_pattern() -> impl CombinatorTrait {
    tag("maybe_star_pattern", choice!(
        star_pattern(),
        deferred(pattern).into_dyn()
    ))
}

pub fn maybe_sequence_pattern() -> impl CombinatorTrait {
    tag("maybe_sequence_pattern", seq!(
        seprep1(deferred(maybe_star_pattern).into_dyn(), python_literal(",")),
        opt(python_literal(","))
    ))
}

pub fn open_sequence_pattern() -> impl CombinatorTrait {
    tag("open_sequence_pattern", seq!(
        deferred(maybe_star_pattern).into_dyn(),
        python_literal(","),
        opt(deferred(maybe_sequence_pattern).into_dyn())
    ))
}

pub fn sequence_pattern() -> impl CombinatorTrait {
    tag("sequence_pattern", choice!(
        seq!(python_literal("["), opt(deferred(maybe_sequence_pattern).into_dyn()), python_literal("]")),
        seq!(python_literal("("), opt(deferred(open_sequence_pattern).into_dyn()), python_literal(")"))
    ))
}

pub fn group_pattern() -> impl CombinatorTrait {
    tag("group_pattern", seq!(python_literal("("), deferred(pattern).into_dyn(), python_literal(")")))
}

pub fn name_or_attr() -> impl CombinatorTrait {
    tag("name_or_attr", seq!(
        deferred(NAME).into_dyn(),
        repeat0(seq!(python_literal("."), deferred(NAME).into_dyn()))
    ))
}

pub fn attr() -> impl CombinatorTrait {
    tag("attr", seq!(
        deferred(name_or_attr).into_dyn(),
        python_literal("."),
        deferred(NAME).into_dyn()
    ))
}

pub fn value_pattern() -> impl CombinatorTrait {
    tag("value_pattern", seq!(
        deferred(attr).into_dyn(),
        negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("=")))
    ))
}

pub fn wildcard_pattern() -> impl CombinatorTrait {
    tag("wildcard_pattern", python_literal("_"))
}

pub fn pattern_capture_target() -> impl CombinatorTrait {
    tag("pattern_capture_target", seq!(
        negative_lookahead(python_literal("_")),
        deferred(NAME).into_dyn(),
        negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("=")))
    ))
}

pub fn capture_pattern() -> impl CombinatorTrait {
    tag("capture_pattern", deferred(pattern_capture_target).into_dyn())
}

pub fn imaginary_number() -> impl CombinatorTrait {
    tag("imaginary_number", deferred(NUMBER).into_dyn())
}

pub fn real_number() -> impl CombinatorTrait {
    tag("real_number", deferred(NUMBER).into_dyn())
}

pub fn signed_real_number() -> impl CombinatorTrait {
    tag("signed_real_number", choice!(
        deferred(real_number).into_dyn(),
        seq!(python_literal("-"), deferred(real_number).into_dyn())
    ))
}

pub fn signed_number() -> impl CombinatorTrait {
    tag("signed_number", choice!(
        deferred(NUMBER).into_dyn(),
        seq!(python_literal("-"), deferred(NUMBER).into_dyn())
    ))
}

pub fn complex_number() -> impl CombinatorTrait {
    tag("complex_number", seq!(
        signed_real_number(),
        choice!(seq!(python_literal("+"), deferred(imaginary_number).into_dyn()), seq!(python_literal("-"), deferred(imaginary_number).into_dyn()))
    ))
}

pub fn literal_expr() -> impl CombinatorTrait {
    tag("literal_expr", choice!(
        seq!(deferred(signed_number).into_dyn(), negative_lookahead(choice!(python_literal("+"), python_literal("-")))),
        deferred(complex_number).into_dyn(),
        deferred(strings).into_dyn(),
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    ))
}

pub fn literal_pattern() -> impl CombinatorTrait {
    tag("literal_pattern", choice!(
        seq!(deferred(signed_number).into_dyn(), negative_lookahead(choice!(python_literal("+"), python_literal("-")))),
        deferred(complex_number).into_dyn(),
        deferred(strings).into_dyn(),
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    ))
}

pub fn closed_pattern() -> impl CombinatorTrait {
    cached(tag("closed_pattern", choice!(
        literal_pattern(),
        capture_pattern(),
        deferred(wildcard_pattern).into_dyn(),
        value_pattern(),
        group_pattern(),
        sequence_pattern(),
        mapping_pattern(),
        class_pattern()
    )))
}

pub fn or_pattern() -> impl CombinatorTrait {
    tag("or_pattern", seprep1(closed_pattern(), python_literal("|")))
}

pub fn as_pattern() -> impl CombinatorTrait {
    tag("as_pattern", seq!(
        deferred(or_pattern).into_dyn(),
        python_literal("as"),
        deferred(pattern_capture_target).into_dyn()
    ))
}

pub fn pattern() -> impl CombinatorTrait {
    tag("pattern", choice!(
        as_pattern(),
        deferred(or_pattern).into_dyn()
    ))
}

pub fn patterns() -> impl CombinatorTrait {
    tag("patterns", choice!(
        deferred(open_sequence_pattern).into_dyn(),
        deferred(pattern).into_dyn()
    ))
}

pub fn guard() -> impl CombinatorTrait {
    tag("guard", seq!(python_literal("if"), deferred(named_expression).into_dyn()))
}

pub fn case_block() -> impl CombinatorTrait {
    tag("case_block", seq!(
        python_literal("case"),
        patterns(),
        opt(guard()),
        python_literal(":"),
        deferred(block).into_dyn()
    ))
}

pub fn subject_expr() -> impl CombinatorTrait {
    tag("subject_expr", choice!(
        seq!(deferred(star_named_expression).into_dyn(), python_literal(","), opt(deferred(star_named_expressions).into_dyn())),
        deferred(named_expression).into_dyn()
    ))
}

pub fn match_stmt() -> impl CombinatorTrait {
    tag("match_stmt", seq!(
        python_literal("match"),
        subject_expr(),
        python_literal(":"),
        deferred(NEWLINE).into_dyn(),
        deferred(INDENT).into_dyn(),
        repeat1(case_block()),
        deferred(DEDENT).into_dyn()
    ))
}

pub fn finally_block() -> impl CombinatorTrait {
    tag("finally_block", seq!(python_literal("finally"), python_literal(":"), deferred(block).into_dyn()))
}

pub fn except_star_block() -> impl CombinatorTrait {
    tag("except_star_block", seq!(
        python_literal("except"),
        python_literal("*"),
        deferred(expression).into_dyn(),
        opt(seq!(python_literal("as"), deferred(NAME).into_dyn())),
        python_literal(":"),
        deferred(block).into_dyn()
    ))
}

pub fn except_block() -> impl CombinatorTrait {
    tag("except_block", seq!(
        python_literal("except"),
        choice!(seq!(deferred(expression).into_dyn(), opt(seq!(python_literal("as"), deferred(NAME).into_dyn())), python_literal(":"), deferred(block).into_dyn()), seq!(python_literal(":"), deferred(block).into_dyn()))
    ))
}

pub fn try_stmt() -> impl CombinatorTrait {
    tag("try_stmt", seq!(
        python_literal("try"),
        python_literal(":"),
        deferred(block).into_dyn(),
        choice!(deferred(finally_block).into_dyn(), seq!(repeat1(except_block()), opt(deferred(else_block).into_dyn()), opt(deferred(finally_block).into_dyn())), seq!(repeat1(except_star_block()), opt(deferred(else_block).into_dyn()), opt(deferred(finally_block).into_dyn())))
    ))
}

pub fn with_item() -> impl CombinatorTrait {
    tag("with_item", seq!(
        deferred(expression).into_dyn(),
        opt(seq!(python_literal("as"), deferred(star_target).into_dyn(), lookahead(choice!(python_literal(","), python_literal(")"), python_literal(":")))))
    ))
}

pub fn with_stmt() -> impl CombinatorTrait {
    tag("with_stmt", choice!(
        seq!(python_literal("with"), choice!(seq!(python_literal("("), seprep1(deferred(with_item).into_dyn(), python_literal(",")), opt(python_literal(",")), python_literal(")"), python_literal(":"), opt(deferred(TYPE_COMMENT).into_dyn()), deferred(block).into_dyn()), seq!(seprep1(deferred(with_item).into_dyn(), python_literal(",")), python_literal(":"), opt(deferred(TYPE_COMMENT).into_dyn()), deferred(block).into_dyn()))),
        seq!(python_literal("async"), python_literal("with"), choice!(seq!(python_literal("("), seprep1(deferred(with_item).into_dyn(), python_literal(",")), opt(python_literal(",")), python_literal(")"), python_literal(":"), deferred(block).into_dyn()), seq!(seprep1(deferred(with_item).into_dyn(), python_literal(",")), python_literal(":"), opt(deferred(TYPE_COMMENT).into_dyn()), deferred(block).into_dyn())))
    ))
}

pub fn for_stmt() -> impl CombinatorTrait {
    tag("for_stmt", choice!(
        seq!(python_literal("for"), deferred(star_targets).into_dyn(), python_literal("in"), deferred(star_expressions).into_dyn(), python_literal(":"), opt(deferred(TYPE_COMMENT).into_dyn()), deferred(block).into_dyn(), opt(deferred(else_block).into_dyn())),
        seq!(python_literal("async"), python_literal("for"), deferred(star_targets).into_dyn(), python_literal("in"), deferred(star_expressions).into_dyn(), python_literal(":"), opt(deferred(TYPE_COMMENT).into_dyn()), deferred(block).into_dyn(), opt(deferred(else_block).into_dyn()))
    ))
}

pub fn while_stmt() -> impl CombinatorTrait {
    tag("while_stmt", seq!(
        python_literal("while"),
        deferred(named_expression).into_dyn(),
        python_literal(":"),
        deferred(block).into_dyn(),
        opt(deferred(else_block).into_dyn())
    ))
}

pub fn else_block() -> impl CombinatorTrait {
    tag("else_block", seq!(python_literal("else"), python_literal(":"), deferred(block).into_dyn()))
}

pub fn elif_stmt() -> impl CombinatorTrait {
    tag("elif_stmt", seq!(
        python_literal("elif"),
        deferred(named_expression).into_dyn(),
        python_literal(":"),
        deferred(block).into_dyn(),
        choice!(deferred(elif_stmt).into_dyn(), opt(deferred(else_block).into_dyn()))
    ))
}

pub fn if_stmt() -> impl CombinatorTrait {
    tag("if_stmt", seq!(
        python_literal("if"),
        deferred(named_expression).into_dyn(),
        python_literal(":"),
        deferred(block).into_dyn(),
        choice!(deferred(elif_stmt).into_dyn(), opt(deferred(else_block).into_dyn()))
    ))
}

pub fn default() -> impl CombinatorTrait {
    tag("default", seq!(python_literal("="), deferred(expression).into_dyn()))
}

pub fn star_annotation() -> impl CombinatorTrait {
    tag("star_annotation", seq!(python_literal(":"), deferred(star_expression).into_dyn()))
}

pub fn annotation() -> impl CombinatorTrait {
    tag("annotation", seq!(python_literal(":"), deferred(expression).into_dyn()))
}

pub fn param_star_annotation() -> impl CombinatorTrait {
    tag("param_star_annotation", seq!(deferred(NAME).into_dyn(), star_annotation()))
}

pub fn param() -> impl CombinatorTrait {
    tag("param", seq!(deferred(NAME).into_dyn(), opt(annotation())))
}

pub fn param_maybe_default() -> impl CombinatorTrait {
    tag("param_maybe_default", seq!(
        deferred(param).into_dyn(),
        opt(deferred(default).into_dyn()),
        choice!(seq!(python_literal(","), opt(deferred(TYPE_COMMENT).into_dyn())), seq!(opt(deferred(TYPE_COMMENT).into_dyn()), lookahead(python_literal(")"))))
    ))
}

pub fn param_with_default() -> impl CombinatorTrait {
    tag("param_with_default", seq!(
        deferred(param).into_dyn(),
        deferred(default).into_dyn(),
        choice!(seq!(python_literal(","), opt(deferred(TYPE_COMMENT).into_dyn())), seq!(opt(deferred(TYPE_COMMENT).into_dyn()), lookahead(python_literal(")"))))
    ))
}

pub fn param_no_default_star_annotation() -> impl CombinatorTrait {
    tag("param_no_default_star_annotation", seq!(
        param_star_annotation(),
        choice!(seq!(python_literal(","), opt(deferred(TYPE_COMMENT).into_dyn())), seq!(opt(deferred(TYPE_COMMENT).into_dyn()), lookahead(python_literal(")"))))
    ))
}

pub fn param_no_default() -> impl CombinatorTrait {
    tag("param_no_default", seq!(
        deferred(param).into_dyn(),
        choice!(seq!(python_literal(","), opt(deferred(TYPE_COMMENT).into_dyn())), seq!(opt(deferred(TYPE_COMMENT).into_dyn()), lookahead(python_literal(")"))))
    ))
}

pub fn kwds() -> impl CombinatorTrait {
    tag("kwds", seq!(python_literal("**"), deferred(param_no_default).into_dyn()))
}

pub fn star_etc() -> impl CombinatorTrait {
    tag("star_etc", choice!(
        seq!(python_literal("*"), choice!(seq!(deferred(param_no_default).into_dyn(), repeat0(deferred(param_maybe_default).into_dyn()), opt(deferred(kwds).into_dyn())), seq!(param_no_default_star_annotation(), repeat0(deferred(param_maybe_default).into_dyn()), opt(deferred(kwds).into_dyn())), seq!(python_literal(","), repeat1(deferred(param_maybe_default).into_dyn()), opt(deferred(kwds).into_dyn())))),
        deferred(kwds).into_dyn()
    ))
}

pub fn slash_with_default() -> impl CombinatorTrait {
    tag("slash_with_default", seq!(
        repeat0(deferred(param_no_default).into_dyn()),
        repeat1(deferred(param_with_default).into_dyn()),
        python_literal("/"),
        choice!(python_literal(","), lookahead(python_literal(")")))
    ))
}

pub fn slash_no_default() -> impl CombinatorTrait {
    tag("slash_no_default", seq!(
        repeat1(deferred(param_no_default).into_dyn()),
        python_literal("/"),
        choice!(python_literal(","), lookahead(python_literal(")")))
    ))
}

pub fn parameters() -> impl CombinatorTrait {
    tag("parameters", choice!(
        seq!(slash_no_default(), repeat0(deferred(param_no_default).into_dyn()), repeat0(deferred(param_with_default).into_dyn()), opt(deferred(star_etc).into_dyn())),
        seq!(slash_with_default(), repeat0(deferred(param_with_default).into_dyn()), opt(deferred(star_etc).into_dyn())),
        seq!(repeat1(deferred(param_no_default).into_dyn()), repeat0(deferred(param_with_default).into_dyn()), opt(deferred(star_etc).into_dyn())),
        seq!(repeat1(deferred(param_with_default).into_dyn()), opt(deferred(star_etc).into_dyn())),
        deferred(star_etc).into_dyn()
    ))
}

pub fn params() -> impl CombinatorTrait {
    tag("params", parameters())
}

pub fn function_def_raw() -> impl CombinatorTrait {
    tag("function_def_raw", choice!(
        seq!(python_literal("def"), deferred(NAME).into_dyn(), opt(deferred(type_params).into_dyn()), python_literal("("), opt(deferred(params).into_dyn()), python_literal(")"), opt(seq!(python_literal("->"), deferred(expression).into_dyn())), python_literal(":"), opt(deferred(func_type_comment).into_dyn()), deferred(block).into_dyn()),
        seq!(python_literal("async"), python_literal("def"), deferred(NAME).into_dyn(), opt(deferred(type_params).into_dyn()), python_literal("("), opt(deferred(params).into_dyn()), python_literal(")"), opt(seq!(python_literal("->"), deferred(expression).into_dyn())), python_literal(":"), opt(deferred(func_type_comment).into_dyn()), deferred(block).into_dyn())
    ))
}

pub fn function_def() -> impl CombinatorTrait {
    tag("function_def", choice!(
        seq!(deferred(decorators).into_dyn(), deferred(function_def_raw).into_dyn()),
        deferred(function_def_raw).into_dyn()
    ))
}

pub fn class_def_raw() -> impl CombinatorTrait {
    tag("class_def_raw", seq!(
        python_literal("class"),
        deferred(NAME).into_dyn(),
        opt(deferred(type_params).into_dyn()),
        opt(seq!(python_literal("("), opt(deferred(arguments).into_dyn()), python_literal(")"))),
        python_literal(":"),
        deferred(block).into_dyn()
    ))
}

pub fn class_def() -> impl CombinatorTrait {
    tag("class_def", choice!(
        seq!(deferred(decorators).into_dyn(), deferred(class_def_raw).into_dyn()),
        deferred(class_def_raw).into_dyn()
    ))
}

pub fn decorators() -> impl CombinatorTrait {
    tag("decorators", repeat1(seq!(python_literal("@"), deferred(named_expression).into_dyn(), deferred(NEWLINE).into_dyn())))
}

pub fn block() -> impl CombinatorTrait {
    cached(tag("block", choice!(
        seq!(deferred(NEWLINE).into_dyn(), deferred(INDENT).into_dyn(), deferred(statements).into_dyn(), deferred(DEDENT).into_dyn()),
        deferred(simple_stmts).into_dyn()
    )))
}

pub fn dotted_name() -> impl CombinatorTrait {
    tag("dotted_name", seq!(
        deferred(NAME).into_dyn(),
        repeat0(seq!(python_literal("."), deferred(NAME).into_dyn()))
    ))
}

pub fn dotted_as_name() -> impl CombinatorTrait {
    tag("dotted_as_name", seq!(
        deferred(dotted_name).into_dyn(),
        opt(seq!(python_literal("as"), deferred(NAME).into_dyn()))
    ))
}

pub fn dotted_as_names() -> impl CombinatorTrait {
    tag("dotted_as_names", seprep1(dotted_as_name(), python_literal(",")))
}

pub fn import_from_as_name() -> impl CombinatorTrait {
    tag("import_from_as_name", seq!(
        deferred(NAME).into_dyn(),
        opt(seq!(python_literal("as"), deferred(NAME).into_dyn()))
    ))
}

pub fn import_from_as_names() -> impl CombinatorTrait {
    tag("import_from_as_names", seprep1(import_from_as_name(), python_literal(",")))
}

pub fn import_from_targets() -> impl CombinatorTrait {
    tag("import_from_targets", choice!(
        seq!(python_literal("("), deferred(import_from_as_names).into_dyn(), opt(python_literal(",")), python_literal(")")),
        seq!(deferred(import_from_as_names).into_dyn(), negative_lookahead(python_literal(","))),
        python_literal("*")
    ))
}

pub fn import_from() -> impl CombinatorTrait {
    tag("import_from", seq!(
        python_literal("from"),
        choice!(seq!(repeat0(choice!(python_literal("."), python_literal("..."))), deferred(dotted_name).into_dyn(), python_literal("import"), deferred(import_from_targets).into_dyn()), seq!(repeat1(choice!(python_literal("."), python_literal("..."))), python_literal("import"), deferred(import_from_targets).into_dyn()))
    ))
}

pub fn import_name() -> impl CombinatorTrait {
    tag("import_name", seq!(python_literal("import"), dotted_as_names()))
}

pub fn import_stmt() -> impl CombinatorTrait {
    tag("import_stmt", choice!(
        import_name(),
        import_from()
    ))
}

pub fn assert_stmt() -> impl CombinatorTrait {
    tag("assert_stmt", seq!(
        python_literal("assert"),
        deferred(expression).into_dyn(),
        opt(seq!(python_literal(","), deferred(expression).into_dyn()))
    ))
}

pub fn yield_stmt() -> impl CombinatorTrait {
    tag("yield_stmt", deferred(yield_expr).into_dyn())
}

pub fn del_stmt() -> impl CombinatorTrait {
    tag("del_stmt", seq!(
        python_literal("del"),
        deferred(del_targets).into_dyn(),
        lookahead(choice!(python_literal(";"), deferred(NEWLINE).into_dyn()))
    ))
}

pub fn nonlocal_stmt() -> impl CombinatorTrait {
    tag("nonlocal_stmt", seq!(
        python_literal("nonlocal"),
        seprep1(deferred(NAME).into_dyn(), python_literal(","))
    ))
}

pub fn global_stmt() -> impl CombinatorTrait {
    tag("global_stmt", seq!(
        python_literal("global"),
        seprep1(deferred(NAME).into_dyn(), python_literal(","))
    ))
}

pub fn raise_stmt() -> impl CombinatorTrait {
    tag("raise_stmt", seq!(
        python_literal("raise"),
        opt(seq!(deferred(expression).into_dyn(), opt(seq!(python_literal("from"), deferred(expression).into_dyn()))))
    ))
}

pub fn return_stmt() -> impl CombinatorTrait {
    tag("return_stmt", seq!(python_literal("return"), opt(deferred(star_expressions).into_dyn())))
}

pub fn augassign() -> impl CombinatorTrait {
    tag("augassign", choice!(
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
    ))
}

pub fn annotated_rhs() -> impl CombinatorTrait {
    tag("annotated_rhs", choice!(
        deferred(yield_expr).into_dyn(),
        deferred(star_expressions).into_dyn()
    ))
}

pub fn assignment() -> impl CombinatorTrait {
    tag("assignment", choice!(
        seq!(deferred(NAME).into_dyn(), python_literal(":"), deferred(expression).into_dyn(), opt(seq!(python_literal("="), deferred(annotated_rhs).into_dyn()))),
        seq!(choice!(seq!(python_literal("("), deferred(single_target).into_dyn(), python_literal(")")), deferred(single_subscript_attribute_target).into_dyn()), python_literal(":"), deferred(expression).into_dyn(), opt(seq!(python_literal("="), deferred(annotated_rhs).into_dyn()))),
        seq!(repeat1(seq!(deferred(star_targets).into_dyn(), python_literal("="))), choice!(deferred(yield_expr).into_dyn(), deferred(star_expressions).into_dyn()), negative_lookahead(python_literal("=")), opt(deferred(TYPE_COMMENT).into_dyn())),
        seq!(deferred(single_target).into_dyn(), augassign(), choice!(deferred(yield_expr).into_dyn(), deferred(star_expressions).into_dyn()))
    ))
}

pub fn compound_stmt() -> impl CombinatorTrait {
    tag("compound_stmt", choice!(
        seq!(lookahead(choice!(python_literal("def"), python_literal("@"), python_literal("async"))), function_def()),
        seq!(lookahead(python_literal("if")), if_stmt()),
        seq!(lookahead(choice!(python_literal("class"), python_literal("@"))), class_def()),
        seq!(lookahead(choice!(python_literal("with"), python_literal("async"))), with_stmt()),
        seq!(lookahead(choice!(python_literal("for"), python_literal("async"))), for_stmt()),
        seq!(lookahead(python_literal("try")), try_stmt()),
        seq!(lookahead(python_literal("while")), while_stmt()),
        match_stmt()
    ))
}

pub fn simple_stmt() -> impl CombinatorTrait {
    cached(tag("simple_stmt", choice!(
        assignment(),
        seq!(lookahead(python_literal("type")), type_alias()),
        deferred(star_expressions).into_dyn(),
        seq!(lookahead(python_literal("return")), return_stmt()),
        seq!(lookahead(choice!(python_literal("import"), python_literal("from"))), import_stmt()),
        seq!(lookahead(python_literal("raise")), raise_stmt()),
        python_literal("pass"),
        seq!(lookahead(python_literal("del")), del_stmt()),
        seq!(lookahead(python_literal("yield")), yield_stmt()),
        seq!(lookahead(python_literal("assert")), assert_stmt()),
        python_literal("break"),
        python_literal("continue"),
        seq!(lookahead(python_literal("global")), global_stmt()),
        seq!(lookahead(python_literal("nonlocal")), nonlocal_stmt())
    )))
}

pub fn simple_stmts() -> impl CombinatorTrait {
    tag("simple_stmts", choice!(
        seq!(deferred(simple_stmt).into_dyn(), negative_lookahead(python_literal(";")), deferred(NEWLINE).into_dyn()),
        seq!(seprep1(deferred(simple_stmt).into_dyn(), python_literal(";")), opt(python_literal(";")), deferred(NEWLINE).into_dyn())
    ))
}

pub fn statement_newline() -> impl CombinatorTrait {
    tag("statement_newline", choice!(
        seq!(deferred(compound_stmt).into_dyn(), deferred(NEWLINE).into_dyn()),
        deferred(simple_stmts).into_dyn(),
        deferred(NEWLINE).into_dyn(),
        deferred(ENDMARKER).into_dyn()
    ))
}

pub fn statement() -> impl CombinatorTrait {
    tag("statement", choice!(
        deferred(compound_stmt).into_dyn(),
        deferred(simple_stmts).into_dyn()
    ))
}

pub fn statements() -> impl CombinatorTrait {
    tag("statements", repeat1(statement()))
}

pub fn func_type() -> impl CombinatorTrait {
    tag("func_type", seq!(
        python_literal("("),
        opt(type_expressions()),
        python_literal(")"),
        python_literal("->"),
        deferred(expression).into_dyn(),
        repeat0(deferred(NEWLINE).into_dyn()),
        deferred(ENDMARKER).into_dyn()
    ))
}

pub fn eval() -> impl CombinatorTrait {
    tag("eval", seq!(
        expressions(),
        repeat0(deferred(NEWLINE).into_dyn()),
        deferred(ENDMARKER).into_dyn()
    ))
}

pub fn interactive() -> impl CombinatorTrait {
    tag("interactive", statement_newline())
}

pub fn file() -> impl CombinatorTrait {
    tag("file", seq!(opt(deferred(statements).into_dyn()), deferred(ENDMARKER).into_dyn()))
}


pub fn python_file() -> impl CombinatorTrait {

    cache_context(tag("main", seq!(opt(deferred(NEWLINE).into_dyn()), deferred(file).into_dyn()))).compile()
}
