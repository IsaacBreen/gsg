from __future__ import annotations

import io
import time
import tokenize
from pathlib import Path
from typing import Any

import numpy as np
import pegen.grammar
import pegen.grammar_parser
import pegen.tokenizer
import torch
from _sep1 import PyRegexExpr as Regex, PyGrammar, PyGrammarExpr as ge, PyGrammarConstraint, PyGrammarConstraintState
from transformers import LogitsProcessor, AutoModelForCausalLM, AutoTokenizer


def eat_string(s: bytes) -> Regex:
    return Regex.seq([Regex.eat_u8(ord(c)) for c in s])

def pegen_to_sep1_regex(item: pegen.grammar.BaseGrammar, memo: dict) -> Regex:
    if isinstance(item, pegen.grammar.NameLeaf):
        return ge.ref(item.value)
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

def define_tokens() -> list[tuple[str, Any]]:
    tokens = {}

    choice = Regex.choice
    eat_u8 = Regex.eat_u8
    eat_u8_negation = Regex.eat_u8_negation
    seq = Regex.seq
    rep = Regex.rep
    eps = Regex.eps

    def eat_u8_choice(s):
        return choice([eat_u8(ord(c)) for c in s])

    ignore = rep(choice([
        eat_u8(ord(" ")),
        seq([eat_u8(ord("#")), rep(eat_u8_negation(ord("\n"))), eat_u8(ord("\n"))]),
    ]))

    def regex(expr):
        return ge.regex(seq([ignore, expr]))
#         return ge.regex(expr)

    digit = choice([eat_u8(c) for c in range(ord("0"), ord("9") + 1)])
    alph_lower = choice([eat_u8(c) for c in range(ord("a"), ord("z") + 1)])
    alph_upper = choice([eat_u8(c) for c in range(ord("A"), ord("Z") + 1)])

    name_start = choice([
        alph_lower,
        alph_upper,
        eat_u8(ord("_"))
    ])
    name_middle = choice([
        name_start,
        digit,
    ])

    tokens["NAME"] = seq([name_start, rep(name_middle)])
    tokens["NUMBER"] = choice([
        rep(digit),
        seq([rep(digit), eat_u8(ord(".")), rep(digit)]),
    ])
    tokens["NEWLINE"] = eps()
    tokens["INDENT"] = eps()
    tokens["DEDENT"] = eps()
    tokens["STRING"] = choice([
        seq([eat_u8(ord('"')), rep(eat_u8_negation(ord('"'))), eat_u8(ord('"'))]),
        seq([eat_u8(ord("'")), rep(eat_u8_negation(ord("'"))), eat_u8(ord("'"))]),
    ])
    tokens["FSTRING_START"] = choice([
        eat_string('"""'),
        eat_string("'''"),
    ])
    tokens["FSTRING_END"] = choice([
        eat_string('"""'),
        eat_string("'''"),
    ])
    tokens["FSTRING_MIDDLE"] = rep(choice([
        eat_u8_negation(ord("{")),
        eat_string("{{"),
    ]))
    tokens["TYPE_COMMENT"] = eps()
    tokens["ENDMARKER"] = eps()
    return [(name, regex(expr)) for name, expr in tokens.items()]

def pegen_to_sep1_grammar(grammar: pegen.grammar.Grammar) -> PyGrammar:
    memo = {}
    exprs: list[tuple[str, Any]] = []

    # Make sure the start production is first
    # exprs.append(("start", ))
    # TODO: remove this
    temp = "NUMBER"
    # exprs.append(( "start'", ge.ref(temp)))

    # for rule in grammar.rules.values():
    #     memo[rule.name] = ge.ref(rule.name)
    #     exprs.append((rule.name, pegen_to_sep1_regex(rule.rhs, memo)))

    tokens = define_tokens()
    exprs.extend(tokens)
#     # TODO: remove this
#     for (name, expr) in tokens:
#         if name in [temp]:
#             exprs.append((name, expr))
#         else:
#             exprs.append((name, ge.regex(Regex.eps())))

    # todo: remove this
#     exprs = [("start", ge.regex(Regex.eat_u8(ord("a"))))]
    exprs = [("start", dict(tokens)["NAME"])]

    return PyGrammar(exprs)

def define_python_grammar():
    with Path(__file__).parent / "python.gram" as f:
        grammar_text = f.read_text()

    with io.StringIO(grammar_text) as f:
        tokenizer = pegen.tokenizer.Tokenizer(tokenize.generate_tokens(f.readline))
        parser = pegen.grammar_parser.GeneratedParser(tokenizer)
        grammar = parser.start()

    return pegen_to_sep1_grammar(grammar)

def debug_print(message):
    print(message, end='; ')

def timeit(func):
    def wrapper(*args, **kwargs):
        start_time = time.time()
        result = func(*args, **kwargs)
        end_time = time.time()
        debug_print(f"Time taken: {(end_time - start_time) * 1000:.2f} ms")
        return result
    return wrapper

class GrammarConstrainedLogitsProcessor(LogitsProcessor):
    def __init__(self, grammar_constraint_state, llm_token_to_id):
        self.grammar_constraint_state = grammar_constraint_state
        self.seen_input_ids = []
        self.llm_token_to_id = llm_token_to_id
        self.llm_token_id_to_token = {id: token for token, id in llm_token_to_id.items()}

    def __call__(self, input_ids, scores):
        current_input_ids = input_ids.view(-1).tolist()
        new_token_ids = current_input_ids[len(self.seen_input_ids):]

        for token_id in new_token_ids:
#             debug_print(f"Committing token: {self.llm_token_to_id[token_id]} (ID: {token_id})")
            debug_print(f"Committing token: {self.llm_token_id_to_token.get(token_id)} (ID: {token_id})")
            timeit(self.grammar_constraint_state.commit)(token_id)

        self.seen_input_ids = current_input_ids
        mask = timeit(self.grammar_constraint_state.get_mask)()

        if len(mask) < scores.shape[-1]:
            padding = np.zeros(scores.shape[-1] - len(mask), dtype=bool)
            mask = np.concatenate((mask, padding))
        elif len(mask) > scores.shape[-1]:
            mask = mask[:scores.shape[-1]]

        mask_ids = np.where(mask)[0]
        mask_id_map = {id: self.llm_token_id_to_token.get(id) for id in mask_ids}
        debug_print(f"Mask IDs: {mask_id_map}")
        print("")

        scores = np.where(mask, scores, -np.inf)
        return torch.tensor(scores)

def initialize_grammar_constraint(grammar, llm_token_to_id, max_token_id):
    print("Initializing PyGrammarConstraint...")
    grammar_constraint = PyGrammarConstraint(grammar, llm_token_to_id, max_token_id)
#     grammar_constraint.print()
    print("Initializing Grammar Constraint State...")
    grammar_constraint_state = PyGrammarConstraintState(grammar_constraint)
    print("Getting Initial Mask...")
    initial_mask = grammar_constraint_state.get_mask()
    initial_mask_ids = np.where(initial_mask)[0]
    llm_token_id_to_token = {id: token for token, id in llm_token_to_id.items()}
    initial_mask_id_map = {id: llm_token_id_to_token.get(id) for id in initial_mask_ids}
    print(f"Initial Mask IDs: {initial_mask_id_map}")
    assert len(initial_mask_id_map) > 0, f"Initial mask is empty: {initial_mask}"
    return grammar_constraint_state

def generate_text(model, tokenizer, grammar_processor, input_text, max_new_tokens=50):
    input_ids = tokenizer.encode(input_text, return_tensors="pt")
    grammar_processor.seen_input_ids = input_ids[0].tolist()
    output = model.generate(
        input_ids,
        max_new_tokens=max_new_tokens,
        logits_processor=[grammar_processor]
    )
    return tokenizer.decode(output[0], skip_special_tokens=True)

if __name__ == "__main__":
#     model_name = "Qwen/Qwen2.5-Coder-0.5B"
    model_name = "gpt2"
    tokenizer = AutoTokenizer.from_pretrained(model_name)

#     llm_tokens = [x.encode() for x in ['a', ' b', '1']]
#     llm_tokens = [tokenizer.convert_ids_to_tokens(i).replace("Ġ", " ").encode() for i in range(tokenizer.vocab_size)]
#     llm_token_to_id = {token: i for i, token in enumerate(llm_tokens)}

    ts = ['Paris', 'London']
    llm_tokens = [x.encode() for x in ts]
    llm_token_to_id = {token.encode(): tokenizer.convert_tokens_to_ids(token) for token in ts}


    print("Defining grammar...")
    grammar = define_python_grammar()
    # print(grammar)
    grammar.print()
    print("Initializing Grammar Constraint...")
    grammar_constraint_state = initialize_grammar_constraint(grammar, llm_token_to_id, tokenizer.vocab_size)
    print("Initializing grammar processor...")
    grammar_processor = GrammarConstrainedLogitsProcessor(grammar_constraint_state, llm_token_to_id)

    model = AutoModelForCausalLM.from_pretrained(model_name)

    print("Generating text...")
#     input_text = "i^10=i*"
    input_text = "A city in France is:"
    output_text = generate_text(model, tokenizer, grammar_processor, input_text)
    print(output_text)