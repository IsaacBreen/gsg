from dataclasses import dataclass
from functools import reduce
from typing import Callable, Dict, Any, Generator, Type

from u8set import U8Set


@dataclass
class ParserIterationResult:
    u8set: U8Set
    end: bool

    def __or__(self, other):
        return ParserIterationResult(self.u8set | other.u8set, self.end | other.end)

    def __and__(self, other):
        return ParserIterationResult(self.u8set & other.u8set, self.end & other.end)


class ParseError(Exception):
    pass

type u8 = str
type Data = Any
type Combinator = Callable[[Data], Generator[ParserIterationResult, u8, None]]


def process(c, its):
    final_result = ParserIterationResult(U8Set.none(), False)
    for i, it in reversed(list(enumerate(its))):
        try:
            result = it.send(c)
            final_result |= result
        except StopIteration:
            its.pop(i)
    return final_result


def seq2(A: Combinator, B: Combinator) -> Combinator:
    def _seq2(d: Data) -> Generator[ParserIterationResult, u8, None]:
        A_it = A(d)
        next(A_it)
        A_its = [A_it]
        B_its = []

        c = yield
        while A_its or B_its:
            A_result = process(c, A_its)
            B_result = process(c, B_its)
            if A_result.end:
                B_it = B(d)
                next(B_it)
                B_its.append(B_it)
                A_result.end = False
            c = yield A_result | B_result

    return _seq2


def seq(*args: Combinator) -> Combinator:
    return reduce(seq2, args)


def choice2(A: Combinator, B: Combinator) -> Combinator:
    def _choice2(d: Data) -> Generator[ParserIterationResult, u8, None]:
        A_it = A(d)
        B_it = B(d)
        next(A_it)
        next(B_it)
        its = [A_it, B_it]
        c = yield
        while its:
            c = yield process(c, its)

    return _choice2


def choice(*args: Combinator) -> Combinator:
    return reduce(choice2, args)


def eat_u8(value: u8) -> Combinator:
    def _eat_u8(d: Data) -> Generator[ParserIterationResult, u8, None]:
        c = yield
        yield ParserIterationResult(U8Set.none(), c == value)

    return _eat_u8


def eat_u8_range(start: u8, end: u8) -> Combinator:
    def _eat_u8_range(d: Data) -> Generator[ParserIterationResult, u8, None]:
        c = yield
        if start <= c <= end:
            yield ParserIterationResult(U8Set.none(), True)
        else:
            raise ParseError(f"Expected {start}-{end}, got {c}")

    return _eat_u8_range


def eat_u8_range_complement(start: u8, end: u8) -> Combinator:
    def _eat_u8_range_complement(d: Data) -> Generator[ParserIterationResult, u8, None]:
        c = yield
        if start <= c <= end:
            raise ParseError(f"Expected not {start}-{end}, got {c}")
        yield ParserIterationResult(U8Set.none(), True)

    return _eat_u8_range_complement


def eat_string(value: str) -> Combinator:
    return seq(*[eat_u8(c) for c in value])


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
    next(it)
    result1 = it.send("a")
    assert result1 == ParserIterationResult(U8Set.from_chars("bc"), False)
    result2 = it.send("b")
    assert result2 == ParserIterationResult(U8Set.from_chars("c"), False)
    result3 = it.send("c")
    assert result3 == ParserIterationResult(U8Set.none(), True)


def test_json():
    ...