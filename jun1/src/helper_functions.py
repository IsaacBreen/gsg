from typing import cast

from .combinators import Choice, EatString, EatU8Matching, Eps, ForwardRef, Repeat1, Seq
from .parse_iteration_result import FrameStack
from .u8set import U8Set


def seq(*args) -> Seq:
    return Seq(list(args))


def repeat1(a: Combinator) -> Repeat1:
    return Repeat1(a)


def choice(*args) -> Choice:
    return Choice(list(args))


def eat_u8_matching(u8set: U8Set) -> EatU8Matching:
    return EatU8Matching(u8set)


def eat_u8(value: str) -> EatU8Matching:
    return eat_u8_matching(U8Set.from_chars(value))


def eat_u8_range(start: str, end: str) -> EatU8Matching:
    return eat_u8_matching(U8Set.from_range(ord(start), ord(end)))


def eat_string(value: str) -> EatString:
    return EatString(value)


def eps() -> Eps:
    return Eps()


def opt(a: Combinator) -> Choice:
    return choice(a, eps())


def repeat(a: Combinator) -> Choice:
    return opt(repeat1(a))


def forward_ref() -> ForwardRef:
    return ForwardRef()


def eat_u8_range_complement(start: str, end: str) -> Choice:
    return choice(eat_u8_range(chr(0), start), eat_u8_range(end, chr(255)))


def in_frame_stack(a: Combinator) -> Combinator:
    return a


def add_to_frame_stack(a: Combinator) -> Combinator:
    return a
