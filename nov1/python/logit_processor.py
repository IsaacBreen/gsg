import numpy as np
import _sep1
from transformers import LogitsProcessor, AutoModelForCausalLM, AutoTokenizer
import torch
import time

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

def define_grammar():
    plus_regex = _sep1.PyRegexExpr.eat_u8(ord('+'))
    times_regex = _sep1.PyRegexExpr.eat_u8(ord('*'))
    open_paren_regex = _sep1.PyRegexExpr.eat_u8(ord('('))
    close_paren_regex = _sep1.PyRegexExpr.eat_u8(ord(')'))
    i_regex = _sep1.PyRegexExpr.eat_u8(ord('i'))

    exprs = [
        ("E", _sep1.PyGrammarExpr.choice([
            _sep1.PyGrammarExpr.sequence([
                _sep1.PyGrammarExpr.ref("E"),
                _sep1.PyGrammarExpr.regex(plus_regex),
                _sep1.PyGrammarExpr.ref("T"),
            ]),
            _sep1.PyGrammarExpr.ref("T"),
        ])),
        ("T", _sep1.PyGrammarExpr.choice([
            _sep1.PyGrammarExpr.sequence([
                _sep1.PyGrammarExpr.ref("T"),
                _sep1.PyGrammarExpr.regex(times_regex),
                _sep1.PyGrammarExpr.ref("F"),
            ]),
            _sep1.PyGrammarExpr.ref("F"),
        ])),
        ("F", _sep1.PyGrammarExpr.choice([
            _sep1.PyGrammarExpr.sequence([
                _sep1.PyGrammarExpr.regex(open_paren_regex),
                _sep1.PyGrammarExpr.ref("E"),
                _sep1.PyGrammarExpr.regex(close_paren_regex),
            ]),
            _sep1.PyGrammarExpr.regex(i_regex),
        ])),
    ]
    return _sep1.PyGrammar(exprs)

def initialize_grammar_constraint(grammar, llm_tokens):
    grammar_constraint = _sep1.PyGrammarConstraint(grammar, llm_tokens)
    grammar_constraint_state = _sep1.PyGrammarConstraintState(grammar_constraint)
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
    model_name = "gpt2"
    tokenizer, model = load_model_and_tokenizer(model_name)

    llm_tokens = [tokenizer.convert_ids_to_tokens(i).encode() for i in range(tokenizer.vocab_size)]
    llm_token_to_id = {token: i for i, token in enumerate(llm_tokens)}

    grammar = define_grammar()
    grammar_constraint_state = initialize_grammar_constraint(grammar, llm_tokens)
    grammar_processor = GrammarConstrainedLogitsProcessor(grammar_constraint_state, llm_tokens)

    input_text = "(i-i)*(i+i)="
    output_text = generate_text(model, tokenizer, grammar_processor, input_text)
    print(output_text)