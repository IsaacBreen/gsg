import io
import logging
import tokenize
from io import StringIO
from pprint import pprint
from typing import Dict

import requests
from pegen.grammar import Grammar, Rhs, Alt, NamedItem, Leaf, NameLeaf, StringLeaf, Group, Opt, Repeat, Forced, Lookahead, \
    PositiveLookahead, NegativeLookahead, Repeat0, Repeat1, Gather, Cut, Rule
from pegen.grammar_parser import GeneratedParser
from pegen.tokenizer import Tokenizer

from remove_left_recursion import resolve_left_recursion, Node, Seq, Choice, Term, Ref, eps, fail, Repeat1, RuleType, validate_rules, \
    first_refs
from remove_left_recursion import get_nullable_rules, get_firsts

def fetch_grammar(url: str) -> str:
    response = requests.get(url)
    response.raise_for_status()
    return response.text

def parse_grammar(text: str) -> Grammar:
    with StringIO(text) as f:
        tokenizer = Tokenizer(tokenize.generate_tokens(f.readline))
        parser = GeneratedParser(tokenizer)
        grammar = parser.start()
        return grammar

def pegen_to_custom(grammar: Grammar) -> dict[Ref, Node]:
    def rhs_to_node(rhs: Rhs) -> Node:
        if len(rhs.alts) == 1:
            return alt_to_node(rhs.alts[0])
        return Choice([alt_to_node(alt) for alt in rhs.alts])

    def alt_to_node(alt: Alt) -> Node:
        if len(alt.items) == 1:
            return named_item_to_node(alt.items[0])
        return Seq([named_item_to_node(item) for item in alt.items])

    def named_item_to_node(item: NamedItem) -> Node:
        return item_to_node(item.item)

    def item_to_node(item) -> Node:
        if isinstance(item, NameLeaf):
            value = item.value
            if value.startswith('invalid_'):
                return fail()
            else:
                return Ref(value)
        elif isinstance(item, StringLeaf):
            value = item.value
            return Term(value)
        elif isinstance(item, Group):
            return rhs_to_node(item.rhs)
        elif isinstance(item, Opt):
            return Choice([item_to_node(item.node), eps()])
        elif isinstance(item, Gather):
            return Seq([item_to_node(item.node), Repeat1(item_to_node(item.separator))])
        elif isinstance(item, Repeat):
            return Repeat1(item_to_node(item.node))
        elif isinstance(item, Repeat0):
            return Choice([Repeat1(item_to_node(item.node)), eps()])
        elif isinstance(item, Repeat1):
            return Repeat1(item_to_node(item.node))
        elif isinstance(item, Forced):
            return item_to_node(item.node)
        elif isinstance(item, Lookahead):
            return eps()
        elif isinstance(item, PositiveLookahead):
            return eps()
        elif isinstance(item, NegativeLookahead):
            return eps()
        elif isinstance(item, Rhs):
            return rhs_to_node(item)
        elif isinstance(item, Cut):
            return eps()
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    rules = {}
    for name, rule in grammar.rules.items():
        ref = Ref(name)
        if not ref.name.startswith('invalid_'):
            rules[ref] = rhs_to_node(rule.rhs)
    return rules

def custom_to_pegen(rules: dict[Ref, Node]) -> Grammar:
    def node_to_rhs(node: Node) -> Rhs:
        if isinstance(node, Choice):
            return Rhs([node_to_alt(child) for child in node.children])
        return Rhs([node_to_alt(node)])

    def node_to_alt(node: Node) -> Alt:
        if isinstance(node, Seq):
            assert len(node.children) > 0
            return Alt([NamedItem(None, node_to_item(child)) for child in node.children])
        return Alt([NamedItem(None, node_to_item(node))])

    def node_to_item(node: Node):
        if isinstance(node, Term):
            return StringLeaf(node.value)
        elif isinstance(node, Ref):
            return NameLeaf(node.name)
        elif isinstance(node, Seq):
            assert len(node.children) > 0
            return Group(node_to_rhs(node))
        elif isinstance(node, Choice):
            if eps() in node.children:
                children = node.children.copy()
                children.remove(eps())
                return Opt(node_to_rhs(Choice(children)))
            assert len(node.children) > 0
            return Group(node_to_rhs(node))
        elif isinstance(node, Repeat1):
            return Repeat(node_to_item(node.child))
        else:
            raise ValueError(f"Unknown node type: {type(node)}")

    pegen_rules = {}
    for ref, node in rules.items():
        pegen_rules[ref.name] = Rule(ref.name, None, node_to_rhs(node))
    return Grammar(pegen_rules.values(), {})

def grammar_to_rust(grammar: Grammar) -> str:
    def rhs_to_rust(rhs: Rhs, top_level: bool = False) -> str:
        if top_level:
            return "choice!(\n        " + ",\n        ".join(alt_to_rust(alt) for alt in rhs.alts) + "\n    )"
        else:
            return "choice!(" + ", ".join(alt_to_rust(alt) for alt in rhs.alts) + ")"

    def alt_to_rust(alt: Alt) -> str:
        return "seq!(" + ", ".join(named_item_to_rust(item) for item in alt.items) + ")"

    def named_item_to_rust(item: NamedItem) -> str:
        return item_to_rust(item.item)

    def item_to_rust(item) -> str:
        if isinstance(item, NameLeaf):
            value = item.value
            return f'&{value}'
        elif isinstance(item, StringLeaf):
            value = item.value
            if value[0] == '"' and value[-1] == '"':
                value = value[1:-1]
            assert not value[0] == value[-1] == '"', f"Invalid string literal: {value}"
            value = value[1:-1]
            return f'eat_string("{value}")'
        elif isinstance(item, Group):
            logging.warning(f"Passing through group: {item}")
            return item_to_rust(item.rhs)
        elif isinstance(item, Opt):
            return f'opt({item_to_rust(item.node)})'
        elif isinstance(item, Gather):
            return f'seq!({item_to_rust(item.node)}, {item_to_rust(item.separator)})'
        elif isinstance(item, Repeat):
            return f'repeat({item_to_rust(item.node)})'
        elif isinstance(item, Repeat0):
            return f'repeat0({item_to_rust(item.node)})'
        elif isinstance(item, Repeat1):
            return f'repeat1({item_to_rust(item.node)})'
        elif isinstance(item, Forced):
            logging.warning(f"Passing through forced: {item}")
            return item_to_rust(item.node)
        elif isinstance(item, Lookahead):
            logging.warning(f"Doing nothing with lookahead: {item}")
            return "eps()"
        elif isinstance(item, PositiveLookahead):
            logging.warning(f"Doing nothing with positive lookahead: {item}")
            return "eps()"
        elif isinstance(item, NegativeLookahead):
            logging.warning(f"Doing nothing with negative lookahead: {item}")
            return "eps()"
        elif isinstance(item, Rhs):
            return rhs_to_rust(item)
        elif isinstance(item, Cut):
            logging.warning(f"Doing nothing with cut: {item}")
            return 'eps()'
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    rules = grammar.rules.items()
    rules = list(reversed(rules))

    tokens = ['NAME', 'TYPE_COMMENT', 'FSTRING_START', 'FSTRING_MIDDLE', 'FSTRING_END', 'NUMBER', 'STRING']

    f = io.StringIO()
    f.write('use std::rc::Rc;\n')
    f.write(
        'use crate::{choice, seq, repeat, repeat as repeat0, repeat1, opt, eat_char_choice, eat_string, eat_char_range, forward_ref, eps, python_newline, indent, dedent, DynCombinator, CombinatorTrait, symbol};\n'
    )
    f.write('use super::python_tokenizer::{' + ", ".join(tokens) + '};\n')
    f.write('\n')
    f.write('pub fn python_file() -> Rc<DynCombinator> {\n')
    for token in tokens:
        f.write(f"    let {token} = symbol({token}());\n")
    f.write("    let NEWLINE = symbol(python_newline());\n")
    f.write('    let INDENT = symbol(indent());\n')
    f.write('    let DEDENT = symbol(dedent());\n')
    f.write("    let ENDMARKER = symbol(eps());\n")
    f.write('\n')
    f.write('\n'.join(f'    let mut {name} = forward_ref();' for name, rule in rules))
    f.write('\n')
    # f.write('\n'.join(f'    let {name} = {name}.set({rhs_to_rust(rule.rhs, top_level=True)});' for name, rule in rules))
    f.write('\n'.join(f'    let {name} = Rc::new({name}.set({rhs_to_rust(rule.rhs, top_level=True)}).into_boxed());' for name, rule in rules))
    f.write('\n    file.into_boxed().into()\n')
    f.write('}\n')
    return f.getvalue()

def save_grammar_to_rust(grammar: Grammar, filename: str) -> None:
    rust_code = grammar_to_rust(grammar)
    with open(filename, 'w') as f:
        f.write(rust_code)

if __name__ == "__main__":
    # Fetch and parse the Python grammar
    grammar_url = "https://raw.githubusercontent.com/python/cpython/main/Grammar/python.gram"
    grammar_text = fetch_grammar(grammar_url)
    with open('python.gram', 'w') as f:
        f.write(grammar_text)
    pegen_grammar = parse_grammar(grammar_text)

    # Convert to custom grammar format and remove left recursion
    custom_grammar = pegen_to_custom(pegen_grammar)
    validate_rules(custom_grammar)
    resolved_grammar = resolve_left_recursion(custom_grammar)

    # Convert back to pegen format and save to Rust
    resolved_pegen_grammar = custom_to_pegen(resolved_grammar)
    save_grammar_to_rust(resolved_pegen_grammar, 'python_grammar.rs')

    # Print some useful stats
    nullable_rules = get_nullable_rules(resolved_grammar)
    firsts_by_rule = {ref: {first for first in get_firsts(node, nullable_rules)} for ref, node in resolved_grammar.items()}
    for ref, firsts in firsts_by_rule.items():
        refs = [ref for ref in firsts if isinstance(ref, Ref)]
        terms = [term for term in firsts if isinstance(term, Term)]
        # Pad the ref so firsts line up
        print(f"\033[31m{ref.name}\033[0m", end="")
        i = len(ref.name)
        PADDING = 40
        if len(refs) > 0:
            # Red
            print(" " * (PADDING - i), end="")
            print("refs:  ", end="")
            for ref in refs:
                print(f"\033[31m{ref.name}\033[0m, ", end=" ")
            print()
            i = 0
        if len(terms) > 0:
            # Green
            print(" " * (PADDING - i), end="")
            print("terms ", end="")
            for term in terms:
                print(f"\033[32m{term.value}\033[0m, ", end=" ")
            print()

    print(f"Nullable rules:")
    for ref in nullable_rules:
        print(f"  {ref.name}")

    # Print number of rules active at the first step, starting from 'file'
    active_count = {}
    queue = [Ref('file')]
    while len(queue) > 0:
        ref = queue.pop()
        active_count.setdefault(ref, 0)
        active_count[ref] += 1
        if ref in resolved_grammar:
            for ref in first_refs(resolved_grammar[ref], nullable_rules):
                queue.append(ref)

    print("Number of rules active at the first step:")
    for ref, count in sorted(active_count.items(), key=lambda x: x[1]):
        print(f"  {ref.name}: {count}")