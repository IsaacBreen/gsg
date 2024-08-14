# main.py
import io
import logging
import textwrap
import tokenize
from io import StringIO

import requests
import pegen.grammar
from pegen.grammar_parser import GeneratedParser
from pegen.tokenizer import Tokenizer

import remove_left_recursion
from remove_left_recursion import ref, term, opt


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


def pegen_to_custom(grammar: pegen.grammar.Grammar, omit_invalid: bool = True,
                    include_lookaheads: bool = True) -> dict[
    remove_left_recursion.Ref, remove_left_recursion.Node]:
    def rhs_to_node(rhs: pegen.grammar.Rhs) -> remove_left_recursion.Node:
        if len(rhs.alts) == 1:
            return alt_to_node(rhs.alts[0])
        return remove_left_recursion.Choice([alt_to_node(alt) for alt in rhs.alts])

    def alt_to_node(alt: pegen.grammar.Alt) -> remove_left_recursion.Node:
        if len(alt.items) == 1:
            return named_item_to_node(alt.items[0])
        return remove_left_recursion.Seq([named_item_to_node(item) for item in alt.items])

    def named_item_to_node(item: pegen.grammar.NamedItem) -> remove_left_recursion.Node:
        return item_to_node(item.item)

    def item_to_node(item) -> remove_left_recursion.Node:
        if isinstance(item, pegen.grammar.NameLeaf):
            value = item.value
            if omit_invalid and value.startswith('invalid_'):
                return remove_left_recursion.fail()
            else:
                return remove_left_recursion.ref(value)
        elif isinstance(item, pegen.grammar.StringLeaf):
            value = item.value
            return remove_left_recursion.term(value)
        elif isinstance(item, pegen.grammar.Group):
            return rhs_to_node(item.rhs)
        elif isinstance(item, pegen.grammar.Opt):
            return remove_left_recursion.opt(item_to_node(item.node))
        elif isinstance(item, pegen.grammar.Gather):
            return remove_left_recursion.sep_rep1(item_to_node(item.node), item_to_node(item.separator))
        elif isinstance(item, pegen.grammar.Repeat0):
            return remove_left_recursion.repeat0(item_to_node(item.node))
        elif isinstance(item, pegen.grammar.Repeat1):
            return remove_left_recursion.repeat1(item_to_node(item.node))
        elif isinstance(item, pegen.grammar.Forced):
            return item_to_node(item.node)
        elif isinstance(item, pegen.grammar.PositiveLookahead):
            if include_lookaheads:
                return remove_left_recursion.lookahead(item_to_node(item.node))
            else:
                return remove_left_recursion.eps()
        elif isinstance(item, pegen.grammar.NegativeLookahead):
            if include_lookaheads:
                return remove_left_recursion.negative_lookahead(item_to_node(item.node))
            else:
                return remove_left_recursion.eps()
        elif isinstance(item, pegen.grammar.Rhs):
            return rhs_to_node(item)
        elif isinstance(item, pegen.grammar.Cut):
            return remove_left_recursion.eps()
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    rules = {}
    for name, rule in grammar.rules.items():
        if not (omit_invalid and name.startswith('invalid_')):
            rules[remove_left_recursion.ref(name)] = rhs_to_node(rule.rhs)
    return rules


def custom_to_pegen(rules: dict[remove_left_recursion.Ref, remove_left_recursion.Node]) -> pegen.grammar.Grammar:
    def node_to_rhs(node: remove_left_recursion.Node) -> pegen.grammar.Rhs:
        if isinstance(node, remove_left_recursion.Choice):
            return pegen.grammar.Rhs([node_to_alt(child) for child in node.children])
        return pegen.grammar.Rhs([node_to_alt(node)])

    def node_to_alt(node: remove_left_recursion.Node) -> pegen.grammar.Alt:
        if isinstance(node, remove_left_recursion.Seq):
            assert len(node.children) > 0
            return pegen.grammar.Alt(
                [pegen.grammar.NamedItem(None, node_to_item(child)) for child in node.children])
        return pegen.grammar.Alt([pegen.grammar.NamedItem(None, node_to_item(node))])

    def node_to_item(node: remove_left_recursion.Node):
        if isinstance(node, remove_left_recursion.Term):
            return pegen.grammar.StringLeaf(node.value)
        elif isinstance(node, remove_left_recursion.Ref):
            return pegen.grammar.NameLeaf(node.name)
        elif isinstance(node, remove_left_recursion.Lookahead):
            if node.positive:
                return pegen.grammar.PositiveLookahead(node_to_item(node.child))
            else:
                return pegen.grammar.NegativeLookahead(node_to_item(node.child))
        elif isinstance(node, remove_left_recursion.Seq):
            assert len(node.children) > 0
            return pegen.grammar.Group(node_to_rhs(node))
        elif isinstance(node, remove_left_recursion.Choice):
            if remove_left_recursion.eps() in node.children:
                children = node.children.copy()
                children.remove(remove_left_recursion.eps())
                return pegen.grammar.Opt(node_to_rhs(remove_left_recursion.Choice(children)))
            assert len(node.children) > 0
            return pegen.grammar.Group(node_to_rhs(node))
        elif isinstance(node, remove_left_recursion.Repeat1):
            return pegen.grammar.Repeat1(node_to_item(node.child))
        elif isinstance(node, remove_left_recursion.SepRep1):
            return pegen.grammar.Gather(node_to_item(node.separator), node_to_item(node.child))
        else:
            raise ValueError(f"Unknown node type: {type(node)}")

    pegen_rules = {}
    for ref, node in rules.items():
        pegen_rules[ref.name] = pegen.grammar.Rule(ref.name, None, node_to_rhs(node.simplify()))
    return pegen.grammar.Grammar(pegen_rules.values(), {})


def grammar_to_rust(
        grammar: pegen.grammar.Grammar,
        unresolved_follows_table: dict[remove_left_recursion.Ref, list[remove_left_recursion.Ref]]
) -> str:
    def rhs_to_rust(rhs: pegen.grammar.Rhs, top_level: bool = False) -> str:
        if len(rhs.alts) == 1:
            return alt_to_rust(rhs.alts[0], top_level=top_level)
        if top_level:
            return "choice!(\n    " + ",\n    ".join(alt_to_rust(alt) for alt in rhs.alts) + "\n)"
        else:
            return "choice!(" + ", ".join(alt_to_rust(alt) for alt in rhs.alts) + ")"

    def alt_to_rust(alt: pegen.grammar.Alt, top_level: bool = False) -> str:
        if len(alt.items) == 1:
            return named_item_to_rust(alt.items[0])
        if top_level and len(alt.items) > 4:
            return "seq!(\n    " + ",\n     ".join(named_item_to_rust(item) for item in alt.items) + "\n)"
        else:
            s = "seq!(" + ", ".join(named_item_to_rust(item) for item in alt.items) + ")"
            return s

    def named_item_to_rust(item: pegen.grammar.NamedItem) -> str:
        return item_to_rust(item.item)

    def item_to_rust(item) -> str:
        if isinstance(item, pegen.grammar.NameLeaf):
            value = item.value
            return name_to_rust(value)
        elif isinstance(item, pegen.grammar.StringLeaf):
            value = item.value
            if value[0] == value[-1] in {'"', "'"}:
                value = value[1:-1]
            else:
                raise ValueError(f"Invalid string literal: {value}")
            return f'python_literal("{value}")'
        elif isinstance(item, pegen.grammar.Group):
            logging.warning(f"Passing through group: {item}")
            return item_to_rust(item.rhs)
        elif isinstance(item, pegen.grammar.Opt):
            return f'opt({item_to_rust(item.node)})'
        elif isinstance(item, pegen.grammar.Gather):
            return f'seprep1({item_to_rust(item.node)}, {item_to_rust(item.separator)})'
        elif isinstance(item, pegen.grammar.Repeat0):
            return f'repeat0({item_to_rust(item.node)})'
        elif isinstance(item, pegen.grammar.Repeat1):
            return f'repeat1({item_to_rust(item.node)})'
        elif isinstance(item, pegen.grammar.Forced):
            logging.warning(f"Passing through forced: {item}")
            return item_to_rust(item.node)
        elif isinstance(item, pegen.grammar.PositiveLookahead):
            return f"lookahead({item_to_rust(item.node)})"
        elif isinstance(item, pegen.grammar.NegativeLookahead):
            return f"negative_lookahead({item_to_rust(item.node)})"
        elif isinstance(item, pegen.grammar.Rhs):
            return rhs_to_rust(item)
        elif isinstance(item, pegen.grammar.Cut):
            return 'cut()'
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    def name_to_rust(name: str) -> str:
        return f'&{name}'

    rules = grammar.rules.items()
    rules = list(reversed(rules))

    tokens = ['WS', 'NAME', 'TYPE_COMMENT', 'FSTRING_START', 'FSTRING_MIDDLE', 'FSTRING_END', 'NUMBER', 'STRING',
              'NEWLINE', 'INDENT', 'DEDENT', 'ENDMARKER']

    f = io.StringIO()
    f.write('use std::rc::Rc;\n')
    f.write(
        'use crate::{cache_context, cached, symbol, Symbol, Choice, deferred, Combinator, CombinatorTrait, eat_char_choice, eat_char_range, eat_string, eps, Eps, forbid_follows, forbid_follows_check_not, forbid_follows_clear, Repeat1, Seq, tag, lookahead, negative_lookahead};\n')
    f.write('use super::python_tokenizer::python_literal;\n')
    f.write('use crate::seq;\n')
    f.write(
        'use crate::{' + ', '.join(f'{name}_greedy as {name}' for name in ['opt', 'choice', 'seprep0', 'seprep1', 'repeat0', 'repeat1']) + '};\n')
    f.write('\n')

    f.write('enum Forbidden {\n')
    for token in tokens:
        f.write(f'    {token},\n')
    f.write('}\n')
    f.write('\n')

    def make_tokens() -> str:
        f = io.StringIO()
        f.write('use super::python_tokenizer as token;\n')
        for token in tokens:
            expr = f'token::{token}()'
            expr = f'{expr}.compile()'

            token_ref = remove_left_recursion.ref(token)
            if token_ref in unresolved_follows_table and any(
                    token_ref in forbidden_follow_set for forbidden_follow_set in
                    unresolved_follows_table.values()):
                expr = f'seq!(forbid_follows_check_not(Forbidden::{token} as usize), {expr}, forbid_follows(&[{", ".join(f"Forbidden::{ref.name} as usize" for ref in unresolved_follows_table.get(token_ref, []))}]))'
            elif token_ref in unresolved_follows_table:
                expr = f'seq!({expr}, forbid_follows(&[{", ".join(f"Forbidden::{ref.name} as usize" for ref in unresolved_follows_table.get(token_ref, []))}]))'
            elif any(token_ref in forbidden_follow_set for forbidden_follow_set in unresolved_follows_table.values()):
                expr = f'seq!(forbid_follows_check_not(Forbidden::{token} as usize), {expr})'
            else:
                expr = f'seq!(forbid_follows_clear(), {expr})'
            expr = f'crate::profile("{token}", {expr})'
            expr = f'tag("{token}", {expr})'
            if remove_left_recursion.ref('WS') not in unresolved_follows_table.get(token_ref, set()):
                expr = f'seq!({expr}, opt(&WS))'
            expr = f'cached({expr})'
            f.write('fn ' + token + '() -> Combinator { ' + expr + '.into() }\n')
        f.write('\n')
        return f.getvalue()

    def make_rules() -> str:
        f = io.StringIO()
        for name, rule in rules:
            expr = rhs_to_rust(rule.rhs, top_level=True)
            expr = f'tag("{name}", {expr})'
            if rule.memo:
                expr = f'cached({expr})'
            expr = f'{expr}.into()'
            f.write('fn ' + name + '() -> Combinator {\n')
            f.write(f'{textwrap.indent(expr, "    ")}\n')
            f.write('}\n')
            f.write('\n')
        f.write('\n')
        return f.getvalue()

    f.write(make_tokens())
    f.write(make_rules())

    f.write('pub fn python_file() -> Combinator {\n')
    expr = f'seq!(opt({name_to_rust("NEWLINE")}), {name_to_rust("file")})'
    expr = f'tag("main", {expr})'
    expr = f'cache_context({expr})'
    f.write(f'\n    {expr}.compile()\n')
    f.write('}\n')
    return f.getvalue()


def save_grammar_to_rust(grammar: pegen.grammar.Grammar, filename: str,
                         unresolved_follows_table: dict[remove_left_recursion.Ref, list[
                             remove_left_recursion.Ref]]) -> None:
    rust_code = grammar_to_rust(grammar, unresolved_follows_table)
    with open(filename, 'w') as f:
        f.write(rust_code)


if __name__ == "__main__":
    grammar_url = "https://raw.githubusercontent.com/python/cpython/main/Grammar/python.gram"
    grammar_text = fetch_grammar(grammar_url)
    pegen_grammar = parse_grammar(grammar_text)

    custom_grammar = pegen_to_custom(pegen_grammar)

    custom_grammar = remove_left_recursion.resolve_left_recursion(custom_grammar)

    forbidden_follows_table = {
        ref('FSTRING_START'): {ref('WS'), ref('NEWLINE')},
        ref('FSTRING_MIDDLE'): {ref('FSTRING_MIDDLE'), ref('WS')},
        ref('NEWLINE'): {ref('WS')},
        ref('INDENT'): {ref('WS')},
        ref('DEDENT'): {ref('WS')},
        ref('NAME'): {ref('NAME'), ref('NUMBER')},
        ref('NUMBER'): {ref('NUMBER')},
        ref('WS'): {ref('WS'), ref('NEWLINE'), ref('INDENT'), ref('DEDENT')},
    }

    remove_left_recursion.prettify_rules(custom_grammar)

    resolved_pegen_grammar = custom_to_pegen(custom_grammar)

    for rule_name in resolved_pegen_grammar.rules:
        resolved_pegen_grammar.rules[rule_name].memo = pegen_grammar.rules[rule_name].memo

    save_grammar_to_rust(resolved_pegen_grammar, 'python_grammar.rs', forbidden_follows_table)