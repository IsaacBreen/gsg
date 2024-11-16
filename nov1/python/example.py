# example.py
import _sep1

# Define grammar rules using the Python bindings
exprs = [
    ("E", _sep1.PyGrammarExpr.choice([
        _sep1.PyGrammarExpr.sequence([
            _sep1.PyGrammarExpr.r#ref("E"),
            _sep1.PyGrammarExpr.regex("'+'"),
            _sep1.PyGrammarExpr.r#ref("T"),
        ]),
        _sep1.PyGrammarExpr.r#ref("T"),
    ])),
    ("T", _sep1.PyGrammarExpr.choice([
        _sep1.PyGrammarExpr.sequence([
            _sep1.PyGrammarExpr.r#ref("T"),
            _sep1.PyGrammarExpr.regex("'*'"),
            _sep1.PyGrammarExpr.r#ref("F"),
        ]),
        _sep1.PyGrammarExpr.r#ref("F"),
    ])),
    ("F", _sep1.PyGrammarExpr.choice([
        _sep1.PyGrammarExpr.sequence([
            _sep1.PyGrammarExpr.regex("'('"),
            _sep1.PyGrammarExpr.r#ref("E"),
            _sep1.PyGrammarExpr.regex("')'"),
        ]),
        _sep1.PyGrammarExpr.regex("'i'"),
    ])),
]

grammar = _sep1.PyGrammar(exprs)
llm_tokens = [b"i", b"+", b"*", b"(", b")", b"(i", b"+i"]
grammar_constraint = _sep1.PyGrammarConstraint(grammar, llm_tokens)
grammar_constraint_state = grammar_constraint.init()

mask = grammar_constraint_state.get_mask()
print(f"Initial Mask: {mask}")

grammar_constraint_state.commit(0) # Commit 'i'

mask = grammar_constraint_state.get_mask()
print(f"Mask after committing 'i': {mask}")