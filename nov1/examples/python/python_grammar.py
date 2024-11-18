from __future__ import annotations

import io
import tokenize
from typing import List

import pegen.grammar
import pegen.grammar_parser
import pegen.tokenizer

from _sep1 import PyRegexExpr as Regex, PyGrammar, PyGrammarExpr as ge, PyGrammarConstraint, PyGrammarConstraintState

def eat_string(s: bytes) -> Regex:
    return Regex.seq([Regex.eat_u8(ord(c)) for c in s])

def pegen_to_sep1_regex(item: pegen.grammar.BaseGrammar, memo: dict) -> Regex:
    if isinstance(item, pegen.grammar.NameLeaf):
        return ge.regex(eat_string(item.value))
    elif isinstance(item, pegen.grammar.StringLeaf):
        value = item.value
        if value[0] == value[-1] in {'"', "'"}:
            value = value[1:-1]
        else:
            raise ValueError(f"Invalid string literal: {value}")
        return ge.regex(eat_string(value))
    elif isinstance(item, pegen.grammar.Opt):
        return ge.optional(pegen_to_sep1_regex(item.node, memo))
    elif isinstance(item, pegen.grammar.Gather):
        expr = pegen_to_sep1_regex(item.node, memo)
        sep = pegen_to_sep1_regex(item.separator, memo)
        return ge.sequence([expr, ge.repeat(ge.sequence([sep, expr]))])
    elif isinstance(item, pegen.grammar.Repeat0):
        return ge.repeat(pegen_to_sep1_regex(item.node, memo))
    elif isinstance(item, pegen.grammar.Repeat1):
        expr = pegen_to_sep1_regex(item.node, memo)
        return ge.sequence([expr, ge.repeat(expr)])
    elif isinstance(item, pegen.grammar.Group):
        return pegen_to_sep1_regex(item.rhs, memo)
    elif isinstance(item, pegen.grammar.Rhs):
        if len(item.alts) == 1:
            return pegen_to_sep1_regex(item.alts[0], memo)
        return ge.choice([pegen_to_sep1_regex(alt, memo) for alt in item.alts])
    elif isinstance(item, pegen.grammar.Alt):
        if len(item.items) == 1:
            return pegen_to_sep1_regex(item.items[0], memo)
        return ge.sequence([pegen_to_sep1_regex(named_item.item, memo) for named_item in item.items])
    elif isinstance(item, pegen.grammar.NamedItem):
        return pegen_to_sep1_regex(item.item, memo)
    elif isinstance(item, pegen.grammar.Forced):
        return pegen_to_sep1_regex(item.node, memo)
    elif isinstance(item, pegen.grammar.PositiveLookahead):
        # return ge.lookahead(pegen_to_sep1_regex(item.node, memo))
        return ge.sequence([])
    elif isinstance(item, pegen.grammar.NegativeLookahead):
        # return ge.negative_lookahead(pegen_to_sep1_regex(item.node, memo))
        return ge.sequence([])
    elif isinstance(item, pegen.grammar.Cut):
        # return ge.cut()
        return ge.sequence([])
    else:
        raise ValueError(f"Unknown item type: {type(item)}")

def pegen_to_sep1_grammar(grammar: pegen.grammar.Grammar) -> PyGrammar:
    memo = {}
    exprs = []
    for rule in grammar.rules.values():
        memo[rule.name] = ge.ref(rule.name)
        exprs.append((rule.name, pegen_to_sep1_regex(rule.rhs, memo)))

    return PyGrammar(exprs)

def define_python_grammar():
    with open("python.gram") as f:
        grammar_text = f.read()

    with io.StringIO(grammar_text) as f:
        tokenizer = pegen.tokenizer.Tokenizer(tokenize.generate_tokens(f.readline))
        parser = pegen.grammar_parser.GeneratedParser(tokenizer)
        grammar = parser.start()

    return pegen_to_sep1_grammar(grammar)

if __name__ == "__main__":
    grammar = define_python_grammar()
    print(grammar)