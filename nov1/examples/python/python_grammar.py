from __future__ import annotations

import io
import time
import tokenize
from pathlib import Path

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
    with Path(__file__).parent / "python.gram" as f:
        grammar_text = f.read()

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
    def __init__(self, grammar_constraint_state, llm_tokens):
        self.grammar_constraint_state = grammar_constraint_state
        self.seen_input_ids = []
        self.llm_tokens = llm_tokens

    def __call__(self, input_ids, scores):
        current_input_ids = input_ids.view(-1).tolist()
        new_token_ids = current_input_ids[len(self.seen_input_ids):]

        for token_id in new_token_ids:
            debug_print(f"Committing token: {self.llm_tokens[token_id]} (ID: {token_id})")
            timeit(self.grammar_constraint_state.commit)(token_id)

        self.seen_input_ids = current_input_ids
        mask = timeit(self.grammar_constraint_state.get_mask)()

        if len(mask) < scores.shape[-1]:
            padding = np.zeros(scores.shape[-1] - len(mask), dtype=bool)
            mask = np.concatenate((mask, padding))
        elif len(mask) > scores.shape[-1]:
            mask = mask[:scores.shape[-1]]

        mask_ids = np.where(mask)[0]
        mask_id_map = {id: self.llm_tokens[id] for id in mask_ids}
        debug_print(f"Mask IDs: {mask_id_map}")
        print("")

        scores = np.where(mask, scores, -np.inf)
        return torch.tensor(scores)

def load_model_and_tokenizer(model_name):
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForCausalLM.from_pretrained(model_name)
    return tokenizer, model

def initialize_grammar_constraint(grammar, llm_tokens):
    grammar_constraint = PyGrammarConstraint(grammar, llm_tokens)
    grammar_constraint_state = PyGrammarConstraintState(grammar_constraint)
    initial_mask = grammar_constraint_state.get_mask()
    initial_mask_ids = np.where(initial_mask)[0]
    initial_mask_id_map = {id: llm_tokens[id] for id in initial_mask_ids}
    print(f"Initial Mask IDs: {initial_mask_id_map}")
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
    tokenizer, model = load_model_and_tokenizer(model_name)

    llm_tokens = [tokenizer.convert_ids_to_tokens(i).encode() for i in range(tokenizer.vocab_size)]
    llm_token_to_id = {token: i for i, token in enumerate(llm_tokens)}

    grammar = define_python_grammar()
    grammar_constraint_state = initialize_grammar_constraint(grammar, llm_tokens)
    grammar_processor = GrammarConstrainedLogitsProcessor(grammar_constraint_state, llm_tokens)

#     input_text = "i^10=i*"
    input_text = "(i)+((i))+(((i)))+"
    output_text = generate_text(model, tokenizer, grammar_processor, input_text)
    print(output_text)