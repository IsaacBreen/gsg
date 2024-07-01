from dataclasses import dataclass
from functools import reduce
from typing import Callable, Any, Generator, Optional, List

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


type u8 = str
type Data = Any
type ActiveCombinator = Generator[ParserIterationResult, u8, None]
type Combinator = Callable[[Data], ActiveCombinator]


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
        B_it = B(d)
        B_its.append(B_it)
        B_result = next(B_it)
        A_result.is_complete = B_result.is_complete
        A_result.u8set |= B_result.u8set
    return A_result


def seq2(A: Combinator, B: Combinator) -> Combinator:
    def _seq2(d: Data) -> Generator[ParserIterationResult, u8, None]:
        A_its, B_its, c = [A(d)], [], None
        while A_its or B_its:
            A_result = process(c, A_its)
            B_result = process(c, B_its)
            c = yield seq2_helper(B, d, A_result, B_its) | B_result

    return _seq2


def seq(*args: Combinator) -> Combinator:
    return balanced_tree_reduce(seq2, args)


def repeat1(A: Combinator) -> Combinator:
    def _repeat1(d: Data) -> Generator[ParserIterationResult, u8, None]:
        its, c = [A(d)], None
        while its:
            A_result = process(c, its)
            B_result = A_result.copy()
            c = yield seq2_helper(A, d, A_result, its) | B_result

    return _repeat1


def choice(*parsers: Combinator) -> Combinator:
    def _choice(data: Data) -> Generator[ParserIterationResult, u8, None]:
        active_parsers = [parser(data) for parser in parsers]
        char = yield reduce(lambda a, b: a | b, (next(parser) for parser in active_parsers))

        while active_parsers:
            char = yield process(char, active_parsers)

    return _choice


def eat_u8(value: u8) -> Combinator:
    def _eat_u8(d: Data) -> Generator[ParserIterationResult, u8, None]:
        c = yield ParserIterationResult(U8Set.from_chars(value), False)
        yield ParserIterationResult(U8Set.none(), c == value)

    return _eat_u8


def eat_u8_range(start: u8, is_complete: u8) -> Combinator:
    def _eat_u8_range(d: Data) -> Generator[ParserIterationResult, u8, None]:
        chars = [chr(c) for c in range(ord(start), ord(is_complete) + 1)]
        chars = "".join(chars)
        c = yield ParserIterationResult(U8Set.from_chars(chars), False)
        yield ParserIterationResult(U8Set.none(), start <= c <= is_complete)

    return _eat_u8_range


def eat_u8_range_complement(start: u8, is_complete: u8) -> Combinator:
    def _eat_u8_range_complement(d: Data) -> Generator[ParserIterationResult, u8, None]:
        chars = [chr(c) for c in range(0, ord(start))] + [chr(c) for c in range(ord(is_complete) + 1, 256)]
        chars = "".join(chars)
        c = yield ParserIterationResult(U8Set.from_chars(chars), False)
        yield ParserIterationResult(U8Set.none(), not (start <= c <= is_complete))

    return _eat_u8_range_complement


def eat_string(value: str) -> Combinator:
    return seq(*[eat_u8(c) for c in value])


def eps() -> Combinator:
    def _eps(d: Data) -> Generator[ParserIterationResult, u8, None]:
        yield ParserIterationResult(U8Set.none(), True)

    return _eps


def opt(A: Combinator) -> Combinator:
    return choice(A, eps())


def repeat(A: Combinator) -> Combinator:
    return opt(repeat1(A))

def test_eat_u8():
    it = eat_u8("a")(None)
    next(it)
    result = it.send("a")
    assert result == ParserIterationResult(U8Set.none(), True)


def test_seq():
    it = seq(eat_u8("a"), eat_u8("b"))(None)
    next(it)
    result1 = it.send("a")
    assert result1 == ParserIterationResult(U8Set.from_chars("b"), False)
    result2 = it.send("b")
    assert result2 == ParserIterationResult(U8Set.none(), True)


def test_choice():
    it = choice(eat_u8("a"), eat_u8("b"))(None)
    next(it)
    result1 = it.send("a")
    assert result1 == ParserIterationResult(U8Set.none(), True)
    it = choice(eat_u8("a"), eat_u8("b"))(None)
    next(it)
    result2 = it.send("b")
    assert result2 == ParserIterationResult(U8Set.none(), True)


def test_seq_choice_seq():
    # Matches "ac" or "abc"
    it = seq(choice(eat_u8("a"), seq(eat_u8("a"), eat_u8("b"))), eat_u8("c"))(None)
    result0 = next(it)
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
        # print(f"Parsing JSON string: {json_string}")
        it = json_parser(None)
        result = next(it)
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
    open("GeneratedCSV_10.json").read(),
    open("GeneratedCSV_20.json").read(),
])
def test_json_valid(json_string):
    assert parse_json(json_string)


@pytest.mark.parametrize("json_string", [
    '{"unclosed": "object"',
    '[1, 2, 3',
    '{"invalid": "json",',
])
def test_json_invalid(json_string):
    assert not parse_json(json_string)
