import rust_grammar_constraint as rgc

# Define grammar rules
exprs = [
    ("E", rgc.choice([
        rgc.sequence([rgc.r#ref("E"), rgc.regex(rgc.eat_u8(b'+')), rgc.r#ref("T")]),
        rgc.r#ref("T")
    ])),
    ("T", rgc.choice([
        rgc.sequence([rgc.r#ref("T"), rgc.regex(rgc.eat_u8(b'*')), rgc.r#ref("F")]),
        rgc.r#ref("F")
    ])),
    ("F", rgc.choice([
        rgc.sequence([rgc.regex(rgc.eat_u8(b'(')), rgc.r#ref("E"), rgc.regex(rgc.eat_u8(b')'))]),
        rgc.regex(rgc.eat_u8(b'i'))
    ]))
]

# Create grammar
grammar = rgc.PyGrammar(exprs)

# Define LLM tokens
llm_tokens = [b"i", b"+", b"*", b"(", b")", b"(i", b"+i"]

# Create grammar constraint
constraint = rgc.PyGrammarConstraint(grammar, llm_tokens)

# Initialize constraint state
state = constraint.init()

# Get mask
mask = state.get_mask()
print(f"Initial mask: {mask}")

# Commit token
state.commit(0)  # Commit "i"

# Get updated mask
mask = state.get_mask()
print(f"Mask after committing 'i': {mask}")

state.commit_many([1, 0]) # Commit "+i"

mask = state.get_mask()
print(f"Mask after committing '+i': {mask}")