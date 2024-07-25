import io
import logging
import textwrap
import tokenize
from io import StringIO
from pprint import pprint
from typing import Dict

import requests
import pegen.grammar
from ansiwrap import ansilen
from pegen.grammar_parser import GeneratedParser
from pegen.tokenizer import Tokenizer

import remove_left_recursion
from remove_left_recursion import get_nullable_rules, get_firsts
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

def pegen_to_custom(grammar: pegen.grammar.Grammar, ignore_invalid: bool = True) -> dict[remove_left_recursion.Ref, remove_left_recursion.Node]:
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
            if ignore_invalid and value.startswith('invalid_'):
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
            return remove_left_recursion.sep1(item_to_node(item.node), item_to_node(item.separator))
        elif isinstance(item, pegen.grammar.Repeat0):
            return remove_left_recursion.repeat0(item_to_node(item.node))
        elif isinstance(item, pegen.grammar.Repeat1):
            return remove_left_recursion.repeat1(item_to_node(item.node))
        elif isinstance(item, pegen.grammar.Forced):
            return item_to_node(item.node)
        elif isinstance(item, pegen.grammar.PositiveLookahead):
            return remove_left_recursion.eps()
        elif isinstance(item, pegen.grammar.NegativeLookahead):
            return remove_left_recursion.eps()
        elif isinstance(item, pegen.grammar.Rhs):
            return rhs_to_node(item)
        elif isinstance(item, pegen.grammar.Cut):
            # return remove_left_recursion.eps_external(item)
            return remove_left_recursion.eps()
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    rules = {}
    for name, rule in grammar.rules.items():
        ref = remove_left_recursion.ref(name)
        if not (ignore_invalid and ref.name.startswith('invalid_')):
            rules[ref] = rhs_to_node(rule.rhs)
    return rules

def custom_to_pegen(rules: dict[remove_left_recursion.Ref, remove_left_recursion.Node]) -> pegen.grammar.Grammar:
    def node_to_rhs(node: remove_left_recursion.Node) -> pegen.grammar.Rhs:
        if isinstance(node, remove_left_recursion.Choice):
            return pegen.grammar.Rhs([node_to_alt(child) for child in node.children])
        return pegen.grammar.Rhs([node_to_alt(node)])

    def node_to_alt(node: remove_left_recursion.Node) -> pegen.grammar.Alt:
        if isinstance(node, remove_left_recursion.Seq):
            assert len(node.children) > 0
            return pegen.grammar.Alt([pegen.grammar.NamedItem(None, node_to_item(child)) for child in node.children])
        return pegen.grammar.Alt([pegen.grammar.NamedItem(None, node_to_item(node))])

    def node_to_item(node: remove_left_recursion.Node):
        if isinstance(node, remove_left_recursion.Term):
            return pegen.grammar.StringLeaf(node.value)
        elif isinstance(node, remove_left_recursion.Ref):
            return pegen.grammar.NameLeaf(node.name)
        elif isinstance(node, remove_left_recursion.EpsExternal):
            return node.data
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
        else:
            raise ValueError(f"Unknown node type: {type(node)}")

    pegen_rules = {}
    for ref, node in rules.items():
        pegen_rules[ref.name] = pegen.grammar.Rule(ref.name, None, node_to_rhs(node.simplify()))
    return pegen.grammar.Grammar(pegen_rules.values(), {})

def grammar_to_rust(grammar: pegen.grammar.Grammar) -> str:
    def rhs_to_rust(rhs: pegen.grammar.Rhs, top_level: bool = False) -> str:
        if len(rhs.alts) == 1:
            return alt_to_rust(rhs.alts[0])
        if top_level:
            return "choice!(\n        " + ",\n        ".join(alt_to_rust(alt) for alt in rhs.alts) + "\n    )"
        else:
            return "choice!(" + ", ".join(alt_to_rust(alt) for alt in rhs.alts) + ")"

    def alt_to_rust(alt: pegen.grammar.Alt) -> str:
        if len(alt.items) == 1:
            return named_item_to_rust(alt.items[0])
        return "seq!(" + ", ".join(named_item_to_rust(item) for item in alt.items) + ")"

    def named_item_to_rust(item: pegen.grammar.NamedItem) -> str:
        return item_to_rust(item.item)

    def item_to_rust(item) -> str:
        if isinstance(item, pegen.grammar.NameLeaf):
            value = item.value
            return f'&{value}'
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
            return f'sep1!({item_to_rust(item.node)}, {item_to_rust(item.separator)})'
        elif isinstance(item, pegen.grammar.Repeat0):
            return f'repeat0({item_to_rust(item.node)})'
        elif isinstance(item, pegen.grammar.Repeat1):
            return f'repeat1({item_to_rust(item.node)})'
        elif isinstance(item, pegen.grammar.Forced):
            logging.warning(f"Passing through forced: {item}")
            return item_to_rust(item.node)
        elif isinstance(item, pegen.grammar.PositiveLookahead):
            logging.warning(f"Doing nothing with positive lookahead: {item}")
            return "eps()"
        elif isinstance(item, pegen.grammar.NegativeLookahead):
            logging.warning(f"Doing nothing with negative lookahead: {item}")
            return "eps()"
        elif isinstance(item, pegen.grammar.Rhs):
            return rhs_to_rust(item)
        elif isinstance(item, pegen.grammar.Cut):
            return 'cut()'
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    rules = grammar.rules.items()
    rules = list(reversed(rules))

    tokens = ['WS', 'NAME', 'TYPE_COMMENT', 'FSTRING_START', 'FSTRING_MIDDLE', 'FSTRING_END', 'NUMBER', 'STRING', 'NEWLINE', 'INDENT', 'DEDENT', 'ENDMARKER']

    f = io.StringIO()
    f.write('use std::rc::Rc;\n')
    f.write('use crate::{choice, opt, eat_char_choice, eat_string, eat_char_range, forward_ref, eps, cut, tag, cached, cache_context, prevent_consecutive_matches, DynCombinator, CombinatorTrait, forward_decls, seprep0, seprep1, IntoCombinator, Seq2, Choice2, Repeat1, Eps};\n')
    f.write('use super::python_tokenizer::{' + ", ".join(tokens) + '};\n')
    f.write('use super::python_tokenizer::python_literal;\n')
    f.write('use crate::{seq, repeat0, repeat1};\n')
    f.write('\n')
    f.write('pub fn python_file() -> Rc<DynCombinator> {\n')
    for token in tokens:
        expr = f'{token}()'
        expr = f'tag("{token}", {expr})'
        expr = f'{expr}.into_rc_dyn()'
        f.write(f"    let {token} = {expr};\n")
    f.write('\n')
    f.write(f'    forward_decls!({", ".join(name for name, rule in rules)});\n')
    f.write('\n')
    for name, rule in rules:
        expr = rhs_to_rust(rule.rhs, top_level=True)
        expr = f'tag("{name}", {expr})'
        if rule.memo:
            expr = f'cached({expr})'
        f.write(f'    let {name} = {name}.set({expr}).into_rc_dyn();\n')
    if any(rule.memo for name, rule in rules):
        f.write('\n    cache_context(seq!(repeat0(NEWLINE), file)).into_rc_dyn()\n')
    else:
        f.write('\n    seq!(repeat0(NEWLINE), file).into_rc_dyn()\n')
    f.write('}\n')
    return f.getvalue()

def save_grammar_to_rust(grammar: pegen.grammar.Grammar, filename: str) -> None:
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

    # Convert to custom grammar format
    custom_grammar = pegen_to_custom(pegen_grammar)
    remove_left_recursion.validate_rules(custom_grammar)
    remove_left_recursion.prettify_rules(custom_grammar)

    # Remove left recursion
    custom_grammar = remove_left_recursion.resolve_left_recursion(custom_grammar)

    # Intersperse opt(WS)
    custom_grammar |= remove_left_recursion.intersperse_separator(custom_grammar, opt(ref('WS')))

    # Forbid some follows
    forbidden_follows_table = {
        ref('FSTRING_START'): {ref('WS')},
        ref('FSTRING_MIDDLE'): {ref('FSTRING_MIDDLE'), ref('WS')},
        ref('NEWLINE'): {ref('WS')},
        ref('INDENT'): {ref('WS')},
        ref('DEDENT'): {ref('WS')},
        ref('NAME'): {ref('NAME')},
        ref('NUMBER'): {ref('NUMBER')},
        ref('WS'): {ref('WS')},
        # ref('WS'): {ref('WS'), ref('NEWLINE'), ref('INDENT'), ref('DEDENT')},
    }
    custom_grammar |= remove_left_recursion.forbid_follows(custom_grammar, forbidden_follows_table)

    # Convert back to pegen format
    resolved_pegen_grammar = custom_to_pegen(custom_grammar)

    # Restore memo flags
    for rule_name in resolved_pegen_grammar.rules:
        resolved_pegen_grammar.rules[rule_name].memo = pegen_grammar.rules[rule_name].memo

    # Save to Rust
    save_grammar_to_rust(resolved_pegen_grammar, 'python_grammar.rs')

    # Print some useful stats
    nullable_rules = get_nullable_rules(custom_grammar)
    firsts_by_rule = {ref: {first for first in get_firsts(node, nullable_rules)} for ref, node in custom_grammar.items()}
    for ref, firsts in firsts_by_rule.items():
        refs = [ref for ref in firsts if isinstance(ref, remove_left_recursion.Ref)]
        terms = [term for term in firsts if isinstance(term, remove_left_recursion.Term)]
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
    queue = [remove_left_recursion.Ref('file')]
    while len(queue) > 0:
        ref = queue.pop()
        active_count.setdefault(ref, 0)
        active_count[ref] += 1
        if ref in custom_grammar:
            for ref in remove_left_recursion.first_refs(custom_grammar[ref], nullable_rules):
                queue.append(ref)

    print("Number of rules active at the first step:")
    for ref, count in sorted(active_count.items(), key=lambda x: x[1]):
        print(f"  {ref.name}: {count}")

    # Print follow sets
    follows = remove_left_recursion.get_follows(custom_grammar)
    print(f"Follows:")
    for node, follow_set in sorted(follows.items(), key=lambda x: (type(x[0]), str(x[0]))):
        if node not in custom_grammar:
            # Assume such a node is a token
            follow_set = follow_set - set(custom_grammar.keys())
            refs = [ref for ref in follow_set if isinstance(ref, remove_left_recursion.Ref)]
            terms = [term for term in follow_set if isinstance(term, remove_left_recursion.Term)]
            other = [other for other in follow_set if not isinstance(other, remove_left_recursion.Term) and not isinstance(other, remove_left_recursion.Ref)]

            def ansi_ljust(s, width):
                needed = width - ansilen(s)
                if needed > 0:
                    return s + ' ' * needed
                else:
                    return s

            s = str(node) + ':'
            max_padding = 32
            s = ansi_ljust(s, max_padding)
            print(s, end="")
            padding = 0

            if len(refs) > 0:
                print(" " * padding, end="")
                print("refs : ", end="")
                print(", ".join(f"\033[31m{ref.name}\033[0m" for ref in refs))
                padding = max_padding
            if len(terms) > 0:
                print(" " * padding, end="")
                print("terms: ", end="")
                print(", ".join(f"\033[32m{term.value}\033[0m" for term in terms))
                padding = max_padding
            if len(other) > 0:
                print(" " * padding, end="")
                print("other: ", end="")
                print(", ".join(f"\033[33m{other}\033[0m" for other in sorted(other)))
                padding = max_padding
            if len(terms) == len(refs) == len(other) == 0:
                print()

            if isinstance(node, remove_left_recursion.Ref) and node in follow_set:
                logging.warning(f"Ref can follow itself: {node}")