# example.py
import _sep1

# Define a regex for '+' (using PyRegexExpr)
plus_regex = _sep1.PyRegexExpr.eat_u8(ord('+'))

# Define a regex for '*' (using PyRegexExpr)
times_regex = _sep1.PyRegexExpr.eat_u8(ord('*'))

# Define a regex for '(' (using PyRegexExpr)
open_paren_regex = _sep1.PyRegexExpr.eat_u8(ord('('))

# Define a regex for ')' (using PyRegexExpr)
close_paren_regex = _sep1.PyRegexExpr.eat_u8(ord(')'))

# Define a regex for 'i' (using PyRegexExpr)
i_regex = _sep1.PyRegexExpr.eat_u8(ord('i'))



# Define grammar rules, now using the regexes built above
exprs = [
    ("E", _sep1.PyGrammarExpr.choice([
        _sep1.PyGrammarExpr.sequence([
            _sep1.PyGrammarExpr.ref("E"),
            _sep1.PyGrammarExpr.regex(plus_regex),  # Use the regex object here
            _sep1.PyGrammarExpr.ref("T"),
        ]),
        _sep1.PyGrammarExpr.ref("T"),
    ])),
    ("T", _sep1.PyGrammarExpr.choice([
        _sep1.PyGrammarExpr.sequence([
            _sep1.PyGrammarExpr.ref("T"),
            _sep1.PyGrammarExpr.regex(times_regex),  # Use the regex object here
            _sep1.PyGrammarExpr.ref("F"),
        ]),
        _sep1.PyGrammarExpr.ref("F"),
    ])),
    ("F", _sep1.PyGrammarExpr.choice([
        _sep1.PyGrammarExpr.sequence([
            _sep1.PyGrammarExpr.regex(open_paren_regex),  # Use regex object
            _sep1.PyGrammarExpr.ref("E"),
            _sep1.PyGrammarExpr.regex(close_paren_regex), # Use regex object
        ]),
        _sep1.PyGrammarExpr.regex(i_regex),  # Use the regex object here
    ])),
]

grammar = _sep1.PyGrammar(exprs)

llm_tokens = [b"i", b"+", b"*", b"(", b")"]  # Example tokens
grammar_constraint = _sep1.PyGrammarConstraint(grammar, llm_tokens)
grammar_constraint_state = _sep1.PyGrammarConstraintState(grammar_constraint)

mask = grammar_constraint_state.get_mask()
print(f"Initial Mask: {mask}")

grammar_constraint_state.commit(0)  # Commit 'i'

mask = grammar_constraint_state.get_mask()
print(f"Mask after committing 'i': {mask}")