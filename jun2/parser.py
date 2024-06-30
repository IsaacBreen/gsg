from dataclasses import dataclass
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
type Combinator = Callable[[u8, Data], Generator[ParserIterationResult, u8, None]]


def process(A, B, c, its):
    final_result = ParserIterationResult(U8Set.none(), False)
    for i, it in reversed(list(enumerate(its))):
        result = it.send(c)
        if result.end:
            its.pop(i)
        final_result |= result
    return final_result


def seq(A: Combinator, B: Combinator) -> Combinator:
    def _seq(c: u8, d: Data) -> Generator[ParserIterationResult, u8, None]:
        A_its = [A(c, d)]
        B_its = []

        while A_its or B_its:
            A_result = process(A, B, c, A_its)
            if A_result.end:
                B_its.append(B(c, d))
                A_result.end = False
            B_result = process(A, B, c, B_its)
            c = yield A_result | B_result

    return _seq


def choice(A: Combinator, B: Combinator) -> Combinator:
    def _choice(c: u8, d: Data) -> Generator[ParserIterationResult, u8, None]:
        its = [A(c, d), B(c, d)]
        while its:
            c = yield process(A, B, c, its)

    return _choice


def eat_u8(value: u8) -> Combinator:
    def _eat_u8(c: u8, d: Data) -> Generator[ParserIterationResult, u8, None]:
        if c != value:
            raise ParseError(f"Expected {value}, got {c}")
        yield ParserIterationResult(U8Set.none(), True)

    return _eat_u8


def eat_u8_range(start: u8, end: u8) -> Combinator:
    def _eat_u8_range(c: u8, d: Data) -> Generator[ParserIterationResult, u8, None]:
        if start <= c <= end:
            yield ParserIterationResult(U8Set.none(), True)
        else:
            raise ParseError(f"Expected {start}-{end}, got {c}")

    return _eat_u8_range


def eat_u8_range_complement(start: u8, end: u8) -> Combinator:
    def _eat_u8_range_complement(c: u8, d: Data) -> Generator[ParserIterationResult, u8, None]:
        if start <= c <= end:
            raise ParseError(f"Expected not {start}-{end}, got {c}")
        yield ParserIterationResult(U8Set.none(), True)

    return _eat_u8_range_complement
