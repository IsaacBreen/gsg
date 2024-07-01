from __future__ import annotations

from dataclasses import dataclass
from functools import reduce
from typing import Callable, Any, Optional, List

import pytest

from balanced_tree_reduce import balanced_tree_reduce
from u8set import U8Set


@dataclass
class ParserIterationResult:
    u8set: U8Set
    is_complete: bool

    def __or__(self, other):
        return ParserIterationResult(self.u8set | other.u8set, self.is_complete | other.is_complete)

    def __and__(self, other):
        return ParserIterationResult(self.u8set & other.u8set, self.is_complete & other.is_complete)

    def copy(self):
        return ParserIterationResult(self.u8set, self.is_complete)


u8 = str
Data = Any


class ActiveCombinator:
    def __init__(self, combinator: Combinator, data: Data):
        self.combinator = combinator
        self.data = data
        self.state = self.combinator.initial_state(data)

    def send(self, c: Optional[u8]) -> ParserIterationResult:
        return self.combinator.next_state(self.state, c)

    def clone(self) -> ActiveCombinator:
        new_active = ActiveCombinator(self.combinator, self.data)
        new_active.state = self.combinator.clone_state(self.state)
        return new_active


class Combinator:
    def __call__(self, data: Data) -> ActiveCombinator:
        return ActiveCombinator(self, data)

    def initial_state(self, data: Data) -> Any:
        raise NotImplementedError

    def next_state(self, state: Any, c: Optional[u8]) -> ParserIterationResult:
        raise NotImplementedError

    def clone_state(self, state: Any) -> Any:
        return state  # Default implementation, override if necessary


def process(c: Optional[u8], its: List[ActiveCombinator]) -> ParserIterationResult:
    final_result = ParserIterationResult(U8Set.none(), False)
    for i in reversed(range(len(its))):
        result = its[i].send(c)
        if result.is_complete and result.u8set.is_empty():
            its.pop(i)
        else:
            final_result |= result
    return final_result


class Seq2(Combinator):
    def __init__(self, A: Combinator, B: Combinator):
        self.A = A
        self.B = B

    def initial_state(self, data: Data):
        return {
            'A_its': [self.A(data)],
            'B_its': [],
            'A_complete': False,
        }

    def next_state(self, state, c):
        A_result = process(c, state['A_its'])
        B_result = process(c, state['B_its'])

        if A_result.is_complete and not state['A_complete']:
            state['A_complete'] = True
            state['B_its'].append(self.B(state['A_its'][0].data))

        return A_result | B_result

    def clone_state(self, state):
        return {
            'A_its': [it.clone() for it in state['A_its']],
            'B_its': [it.clone() for it in state['B_its']],
            'A_complete': state['A_complete'],
        }


def seq2(A: Combinator, B: Combinator) -> Combinator:
    return Seq2(A, B)


def seq(*args: Combinator) -> Combinator:
    return balanced_tree_reduce(seq2, args)


class Repeat1(Combinator):
    def __init__(self, A: Combinator):
        self.A = A

    def initial_state(self, data: Data):
        return {
            'A_its': [self.A(data)],
            'data': data,
        }

    def next_state(self, state, c):
        A_result = process(c, state['A_its'])
        if A_result.is_complete:
            state['A_its'].append(self.A(state['data']))
        return A_result

    def clone_state(self, state):
        return {
            'A_its': [it.clone() for it in state['A_its']],
            'data': state['data'],
        }


def repeat1(A: Combinator) -> Combinator:
    return Repeat1(A)


class Choice(Combinator):
    def __init__(self, *parsers: Combinator):
        self.parsers = parsers

    def initial_state(self, data: Data):
        return {
            'active_parsers': [parser(data) for parser in self.parsers],
        }

    def next_state(self, state, c):
        return process(c, state['active_parsers'])

    def clone_state(self, state):
        return {
            'active_parsers': [it.clone() for it in state['active_parsers']],
        }


def choice(*parsers: Combinator) -> Combinator:
    return Choice(*parsers)


class EatU8Matching(Combinator):
    def __init__(self, fn: Callable[[int], bool]):
        self.fn = fn

    def initial_state(self, data: Data):
        return {'stage': 0}

    def next_state(self, state, c):
        if state['stage'] == 0:
            state['stage'] = 1
            return ParserIterationResult(U8Set.from_match_fn(self.fn), False)
        elif state['stage'] == 1:
            state['stage'] = 2
            return ParserIterationResult(U8Set.none(), self.fn(ord(c)))
        else:
            return ParserIterationResult(U8Set.none(), True)


def eat_u8_matching(fn: Callable[[int], bool]) -> Combinator:
    return EatU8Matching(fn)


def eat_u8(value: u8) -> Combinator:
    def match_fn(c: int) -> bool:
        return c == ord(value)
    return eat_u8_matching(match_fn)


def eat_u8_range(start: u8, end: u8) -> Combinator:
    def match_fn(c: int) -> bool:
        return ord(start) <= c <= ord(end)
    return eat_u8_matching(match_fn)


def eat_u8_range_complement(start: u8, end: u8) -> Combinator:
    def match_fn(c: int) -> bool:
        return not ord(start) <= c <= ord(end)
    return eat_u8_matching(match_fn)


class EatString(Combinator):
    def __init__(self, value: str):
        self.value = value

    def initial_state(self, data: Data):
        return {'index': 0}

    def next_state(self, state, c):
        if state['index'] < len(self.value):
            expected = self.value[state['index']]
            if c == expected:
                state['index'] += 1
                is_complete = state['index'] == len(self.value)
                return ParserIterationResult(U8Set.none(), is_complete)
            else:
                return ParserIterationResult(U8Set.none(), True)
        else:
            return ParserIterationResult(U8Set.none(), True)


def eat_string(value: str) -> Combinator:
    return EatString(value)


class Eps(Combinator):
    def initial_state(self, data: Data):
        return {}

    def next_state(self, state, c):
        return ParserIterationResult(U8Set.none(), True)


def eps() -> Combinator:
    return Eps()


def opt(A: Combinator) -> Combinator:
    return choice(A, eps())


def repeat(A: Combinator) -> Combinator:
    return opt(repeat1(A))


def test_eat_u8():
    it = eat_u8("a")(None)
    it.send(None)
    result = it.send("a")
    assert result == ParserIterationResult(U8Set.none(), True)


def test_seq():
    it = seq(eat_u8("a"), eat_u8("b"))(None)
    it.send(None)
    result1 = it.send("a")
    assert result1 == ParserIterationResult(U8Set.from_chars("b"), False)
    result2 = it.send("b")
    assert result2 == ParserIterationResult(U8Set.none(), True)


def test_choice():
    it = choice(eat_u8("a"), eat_u8("b"))(None)
    it.send(None)
    result1 = it.send("a")
    assert result1 == ParserIterationResult(U8Set.none(), True)
    it = choice(eat_u8("a"), eat_u8("b"))(None)
    it.send(None)
    result2 = it.send("b")
    assert result2 == ParserIterationResult(U8Set.none(), True)


def test_seq_choice_seq():
    # Matches "ac" or "abc"
    it = seq(choice(eat_u8("a"), seq(eat_u8("a"), eat_u8("b"))), eat_u8("c"))(None)
    result0 = it.send(None)
    assert result0 == ParserIterationResult(U8Set.from_chars("a"), False)
    result1 = it.send("a")
    assert result1 == ParserIterationResult(U8Set.from_chars("bc"), False)
    result2 = it.send("b")
    assert result2 == ParserIterationResult(U8Set.from_chars("c"), False)
    result3 = it.send("c")
    assert result3 == ParserIterationResult(U8Set.none(), True)


# Helper combinators for JSON parsing
whitespace = repeat(choice(eat_u8(" "), eat_u8("\t"), eat_u8("\n"), eat_u8("\r")))
digit = eat_u8_range("0", "9")
digits = repeat(digit)
integer = seq(opt(choice(eat_u8("-"), eat_u8("+"))), digits)
fraction = seq(eat_u8("."), digits)
exponent = seq(choice(eat_u8("e"), eat_u8("E")), choice(eat_u8("+"), eat_u8("-"), eps()), digits)
number = seq(integer, choice(fraction, eps()), choice(exponent, eps()))

string_char = choice(
    eat_u8_range_complement("\"", "\""),
    seq(eat_u8("\\"), choice(
        eat_u8("\""), eat_u8("\\"), eat_u8("/"), eat_u8("b"),
        eat_u8("f"), eat_u8("n"), eat_u8("r"), eat_u8("t"),
        seq(eat_u8("u"), eat_u8_range("0", "9"), eat_u8_range("0", "9"), eat_u8_range("0", "9"), eat_u8_range("0", "9"))
    ))
)
string = seq(eat_u8("\""), repeat(string_char), eat_u8("\""))

def json_value(d: Data) -> Generator[ParserIterationResult, u8, None]:
    return choice(
        string,
        number,
        eat_string("true"),
        eat_string("false"),
        eat_string("null"),
        json_array,
        json_object
    )(d)

def json_array(d: Data) -> Generator[ParserIterationResult, u8, None]:
    return seq(
        eat_u8("["),
        whitespace,
        choice(
            seq(
                json_value,
                repeat(seq(whitespace, eat_u8(","), whitespace, json_value)),
                whitespace
            ),
            eps()
        ),
        eat_u8("]")
    )(d)

def json_object(d: Data) -> Generator[ParserIterationResult, u8, None]:
    return seq(
        eat_u8("{"),
        whitespace,
        choice(
            seq(
                string,
                whitespace,
                eat_u8(":"),
                whitespace,
                json_value,
                repeat(seq(whitespace, eat_u8(","), whitespace, string, whitespace, eat_u8(":"), whitespace, json_value)),
                whitespace
            ),
            eps()
        ),
        eat_u8("}")
    )(d)

# Test cases
json_parser = seq(whitespace, json_value, whitespace)


def parse_json(json_string):
    try:
        it = json_parser(None)
        result = it.send(None)
        assert json_string[0] in result.u8set
        result = it.send(json_string[0])
        for char in json_string[1:]:
            assert char in result.u8set
            result = it.send(char)
        return result.is_complete
    except (AssertionError, StopIteration):
        print(f"Failed to parse JSON string: {json_string}")
        return False


@pytest.mark.parametrize("json_string", [
    '42',
    '{"key": "value"}',
    '[1, 2, 3]',
    '{"nested": {"array": [1, 2, 3], "object": {"a": true, "b": false}}}',
    '"Hello, world!"',
    'null',
    'true',
    'false',
])
def test_json_valid(json_string):
    assert parse_json(json_string)


@pytest.mark.parametrize("json_string", [
    open("GeneratedCSV_10.json").read(),
    open("GeneratedCSV_20.json").read(),
    # open("GeneratedCSV_100.json").read(),
    # open("GeneratedCSV_200.json").read(),
], ids=[
    "10 lines",
    "20 lines",
    # "100 lines",
    # "200 lines",
])
def test_json_valid_long(json_string):
    assert parse_json(json_string)


@pytest.mark.parametrize("json_string", [
    '{"unclosed": "object"',
    '[1, 2, 3',
    '{"invalid": "json",',
])
def test_json_invalid(json_string):
    assert not parse_json(json_string)
