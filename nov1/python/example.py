# example.py
import _tre

# Define grammar rules using the Python bindings
exprs = [
    ("E", _tre.PyGrammarExpr.choice([
        _tre.PyGrammarExpr.sequence([
            _tre.PyGrammarExpr.r#ref("E"),
            _tre.PyGrammarExpr.regex("'+'"),
            _tre.PyGrammarExpr.r#ref("T"),
        ]),
        _tre.PyGrammarExpr.r#ref("T"),
    ])),
    ("T", _tre.PyGrammarExpr.choice([
        _tre.PyGrammarExpr.sequence([
            _tre.PyGrammarExpr.r#ref("T"),
            _tre.PyGrammarExpr.regex("'*'"),
            _tre.PyGrammarExpr.r#ref("F"),
        ]),
        _tre.PyGrammarExpr.r#ref("F"),
    ])),
    ("F", _tre.PyGrammarExpr.choice([
        _tre.PyGrammarExpr.sequence([
            _tre.PyGrammarExpr.regex("'('"),
            _tre.PyGrammarExpr.r#ref("E"),
            _tre.PyGrammarExpr.regex("')'"),
        ]),
        _tre.PyGrammarExpr.regex("'i'"),
    ])),
]

grammar = _tre.PyGrammar(exprs)
llm_tokens = [b"i", b"+", b"*", b"(", b")", b"(i", b"+i"]
grammar_constraint = _tre.PyGrammarConstraint(grammar, llm_tokens)
grammar_constraint_state = grammar_constraint.init()

mask = grammar_constraint_state.get_mask()
print(f"Initial Mask: {mask}")

grammar_constraint_state.commit(0) # Commit 'i'

mask = grammar_constraint_state.get_mask()
print(f"Mask after committing 'i': {mask}")