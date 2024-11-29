from __future__ import annotations

import numpy as np
from _sep1 import PyRegexExpr as Regex, PyGrammar, PyGrammarExpr as ge, PyGrammarConstraint, PyGrammarConstraintState

# Define the grammar: start <- "a"
grammar = PyGrammar([("start", ge.regex(Regex.eat_u8(ord("a"))))])

# Define the LLM tokens
llm_tokens = [b"a", b"b"]

# Initialize grammar constraint
grammar_constraint = PyGrammarConstraint(grammar, llm_tokens)

# Initialize grammar constraint state
grammar_constraint_state = PyGrammarConstraintState(grammar_constraint)

# Get the initial mask
initial_mask = grammar_constraint_state.get_mask()

# Get the initial mask IDs
initial_mask_ids = np.where(initial_mask)[0]

# Map IDs to tokens for printing
initial_mask_id_map = {id: llm_tokens[id] for id in initial_mask_ids}

# Print the initial mask IDs
print(f"Initial Mask IDs: {initial_mask_id_map}")