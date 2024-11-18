import numpy as np
import _sep1
from transformers import LogitsProcessor, AutoModelForCausalLM, AutoTokenizer
import torch
import time

def timeit(func):
    def wrapper(*args, **kwargs):
        start_time = time.time()
        result = func(*args, **kwargs)
        end_time = time.time()
        print(f"Time taken: {(end_time - start_time) * 1000:.2f} ms")
        return result
    return wrapper

class GrammarConstrainedLogitsProcessor(LogitsProcessor):
    def __init__(self, grammar_constraint_state):
        self.grammar_constraint_state = grammar_constraint_state
        self.seen_input_ids = []  # Track the input IDs seen so far

    def __call__(self, input_ids, scores):
        # Flatten input_ids to a 1D list
        current_input_ids = input_ids.view(-1).tolist()

        # Find the new tokens by comparing with seen_input_ids
        new_token_ids = current_input_ids[len(self.seen_input_ids):]

        # Commit the new tokens to the grammar constraint state
        for token_id in new_token_ids:
            print(f"Committing token: {llm_tokens[token_id]} (ID: {token_id})")
            self.grammar_constraint_state.commit(token_id)

        # Update seen_input_ids
        self.seen_input_ids = current_input_ids

        # Get the mask and apply it (as before)
        print(self.grammar_constraint_state.performance_report(), end="")
        mask = timeit(self.grammar_constraint_state.get_mask)()
        if len(mask) < scores.shape[-1]:
            padding = np.zeros(scores.shape[-1] - len(mask), dtype=bool)
            mask = np.concatenate((mask, padding))
        elif len(mask) > scores.shape[-1]:
            mask = mask[:scores.shape[-1]]

        mask_ids = np.where(mask)[0]
        mask_id_map = {id: llm_tokens[id] for id in mask_ids}
        print(f"Mask IDs: {mask_id_map}")
        print(f"----------------------------")

        scores = np.where(mask, scores, -np.inf)
        return torch.tensor(scores)

# --- Example Usage with GPT-2 ---

# Load the GPT-2 tokenizer and model
model_name = "gpt2"
tokenizer = AutoTokenizer.from_pretrained(model_name)

# Get the actual LLM tokens from the tokenizer
llm_tokens = [tokenizer.convert_ids_to_tokens(i).encode() for i in range(tokenizer.vocab_size)]
llm_token_to_id = {token: i for i, token in enumerate(llm_tokens)}

# --- Define your grammar using _sep1 (as before) ---

# Define regexes using PyRegexExpr
plus_regex = _sep1.PyRegexExpr.eat_u8(ord('+'))
times_regex = _sep1.PyRegexExpr.eat_u8(ord('*'))
open_paren_regex = _sep1.PyRegexExpr.eat_u8(ord('('))
close_paren_regex = _sep1.PyRegexExpr.eat_u8(ord(')'))
i_regex = _sep1.PyRegexExpr.eat_u8(ord('i'))

# Define grammar rules using the regexes
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

grammar = _sep1.PyGrammar(exprs)

# Create grammar constraint using the actual LLM tokens
grammar_constraint = _sep1.PyGrammarConstraint(grammar, llm_tokens)
grammar_constraint_state = _sep1.PyGrammarConstraintState(grammar_constraint)
initial_mask = grammar_constraint_state.get_mask()
initial_mask_ids = np.where(initial_mask)[0]
initial_mask_id_map = {id: llm_tokens[id] for id in initial_mask_ids}
print(f"Initial Mask IDs: {initial_mask_id_map}")

def llm_tokens_to_ids(tokens):
    return [llm_token_to_id[token] for token in tokens]

# Create the custom logits processor
grammar_processor = GrammarConstrainedLogitsProcessor(grammar_constraint_state)

# --- Generating text with grammar constraints ---

input_text = "(i-i)*(i+i)="
input_ids = tokenizer.encode(input_text, return_tensors="pt")

# Commit prefill tokens (using the actual LLM token IDs)
prefill_tokens = []
prefill_ids = [llm_token_to_id[token.encode()] for token in prefill_tokens if token.encode() in llm_token_to_id]
for token_id in prefill_ids:
    grammar_processor.grammar_constraint_state.commit(token_id)  # Commit to the state in the processor

grammar_processor.seen_input_ids = input_ids[0].tolist()

model = AutoModelForCausalLM.from_pretrained(model_name)

output = model.generate(
    input_ids,
    max_new_tokens=2,  # Adjust as needed
    logits_processor=[grammar_processor]
)

out = tokenizer.decode(output[0], skip_special_tokens=True)
print(out)