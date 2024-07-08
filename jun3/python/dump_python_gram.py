import json
import requests
from pegen.grammar import Grammar, Rule, Rhs, Alt, NamedItem, Leaf, NameLeaf, StringLeaf, Group, Opt, Repeat, Forced, Lookahead, PositiveLookahead, NegativeLookahead, Repeat0, Repeat1, Gather, Cut
from pegen.grammar_parser import GeneratedParser
from pegen.tokenizer import Tokenizer
import tokenize
from io import StringIO

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

def save_grammar_to_json(grammar_dict: dict, filename: str) -> None:
    with open(filename, 'w') as file:
        json.dump(grammar_dict, file, indent=4)

def main():
    url = "https://raw.githubusercontent.com/python/cpython/main/Grammar/python.gram"
    grammar_text = fetch_grammar(url)
    grammar = parse_grammar(grammar_text)
    grammar_dict = grammar_to_dict(grammar)
    save_grammar_to_json(grammar_dict, "grammar.json")

if __name__ == "__main__":
    main()