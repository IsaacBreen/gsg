import numpy as np
import _sep1
from transformers import LogitsProcessor

class GrammarConstrainedLogitsProcessor(LogitsProcessor):
    def __init__(self, grammar_constraint_state):
        self.grammar_constraint_state = grammar_constraint_state

    def __call__(self, input_ids, scores):
        return scores
        mask = self.grammar_constraint_state.get_mask()
        # We need to ensure the mask aligns with the scores shape
        if len(mask) < scores.shape[-1]:
            padding = np.zeros(scores.shape[-1] - len(mask), dtype=bool)
            mask = np.concatenate((mask, padding))
        elif len(mask) > scores.shape[-1]:
            mask = mask[:scores.shape[-1]]  # Truncate if necessary

        # Apply the mask to the scores
        scores = np.where(mask, scores, -np.inf)  # -inf masks out the logits
        return scores

# --- Example Usage (using your provided _sep1 setup) ---

# Define regexes using PyRegexExpr (as in your example)
plus_regex = _sep1.PyRegexExpr.eat_u8(ord('+'))
times_regex = _sep1.PyRegexExpr.eat_u8(ord('*'))
open_paren_regex = _sep1.PyRegexExpr.eat_u8(ord('('))
close_paren_regex = _sep1.PyRegexExpr.eat_u8(ord(')'))
i_regex = _sep1.PyRegexExpr.eat_u8(ord('i'))

# Define grammar rules using the regexes (as in your example)
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

# Define LLM tokens (as in your example)
llm_tokens = [b"i", b"+", b"*", b"(", b")", b"(i", b"+i"]
llm_token_to_id = {token: i for i, token in enumerate(llm_tokens)}

# Create grammar constraint (as in your example)
grammar_constraint = _sep1.PyGrammarConstraint(grammar, llm_tokens)
grammar_constraint_state = _sep1.PyGrammarConstraintState(grammar_constraint)

def llm_tokens_to_ids(tokens):
    return [llm_token_to_id[token] for token in tokens]

# Create the custom logits processor
grammar_processor = GrammarConstrainedLogitsProcessor(grammar_constraint_state)

# --- Integrating with transformers (Illustrative Example) ---
# This part assumes you have a transformers model and tokenizer set up.
# Replace with your actual model and tokenizer initialization.

from transformers import AutoModelForCausalLM, AutoTokenizer
model_name = "gpt2"
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForCausalLM.from_pretrained(model_name)

input_text = "2 + 2 ="
input_ids = tokenizer.encode(input_text, return_tensors="pt")

# # Commit prefill tokens (as in your example)
prefill = llm_tokens_to_ids([b"(i", b"+", b"i", b"*", b"i"])
for token_id in prefill:
    grammar_constraint_state.commit(token_id)

output = model.generate(
    input_ids,
    max_length=50,  # Adjust as needed
    logits_processor=[grammar_processor]
)

out = tokenizer.decode(output[0], skip_special_tokens=True)
print(out)
