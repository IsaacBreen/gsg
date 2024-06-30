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
        A_result = next(A_it)
        A_its = [A_it]
        B_its = []
        if A_result.end:
            B_it = B(d)
            B_result = next(B_it)
            B_its.append(B_it)
            A_result.end = False
            c = yield A_result | B_result
        else:
            c = yield A_result

        while A_its or B_its:
            A_result = process(c, A_its)
            B_result = process(c, B_its)
            if A_result.end:
                B_it = B(d)
                B_result |= next(B_it)
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
        c = yield next(A_it) | next(B_it)
        its = [A_it, B_it]
        while its:
            c = yield process(c, its)

    return _choice2


def choice(*args: Combinator) -> Combinator:
    return reduce(choice2, args)


def eat_u8(value: u8) -> Combinator:
    def _eat_u8(d: Data) -> Generator[ParserIterationResult, u8, None]:
        c = yield ParserIterationResult(U8Set.from_chars(value), False)
        yield ParserIterationResult(U8Set.none(), c == value)

    return _eat_u8


def eat_u8_range(start: u8, end: u8) -> Combinator:
    def _eat_u8_range(d: Data) -> Generator[ParserIterationResult, u8, None]:
        chars = [chr(c) for c in range(ord(start), ord(end) + 1)]
        c = yield ParserIterationResult(U8Set.from_chars(chars), False)
        yield ParserIterationResult(U8Set.none(), start <= c <= end)

    return _eat_u8_range


def eat_u8_range_complement(start: u8, end: u8) -> Combinator:
    def _eat_u8_range_complement(d: Data) -> Generator[ParserIterationResult, u8, None]:
        chars = [chr(c) for c in range(0, ord(start))] + [chr(c) for c in range(ord(end) + 1, 256)]
        c = yield ParserIterationResult(U8Set.from_chars(chars), False)
        yield ParserIterationResult(U8Set.none(), not (start <= c <= end))

    return _eat_u8_range_complement


def eat_string(value: str) -> Combinator:
    return seq(*[eat_u8(c) for c in value])


def repeat(A: Combinator) -> Combinator:
    def _repeat(d: Data) -> Generator[ParserIterationResult, u8, None]:
        A_it = A(d)
        result = next(A_it)
        result.end = True
        c = yield result
        its = [A_it]
        while its:
            result = process(c, its)
            if result.end:
                A_it = A(d)
                result |= next(A_it)
                its.append(A_it)
            c = yield result

    return _repeat


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
    # Helper combinators for JSON parsing
    whitespace = repeat(choice(eat_u8(" "), eat_u8("\t"), eat_u8("\n"), eat_u8("\r")))
    digit = eat_u8_range("0", "9")
    digits = repeat(digit)
    integer = seq(choice(eat_u8("-"), eat_u8("+")), digits)
    fraction = seq(eat_u8("."), digits)
    exponent = seq(choice(eat_u8("e"), eat_u8("E")), choice(eat_u8("+"), eat_u8("-"), eat_u8("")), digits)
    number = seq(integer, choice(fraction, eat_u8("")), choice(exponent, eat_u8("")))

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
                eat_u8("")
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
                eat_u8("")
            ),
            eat_u8("}")
        )(d)

    # Test cases
    json_parser = seq(whitespace, json_value, whitespace)

    def parse_json(json_string):
        it = json_parser(None)
        next(it)
        print(json_string[0])
        result = it.send(json_string[0])
        for char in json_string[1:]:
            print(char)
            print(result)
            result = it.send(char)
        print(result)
        return result.end

    assert parse_json('{"key": "value"}')
    assert parse_json('[1, 2, 3]')
    assert parse_json('{"nested": {"array": [1, 2, 3], "object": {"a": true, "b": false}}}')
    assert parse_json('42')
    assert parse_json('"Hello, world!"')
    assert parse_json('null')
    assert parse_json('true')
    assert parse_json('false')
    assert not parse_json('{"unclosed": "object"')
    assert not parse_json('[1, 2, 3')
    assert not parse_json('{"invalid": "json",}')

    print("All JSON tests passed successfully!")
