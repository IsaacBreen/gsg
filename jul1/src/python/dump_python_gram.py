import io
import json
import logging
import tokenize
from io import StringIO

import requests
from pegen.grammar import Grammar, Rhs, Alt, NamedItem, Leaf, NameLeaf, StringLeaf, Group, Opt, Repeat, Forced, Lookahead, \
    PositiveLookahead, NegativeLookahead, Repeat0, Repeat1, Gather, Cut, Rule, Item
from pegen.grammar_parser import GeneratedParser
from pegen.tokenizer import Tokenizer


def fetch_grammar(url: str) -> str:
    response = requests.get(url)
    response.raise_for_status()
    return response.text


def parse_grammar(text: str) -> Grammar:
    # Assume `parse_string` is the function to parse grammar text into a Grammar object
    # You might need to implement this function based on how pegen parses the grammar text
    # from pegen.parser import parse_string
    with StringIO(text) as f:
        tokenizer = Tokenizer(tokenize.generate_tokens(f.readline))
        parser = GeneratedParser(tokenizer)
        grammar = parser.start()
        return grammar


def grammar_to_dict(grammar: Grammar) -> dict:
    def rhs_to_dict(rhs: Rhs) -> dict:
        return {
            "node_type": "rhs",
            "alts": [alt_to_dict(alt) for alt in rhs.alts],
        }

    def alt_to_dict(alt: Alt) -> dict:
        return {
            "node_type": "alt",
            "items": [named_item_to_dict(item) for item in alt.items],
            "icut": alt.icut,
            "action": alt.action
        }

    def named_item_to_dict(item: NamedItem) -> dict:
        return {
            "node_type": "named_item",
            "name": item.name,
            "item": item_to_dict(item.item),
            "type": item.type,
        }

    def item_to_dict(item) -> dict:
        if isinstance(item, Leaf):
            return {"node_type": "leaf", "value": item.value}
        elif isinstance(item, NameLeaf):
            return {"node_type": "name_leaf", "value": item.value}
        elif isinstance(item, StringLeaf):
            return {"node_type": "string_leaf", "value": item.value}
        elif isinstance(item, Group):
            return {"node_type": "group", "rhs": rhs_to_dict(item.rhs)}
        elif isinstance(item, Opt):
            return {"node_type": "opt", "node": item_to_dict(item.node)}
        elif isinstance(item, Gather):
            return {
                "node_type": "gather",
                "separator": item_to_dict(item.separator),
                "node": item_to_dict(item.node),
            }
        elif isinstance(item, Repeat):
            return {
                "node_type": "repeat",
                "node": item_to_dict(item.node),
            }
        elif isinstance(item, Repeat0):
            return {
                "node_type": "repeat0",
                "node": item_to_dict(item.node),
            }
        elif isinstance(item, Repeat1):
            return {
                "node_type": "repeat1",
                "node": item_to_dict(item.node),
            }
        elif isinstance(item, Forced):
            return {"node_type": "forced", "node": item_to_dict(item.node)}
        elif isinstance(item, Lookahead):
            return {
                "node_type": "lookahead",
                "node": item_to_dict(item.node),
                "sign": item.sign
            }
        elif isinstance(item, PositiveLookahead):
            return {
                "node_type": "positive_lookahead",
                "node": item_to_dict(item.node),
            }
        elif isinstance(item, NegativeLookahead):
            return {
                "node_type": "negative_lookahead",
                "node": item_to_dict(item.node),
            }
        elif isinstance(item, Rhs):
            return rhs_to_dict(item)
        elif isinstance(item, Cut):
            return {"node_type": "cut"}
        else:
            raise ValueError(f"Unknown item type: {type(item)}")


    def rule_to_dict(name: str, rule: Rule) -> dict:
        return {
            "name": name,
            "type": rule.type,
            "rhs": rhs_to_dict(rule.rhs),
            "visited": rule.visited,
            "nullable": rule.nullable,
            "left_recursive": rule.left_recursive,
            "leader": rule.leader
        }

    return {
        "rules": [rule_to_dict(name, rule) for name, rule in grammar.rules.items()],
        "metas": {meta: grammar.metas[meta] for meta in grammar.metas}
    }


def dict_to_grammar(grammar_dict: dict) -> Grammar:
    def dict_to_rhs(rhs_dict: dict) -> Rhs:
        return Rhs(alts=[dict_to_alt(alt) for alt in rhs_dict["alts"]])

    def dict_to_alt(alt_dict: dict) -> Alt:
        return Alt(items=[dict_to_named_item(item) for item in alt_dict["items"]],
                   icut=alt_dict["icut"],
                   action=alt_dict["action"])

    def dict_to_named_item(item_dict: dict) -> NamedItem:
        return NamedItem(name=item_dict["name"],
                         item=dict_to_item(item_dict["item"]),
                         type=item_dict["type"])

    def dict_to_item(item_dict: dict) -> Item:
        if "node_type" not in item_dict:
            raise ValueError(f"Item dict does not have node_type: {item_dict}")
        node_type = item_dict["node_type"]
        if node_type == "leaf":
            return Leaf(value=item_dict["value"])
        elif node_type == "name_leaf":
            return NameLeaf(value=item_dict["value"])
        elif node_type == "string_leaf":
            return StringLeaf(value=item_dict["value"])
        elif node_type == "group":
            return Group(rhs=dict_to_rhs(item_dict["rhs"]))
        elif node_type == "opt":
            return Opt(node=dict_to_item(item_dict["node"]))
        elif node_type == "gather":
            return Gather(separator=dict_to_item(item_dict["separator"]),
                          node=dict_to_item(item_dict["node"]))
        elif node_type == "repeat":
            return Repeat(node=dict_to_item(item_dict["node"]))
        elif node_type == "repeat0":
            return Repeat0(node=dict_to_item(item_dict["node"]))
        elif node_type == "repeat1":
            return Repeat1(node=dict_to_item(item_dict["node"]))
        elif node_type == "forced":
            return Forced(node=dict_to_item(item_dict["node"]))
        elif node_type == "lookahead":
            return Lookahead(node=dict_to_item(item_dict["node"]),
                             sign=item_dict["sign"])
        elif node_type == "positive_lookahead":
            return PositiveLookahead(node=dict_to_item(item_dict["node"]))
        elif node_type == "negative_lookahead":
            return NegativeLookahead(node=dict_to_item(item_dict["node"]))
        elif node_type == "rhs":
            return dict_to_rhs(item_dict)
        elif node_type == "cut":
            return Cut()
        else:
            raise ValueError(f"Unknown node type: {node_type}")

    def dict_to_rule(rule_dict: dict) -> Rule:
        return Rule(name=rule_dict["name"],
                    type=rule_dict["type"],
                    rhs=dict_to_rhs(rule_dict["rhs"]))

    rules = {rule_dict["name"]: dict_to_rule(rule_dict) for rule_dict in grammar_dict["rules"]}
    metas = grammar_dict["metas"]
    return Grammar(rules=rules.values(), metas=metas)


def load_grammar_from_json(filename: str) -> Grammar:
    with open(filename, 'r') as file:
        grammar_dict = json.load(file)
    return dict_to_grammar(grammar_dict)


def grammar_to_rust(grammar: Grammar) -> str:
    def rhs_to_rust(rhs: Rhs, top_level: bool = False) -> str:
        if top_level:
            return "choice!(" + ",\n        ".join(alt_to_rust(alt) for alt in rhs.alts) + ")"
        else:
            return "choice!(" + ", ".join(alt_to_rust(alt) for alt in rhs.alts) + ")"

    def alt_to_rust(alt: Alt) -> str:
        # return ' '.join(named_item_to_rust(item) for item in alt.items)
        return "seq!(" + ", ".join(named_item_to_rust(item) for item in alt.items) + ")"

    def named_item_to_rust(item: NamedItem) -> str:
        return item_to_rust(item.item)

    def item_to_rust(item) -> str:
        if isinstance(item, Leaf):
            value = item.value
            if value[0] == value[-1] == "'":
                value = value[1:-1]
                return f'eat_string("{value}")'
            elif value[0] == value[-1] == '"':
                value = value[1:-1]
                return f'eat_string("{value}")'
            else:
                return f'&{value}'
        elif isinstance(item, NameLeaf):
            value = item.value
            assert value[0] == value[-1] == '"'
            value = value[1:-1]
            return f'eat_string("{value}")'
        elif isinstance(item, StringLeaf):
            value = item.value
            assert value[0] == value[-1] == '"'
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
    f.write('\n'.join(f'    let mut {name} = forward_ref();' for name, rule in grammar.rules.items()))
    f.write('\n')
    f.write('\n'.join(f'    let {name} = {name}.set({rhs_to_rust(rule.rhs, top_level=True)});' for name, rule in grammar.rules.items()))
    f.write('\n    file.into_boxed().into()\n')
    f.write('}\n')
    return f.getvalue()


def save_grammar_to_json(grammar_dict: dict, filename: str) -> None:
    with open(filename, 'w') as file:
        json.dump(grammar_dict, file, indent=4)


def save_grammar_to_rust(grammar: Grammar, filename: str) -> None:
    rust_code = grammar_to_rust(grammar)
    with open(filename, 'w') as file:
        file.write(rust_code)


def main():
    url = "https://raw.githubusercontent.com/python/cpython/main/Grammar/python.gram"
    grammar_text = fetch_grammar(url)
    grammar = parse_grammar(grammar_text)
    grammar_dict = grammar_to_dict(grammar)
    save_grammar_to_json(grammar_dict, "python_grammar.json")
    grammar = dict_to_grammar(grammar_dict)
    save_grammar_to_rust(grammar, "python_grammar.rs")


if __name__ == "__main__":
    main()
