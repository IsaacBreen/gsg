use std::rc::Rc;
use crate::{cache_context, cached, symbol, Symbol, mutate_right_data, RightData, Choice, deferred, Combinator, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, Repeat1, Seq, tag, lookahead, negative_lookahead};
use crate::seq;
use crate::{opt_greedy as opt, choice_greedy as choice, seprep0_greedy as seprep0, seprep1_greedy as seprep1, repeat0_greedy as repeat0, repeat1_greedy as repeat1};

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

pub fn python_literal(s: &str)-> impl CombinatorTrait {
    let increment_scope_count = |right_data: &mut RightData| { Rc::make_mut(&mut right_data.right_data_inner).fields1.scope_count += 1; true };
    let decrement_scope_count = |right_data: &mut RightData| { Rc::make_mut(&mut right_data.right_data_inner).fields1.scope_count -= 1; true };

    match s {
        "(" | "[" | "{" => seq!(eat_string(s), mutate_right_data(increment_scope_count), forbid_follows_clear(), opt(&WS)).into(),
        ")" | "]" | "}" => seq!(eat_string(s), mutate_right_data(decrement_scope_count), forbid_follows_clear(), opt(&WS)).into(),
        _ => seq!(eat_string(s), forbid_follows_clear(), opt(&WS)).into(),
    }
}
pub fn WS()-> impl CombinatorTrait { cached(tag("WS", crate::profile("WS", seq!(forbid_follows_check_not(Forbidden::WS as usize), token::WS().compile(), forbid_follows(&[Forbidden::INDENT as usize, Forbidden::DEDENT as usize]))))).into() }
pub fn NAME()-> impl CombinatorTrait { cached(seq!(tag("NAME", crate::profile("NAME", seq!(forbid_follows_check_not(Forbidden::NAME as usize), token::NAME().compile(), forbid_follows(&[Forbidden::NAME as usize, Forbidden::NUMBER as usize])))), opt(&WS))).into() }
pub fn TYPE_COMMENT()-> impl CombinatorTrait { cached(seq!(tag("TYPE_COMMENT", crate::profile("TYPE_COMMENT", seq!(token::TYPE_COMMENT().compile(), forbid_follows_clear()))), opt(&WS))).into() }
pub fn FSTRING_START()-> impl CombinatorTrait { cached(tag("FSTRING_START", crate::profile("FSTRING_START", seq!(token::FSTRING_START().compile(), forbid_follows(&[Forbidden::WS as usize, Forbidden::NEWLINE as usize]))))).into() }
pub fn FSTRING_MIDDLE()-> impl CombinatorTrait { cached(tag("FSTRING_MIDDLE", crate::profile("FSTRING_MIDDLE", seq!(forbid_follows_check_not(Forbidden::FSTRING_MIDDLE as usize), token::FSTRING_MIDDLE().compile(), forbid_follows(&[Forbidden::FSTRING_MIDDLE as usize, Forbidden::WS as usize]))))).into() }
pub fn FSTRING_END()-> impl CombinatorTrait { cached(seq!(tag("FSTRING_END", crate::profile("FSTRING_END", seq!(token::FSTRING_END().compile(), forbid_follows_clear()))), opt(&WS))).into() }
pub fn NUMBER()-> impl CombinatorTrait { cached(seq!(tag("NUMBER", crate::profile("NUMBER", seq!(forbid_follows_check_not(Forbidden::NUMBER as usize), token::NUMBER().compile(), forbid_follows(&[Forbidden::NUMBER as usize])))), opt(&WS))).into() }
pub fn STRING()-> impl CombinatorTrait { cached(seq!(tag("STRING", crate::profile("STRING", seq!(token::STRING().compile(), forbid_follows_clear()))), opt(&WS))).into() }
pub fn NEWLINE()-> impl CombinatorTrait { cached(tag("NEWLINE", crate::profile("NEWLINE", seq!(forbid_follows_check_not(Forbidden::NEWLINE as usize), token::NEWLINE().compile(), forbid_follows(&[Forbidden::WS as usize]))))).into() }
pub fn INDENT()-> impl CombinatorTrait { cached(tag("INDENT", crate::profile("INDENT", seq!(forbid_follows_check_not(Forbidden::INDENT as usize), token::INDENT().compile(), forbid_follows(&[Forbidden::WS as usize]))))).into() }
pub fn DEDENT()-> impl CombinatorTrait { cached(tag("DEDENT", crate::profile("DEDENT", seq!(forbid_follows_check_not(Forbidden::DEDENT as usize), token::DEDENT().compile(), forbid_follows(&[Forbidden::WS as usize]))))).into() }
pub fn ENDMARKER()-> impl CombinatorTrait { cached(seq!(tag("ENDMARKER", crate::profile("ENDMARKER", seq!(token::ENDMARKER().compile(), forbid_follows_clear()))), opt(&WS))).into() }

pub fn expression_without_invalid()-> impl CombinatorTrait {
    tag("expression_without_invalid", choice!(
        seq!(&conjunction, repeat0(seq!(python_literal("or"), &conjunction)), opt(seq!(python_literal("if"), &conjunction, repeat0(seq!(python_literal("or"), &conjunction)), python_literal("else"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)))),
        seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))
    )).into()
}

pub fn func_type_comment()-> impl CombinatorTrait {
    tag("func_type_comment", choice!(
        seq!(&NEWLINE, &TYPE_COMMENT, lookahead(seq!(&NEWLINE, &INDENT))),
        &TYPE_COMMENT
    )).into()
}

pub fn type_expressions()-> impl CombinatorTrait {
    tag("type_expressions", choice!(
        seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), repeat0(seq!(python_literal(","), &expression)), opt(seq!(python_literal(","), choice!(seq!(python_literal("*"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), opt(seq!(python_literal(","), python_literal("**"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)))), seq!(python_literal("**"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)))))),
        seq!(python_literal("*"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), opt(seq!(python_literal(","), python_literal("**"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)))),
        seq!(python_literal("**"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))
    )).into()
}

pub fn del_t_atom()-> impl CombinatorTrait {
    tag("del_t_atom", choice!(
        &NAME,
        seq!(python_literal("("), choice!(seq!(choice!(seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead), repeat0(choice!(seq!(python_literal("."), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))), python_literal("]"), lookahead(&t_lookahead)), seq!(python_literal("("), choice!(seq!(choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), repeat1(&for_if_clause), python_literal(")"), lookahead(&t_lookahead)), seq!(opt(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")")))), python_literal(")"), lookahead(&t_lookahead)))))), choice!(seq!(python_literal("."), &NAME, negative_lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead)))), &del_t_atom), python_literal(")")), seq!(opt(seq!(seprep1(&del_target, python_literal(",")), opt(python_literal(",")))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(seprep1(&del_target, python_literal(",")), opt(python_literal(",")))), python_literal("]"))
    )).into()
}

pub fn del_target()-> impl CombinatorTrait {
    cached(tag("del_target", choice!(
        seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead), repeat0(choice!(seq!(python_literal("."), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))), python_literal("]"), lookahead(&t_lookahead)), seq!(python_literal("("), choice!(seq!(choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), repeat1(&for_if_clause), python_literal(")"), lookahead(&t_lookahead)), seq!(opt(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")")))), python_literal(")"), lookahead(&t_lookahead)))))), choice!(seq!(python_literal("."), &NAME, negative_lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead)))),
        &del_t_atom
    ))).into()
}

pub fn del_targets()-> impl CombinatorTrait {
    tag("del_targets", seq!(seprep1(&del_target, python_literal(",")), opt(python_literal(",")))).into()
}

pub fn t_lookahead()-> impl CombinatorTrait {
    tag("t_lookahead", choice!(
        python_literal("("),
        python_literal("["),
        python_literal(".")
    )).into()
}

pub fn t_primary()-> impl CombinatorTrait {
    tag("t_primary", seq!(choice!(&NAME, python_literal("True"), python_literal("False"), python_literal("None"), seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings), &NUMBER, seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)), seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)), seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)), python_literal("...")), lookahead(&t_lookahead), repeat0(choice!(seq!(python_literal("."), &NAME, lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))), python_literal("]"), lookahead(&t_lookahead)), seq!(python_literal("("), choice!(seq!(choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), repeat1(&for_if_clause), python_literal(")"), lookahead(&t_lookahead)), seq!(opt(seq!(&args, opt(python_literal(",")), lookahead(python_literal(")")))), python_literal(")"), lookahead(&t_lookahead)))))))).into()
}

pub fn single_subscript_attribute_target()-> impl CombinatorTrait {
    tag("single_subscript_attribute_target", seq!(&t_primary, choice!(seq!(python_literal("."), &NAME, negative_lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead))))).into()
}

pub fn single_target()-> impl CombinatorTrait {
    tag("single_target", choice!(
        &single_subscript_attribute_target,
        &NAME,
        seq!(python_literal("("), &single_target, python_literal(")"))
    )).into()
}

pub fn star_atom()-> impl CombinatorTrait {
    tag("star_atom", choice!(
        &NAME,
        seq!(python_literal("("), choice!(seq!(choice!(seq!(&t_primary, choice!(seq!(python_literal("."), &NAME, negative_lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead)))), &star_atom), python_literal(")")), seq!(opt(seq!(&star_target, choice!(seq!(repeat1(seq!(python_literal(","), &star_target)), opt(python_literal(","))), python_literal(",")))), python_literal(")")))),
        seq!(python_literal("["), opt(seq!(seprep1(&star_target, python_literal(",")), opt(python_literal(",")))), python_literal("]"))
    )).into()
}

pub fn target_with_star_atom()-> impl CombinatorTrait {
    cached(tag("target_with_star_atom", choice!(
        seq!(&t_primary, choice!(seq!(python_literal("."), &NAME, negative_lookahead(&t_lookahead)), seq!(python_literal("["), choice!(seq!(&slice, negative_lookahead(python_literal(","))), seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))), python_literal("]"), negative_lookahead(&t_lookahead)))),
        &star_atom
    ))).into()
}

pub fn star_target()-> impl CombinatorTrait {
    cached(tag("star_target", choice!(
        seq!(python_literal("*"), negative_lookahead(python_literal("*")), &star_target),
        &target_with_star_atom
    ))).into()
}

pub fn star_targets_tuple_seq()-> impl CombinatorTrait {
    tag("star_targets_tuple_seq", seq!(&star_target, choice!(seq!(repeat1(seq!(python_literal(","), &star_target)), opt(python_literal(","))), python_literal(",")))).into()
}

pub fn star_targets_list_seq()-> impl CombinatorTrait {
    tag("star_targets_list_seq", seq!(seprep1(&star_target, python_literal(",")), opt(python_literal(",")))).into()
}

pub fn star_targets()-> impl CombinatorTrait {
    tag("star_targets", seq!(&star_target, choice!(negative_lookahead(python_literal(",")), seq!(repeat0(seq!(python_literal(","), &star_target)), opt(python_literal(",")))))).into()
}

pub fn kwarg_or_double_starred()-> impl CombinatorTrait {
    tag("kwarg_or_double_starred", choice!(
        seq!(&NAME, python_literal("="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)),
        seq!(python_literal("**"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))
    )).into()
}

pub fn kwarg_or_starred()-> impl CombinatorTrait {
    tag("kwarg_or_starred", choice!(
        seq!(&NAME, python_literal("="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)),
        seq!(python_literal("*"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))
    )).into()
}

pub fn starred_expression()-> impl CombinatorTrait {
    tag("starred_expression", seq!(python_literal("*"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))).into()
}

pub fn kwargs()-> impl CombinatorTrait {
    tag("kwargs", choice!(
        seq!(seprep1(&kwarg_or_starred, python_literal(",")), opt(seq!(python_literal(","), seprep1(&kwarg_or_double_starred, python_literal(","))))),
        seprep1(&kwarg_or_double_starred, python_literal(","))
    )).into()
}

pub fn args()-> impl CombinatorTrait {
    tag("args", choice!(
        seq!(seprep1(choice!(&starred_expression, seq!(choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), negative_lookahead(python_literal("=")))), python_literal(",")), opt(seq!(python_literal(","), &kwargs))),
        &kwargs
    )).into()
}

pub fn arguments()-> impl CombinatorTrait {
    cached(tag("arguments", seq!(&args, opt(python_literal(",")), lookahead(python_literal(")"))))).into()
}

pub fn dictcomp()-> impl CombinatorTrait {
    tag("dictcomp", seq!(
        python_literal("{"),
         choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef),
         python_literal(":"),
         choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef),
         repeat1(&for_if_clause),
         python_literal("}")
    )).into()
}

pub fn genexp()-> impl CombinatorTrait {
    tag("genexp", seq!(python_literal("("), choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), repeat1(&for_if_clause), python_literal(")"))).into()
}

pub fn setcomp()-> impl CombinatorTrait {
    tag("setcomp", seq!(python_literal("{"), choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), repeat1(&for_if_clause), python_literal("}"))).into()
}

pub fn listcomp()-> impl CombinatorTrait {
    tag("listcomp", seq!(python_literal("["), choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":=")))), repeat1(&for_if_clause), python_literal("]"))).into()
}

pub fn for_if_clause()-> impl CombinatorTrait {
    tag("for_if_clause", choice!(
        seq!(python_literal("async"), python_literal("for"), &star_targets, python_literal("in"), &conjunction, repeat0(seq!(python_literal("or"), &conjunction)), repeat0(seq!(python_literal("if"), &conjunction, repeat0(seq!(python_literal("or"), &conjunction))))),
        seq!(python_literal("for"), &star_targets, python_literal("in"), &conjunction, repeat0(seq!(python_literal("or"), &conjunction)), repeat0(seq!(python_literal("if"), &conjunction, repeat0(seq!(python_literal("or"), &conjunction)))))
    )).into()
}

pub fn for_if_clauses()-> impl CombinatorTrait {
    tag("for_if_clauses", repeat1(&for_if_clause)).into()
}

pub fn kvpair()-> impl CombinatorTrait {
    tag("kvpair", seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), python_literal(":"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))).into()
}

pub fn double_starred_kvpair()-> impl CombinatorTrait {
    tag("double_starred_kvpair", choice!(
        seq!(python_literal("**"), &bitwise_xor, repeat0(seq!(python_literal("|"), &bitwise_xor))),
        &kvpair
    )).into()
}

pub fn double_starred_kvpairs()-> impl CombinatorTrait {
    tag("double_starred_kvpairs", seq!(seprep1(&double_starred_kvpair, python_literal(",")), opt(python_literal(",")))).into()
}

pub fn dict()-> impl CombinatorTrait {
    tag("dict", seq!(python_literal("{"), opt(&double_starred_kvpairs), python_literal("}"))).into()
}

pub fn set()-> impl CombinatorTrait {
    tag("set", seq!(python_literal("{"), seprep1(&star_named_expression, python_literal(",")), opt(python_literal(",")), python_literal("}"))).into()
}

pub fn tuple()-> impl CombinatorTrait {
    tag("tuple", seq!(python_literal("("), opt(seq!(choice!(seq!(python_literal("*"), &bitwise_or), &named_expression), python_literal(","), opt(seq!(seprep1(&star_named_expression, python_literal(",")), opt(python_literal(",")))))), python_literal(")"))).into()
}

pub fn list()-> impl CombinatorTrait {
    tag("list", seq!(python_literal("["), opt(seq!(seprep1(&star_named_expression, python_literal(",")), opt(python_literal(",")))), python_literal("]"))).into()
}

pub fn strings()-> impl CombinatorTrait {
    cached(tag("strings", repeat1(choice!(seq!(&FSTRING_START, opt(seq!(choice!(&fstring_replacement_field, &FSTRING_MIDDLE), repeat0(&fstring_middle))), &FSTRING_END), &STRING)))).into()
}

pub fn string()-> impl CombinatorTrait {
    tag("string", &STRING).into()
}

pub fn fstring()-> impl CombinatorTrait {
    tag("fstring", seq!(&FSTRING_START, opt(seq!(choice!(&fstring_replacement_field, &FSTRING_MIDDLE), repeat0(&fstring_middle))), &FSTRING_END)).into()
}

pub fn fstring_format_spec()-> impl CombinatorTrait {
    tag("fstring_format_spec", choice!(
        &FSTRING_MIDDLE,
        seq!(python_literal("{"), choice!(&yield_expr, &star_expressions), opt(python_literal("=")), opt(&fstring_conversion), opt(&fstring_full_format_spec), python_literal("}"))
    )).into()
}

pub fn fstring_full_format_spec()-> impl CombinatorTrait {
    tag("fstring_full_format_spec", seq!(python_literal(":"), repeat0(&fstring_format_spec))).into()
}

pub fn fstring_conversion()-> impl CombinatorTrait {
    tag("fstring_conversion", seq!(python_literal("!"), &NAME)).into()
}

pub fn fstring_replacement_field()-> impl CombinatorTrait {
    tag("fstring_replacement_field", seq!(
        python_literal("{"),
         choice!(&yield_expr, &star_expressions),
         opt(python_literal("=")),
         opt(&fstring_conversion),
         opt(&fstring_full_format_spec),
         python_literal("}")
    )).into()
}

pub fn fstring_middle()-> impl CombinatorTrait {
    tag("fstring_middle", choice!(
        &fstring_replacement_field,
        &FSTRING_MIDDLE
    )).into()
}

pub fn lambda_param()-> impl CombinatorTrait {
    tag("lambda_param", &NAME).into()
}

pub fn lambda_param_maybe_default()-> impl CombinatorTrait {
    tag("lambda_param_maybe_default", seq!(&lambda_param, opt(seq!(python_literal("="), &expression)), choice!(python_literal(","), lookahead(python_literal(":"))))).into()
}

pub fn lambda_param_with_default()-> impl CombinatorTrait {
    tag("lambda_param_with_default", seq!(&lambda_param, python_literal("="), &expression, choice!(python_literal(","), lookahead(python_literal(":"))))).into()
}

pub fn lambda_param_no_default()-> impl CombinatorTrait {
    tag("lambda_param_no_default", seq!(&lambda_param, choice!(python_literal(","), lookahead(python_literal(":"))))).into()
}

pub fn lambda_kwds()-> impl CombinatorTrait {
    tag("lambda_kwds", seq!(python_literal("**"), &lambda_param_no_default)).into()
}

pub fn lambda_star_etc()-> impl CombinatorTrait {
    tag("lambda_star_etc", choice!(
        seq!(python_literal("*"), choice!(seq!(&lambda_param_no_default, repeat0(&lambda_param_maybe_default), opt(&lambda_kwds)), seq!(python_literal(","), repeat1(&lambda_param_maybe_default), opt(&lambda_kwds)))),
        &lambda_kwds
    )).into()
}

pub fn lambda_slash_with_default()-> impl CombinatorTrait {
    tag("lambda_slash_with_default", seq!(repeat0(&lambda_param_no_default), repeat1(&lambda_param_with_default), python_literal("/"), choice!(python_literal(","), lookahead(python_literal(":"))))).into()
}

pub fn lambda_slash_no_default()-> impl CombinatorTrait {
    tag("lambda_slash_no_default", seq!(repeat1(&lambda_param_no_default), python_literal("/"), choice!(python_literal(","), lookahead(python_literal(":"))))).into()
}

pub fn lambda_parameters()-> impl CombinatorTrait {
    tag("lambda_parameters", choice!(
        seq!(&lambda_slash_no_default, repeat0(&lambda_param_no_default), repeat0(&lambda_param_with_default), opt(&lambda_star_etc)),
        seq!(&lambda_slash_with_default, repeat0(&lambda_param_with_default), opt(&lambda_star_etc)),
        seq!(repeat1(&lambda_param_no_default), repeat0(&lambda_param_with_default), opt(&lambda_star_etc)),
        seq!(repeat1(&lambda_param_with_default), opt(&lambda_star_etc)),
        &lambda_star_etc
    )).into()
}

pub fn lambda_params()-> impl CombinatorTrait {
    tag("lambda_params", &lambda_parameters).into()
}

pub fn lambdef()-> impl CombinatorTrait {
    tag("lambdef", seq!(python_literal("lambda"), opt(&lambda_params), python_literal(":"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))).into()
}

pub fn group()-> impl CombinatorTrait {
    tag("group", seq!(python_literal("("), choice!(seq!(python_literal("yield"), choice!(seq!(python_literal("from"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), opt(&star_expressions))), choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":="))))), python_literal(")"))).into()
}

pub fn atom()-> impl CombinatorTrait {
    tag("atom", choice!(
        &NAME,
        python_literal("True"),
        python_literal("False"),
        python_literal("None"),
        seq!(lookahead(choice!(&STRING, &FSTRING_START)), &strings),
        &NUMBER,
        seq!(lookahead(python_literal("(")), choice!(&tuple, &group, &genexp)),
        seq!(lookahead(python_literal("[")), choice!(&list, &listcomp)),
        seq!(lookahead(python_literal("{")), choice!(&dict, &set, &dictcomp, &setcomp)),
        python_literal("...")
    )).into()
}

pub fn slice()-> impl CombinatorTrait {
    tag("slice", choice!(
        seq!(opt(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), python_literal(":"), opt(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), opt(seq!(python_literal(":"), opt(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))))),
        choice!(seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":="))))
    )).into()
}

pub fn slices()-> impl CombinatorTrait {
    tag("slices", choice!(
        seq!(&slice, negative_lookahead(python_literal(","))),
        seq!(seprep1(choice!(&slice, &starred_expression), python_literal(",")), opt(python_literal(",")))
    )).into()
}

pub fn primary()-> impl CombinatorTrait {
    tag("primary", seq!(&atom, repeat0(choice!(seq!(python_literal("."), &NAME), &genexp, seq!(python_literal("("), opt(&arguments), python_literal(")")), seq!(python_literal("["), &slices, python_literal("]")))))).into()
}

pub fn await_primary()-> impl CombinatorTrait {
    cached(tag("await_primary", choice!(
        seq!(python_literal("await"), &primary),
        &primary
    ))).into()
}

pub fn power()-> impl CombinatorTrait {
    tag("power", seq!(&await_primary, opt(seq!(python_literal("**"), choice!(seq!(python_literal("+"), &factor), seq!(python_literal("-"), &factor), seq!(python_literal("~"), &factor), &power))))).into()
}

pub fn factor()-> impl CombinatorTrait {
    cached(tag("factor", choice!(
        seq!(python_literal("+"), &factor),
        seq!(python_literal("-"), &factor),
        seq!(python_literal("~"), &factor),
        &power
    ))).into()
}

pub fn term()-> impl CombinatorTrait {
    tag("term", seq!(&factor, repeat0(choice!(seq!(python_literal("*"), &factor), seq!(python_literal("/"), &factor), seq!(python_literal("//"), &factor), seq!(python_literal("%"), &factor), seq!(python_literal("@"), &factor))))).into()
}

pub fn sum()-> impl CombinatorTrait {
    tag("sum", seq!(&term, repeat0(choice!(seq!(python_literal("+"), &term), seq!(python_literal("-"), &term))))).into()
}

pub fn shift_expr()-> impl CombinatorTrait {
    tag("shift_expr", seq!(&sum, repeat0(choice!(seq!(python_literal("<<"), &sum), seq!(python_literal(">>"), &sum))))).into()
}

pub fn bitwise_and()-> impl CombinatorTrait {
    tag("bitwise_and", seq!(&shift_expr, repeat0(seq!(python_literal("&"), &shift_expr)))).into()
}

pub fn bitwise_xor()-> impl CombinatorTrait {
    tag("bitwise_xor", seq!(&bitwise_and, repeat0(seq!(python_literal("^"), &bitwise_and)))).into()
}

pub fn bitwise_or()-> impl CombinatorTrait {
    tag("bitwise_or", seq!(&bitwise_xor, repeat0(seq!(python_literal("|"), &bitwise_xor)))).into()
}

pub fn is_bitwise_or()-> impl CombinatorTrait {
    tag("is_bitwise_or", seq!(python_literal("is"), &bitwise_or)).into()
}

pub fn isnot_bitwise_or()-> impl CombinatorTrait {
    tag("isnot_bitwise_or", seq!(python_literal("is"), python_literal("not"), &bitwise_or)).into()
}

pub fn in_bitwise_or()-> impl CombinatorTrait {
    tag("in_bitwise_or", seq!(python_literal("in"), &bitwise_or)).into()
}

pub fn notin_bitwise_or()-> impl CombinatorTrait {
    tag("notin_bitwise_or", seq!(python_literal("not"), python_literal("in"), &bitwise_or)).into()
}

pub fn gt_bitwise_or()-> impl CombinatorTrait {
    tag("gt_bitwise_or", seq!(python_literal(">"), &bitwise_or)).into()
}

pub fn gte_bitwise_or()-> impl CombinatorTrait {
    tag("gte_bitwise_or", seq!(python_literal(">="), &bitwise_or)).into()
}

pub fn lt_bitwise_or()-> impl CombinatorTrait {
    tag("lt_bitwise_or", seq!(python_literal("<"), &bitwise_or)).into()
}

pub fn lte_bitwise_or()-> impl CombinatorTrait {
    tag("lte_bitwise_or", seq!(python_literal("<="), &bitwise_or)).into()
}

pub fn noteq_bitwise_or()-> impl CombinatorTrait {
    tag("noteq_bitwise_or", seq!(python_literal("!="), &bitwise_or)).into()
}

pub fn eq_bitwise_or()-> impl CombinatorTrait {
    tag("eq_bitwise_or", seq!(python_literal("=="), &bitwise_or)).into()
}

pub fn compare_op_bitwise_or_pair()-> impl CombinatorTrait {
    tag("compare_op_bitwise_or_pair", choice!(
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
    )).into()
}

pub fn comparison()-> impl CombinatorTrait {
    tag("comparison", seq!(&bitwise_or, repeat0(&compare_op_bitwise_or_pair))).into()
}

pub fn inversion()-> impl CombinatorTrait {
    cached(tag("inversion", choice!(
        seq!(python_literal("not"), &inversion),
        &comparison
    ))).into()
}

pub fn conjunction()-> impl CombinatorTrait {
    cached(tag("conjunction", seq!(&inversion, repeat0(seq!(python_literal("and"), &inversion))))).into()
}

pub fn disjunction()-> impl CombinatorTrait {
    cached(tag("disjunction", seq!(&conjunction, repeat0(seq!(python_literal("or"), &conjunction))))).into()
}

pub fn named_expression()-> impl CombinatorTrait {
    tag("named_expression", choice!(
        seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)),
        seq!(choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef), negative_lookahead(python_literal(":=")))
    )).into()
}

pub fn assignment_expression()-> impl CombinatorTrait {
    tag("assignment_expression", seq!(&NAME, python_literal(":="), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef))).into()
}

pub fn star_named_expression()-> impl CombinatorTrait {
    tag("star_named_expression", choice!(
        seq!(python_literal("*"), &bitwise_or),
        &named_expression
    )).into()
}

pub fn star_named_expressions()-> impl CombinatorTrait {
    tag("star_named_expressions", seq!(seprep1(&star_named_expression, python_literal(",")), opt(python_literal(",")))).into()
}

pub fn star_expression()-> impl CombinatorTrait {
    cached(tag("star_expression", choice!(
        seq!(python_literal("*"), &bitwise_or),
        choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)
    ))).into()
}

pub fn star_expressions()-> impl CombinatorTrait {
    tag("star_expressions", seq!(&star_expression, opt(choice!(seq!(repeat1(seq!(python_literal(","), &star_expression)), opt(python_literal(","))), python_literal(","))))).into()
}

pub fn yield_expr()-> impl CombinatorTrait {
    tag("yield_expr", seq!(python_literal("yield"), choice!(seq!(python_literal("from"), choice!(seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))), &lambdef)), opt(&star_expressions)))).into()
}

pub fn expression()-> impl CombinatorTrait {
    cached(tag("expression", choice!(
        seq!(&disjunction, opt(seq!(python_literal("if"), &disjunction, python_literal("else"), &expression))),
        &lambdef
    ))).into()
}

pub fn expressions()-> impl CombinatorTrait {
    tag("expressions", seq!(&expression, opt(choice!(seq!(repeat1(seq!(python_literal(","), &expression)), opt(python_literal(","))), python_literal(","))))).into()
}

pub fn type_param_starred_default()-> impl CombinatorTrait {
    tag("type_param_starred_default", seq!(python_literal("="), &star_expression)).into()
}

pub fn type_param_default()-> impl CombinatorTrait {
    tag("type_param_default", seq!(python_literal("="), &expression)).into()
}

pub fn type_param_bound()-> impl CombinatorTrait {
    tag("type_param_bound", seq!(python_literal(":"), &expression)).into()
}

pub fn type_param()-> impl CombinatorTrait {
    cached(tag("type_param", choice!(
        seq!(&NAME, opt(&type_param_bound), opt(&type_param_default)),
        seq!(python_literal("*"), &NAME, opt(&type_param_starred_default)),
        seq!(python_literal("**"), &NAME, opt(&type_param_default))
    ))).into()
}

pub fn type_param_seq()-> impl CombinatorTrait {
    tag("type_param_seq", seq!(seprep1(&type_param, python_literal(",")), opt(python_literal(",")))).into()
}

pub fn type_params()-> impl CombinatorTrait {
    tag("type_params", seq!(python_literal("["), &type_param_seq, python_literal("]"))).into()
}

pub fn type_alias()-> impl CombinatorTrait {
    tag("type_alias", seq!(
        python_literal("type"),
         &NAME,
         opt(&type_params),
         python_literal("="),
         &expression
    )).into()
}

pub fn keyword_pattern()-> impl CombinatorTrait {
    tag("keyword_pattern", seq!(&NAME, python_literal("="), choice!(&as_pattern, &or_pattern))).into()
}

pub fn keyword_patterns()-> impl CombinatorTrait {
    tag("keyword_patterns", seprep1(&keyword_pattern, python_literal(","))).into()
}

pub fn positional_patterns()-> impl CombinatorTrait {
    tag("positional_patterns", seprep1(&pattern, python_literal(","))).into()
}

pub fn class_pattern()-> impl CombinatorTrait {
    tag("class_pattern", seq!(&NAME, repeat0(seq!(python_literal("."), &NAME)), python_literal("("), choice!(python_literal(")"), seq!(&positional_patterns, choice!(seq!(opt(python_literal(",")), python_literal(")")), seq!(python_literal(","), &keyword_patterns, opt(python_literal(",")), python_literal(")")))), seq!(&keyword_patterns, opt(python_literal(",")), python_literal(")"))))).into()
}

pub fn double_star_pattern()-> impl CombinatorTrait {
    tag("double_star_pattern", seq!(python_literal("**"), negative_lookahead(python_literal("_")), &NAME, negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("="))))).into()
}

pub fn key_value_pattern()-> impl CombinatorTrait {
    tag("key_value_pattern", seq!(choice!(choice!(seq!(&signed_number, negative_lookahead(choice!(python_literal("+"), python_literal("-")))), &complex_number, &strings, python_literal("None"), python_literal("True"), python_literal("False")), seq!(&name_or_attr, python_literal("."), &NAME)), python_literal(":"), choice!(&as_pattern, &or_pattern))).into()
}

pub fn items_pattern()-> impl CombinatorTrait {
    tag("items_pattern", seprep1(&key_value_pattern, python_literal(","))).into()
}

pub fn mapping_pattern()-> impl CombinatorTrait {
    tag("mapping_pattern", seq!(python_literal("{"), choice!(python_literal("}"), seq!(&double_star_pattern, opt(python_literal(",")), python_literal("}")), seq!(&items_pattern, choice!(seq!(python_literal(","), &double_star_pattern, opt(python_literal(",")), python_literal("}")), seq!(opt(python_literal(",")), python_literal("}"))))))).into()
}

pub fn star_pattern()-> impl CombinatorTrait {
    cached(tag("star_pattern", seq!(python_literal("*"), choice!(seq!(negative_lookahead(python_literal("_")), &NAME, negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("=")))), python_literal("_"))))).into()
}

pub fn maybe_star_pattern()-> impl CombinatorTrait {
    tag("maybe_star_pattern", choice!(
        &star_pattern,
        choice!(&as_pattern, &or_pattern)
    )).into()
}

pub fn maybe_sequence_pattern()-> impl CombinatorTrait {
    tag("maybe_sequence_pattern", seq!(seprep1(&maybe_star_pattern, python_literal(",")), opt(python_literal(",")))).into()
}

pub fn open_sequence_pattern()-> impl CombinatorTrait {
    tag("open_sequence_pattern", seq!(&maybe_star_pattern, python_literal(","), opt(&maybe_sequence_pattern))).into()
}

pub fn sequence_pattern()-> impl CombinatorTrait {
    tag("sequence_pattern", choice!(
        seq!(python_literal("["), opt(&maybe_sequence_pattern), python_literal("]")),
        seq!(python_literal("("), opt(&open_sequence_pattern), python_literal(")"))
    )).into()
}

pub fn group_pattern()-> impl CombinatorTrait {
    tag("group_pattern", seq!(python_literal("("), choice!(&as_pattern, &or_pattern), python_literal(")"))).into()
}

pub fn name_or_attr()-> impl CombinatorTrait {
    tag("name_or_attr", seq!(&NAME, repeat0(seq!(python_literal("."), &NAME)))).into()
}

pub fn attr()-> impl CombinatorTrait {
    tag("attr", seq!(&name_or_attr, python_literal("."), &NAME)).into()
}

pub fn value_pattern()-> impl CombinatorTrait {
    tag("value_pattern", seq!(&attr, negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("="))))).into()
}

pub fn wildcard_pattern()-> impl CombinatorTrait {
    tag("wildcard_pattern", python_literal("_")).into()
}

pub fn pattern_capture_target()-> impl CombinatorTrait {
    tag("pattern_capture_target", seq!(negative_lookahead(python_literal("_")), &NAME, negative_lookahead(choice!(python_literal("."), python_literal("("), python_literal("="))))).into()
}

pub fn capture_pattern()-> impl CombinatorTrait {
    tag("capture_pattern", &pattern_capture_target).into()
}

pub fn imaginary_number()-> impl CombinatorTrait {
    tag("imaginary_number", &NUMBER).into()
}

pub fn real_number()-> impl CombinatorTrait {
    tag("real_number", &NUMBER).into()
}

pub fn signed_real_number()-> impl CombinatorTrait {
    tag("signed_real_number", choice!(
        &real_number,
        seq!(python_literal("-"), &real_number)
    )).into()
}

pub fn signed_number()-> impl CombinatorTrait {
    tag("signed_number", choice!(
        &NUMBER,
        seq!(python_literal("-"), &NUMBER)
    )).into()
}

pub fn complex_number()-> impl CombinatorTrait {
    tag("complex_number", seq!(&signed_real_number, choice!(seq!(python_literal("+"), &imaginary_number), seq!(python_literal("-"), &imaginary_number)))).into()
}

pub fn literal_expr()-> impl CombinatorTrait {
    tag("literal_expr", choice!(
        seq!(&signed_number, negative_lookahead(choice!(python_literal("+"), python_literal("-")))),
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).into()
}

pub fn literal_pattern()-> impl CombinatorTrait {
    tag("literal_pattern", choice!(
        seq!(&signed_number, negative_lookahead(choice!(python_literal("+"), python_literal("-")))),
        &complex_number,
        &strings,
        python_literal("None"),
        python_literal("True"),
        python_literal("False")
    )).into()
}

pub fn closed_pattern()-> impl CombinatorTrait {
    cached(tag("closed_pattern", choice!(
        &literal_pattern,
        &capture_pattern,
        &wildcard_pattern,
        &value_pattern,
        &group_pattern,
        &sequence_pattern,
        &mapping_pattern,
        &class_pattern
    ))).into()
}

pub fn or_pattern()-> impl CombinatorTrait {
    tag("or_pattern", seprep1(&closed_pattern, python_literal("|"))).into()
}

pub fn as_pattern()-> impl CombinatorTrait {
    tag("as_pattern", seq!(&or_pattern, python_literal("as"), &pattern_capture_target)).into()
}

pub fn pattern()-> impl CombinatorTrait {
    tag("pattern", choice!(
        &as_pattern,
        &or_pattern
    )).into()
}

pub fn patterns()-> impl CombinatorTrait {
    tag("patterns", choice!(
        &open_sequence_pattern,
        &pattern
    )).into()
}

pub fn guard()-> impl CombinatorTrait {
    tag("guard", seq!(python_literal("if"), &named_expression)).into()
}

pub fn case_block()-> impl CombinatorTrait {
    tag("case_block", seq!(
        python_literal("case"),
         &patterns,
         opt(&guard),
         python_literal(":"),
         choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))
    )).into()
}

pub fn subject_expr()-> impl CombinatorTrait {
    tag("subject_expr", choice!(
        seq!(&star_named_expression, python_literal(","), opt(&star_named_expressions)),
        &named_expression
    )).into()
}

pub fn match_stmt()-> impl CombinatorTrait {
    tag("match_stmt", seq!(
        python_literal("match"),
         &subject_expr,
         python_literal(":"),
         &NEWLINE,
         &INDENT,
         repeat1(&case_block),
         &DEDENT
    )).into()
}

pub fn finally_block()-> impl CombinatorTrait {
    tag("finally_block", seq!(python_literal("finally"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))))).into()
}

pub fn except_star_block()-> impl CombinatorTrait {
    tag("except_star_block", seq!(
        python_literal("except"),
         python_literal("*"),
         &expression,
         opt(seq!(python_literal("as"), &NAME)),
         python_literal(":"),
         choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))
    )).into()
}

pub fn except_block()-> impl CombinatorTrait {
    tag("except_block", seq!(python_literal("except"), choice!(seq!(&expression, opt(seq!(python_literal("as"), &NAME)), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))), seq!(python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))))))).into()
}

pub fn try_stmt()-> impl CombinatorTrait {
    tag("try_stmt", seq!(python_literal("try"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))), choice!(&finally_block, seq!(repeat1(&except_block), opt(seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))))), opt(&finally_block)), seq!(repeat1(&except_star_block), opt(seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))))), opt(&finally_block))))).into()
}

pub fn with_item()-> impl CombinatorTrait {
    tag("with_item", seq!(&expression, opt(seq!(python_literal("as"), &star_target, lookahead(choice!(python_literal(","), python_literal(")"), python_literal(":"))))))).into()
}

pub fn with_stmt()-> impl CombinatorTrait {
    tag("with_stmt", choice!(
        seq!(python_literal("with"), choice!(seq!(python_literal("("), seprep1(&with_item, python_literal(",")), opt(python_literal(",")), python_literal(")"), python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))), seq!(seprep1(&with_item, python_literal(",")), python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))))),
        seq!(python_literal("async"), python_literal("with"), choice!(seq!(python_literal("("), seprep1(&with_item, python_literal(",")), opt(python_literal(",")), python_literal(")"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))), seq!(seprep1(&with_item, python_literal(",")), python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))))))
    )).into()
}

pub fn for_stmt()-> impl CombinatorTrait {
    tag("for_stmt", choice!(
        seq!(python_literal("for"), &star_targets, python_literal("in"), &star_expressions, python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))), opt(seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))))),
        seq!(python_literal("async"), python_literal("for"), &star_targets, python_literal("in"), &star_expressions, python_literal(":"), opt(&TYPE_COMMENT), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))), opt(seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))))))
    )).into()
}

pub fn while_stmt()-> impl CombinatorTrait {
    tag("while_stmt", seq!(
        python_literal("while"),
         &named_expression,
         python_literal(":"),
         choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))),
         opt(seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))))
    )).into()
}

pub fn else_block()-> impl CombinatorTrait {
    tag("else_block", seq!(python_literal("else"), python_literal(":"), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))))).into()
}

pub fn elif_stmt()-> impl CombinatorTrait {
    tag("elif_stmt", seq!(
        python_literal("elif"),
         &named_expression,
         python_literal(":"),
         choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))),
         choice!(&elif_stmt, opt(&else_block))
    )).into()
}

pub fn if_stmt()-> impl CombinatorTrait {
    tag("if_stmt", seq!(
        python_literal("if"),
         &named_expression,
         python_literal(":"),
         choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))),
         choice!(&elif_stmt, opt(&else_block))
    )).into()
}

pub fn default()-> impl CombinatorTrait {
    tag("default", seq!(python_literal("="), &expression)).into()
}

pub fn star_annotation()-> impl CombinatorTrait {
    tag("star_annotation", seq!(python_literal(":"), &star_expression)).into()
}

pub fn annotation()-> impl CombinatorTrait {
    tag("annotation", seq!(python_literal(":"), &expression)).into()
}

pub fn param_star_annotation()-> impl CombinatorTrait {
    tag("param_star_annotation", seq!(&NAME, &star_annotation)).into()
}

pub fn param()-> impl CombinatorTrait {
    tag("param", seq!(&NAME, opt(&annotation))).into()
}

pub fn param_maybe_default()-> impl CombinatorTrait {
    tag("param_maybe_default", seq!(&param, opt(&default), choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), seq!(opt(&TYPE_COMMENT), lookahead(python_literal(")")))))).into()
}

pub fn param_with_default()-> impl CombinatorTrait {
    tag("param_with_default", seq!(&param, &default, choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), seq!(opt(&TYPE_COMMENT), lookahead(python_literal(")")))))).into()
}

pub fn param_no_default_star_annotation()-> impl CombinatorTrait {
    tag("param_no_default_star_annotation", seq!(&param_star_annotation, choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), seq!(opt(&TYPE_COMMENT), lookahead(python_literal(")")))))).into()
}

pub fn param_no_default()-> impl CombinatorTrait {
    tag("param_no_default", seq!(&param, choice!(seq!(python_literal(","), opt(&TYPE_COMMENT)), seq!(opt(&TYPE_COMMENT), lookahead(python_literal(")")))))).into()
}

pub fn kwds()-> impl CombinatorTrait {
    tag("kwds", seq!(python_literal("**"), &param_no_default)).into()
}

pub fn star_etc()-> impl CombinatorTrait {
    tag("star_etc", choice!(
        seq!(python_literal("*"), choice!(seq!(&param_no_default, repeat0(&param_maybe_default), opt(&kwds)), seq!(&param_no_default_star_annotation, repeat0(&param_maybe_default), opt(&kwds)), seq!(python_literal(","), repeat1(&param_maybe_default), opt(&kwds)))),
        &kwds
    )).into()
}

pub fn slash_with_default()-> impl CombinatorTrait {
    tag("slash_with_default", seq!(repeat0(&param_no_default), repeat1(&param_with_default), python_literal("/"), choice!(python_literal(","), lookahead(python_literal(")"))))).into()
}

pub fn slash_no_default()-> impl CombinatorTrait {
    tag("slash_no_default", seq!(repeat1(&param_no_default), python_literal("/"), choice!(python_literal(","), lookahead(python_literal(")"))))).into()
}

pub fn parameters()-> impl CombinatorTrait {
    tag("parameters", choice!(
        seq!(&slash_no_default, repeat0(&param_no_default), repeat0(&param_with_default), opt(&star_etc)),
        seq!(&slash_with_default, repeat0(&param_with_default), opt(&star_etc)),
        seq!(repeat1(&param_no_default), repeat0(&param_with_default), opt(&star_etc)),
        seq!(repeat1(&param_with_default), opt(&star_etc)),
        &star_etc
    )).into()
}

pub fn params()-> impl CombinatorTrait {
    tag("params", &parameters).into()
}

pub fn function_def_raw()-> impl CombinatorTrait {
    tag("function_def_raw", choice!(
        seq!(python_literal("def"), &NAME, opt(&type_params), python_literal("("), opt(&params), python_literal(")"), opt(seq!(python_literal("->"), &expression)), python_literal(":"), opt(&func_type_comment), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))),
        seq!(python_literal("async"), python_literal("def"), &NAME, opt(&type_params), python_literal("("), opt(&params), python_literal(")"), opt(seq!(python_literal("->"), &expression)), python_literal(":"), opt(&func_type_comment), choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))))
    )).into()
}

pub fn function_def()-> impl CombinatorTrait {
    tag("function_def", choice!(
        seq!(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE)), &function_def_raw),
        &function_def_raw
    )).into()
}

pub fn class_def_raw()-> impl CombinatorTrait {
    tag("class_def_raw", seq!(
        python_literal("class"),
         &NAME,
         opt(&type_params),
         opt(seq!(python_literal("("), opt(&arguments), python_literal(")"))),
         python_literal(":"),
         choice!(seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT), choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)))
    )).into()
}

pub fn class_def()-> impl CombinatorTrait {
    tag("class_def", choice!(
        seq!(repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE)), &class_def_raw),
        &class_def_raw
    )).into()
}

pub fn decorators()-> impl CombinatorTrait {
    tag("decorators", repeat1(seq!(python_literal("@"), &named_expression, &NEWLINE))).into()
}

pub fn block()-> impl CombinatorTrait {
    cached(tag("block", choice!(
        seq!(&NEWLINE, &INDENT, repeat1(&statement), &DEDENT),
        choice!(seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE), seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE))
    ))).into()
}

pub fn dotted_name()-> impl CombinatorTrait {
    tag("dotted_name", seq!(&NAME, repeat0(seq!(python_literal("."), &NAME)))).into()
}

pub fn dotted_as_name()-> impl CombinatorTrait {
    tag("dotted_as_name", seq!(&dotted_name, opt(seq!(python_literal("as"), &NAME)))).into()
}

pub fn dotted_as_names()-> impl CombinatorTrait {
    tag("dotted_as_names", seprep1(&dotted_as_name, python_literal(","))).into()
}

pub fn import_from_as_name()-> impl CombinatorTrait {
    tag("import_from_as_name", seq!(&NAME, opt(seq!(python_literal("as"), &NAME)))).into()
}

pub fn import_from_as_names()-> impl CombinatorTrait {
    tag("import_from_as_names", seprep1(&import_from_as_name, python_literal(","))).into()
}

pub fn import_from_targets()-> impl CombinatorTrait {
    tag("import_from_targets", choice!(
        seq!(python_literal("("), &import_from_as_names, opt(python_literal(",")), python_literal(")")),
        seq!(&import_from_as_names, negative_lookahead(python_literal(","))),
        python_literal("*")
    )).into()
}

pub fn import_from()-> impl CombinatorTrait {
    tag("import_from", seq!(python_literal("from"), choice!(seq!(repeat0(choice!(python_literal("."), python_literal("..."))), &dotted_name, python_literal("import"), &import_from_targets), seq!(repeat1(choice!(python_literal("."), python_literal("..."))), python_literal("import"), &import_from_targets)))).into()
}

pub fn import_name()-> impl CombinatorTrait {
    tag("import_name", seq!(python_literal("import"), &dotted_as_names)).into()
}

pub fn import_stmt()-> impl CombinatorTrait {
    tag("import_stmt", choice!(
        &import_name,
        &import_from
    )).into()
}

pub fn assert_stmt()-> impl CombinatorTrait {
    tag("assert_stmt", seq!(python_literal("assert"), &expression, opt(seq!(python_literal(","), &expression)))).into()
}

pub fn yield_stmt()-> impl CombinatorTrait {
    tag("yield_stmt", &yield_expr).into()
}

pub fn del_stmt()-> impl CombinatorTrait {
    tag("del_stmt", seq!(python_literal("del"), &del_targets, lookahead(choice!(python_literal(";"), &NEWLINE)))).into()
}

pub fn nonlocal_stmt()-> impl CombinatorTrait {
    tag("nonlocal_stmt", seq!(python_literal("nonlocal"), seprep1(&NAME, python_literal(",")))).into()
}

pub fn global_stmt()-> impl CombinatorTrait {
    tag("global_stmt", seq!(python_literal("global"), seprep1(&NAME, python_literal(",")))).into()
}

pub fn raise_stmt()-> impl CombinatorTrait {
    tag("raise_stmt", seq!(python_literal("raise"), opt(seq!(&expression, opt(seq!(python_literal("from"), &expression)))))).into()
}

pub fn return_stmt()-> impl CombinatorTrait {
    tag("return_stmt", seq!(python_literal("return"), opt(&star_expressions))).into()
}

pub fn augassign()-> impl CombinatorTrait {
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
    )).into()
}

pub fn annotated_rhs()-> impl CombinatorTrait {
    tag("annotated_rhs", choice!(
        &yield_expr,
        &star_expressions
    )).into()
}

pub fn assignment()-> impl CombinatorTrait {
    tag("assignment", choice!(
        seq!(&NAME, python_literal(":"), &expression, opt(seq!(python_literal("="), &annotated_rhs))),
        seq!(choice!(seq!(python_literal("("), &single_target, python_literal(")")), &single_subscript_attribute_target), python_literal(":"), &expression, opt(seq!(python_literal("="), &annotated_rhs))),
        seq!(repeat1(seq!(&star_targets, python_literal("="))), choice!(&yield_expr, &star_expressions), negative_lookahead(python_literal("=")), opt(&TYPE_COMMENT)),
        seq!(&single_target, &augassign, choice!(&yield_expr, &star_expressions))
    )).into()
}

pub fn compound_stmt()-> impl CombinatorTrait {
    tag("compound_stmt", choice!(
        seq!(lookahead(choice!(python_literal("def"), python_literal("@"), python_literal("async"))), &function_def),
        seq!(lookahead(python_literal("if")), &if_stmt),
        seq!(lookahead(choice!(python_literal("class"), python_literal("@"))), &class_def),
        seq!(lookahead(choice!(python_literal("with"), python_literal("async"))), &with_stmt),
        seq!(lookahead(choice!(python_literal("for"), python_literal("async"))), &for_stmt),
        seq!(lookahead(python_literal("try")), &try_stmt),
        seq!(lookahead(python_literal("while")), &while_stmt),
        &match_stmt
    )).into()
}

pub fn simple_stmt()-> impl CombinatorTrait {
    cached(tag("simple_stmt", choice!(
        &assignment,
        seq!(lookahead(python_literal("type")), &type_alias),
        &star_expressions,
        seq!(lookahead(python_literal("return")), &return_stmt),
        seq!(lookahead(choice!(python_literal("import"), python_literal("from"))), &import_stmt),
        seq!(lookahead(python_literal("raise")), &raise_stmt),
        python_literal("pass"),
        seq!(lookahead(python_literal("del")), &del_stmt),
        seq!(lookahead(python_literal("yield")), &yield_stmt),
        seq!(lookahead(python_literal("assert")), &assert_stmt),
        python_literal("break"),
        python_literal("continue"),
        seq!(lookahead(python_literal("global")), &global_stmt),
        seq!(lookahead(python_literal("nonlocal")), &nonlocal_stmt)
    ))).into()
}

pub fn simple_stmts()-> impl CombinatorTrait {
    tag("simple_stmts", choice!(
        seq!(&simple_stmt, negative_lookahead(python_literal(";")), &NEWLINE),
        seq!(seprep1(&simple_stmt, python_literal(";")), opt(python_literal(";")), &NEWLINE)
    )).into()
}

pub fn statement_newline()-> impl CombinatorTrait {
    tag("statement_newline", choice!(
        seq!(&compound_stmt, &NEWLINE),
        &simple_stmts,
        &NEWLINE,
        &ENDMARKER
    )).into()
}

pub fn statement()-> impl CombinatorTrait {
    tag("statement", choice!(
        &compound_stmt,
        &simple_stmts
    )).into()
}

pub fn statements()-> impl CombinatorTrait {
    tag("statements", repeat1(&statement)).into()
}

pub fn func_type()-> impl CombinatorTrait {
    tag("func_type", seq!(
        python_literal("("),
         opt(&type_expressions),
         python_literal(")"),
         python_literal("->"),
         &expression,
         repeat0(&NEWLINE),
         &ENDMARKER
    )).into()
}

pub fn eval()-> impl CombinatorTrait {
    tag("eval", seq!(&expressions, repeat0(&NEWLINE), &ENDMARKER)).into()
}

pub fn interactive()-> impl CombinatorTrait {
    tag("interactive", &statement_newline).into()
}

pub fn file()-> impl CombinatorTrait {
    tag("file", seq!(opt(&statements), &ENDMARKER)).into()
}


pub fn python_file()-> impl CombinatorTrait {

    cache_context(tag("main", seq!(opt(&NEWLINE), &file))).compile()
}
