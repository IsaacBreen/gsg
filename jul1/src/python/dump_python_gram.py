# src/python/dump_python_gram.py
import io
import logging
import random
import textwrap
import tokenize
from dataclasses import dataclass, field
from io import StringIO
from typing import Optional

import pegen.grammar
import requests
from pegen.grammar_parser import GeneratedParser
from pegen.tokenizer import Tokenizer

import grammar_analysis
from grammar_analysis import ref

random.seed(0)


def fetch_grammar(url: str) -> str:
    response = requests.get(url)
    response.raise_for_status()
    return response.text


def parse_grammar(text: str) -> pegen.grammar.Grammar:
    with StringIO(text) as f:
        tokenizer = Tokenizer(tokenize.generate_tokens(f.readline))
        parser = GeneratedParser(tokenizer)
        grammar = parser.start()
        return grammar


def pegen_to_custom(
        grammar: pegen.grammar.Grammar, omit_invalid: bool = True,
        include_lookaheads: bool = True) -> dict[
    grammar_analysis.Ref, grammar_analysis.Node]:
    def rhs_to_node(rhs: pegen.grammar.Rhs) -> grammar_analysis.Node:
        if len(rhs.alts) == 1:
            return alt_to_node(rhs.alts[0])
        return grammar_analysis.Choice([alt_to_node(alt) for alt in rhs.alts])

    def alt_to_node(alt: pegen.grammar.Alt) -> grammar_analysis.Node:
        if len(alt.items) == 1:
            return named_item_to_node(alt.items[0])
        return grammar_analysis.Seq([named_item_to_node(item) for item in alt.items])

    def named_item_to_node(item: pegen.grammar.NamedItem) -> grammar_analysis.Node:
        return item_to_node(item.item)

    def item_to_node(item) -> grammar_analysis.Node:
        if isinstance(item, pegen.grammar.NameLeaf):
            value = item.value
            if omit_invalid and value.startswith('invalid_'):
                return grammar_analysis.fail()
            else:
                return grammar_analysis.ref(value)
        elif isinstance(item, pegen.grammar.StringLeaf):
            value = item.value
            return grammar_analysis.term(value)
        elif isinstance(item, pegen.grammar.Group):
            return rhs_to_node(item.rhs)
        elif isinstance(item, pegen.grammar.Opt):
            return grammar_analysis.opt(item_to_node(item.node))
        elif isinstance(item, pegen.grammar.Gather):
            return grammar_analysis.sep_rep1(item_to_node(item.node), item_to_node(item.separator))
        elif isinstance(item, pegen.grammar.Repeat0):
            return grammar_analysis.repeat0(item_to_node(item.node))
        elif isinstance(item, pegen.grammar.Repeat1):
            return grammar_analysis.repeat1(item_to_node(item.node))
        elif isinstance(item, pegen.grammar.Forced):
            return item_to_node(item.node)
        elif isinstance(item, pegen.grammar.PositiveLookahead):
            if include_lookaheads:
                return grammar_analysis.lookahead(item_to_node(item.node))
            else:
                return grammar_analysis.eps()
        elif isinstance(item, pegen.grammar.NegativeLookahead):
            if include_lookaheads:
                return grammar_analysis.negative_lookahead(item_to_node(item.node))
            else:
                return grammar_analysis.eps()
        elif isinstance(item, pegen.grammar.Rhs):
            return rhs_to_node(item)
        elif isinstance(item, pegen.grammar.Cut):
            return grammar_analysis.eps()
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    rules = {}
    for name, rule in grammar.rules.items():
        if not (omit_invalid and name.startswith('invalid_')):
            rules[grammar_analysis.ref(name)] = rhs_to_node(rule.rhs).simplify()
    return rules


def custom_to_pegen(rules: dict[grammar_analysis.Ref, grammar_analysis.Node]) -> pegen.grammar.Grammar:
    def node_to_rhs(node: grammar_analysis.Node) -> pegen.grammar.Rhs:
        if isinstance(node, grammar_analysis.Choice):
            return pegen.grammar.Rhs([node_to_alt(child) for child in node.children])
        return pegen.grammar.Rhs([node_to_alt(node)])

    def node_to_alt(node: grammar_analysis.Node) -> pegen.grammar.Alt:
        if isinstance(node, grammar_analysis.Seq):
            assert len(node.children) > 0
            return pegen.grammar.Alt(
                [pegen.grammar.NamedItem(None, node_to_item(child)) for child in node.children]
            )
        return pegen.grammar.Alt([pegen.grammar.NamedItem(None, node_to_item(node))])

    def node_to_item(node: grammar_analysis.Node):
        if isinstance(node, grammar_analysis.Term):
            return pegen.grammar.StringLeaf(node.value)
        elif isinstance(node, grammar_analysis.Ref):
            return pegen.grammar.NameLeaf(node.name)
        elif isinstance(node, grammar_analysis.Lookahead):
            if node.positive:
                return pegen.grammar.PositiveLookahead(node_to_item(node.child))
            else:
                return pegen.grammar.NegativeLookahead(node_to_item(node.child))
        elif isinstance(node, grammar_analysis.Seq):
            assert len(node.children) > 0
            return pegen.grammar.Group(node_to_rhs(node))
        elif isinstance(node, grammar_analysis.Choice):
            if grammar_analysis.eps() in node.children:
                children = node.children.copy()
                children.remove(grammar_analysis.eps())
                if len(children) == 1:
                    child = children[0]
                    if isinstance(child, grammar_analysis.Repeat1):
                        return pegen.grammar.Repeat0(node_to_item(child.child))
                    else:
                        return pegen.grammar.Opt(node_to_rhs(grammar_analysis.Choice(children)))
                else:
                    return pegen.grammar.Opt(node_to_rhs(grammar_analysis.Choice(children)))
            assert len(node.children) > 0
            return pegen.grammar.Group(node_to_rhs(node))
        elif isinstance(node, grammar_analysis.Repeat1):
            return pegen.grammar.Repeat1(node_to_item(node.child))
        elif isinstance(node, grammar_analysis.SepRep1):
            return pegen.grammar.Gather(node_to_item(node.separator), node_to_item(node.child))
        else:
            raise ValueError(f"Unknown node type: {type(node)}")

    pegen_rules = {}
    for ref, node in rules.items():
        pegen_rules[ref.name] = pegen.grammar.Rule(ref.name, None, node_to_rhs(node.simplify()))
    return pegen.grammar.Grammar(list(pegen_rules.values()), {})


def grammar_to_rust(
        grammar: pegen.grammar.Grammar,
        unresolved_follows_table: dict[grammar_analysis.Ref, list[grammar_analysis.Ref]]
) -> str:
    @dataclass
    class ExtraInfo:
        added_rules: set[str] = field(default_factory=set)
        current_rule: Optional[str] = None
        rule_complexity: dict[str, int] = field(default_factory=dict)

    def generate_combinator_expr(item, extra_info) -> str:
        extra_info.rule_complexity[extra_info.current_rule] += 1
        if isinstance(item, pegen.grammar.NameLeaf):
            return name_to_rust(item.value, extra_info)
        elif isinstance(item, pegen.grammar.StringLeaf):
            value = item.value
            if value[0] == value[-1] in {'"', "'"}:
                value = value[1:-1]
            else:
                raise ValueError(f"Invalid string literal: {value}")
            return f'python_literal("{value}")'
        elif isinstance(item, pegen.grammar.Group):
            return generate_rhs_expr(item.rhs, extra_info)
        elif isinstance(item, pegen.grammar.Opt):
            return f'opt({generate_combinator_expr(item.node, extra_info)})'
        elif isinstance(item, pegen.grammar.Gather):
            return f'seprep1({generate_combinator_expr(item.node, extra_info)}, {generate_combinator_expr(item.separator, extra_info)})'
        elif isinstance(item, pegen.grammar.Repeat0):
            return f'repeat0({generate_combinator_expr(item.node, extra_info)})'
        elif isinstance(item, pegen.grammar.Repeat1):
            return f'repeat1({generate_combinator_expr(item.node, extra_info)})'
        elif isinstance(item, pegen.grammar.Forced):
            return generate_combinator_expr(item.node, extra_info)
        elif isinstance(item, pegen.grammar.PositiveLookahead):
            return f"lookahead({generate_combinator_expr(item.node, extra_info)})"
        elif isinstance(item, pegen.grammar.NegativeLookahead):
            return f"negative_lookahead({generate_combinator_expr(item.node, extra_info)})"
        elif isinstance(item, pegen.grammar.Rhs):
            return generate_rhs_expr(item, extra_info)
        elif isinstance(item, pegen.grammar.Cut):
            return 'cut()'
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    def generate_rhs_expr(rhs: pegen.grammar.Rhs, extra_info, top_level: bool = False) -> str:
        if len(rhs.alts) == 1:
            return generate_alt_expr(rhs.alts[0], extra_info, top_level=top_level)
        if top_level:
            return "choice!(\n    " + ",\n    ".join(
                generate_alt_expr(alt, extra_info) for alt in rhs.alts
            ) + "\n)"
        else:
            return "choice!(" + ", ".join(generate_alt_expr(alt, extra_info) for alt in rhs.alts) + ")"

    def generate_alt_expr(alt: pegen.grammar.Alt, extra_info, top_level: bool = False) -> str:
        if len(alt.items) == 1:
            return generate_combinator_expr(alt.items[0].item, extra_info)
        if top_level and len(alt.items) > 4:
            return "seq!(\n    " + ",\n     ".join(
                generate_combinator_expr(item.item, extra_info) for item in alt.items
            ) + "\n)"
        else:
            return "seq!(" + ", ".join(
                generate_combinator_expr(item.item, extra_info) for item in alt.items
            ) + ")"

    MAX_RULE_COMPLEXITY = 0

    def name_to_rust(name: str, extra_info: ExtraInfo) -> str:
        # if name in extra_info:
        if name in extra_info.added_rules:
            if extra_info.rule_complexity[name] > MAX_RULE_COMPLEXITY:
                return f'deferred({name}).into_dyn()'
            else:
                extra_info.rule_complexity[extra_info.current_rule] += extra_info.rule_complexity[name]
                return f'deferred({name})'
        else:
            return f'deferred({name}).into_dyn()'

    rules = grammar.rules.items()
    rules = list(reversed(rules))

    tokens = ['WS', 'NAME', 'TYPE_COMMENT', 'FSTRING_START', 'FSTRING_MIDDLE', 'FSTRING_END', 'NUMBER', 'STRING',
              'NEWLINE', 'INDENT', 'DEDENT', 'ENDMARKER']

    f = io.StringIO()
    f.write('use std::rc::Rc;\n')
    f.write('use crate::{BaseCombinatorTrait};\n')
    f.write(
        'use crate::{cache_context, cached, symbol, Symbol, mutate_right_data, RightData, Choice, deferred, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, Repeat1, Seq, tag, lookahead, negative_lookahead};\n'
        )
    f.write('use crate::seq;\n')
    f.write(
        'use crate::{' + ', '.join(
            f'{name}_greedy as {name}' for name in ['opt', 'choice', 'seprep0', 'seprep1', 'repeat0', 'repeat1']
            ) + '};\n'
        )
    f.write('use crate::IntoDyn;\n')
    f.write('\n')

    f.write('enum Forbidden {\n')
    for token in tokens:
        f.write(f'    {token},\n')
    f.write('}\n')
    f.write('\n')

    extra_info = ExtraInfo()

    def make_tokens() -> str:
        f = io.StringIO()
        f.write('use super::python_tokenizer as token;\n')

        f.write(
            textwrap.dedent(
                """
            pub fn python_literal(s: &str) -> impl CombinatorTrait {
                let increment_scope_count = |right_data: &mut RightData| { Rc::make_mut(&mut right_data.right_data_inner).fields1.scope_count += 1; true };
                let decrement_scope_count = |right_data: &mut RightData| { Rc::make_mut(&mut right_data.right_data_inner).fields1.scope_count -= 1; true };

                match s {
                    "(" | "[" | "{" => seq!(eat_string(s), mutate_right_data(increment_scope_count), forbid_follows_clear(), opt(deferred(WS))).into_dyn(),
                    ")" | "]" | "}" => seq!(eat_string(s), mutate_right_data(decrement_scope_count), forbid_follows_clear(), opt(deferred(WS))).into_dyn(),
                    _ => seq!(eat_string(s), forbid_follows_clear(), opt(deferred(WS))).into_dyn(),
                }
            }
        """
                )
            )

        for token in tokens:
            extra_info.rule_complexity[token] = 1

            expr = f'token::{token}()'
            expr = f'{expr}.compile()'

            token_ref = grammar_analysis.ref(token)
            if token_ref in unresolved_follows_table and any(
                    token_ref in forbidden_follow_set for forbidden_follow_set in
                    unresolved_follows_table.values()
            ):
                expr = f'seq!(forbid_follows_check_not(Forbidden::{token} as usize), {expr}, forbid_follows(&[{", ".join(f"Forbidden::{ref.name} as usize" for ref in unresolved_follows_table.get(token_ref, []))}]))'
            elif token_ref in unresolved_follows_table:
                expr = f'seq!({expr}, forbid_follows(&[{", ".join(f"Forbidden::{ref.name} as usize" for ref in unresolved_follows_table.get(token_ref, []))}]))'
            elif any(token_ref in forbidden_follow_set for forbidden_follow_set in unresolved_follows_table.values()):
                expr = f'seq!(forbid_follows_check_not(Forbidden::{token} as usize), {expr})'
            else:
                expr = f'seq!({expr}, forbid_follows_clear())'
            expr = f'crate::profile("{token}", {expr})'
            expr = f'tag("{token}", {expr})'
            if token != 'WS' and grammar_analysis.ref('WS') not in unresolved_follows_table.get(token_ref, []):
                expr = f'seq!({expr}, opt(deferred(WS)))'
            expr = f'cached({expr})'
            f.write('pub fn ' + token + '() -> impl CombinatorTrait { ' + expr + ' }\n')
            extra_info.added_rules.add(token)
        f.write('\n')
        return f.getvalue()

    def make_rules() -> str:
        f = io.StringIO()
        for name, rule in rules:
            extra_info.current_rule = name
            extra_info.rule_complexity[name] = 1
            expr = generate_rhs_expr(rule.rhs, extra_info, top_level=True)
            expr = f'tag("{name}", {expr})'
            if rule.memo:
                expr = f'cached({expr})'
            expr = f'{expr}'
            f.write('pub fn ' + name + '() -> impl CombinatorTrait {\n')
            f.write(f'{textwrap.indent(expr, "    ")}\n')
            f.write('}\n')
            f.write('\n')
            extra_info.added_rules.add(name)
        f.write('\n')
        return f.getvalue()

    f.write(make_tokens())
    f.write(make_rules())

    f.write('pub fn python_file() -> impl CombinatorTrait {\n')
    expr = f'seq!(opt({name_to_rust("NEWLINE", extra_info)}), {name_to_rust("file", extra_info)})'
    expr = f'tag("main", {expr})'
    expr = f'cache_context({expr})'
    f.write(f'\n    {expr}.compile()\n')
    f.write('}\n')
    return f.getvalue()


def save_grammar_to_rust(
        grammar: pegen.grammar.Grammar, filename: str,
        unresolved_follows_table: dict[grammar_analysis.Ref, list[
            grammar_analysis.Ref]]) -> None:
    rust_code = grammar_to_rust(grammar, unresolved_follows_table)
    with open(filename, 'w') as f:
        f.write(rust_code)


if __name__ == "__main__":
    grammar_url = "https://raw.githubusercontent.com/python/cpython/main/Grammar/python.gram"
    grammar_text = fetch_grammar(grammar_url)
    pegen_grammar = parse_grammar(grammar_text)

    custom_grammar = pegen_to_custom(pegen_grammar)

    custom_grammar = grammar_analysis.resolve_left_recursion(custom_grammar)

    # Use lists instead of sets for values to ensure deterministic order
    forbidden_follows_table = {
        ref('FSTRING_START'): [ref('WS'), ref('NEWLINE')],
        ref('FSTRING_MIDDLE'): [ref('FSTRING_MIDDLE'), ref('WS')],
        ref('NEWLINE'): [ref('WS')],
        ref('INDENT'): [ref('WS')],
        ref('DEDENT'): [ref('WS')],
        ref('NAME'): [ref('NAME'), ref('NUMBER')],
        ref('NUMBER'): [ref('NUMBER')],
        ref('WS'): [ref('INDENT'), ref('DEDENT')],
    }

    grammar_analysis.prettify_rules(custom_grammar)

    resolved_pegen_grammar = custom_to_pegen(custom_grammar)

    for rule_name in resolved_pegen_grammar.rules:
        resolved_pegen_grammar.rules[rule_name].memo = pegen_grammar.rules[rule_name].memo

    save_grammar_to_rust(resolved_pegen_grammar, 'python_grammar.rs', forbidden_follows_table)