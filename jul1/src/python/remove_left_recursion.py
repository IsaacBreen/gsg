from __future__ import annotations

import abc
from dataclasses import dataclass
from enum import Enum, auto
from io import StringIO
from typing import Self


class Node(abc.ABC):
    @abc.abstractmethod
    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        raise NotImplementedError

    @abc.abstractmethod
    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        raise NotImplementedError

    @abc.abstractmethod
    def simplify(self) -> Node:
        raise NotImplementedError


class RuleType(Enum):
    LEFT_RECURSIVE = auto()
    NULLABLE = auto()
    NORMAL = auto()


def first_refs(node: Node, seen: set[Ref]) -> set[Ref]:
    if seen is None:
        seen = set()
    if isinstance(node, Term):
        return set()
    elif isinstance(node, Ref):
        if node in seen:
            return {node}
        seen.add(node)
        return first_refs(rules[node], seen)
    elif isinstance(node, Seq):
        result = set()
        for child in node.children:
            result.update(first_refs(child, seen))
            if not is_nullable(child, seen):
                break
        return result
    elif isinstance(node, Choice):
        result = set()
        for child in node.children:
            result.update(first_refs(child, seen))
        return result
    elif isinstance(node, Repeat1):
        return first_refs(node.child, seen)
    return set()


def is_nullable(node: Node, seen: set[Ref]) -> bool:
    if seen is None:
        seen = set()
    if isinstance(node, Seq):
        return all(is_nullable(child, seen) for child in node.children)
    elif isinstance(node, Choice):
        return any(is_nullable(child, seen) for child in node.children)
    elif isinstance(node, Repeat1):
        return is_nullable(node.child, seen)
    elif isinstance(node, Ref):
        if node in seen:
            return False
        seen.add(node)
        return is_nullable(rules[node], seen)
    elif isinstance(node, Seq) and len(node.children) == 0:
        return True
    return False


def infer_rule_types(rules: dict[Ref, Node]) -> dict[Ref, RuleType]:

    rule_types = {}
    for ref, node in rules.items():
        firsts = first_refs(node, set())
        if any(isinstance(symbol, Term) or symbol == ref for symbol in firsts):
            rule_types[ref] = RuleType.LEFT_RECURSIVE
        elif is_nullable(node, set()):
            rule_types[ref] = RuleType.NULLABLE
        else:
            rule_types[ref] = RuleType.NORMAL

    return rule_types

def validate_rules(rules: dict[Ref, Node]) -> None:
    rule_types = infer_rule_types(rules)

    for ref, node in rules.items():
        rule_type = rule_types[ref]
        firsts = first_refs(node, set())

        if rule_type == RuleType.LEFT_RECURSIVE:
            if any(rule_types[symbol] != RuleType.LEFT_RECURSIVE for symbol in firsts):
                raise ValueError(f"Invalid LEFT_RECURSIVE rule: {ref}")

        elif rule_type == RuleType.NULLABLE:
            if any(rule_types[symbol] == RuleType.NORMAL for symbol in firsts):
                raise ValueError(f"Invalid NULLABLE rule: {ref}")

        elif rule_type == RuleType.NORMAL:
            if any(rule_types[symbol] == RuleType.LEFT_RECURSIVE for symbol in firsts):
                raise ValueError(f"Invalid NORMAL rule: {ref}")

def resolve_left_recursion_for_rule[T: Node](node: T, ref: Ref, replacements: dict[Ref, Node]) -> Node:
    # Resolve indirect left recursion
    node.replace_left_refs(replacements)
    # Resolve direct left recursion
    first, recursive = node.decompose_on_left_recursion(ref)
    return seq(choice(first), repeat0(choice(recursive))).simplify()


def resolve_left_recursion(rules: dict[Ref, Node]) -> dict[Ref, Node]:
    replacements = {}
    for ref, node in rules.items():
        replacements[ref] = resolve_left_recursion_for_rule(node, ref, replacements)
    return replacements


@dataclass
class Seq(Node):
    children: list[Node]

    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        first, recursive = self.children[0].decompose_on_left_recursion(ref)
        return Seq([first, *self.children[1:]]), Seq([recursive, *self.children[1:]])

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        self.children[0] = self.children[0].replace_left_refs(replacements)
        return self

    def simplify(self) -> Node:
        # Simplify children
        children = [child.simplify() for child in self.children]
        # If there are any fail() children, then the whole thing is fail()
        if any(child == fail() for child in children):
            return fail()
        # Remove any eps() children
        children = [child for child in children if child != eps()]

        _children = []
        for child in children:
            if isinstance(child, Seq):
                _children.extend(child.children)
            else:
                _children.append(child)
        children = _children

        # Merge any subsequences like these into `Repeat1(A)`
        #   - A Choice([Repeat1(A), Eps()])
        #   - Choice([Repeat1(A), Eps()]) A
        #   - A Repeat1(A)
        #   - Repeat1(A) A
        # Keep going until there's no more changes.
        while True:
            for (ix, x), (iy, y) in zip(enumerate(children), enumerate(children[1:], start=1)):
                match (x, y):
                    case (A1, Choice([Repeat1(A2), Seq([])])) if A1 == A2:
                        children[ix] = Repeat1(A1)
                        children.pop(iy)
                    case (Choice([Repeat1(A2), Seq([])]), A1) if A1 == A2:
                        children[iy] = Repeat1(A1)
                        children.pop(ix)
                    case (A1, Repeat1(A2)) if A1 == A2:
                        children[ix] = Repeat1(A1)
                        children.pop(iy)
                    case (Repeat1(A2), A1) if A1 == A2:
                        children[iy] = Repeat1(A1)
                        children.pop(ix)
                    case _:
                        continue
                break
            else:
                break

        match children:
            case []:
                return eps()
            case [child]:
                return child
            case _:
                return Seq(children)

    def copy(self) -> Self:
        return Seq([child.copy() for child in self.children])

    def __str__(self) -> str:
        match self:
            case Seq([]):
                return '\u001b[90meps\u001b[0m()'
            case Seq([x0, Choice([Repeat1(Seq([sep, x1])), Seq([])])]) if x0 == x1:
                return f'\u001b[32msep1\u001b[0m({str(x0)}, {str(sep)})'
            case default:
                return f'\u001b[32mseq\u001b[0m({", ".join(str(child) for child in self.children)})'


@dataclass
class Choice(Node):
    children: list[Node]

    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        firsts, recursives = [], []
        for child in self.children:
            first, recursive = child.decompose_on_left_recursion(ref)
            firsts.append(first)
            recursives.append(recursive)
        return Choice(firsts), Choice(recursives)

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        self.children = [child.replace_left_refs(replacements) for child in self.children]
        return self

    def simplify(self) -> Node:
        # Simplify children
        children = [child.simplify() for child in self.children]
        # Remove any fail() children
        children = [child for child in children if child != fail()]
        _children = []
        for child in children:
            if isinstance(child, Choice):
                _children.extend(child.children)
            else:
                _children.append(child)
        children = _children
        match children:
            case [child]:
                return child
            case _:
                return Choice(children)

    def copy(self) -> Self:
        return Choice([child.copy() for child in self.children])

    def __str__(self) -> str:
        match self:
            case Choice([]):
                return '\u001b[90mfail\u001b[0m()'
            case Choice([Repeat1(child), Seq([])]):
                return f'\u001b[32mrepeat0\u001b[0m({str(child)})'
            case Choice([Seq([x0, Choice([Repeat1(Seq([sep, x1])), Seq([])])]), Seq([])]) if x0 == x1:
                return f'\u001b[32msep0\u001b[0m({str(x0)}, {str(sep)})'
            case Choice([child, Seq([])]):
                return f'\u001b[32mopt\u001b[0m({str(child)})'
            case default:
                return f'\u001b[33mchoice\u001b[0m({", ".join(str(child) for child in self.children)})'


@dataclass
class Repeat1(Node):
    child: Node

    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        return seq(self.child, repeat0(self.child)).decompose_on_left_recursion(ref)

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        return seq(self.child, repeat0(self.child)).replace_left_refs(replacements)

    def simplify(self) -> Node:
        self.child = self.child.simplify()
        return self if self.child != fail() else fail()

    def copy(self) -> Self:
        return Repeat1(self.child.copy())

    def __str__(self) -> str:
        return f'\u001b[35mrepeat1\u001b[0m({str(self.child)})'


@dataclass(frozen=True)
class Ref(Node):
    name: str

    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        if self == ref:
            return fail(), eps()
        else:
            return self, fail()

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        return replacements.get(self, self)

    def simplify(self) -> Node:
        return self

    def copy(self) -> Self:
        return Ref(self.name)

    def __str__(self) -> str:
        return f'ref(\u001b[31m{repr(self.name)}\u001b[0m)'


@dataclass
class Term(Node):
    value: str

    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        return self, fail()

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        return self

    def simplify(self) -> Node:
        return self

    def copy(self) -> Self:
        return Term(self.value)

    def __str__(self) -> str:
        return f'term(\u001b[36m{repr(self.value)}\u001b[0m)'


# Core combinators
def seq(*children: Node) -> Seq: return Seq(list(children))
def choice(*children: Node) -> Choice: return Choice(list(children))
def repeat1(child: Node) -> Repeat1: return Repeat1(child)
def ref(name: str) -> Ref: return Ref(name)
def term(value: str) -> Term: return Term(value)


# Derived combinators
def eps() -> Seq: return Seq([])
def fail() -> Choice: return Choice([])
def opt(child: Node) -> Node: return choice(child, eps())
def repeat0(child: Node) -> Node: return opt(repeat1(child))
def sep1(child: Node, sep: Node) -> Node: return seq(child, repeat0(seq(sep, child)))
def sep0(child: Node, sep: Node) -> Node: return opt(sep1(child, sep))


def prettify_rules(rules: dict[Ref, Node]):
    s = StringIO()
    for ref, node in rules.items():
        if isinstance(node, Choice):
            match node:
                case Choice([]):
                    s.write(f'{ref} -> {node}\n')
                case Choice([Repeat1(child), Seq([])]):
                    s.write(f'{ref} -> {node}\n')
                case Choice([Seq([x0, Choice([Repeat1(Seq([sep, x1])), Seq([])])]), Seq([])]) if x0 == x1:
                    s.write(f'{ref} -> {node}\n')
                case Choice([child, Seq([])]):
                    s.write(f'{ref} -> {node}\n')
                case default:
                    s.write(f'{ref} -> \u001b[33mchoice\u001b[0m(\n')
                    for child in node.children:
                        s.write(f'    {child},\n')
                    s.write(')\n')
        else:
            s.write(f'{ref} = {node}\n')
    print(s.getvalue().strip())


if __name__ == '__main__':
    def make_rules(**kwargs):
        return {Ref(name): kwargs[name] for name in kwargs}


    # Show off the pretty-printing
    rules = make_rules(
        A=seq(term('a'), term('b')),
        B=choice(term('x'), term('y')),
        C=repeat1(term('c')),
        D=ref('A'),
        E=eps(),
        F=fail(),
        G=opt(term('g')),
        H=repeat0(term('h')),
        I=sep1(term('i'), term(',')),
        J=sep0(term('j'), term(';'))
    )
    prettify_rules(rules)
    print()

    # Resolve left recursion
    rules = make_rules(
        A=choice(seq(ref('A'), term('a')), term('b')),
    )
    print(infer_rule_types(rules))
    validate_rules(rules)

    rules = resolve_left_recursion(rules)
    prettify_rules(rules)
    print(infer_rule_types(rules))
    validate_rules(rules)
    print()

