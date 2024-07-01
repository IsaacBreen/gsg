from __future__ import annotations

from dataclasses import dataclass
from functools import reduce
from typing import Callable, Any, Optional, List, Union

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
        self.result = None

    def start(self):
        self.result = self.combinator(self.data)
        return self.result

    def send(self, c: u8) -> ParserIterationResult:
        return self.result.send(c)


Combinator = Callable[[Data], ActiveCombinator]


def process(c: Optional[u8], its: List[ActiveCombinator]) -> ParserIterationResult:
    final_result = ParserIterationResult(U8Set.none(), False)
    for i, it in reversed(list(enumerate(its))):
        try:
            result = it.send(c)
            final_result |= result
        except StopIteration:
            its.pop(i)
    return final_result


def seq2_helper(B: Combinator, d: Data, A_result: ParserIterationResult, B_its: List[ActiveCombinator]) -> ParserIterationResult:
    if A_result.is_complete:
        B_it = ActiveCombinator(B, d)
        B_its.append(B_it)
        B_result = B_it.start()
        A_result.is_complete = B_result.is_complete
        A_result.u8set |= B_result.u8set
    return A_result


def seq2(A: Combinator, B: Combinator) -> Combinator:
    def _seq2(d: Data) -> ActiveCombinator:
        return Seq2Combinator(A, B, d)
    return _seq2


class Seq2Combinator(ActiveCombinator):
    def __init__(self, A: Combinator, B: Combinator, data: Data):
        super().__init__(self.run, data)
        self.A = A
        self.B = B
        self.A_its = [ActiveCombinator(A, data)]
        self.B_its = []
        self.c = None

    def run(self, data: Data):
        while self.A_its or self.B_its:
            A_result = process(self.c, self.A_its)
            B_result = process(self.c, self.B_its)
            self.c = yield seq2_helper(self.B, data, A_result, self.B_its) | B_result


def seq(*args: Combinator) -> Combinator:
    return balanced_tree_reduce(seq2, args)


def repeat1(A: Combinator) -> Combinator:
    def _repeat1(d: Data) -> ActiveCombinator:
        return Repeat1Combinator(A, d)
    return _repeat1


class Repeat1Combinator(ActiveCombinator):
    def __init__(self, A: Combinator, data: Data):
        super().__init__(self.run, data)
        self.A = A
        self.A_its = [ActiveCombinator(A, data)]
        self.c = None

    def run(self, data: Data):
        while self.A_its:
            A_result = process(self.c, self.A_its)
            self.c = yield seq2_helper(self.A, data, A_result.copy(), self.A_its) | A_result


def choice(*parsers: Combinator) -> Combinator:
    def _choice(data: Data) -> ActiveCombinator:
        return ChoiceCombinator(parsers, data)
    return _choice


class ChoiceCombinator(ActiveCombinator):
    def __init__(self, parsers: List[Combinator], data: Data):
        super().__init__(self.run, data)
        self.active_parsers = [ActiveCombinator(parser, data) for parser in parsers]
        self.char = None

    def run(self, data: Data):
        self.char = yield reduce(lambda a, b: a | b, (parser.start() for parser in self.active_parsers))
        while self.active_parsers:
            self.char = yield process(self.char, self.active_parsers)


def eat_u8_matching(fn: Callable[[int], bool]) -> Combinator:
    def _eat_u8_matching(d: Data) -> ActiveCombinator:
        return EatU8MatchingCombinator(fn, d)
    return _eat_u8_matching


class EatU8MatchingCombinator(ActiveCombinator):
    def __init__(self, fn: Callable[[int], bool], data: Data):
        super().__init__(self.run, data)
        self.fn = fn
        self.c = None

    def run(self, data: Data):
        self.c = yield ParserIterationResult(U8Set.from_match_fn(self.fn), False)
        yield ParserIterationResult(U8Set.none(), self.fn(ord(self.c)))


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


def eat_string(value: str) -> Combinator:
    return seq(*[eat_u8(c) for c in value])


def eps() -> Combinator:
    def _eps(d: Data) -> ActiveCombinator:
        return EpsCombinator(d)
    return _eps


class EpsCombinator(ActiveCombinator):
    def run(self, data: Data):
        yield ParserIterationResult(U8Set.none(), True)


def opt(A: Combinator) -> Combinator:
    return choice(A, eps())


def repeat(A: Combinator) -> Combinator:
    return opt(repeat1(A))


def test_eat_u8():
    it = eat_u8("a")(None)
    result = it.start()
    result = it.send("a")
    assert result == ParserIterationResult(U8Set.none(), True)


def test_seq():
    it = seq(eat_u8("a"), eat_u8("b"))(None)
    result = it.start()
    result1 = it.send("a")
    assert result1 == ParserIterationResult(U8Set.from_chars("b"), False)
    result2 = it.send("b")
    assert result2 == ParserIterationResult(U8Set.none(), True)


def test_choice():
    it = choice(eat_u8("a"), eat_u8("b"))(None)
    result = it.start()
    result1 = it.send("a")
    assert result1 == ParserIterationResult(U8Set.none(), True)
    it = choice(eat_u8("a"), eat_u8("b"))(None)
    result = it.start()
    result2 = it.send("b")
    assert result2 == ParserIterationResult(U8Set.none(), True)


def test_seq_choice_seq():
    # Matches "ac" or "abc"
    it = seq(choice(eat_u8("a"), seq(eat_u8("a"), eat_u8("b"))), eat_u8("c"))(None)
    result0 = it.start()
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

def json_value(d: Data) -> ActiveCombinator:
    return choice(
        string,
        number,
        eat_string("true"),
        eat_string("false"),
        eat_string("null"),
        json_array,
        json_object
    )(d)

def json_array(d: Data) -> ActiveCombinator:
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

def json_object(d: Data) -> ActiveCombinator:
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
        # print(f"Parsing JSON string: {json_string}")
        it = json_parser(None)
        result = it.start()
        assert json_string[0] in result.u8set
        # print(json_string[0])
        result = it.send(json_string[0])
        for char in json_string[1:]:
            assert char in result.u8set
            # print(char)
            # print(result)
            result = it.send(char)
        # print(result)
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