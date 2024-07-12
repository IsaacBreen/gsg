import io
import json
import logging
import tokenize
from io import StringIO

import requests
from pegen.grammar import Grammar, Rhs, Alt, NamedItem, Leaf, NameLeaf, StringLeaf, Group, Opt, Repeat, Forced, Lookahead, \
    PositiveLookahead, NegativeLookahead, Repeat0, Repeat1, Gather, Cut
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
            "alts": [alt_to_dict(alt) for alt in rhs.alts],
            "memo": rhs.memo if rhs.memo else None
        }

    def alt_to_dict(alt: Alt) -> dict:
        return {
            "items": [named_item_to_dict(item) for item in alt.items],
            "icut": alt.icut,
            "action": alt.action
        }

    def named_item_to_dict(item: NamedItem) -> dict:
        return {
            "name": item.name,
            "item": item_to_dict(item.item),
            "type": item.type,
            "nullable": item.nullable
        }

    def item_to_dict(item) -> dict:
        if isinstance(item, Leaf):
            return {"value": item.value}
        elif isinstance(item, NameLeaf):
            return {"value": item.value}
        elif isinstance(item, StringLeaf):
            return {"value": item.value}
        elif isinstance(item, Group):
            return {"rhs": rhs_to_dict(item.rhs)}
        elif isinstance(item, Opt):
            return {"node": item_to_dict(item.node)}
        elif isinstance(item, Gather):
            return {
                "node": item_to_dict(item.node),
                "separator": item_to_dict(item.separator),
            }
        elif isinstance(item, Repeat):
            return {
                "node": item_to_dict(item.node),
                "memo": item.memo if item.memo else None
            }
        elif isinstance(item, Repeat0):
            return {
                "node": item_to_dict(item.node),
                "memo": item.memo if item.memo else None
            }
        elif isinstance(item, Repeat1):
            return {
                "node": item_to_dict(item.node),
                "memo": item.memo if item.memo else None
            }
        elif isinstance(item, Forced):
            return {"node": item_to_dict(item.node)}
        elif isinstance(item, Lookahead):
            return {
                "node": item_to_dict(item.node),
                "sign": item.sign
            }
        elif isinstance(item, PositiveLookahead):
            return {
                "node": item_to_dict(item.node),
                "sign": item.sign
            }
        elif isinstance(item, NegativeLookahead):
            return {
                "node": item_to_dict(item.node),
                "sign": item.sign
            }
        elif isinstance(item, Rhs):
            return rhs_to_dict(item)
        elif isinstance(item, Cut):
            return {}
        else:
            raise ValueError(f"Unknown item type: {type(item)}")

    return {
        "rules": {name: {
            "name": rule.name,
            "type": rule.type,
            "rhs": rhs_to_dict(rule.rhs),
            "memo": rule.memo,
            "visited": rule.visited,
            "nullable": rule.nullable,
            "left_recursive": rule.left_recursive,
            "leader": rule.leader
        } for name, rule in grammar.rules.items()},
        "metas": {meta: grammar.metas[meta] for meta in grammar.metas}
    }


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
    save_grammar_to_rust(grammar, "python_grammar.rs")


if __name__ == "__main__":
    main()
