import json
import keyword
import textwrap
import tokenize
from enum import IntEnum
from typing import Tuple, List, Set, Dict

from pegen.grammar import NameLeaf, StringLeaf, Group, Opt, Repeat0, Repeat1, Gather, Lookahead, Leaf, Repeat, Forced, \
    Cut, Rhs, NamedItem, Alt
from pegen.grammar_parser import GeneratedParser as GrammarParser
from pegen.tokenizer import Tokenizer

type Item = Leaf | Group | Opt | Repeat | Forced | Lookahead | Rhs | Cut

llm_provider = "groq"

if llm_provider == "openai":
    from openai import OpenAI
    client = OpenAI()
    MODEL_NAME = "gpt-4-turbo"

elif llm_provider == "groq":
    from groq import Groq
    client = Groq()
    MODEL_NAME = "Llama3-8b-8192"

else:
    raise ValueError(f"Unknown LLM provider: {llm_provider}")

# BEGIN FROM https://github.com/python/cpython/blob/main/Lib/token.py --------------------------
EXACT_TOKEN_TYPES = {
    '!': 'EXCLAMATION',
    '!=': 'NOTEQUAL',
    '%': 'PERCENT',
    '%=': 'PERCENTEQUAL',
    '&': 'AMPER',
    '&=': 'AMPEREQUAL',
    '(': 'LPAR',
    ')': 'RPAR',
    '*': 'STAR',
    '**': 'DOUBLESTAR',
    '**=': 'DOUBLESTAREQUAL',
    '*=': 'STAREQUAL',
    '+': 'PLUS',
    '+=': 'PLUSEQUAL',
    ',': 'COMMA',
    '-': 'MINUS',
    '-=': 'MINEQUAL',
    '->': 'RARROW',
    '.': 'DOT',
    '...': 'ELLIPSIS',
    '/': 'SLASH',
    '//': 'DOUBLESLASH',
    '//=': 'DOUBLESLASHEQUAL',
    '/=': 'SLASHEQUAL',
    ':': 'COLON',
    ':=': 'COLONEQUAL',
    ';': 'SEMI',
    '<': 'LESS',
    '<<': 'LEFTSHIFT',
    '<<=': 'LEFTSHIFTEQUAL',
    '<=': 'LESSEQUAL',
    '=': 'EQUAL',
    '==': 'EQEQUAL',
    '>': 'GREATER',
    '>=': 'GREATEREQUAL',
    '>>': 'RIGHTSHIFT',
    '>>=': 'RIGHTSHIFTEQUAL',
    '@': 'AT',
    '@=': 'ATEQUAL',
    '[': 'LSQB',
    ']': 'RSQB',
    '^': 'CIRCUMFLEX',
    '^=': 'CIRCUMFLEXEQUAL',
    '{': 'LBRACE',
    '|': 'VBAR',
    '|=': 'VBAREQUAL',
    '}': 'RBRACE',
    '~': 'TILDE',
}
# END FROM https://github.com/python/cpython/blob/main/Lib/token.py --------------------------

def explore_grammar(grammar_file):
    with open(grammar_file) as f:
        tokenizer = Tokenizer(tokenize.generate_tokens(f.readline))
        parser = GrammarParser(tokenizer)
        grammar = parser.start()

        if not grammar:
            raise parser.make_syntax_error(grammar_file)

        # Explore the grammar object:
        for rule in grammar.rules.values():
            print(f"Rule: {rule.name}")
            print(f"  Type: {rule.type}")
            print(f"  RHS:")
            for alt in rule.rhs.alts:
                print(f"    - {alt}")
                for item in alt.items:
                    if item.name is None:
                        print(f"      - {item.item}")
                    else:
                        print(f"      - {item.name} = {item.item}")

def generate_rule_name(base_name, suffix):
    return f"{base_name}_{suffix}"

def flatten_item(
        rule_name: str,
        item: Item | NamedItem,
        queue: List[Tuple[str, Item]],
        names: Set[str],
        literals: dict[str, str],
        keywords: Set[str]) -> str:
    if isinstance(item, Leaf):
        if isinstance(item, NameLeaf):
            value = item.value
            names.add(value)
            return value
        elif isinstance(item, StringLeaf):
            v = item.value
            assert v[0] == "'" and v[-1] == "'" or v[0] == '"' and v[-1] == '"', f"Expected string leaf to be enclosed in quotes, got: {v}"
            literal = v[1:-1]
            if literal in literals:
                return literals[literal]
            elif literal.isidentifier():
                assert literal in keyword.kwlist or literal in keyword.softkwlist
                if literal == "_":
                    literal_name = "UNDERSCORE"
                else:
                    literal_name = literal.upper()
                keywords.add(literal_name)
                literals[literal] = literal_name
                return literal_name
            else:
                raise ValueError(f"Unknown literal: {literal}")
        else:
            raise TypeError(f"Unknown leaf type: {type(item)}")
    elif isinstance(item, Group):
        new_rule_name = generate_rule_name(rule_name + "__group", len(queue))
        queue.append((new_rule_name, item.rhs))
        return new_rule_name
    elif isinstance(item, Opt):
        new_rule_name = generate_rule_name(rule_name + "__opt", len(queue))
        rhs = Rhs(alts=[
            Alt(items=[NamedItem(name=None, item=item.node)]),
            Alt(items=[]),
        ])
        queue.append((new_rule_name, rhs))
        return new_rule_name
    elif isinstance(item, Repeat):
        if isinstance(item, Repeat0):
            new_rule_name = generate_rule_name(rule_name + "__repeat0", len(queue))
            rhs = Rhs(alts=[
                Alt(items=[NamedItem(name=None, item=NameLeaf(value=new_rule_name)), NamedItem(name=None, item=item.node)]),
                Alt(items=[]),
            ])
            queue.append((new_rule_name, rhs))
            return new_rule_name
        elif isinstance(item, Repeat1):
            new_rule_name = generate_rule_name(rule_name + "__repeat1", len(queue))
            rhs = Rhs(alts=[
                Alt(items=[NamedItem(name=None, item=NameLeaf(value=new_rule_name)), NamedItem(name=None, item=item.node)]),
                Alt(items=[NamedItem(name=None, item=item.node)]),
            ])
            queue.append((new_rule_name, rhs))
            return new_rule_name
        elif isinstance(item, Gather):
            node = item.node
            separator = item.separator
            new_rule_name = generate_rule_name(rule_name + "__gather", len(queue))
            rhs = Rhs(alts=[
                Alt(items=[NamedItem(name=None, item=NameLeaf(value=new_rule_name)), NamedItem(name=None, item=separator), NamedItem(name=None, item=node)]),
                Alt(items=[NamedItem(name=None, item=node)]),
            ])
            queue.append((new_rule_name, rhs))
            return new_rule_name
        else:
            raise TypeError(f"Unknown repeat type: {type(item)}")
    elif isinstance(item, Forced):
        return flatten_item(rule_name, item.node, queue, names, literals, keywords)
    elif isinstance(item, Lookahead):
        # Ignore
        print("Ignoring Lookahead")
        return ""
    elif isinstance(item, Rhs):
        new_rule_name = generate_rule_name(rule_name, len(queue))
        queue.append((new_rule_name, item))
        return new_rule_name
    elif isinstance(item, Cut):
        # Ignore
        print("Ignoring Cut")
        return ""
    elif isinstance(item, NamedItem):
        return flatten_item(rule_name, item.item, queue, names, literals, keywords)
    else:
        raise TypeError(f"Unknown item type: {type(item)}")

def generate_token_names_for_literals(literals: List[str], existing_token_names: List[str]) -> List[str]:
    print(f"Generating token names for literals: {literals}")

    message = f"""
        In Bison/yacc, what might be a good name for the (capitalised) tokens that matches? Ensure each name is unique.

        `{literals}`
        
        Furthermore, the names must not coincide with the following already-defined tokens:
        
        `{existing_token_names}`

        Give me a JSON mapping the each token name to its pattern, e.g. `{{RPAREN: "(", ...}}`, raw, no code ticks, no other text. Don't say anything else. Just give JSON.
    """

    message = textwrap.dedent(message).strip()

    print(f"message: {message}")

    retry = 0
    while True:
        try:
            chat_completion = client.chat.completions.create(
                messages=[
                    {
                        "role": "user",
                        "content": message,
                    },
                ],
                temperature=0.3,
                model=MODEL_NAME,
                response_format={"type": "json_object"},
            )

            result_text = chat_completion.choices[0].message.content

            print(f"llm reports: {result_text}")

            # Parse the JSON
            result_json = json.loads(result_text)

            # For some reason the models prefer to generate {TOKEN_NAME: pattern} instead of {pattern: TOKEN_NAME}
            # So we need to swap the keys and values
            token_pattern_to_name = {v: k for k, v in result_json.items()}

            # Now we can get the list of token names. Retry for missing literals
            missing = []
            hits = []
            for literal in literals:
                if literal in token_pattern_to_name and token_pattern_to_name[literal] not in existing_token_names:
                    hits.append(token_pattern_to_name[literal])
                else:
                    missing.append(literal)

            # If there are no hits, raise an error (there's no point retrying)
            if not hits:
                raise ValueError(f"Failed to generate token names for literals: {missing}")

        except Exception as e:
            print(f"Error: {e}")
            retry += 1
            if retry > 5:
                raise e
            continue

        else:
            if missing:
                # Recurse
                all_token_names = existing_token_names + hits
                rest_token_names = generate_token_names_for_literals(missing, all_token_names)
                rest_token_name_map = {literal: token_name for literal, token_name in zip(missing, rest_token_names)}
                token_pattern_to_name.update(rest_token_name_map)

            literal_token_names = [token_pattern_to_name[literal] for literal in literals]

            return literal_token_names

def convert_to_bison(grammar_file, output_path, literals_rs_path):
    with open(grammar_file) as f:
        tokenizer = Tokenizer(tokenize.generate_tokens(f.readline))
        parser = GrammarParser(tokenizer)
        grammar = parser.start()

        if not grammar:
            raise parser.make_syntax_error(grammar_file)

        rule_names: Set[str] = set()
        names: Set[str] = set()
        literals: Dict[str, str] = EXACT_TOKEN_TYPES
        keywords: Set[str] = set()

        # Add keywords to names, literals, and keywords
        for kw in [*keyword.kwlist, *keyword.softkwlist]:
            if kw == "_":
                ke_token_name = "UNDERSCORE"
            else:
                ke_token_name = kw.upper()
            names.add(ke_token_name)
            literals[kw] = ke_token_name
            keywords.add(ke_token_name)

        # Exact token types
        for value, token_name in literals.items():
            names.add(token_name)
            literals[value] = token_name

        queue = []

        for rule in grammar.rules.values():
            queue.append((rule.name, rule.rhs))

        grammar_rules: Dict[str, List[List[str]]] = {}

        while queue:
            rule_name, item = queue.pop(0)
            rule_names.add(rule_name)
            assert rule_name.isidentifier()
            choice: List[List[str]] = []
            if isinstance(item, Rhs):
                for alt_items in item.alts:
                    sequence: List[str] = []
                    for alt_item in alt_items.items:
                        item_name = flatten_item(rule_name, alt_item, queue, names, literals, keywords)
                        sequence.append(item_name)
                    choice.append(sequence)
            else:
                item_name = flatten_item(rule_name, item, queue, names, literals, keywords)
                choice.append([item_name])

            grammar_rules[rule_name] = choice

        # Add a special rule for NAME
        grammar_rules["NAME"] = [["_NAME"]]
        for kw in keyword.kwlist + keyword.softkwlist:
            if kw == "_":
                kw = "UNDERSCORE"
            else:
                kw = kw.upper()
            grammar_rules["NAME"].append([kw])

        names.remove("NAME")
        names.add("_NAME")

        # Remove any rules that start with "invalid_" and any rules that reference them.
        for rule_name in list(grammar_rules):
            if rule_name.startswith("invalid_"):
                del grammar_rules[rule_name]
            else:
                for i, sequence in reversed(list(enumerate(grammar_rules[rule_name]))):
                    for j, item in enumerate(sequence):
                        if item.startswith("invalid_"):
                            del grammar_rules[rule_name][i]
                            break
                if not grammar_rules[rule_name]:
                    del grammar_rules[rule_name]

        EXPERIMENT = False
        if EXPERIMENT:
            # Quickly add a new start rule for experimentation
            start_sequence = "parameters__repeat1_254 parameters__repeat0_255".split()
            grammar_rules = {"start": [start_sequence]} | grammar_rules

            grammar_rules['parameters__repeat1_254'] = [['_NAME', 'COMMA', '_NAME', 'COMMA'], ['_NAME', 'COMMA']]
            grammar_rules['parameters__repeat0_255'] = [['_NAME'], []]

            # Remove useless nonterminals
            useful = {"start"}
            queue = ["start"]
            while queue:
                rule_name = queue.pop(0)
                for sequence in grammar_rules[rule_name]:
                    for item in sequence:
                        if item in grammar_rules and item not in useful:
                            queue.append(item)
                            useful.add(item)

            grammar_rules = {name: choice for name, choice in grammar_rules.items() if name in useful}

        with open(output_path, 'w') as out:
            # Now go back to the beginning of the file
            f.seek(0)
            out.write("%{\n")
            out.write("#include <stdio.h>\n")  # Include standard headers or any required definitions
            out.write("%}\n\n")

            # Use IELR
            out.write("%define lr.type ielr\n\n")
            out.write("%define lr.default-reduction accepting\n")
            out.write("\n")


            # GLR
            out.write("%glr-parser\n\n")

            # Define the tokens
            out.write("%token ")
            out.write(" ".join(name for name in names if name.isupper()))
            out.write("\n\n")

            # Write the grammar rules
            out.write("%%\n")  # Start of grammar rules

            for name, choice in grammar_rules.items():
                out.write(f"{name}: {"\n    | ".join(" ".join(sequence) for sequence in choice)};\n\n")

            out.write("%%\n\n")  # End of grammar rules

def execute_bison(grammar_file, output_path):
    with open(grammar_file) as f:
        import subprocess
        # `$ bison bison_grammar.y -x --report=lookaheads`
        subprocess.run(["bison", grammar_file, "-x", "--report=lookaheads"])

def method_name(rule):
    rhs = rule.rhs
    return rhs

if __name__ == "__main__":
    explore_grammar("./python.gram")
    convert_to_bison("./python.gram", "bison_grammar.y", "../../src/tokenizer/python_literals.rs")
    execute_bison("./bison_grammar.y", "bison_grammar.c")
    print("Done")