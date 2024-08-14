from __future__ import annotations

import abc
import functools
from collections import defaultdict
from dataclasses import dataclass
from enum import Enum, auto
from io import StringIO
from typing import Self, Iterable, Callable, Any

# Removed unused imports: tqdm, sys, itertools

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


def is_nullable(node: Node, nullable_rules: set[Ref]) -> bool:
    match node:
        case Ref(_):
            return node in nullable_rules
        case Term(_):
            return False
        case EpsExternal(_):
            return True
        case Lookahead(child):
            return True
        case Seq(children):
            return all(is_nullable(child, nullable_rules) for child in children)
        case Choice(children):
            return any(is_nullable(child, nullable_rules) for child in children)
        case Repeat1(child):
            return is_nullable(child, nullable_rules)
        case SepRep1(child, _):
            return is_nullable(child, nullable_rules)
        case _:
            raise ValueError(f"Unknown node type: {type(node)}")


def get_nullable_rules(rules: dict[Ref, Node]) -> set[Ref]:
    nullable_rules = set(rules.keys())
    while True:
        prev_len = len(nullable_rules)
        for ref, node in rules.items():
            if not is_nullable(node, nullable_rules):
                nullable_rules -= {ref}
        if len(nullable_rules) == prev_len:
            break
    return nullable_rules


def get_firsts(node: Node, nullable_rules: set[Ref]) -> set[Ref | Term | EpsExternal]:
    if isinstance(node, Term):
        return {node}
    elif isinstance(node, Ref):
        return {node}
    elif isinstance(node, EpsExternal):
        return {node}
    elif isinstance(node, Lookahead):
        return set()
    elif isinstance(node, Seq):
        result = set()
        for child in node.children:
            result.update(get_firsts(child, nullable_rules))
            if not is_nullable(child, nullable_rules):
                break
        return result
    elif isinstance(node, Choice):
        result = set()
        for child in node.children:
            result.update(get_firsts(child, nullable_rules))
        return result
    elif isinstance(node, Repeat1):
        return get_firsts(node.child, nullable_rules)
    elif isinstance(node, SepRep1):
        return get_firsts(node.child, nullable_rules)
    else:
        raise ValueError(f"Unknown node type: {type(node)}")


def first_refs(node: Node, nullable_rules: set[Ref]) -> set[Ref]:
    return {ref for ref in get_firsts(node, nullable_rules) if isinstance(ref, Ref)}


def validate(f: Callable[..., Node]) -> Callable[..., Node]:
    def wrapper(*args, **kwargs):
        x = f(*args, **kwargs)
        assert_not_recursive_for_node(x)
        return x
    return functools.wraps(f)(wrapper)


def assert_not_recursive_for_node(node: Node, seen: set[int] = None) -> None:
    seen = seen.copy() if seen is not None else set()
    if id(node) in seen:
        raise ValueError(f"Recursive node with id {id(node)}")
    seen.add(id(node))
    match node:
        case Seq(children):
            for child in children:
                assert_not_recursive_for_node(child, seen)
        case Choice(children):
            for child in children:
                assert_not_recursive_for_node(child, seen)
        case Repeat1(child):
            assert_not_recursive_for_node(child, seen)
        case SepRep1(child, sep):
            assert_not_recursive_for_node(child, seen)
            assert_not_recursive_for_node(sep, seen)
        case Ref(_):
            pass
        case Term(_):
            pass
        case EpsExternal(_):
            pass
        case Lookahead(child):
            assert_not_recursive_for_node(child, seen)
        case _:
            raise ValueError(f"Unknown node type: {type(node)}")

@validate
def resolve_left_recursion_for_rule[T: Node](node: T, ref: Ref, replacements: dict[Ref, Node]) -> Node:
    node.replace_left_refs(replacements)
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
        if len(self.children) == 0:
            return self, fail()
        else:
            first, recursive = self.children[0].decompose_on_left_recursion(ref)
            return Seq([first, *self.children[1:]]), Seq([recursive, *self.children[1:]])

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        if len(self.children) == 0:
            return self
        else:
            self.children[0] = self.children[0].replace_left_refs(replacements)
            return self

    def simplify(self) -> Node:
        children = [child.simplify() for child in self.children]
        if any(child == fail() for child in children):
            return fail()
        children = [child for child in children if child != eps()]

        _children = []
        for child in children:
            if isinstance(child, Seq):
                _children.extend(child.children)
            else:
                _children.append(child)
        children = _children

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

    def __str__(self) -> str:
        match self:
            case Seq([]):
                return '\u001b[90meps\u001b[0m()'
            case Seq([x0, Choice([Repeat1(Seq([sep, x1])), Seq([])])]) if x0 == x1:
                return f'\u001b[32msep1\u001b[0m({str(x0)}, {str(sep)})'
            case default:
                return f'\u001b[32mseq\u001b[0m({", ".join(str(child) for child in self.children)})'


class DumbHashable[T]:
    value: T
    def __init__(self, value: T): self.value = value
    def __hash__(self): return hash(type(self.value))
    def __eq__(self, other): return self.value == other.value


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
        if len(self.children) == 0:
            return self
        else:
            self.children = [child.replace_left_refs(replacements) for child in self.children]
            return self

    def simplify(self) -> Node:
        children = [child.simplify() for child in self.children]
        children = [child for child in children if child != fail()]
        _children = []
        for child in children:
            _children.append(child)
        children = _children
        if len(children) == 1:
            return children[0]

        def group_by_first_element(sequences: list[list[Node]]) -> dict[DumbHashable, list[list[Node]]]:
            groups = defaultdict(list)
            for seq in sequences:
                groups[DumbHashable(seq[0])].append(seq)
            return groups

        sequences = []
        for child in children:
            if isinstance(child, Seq):
                if len(child.children) > 0:
                    sequences.append(child.children)
            else:
                sequences.append([child])

        groups = group_by_first_element(sequences)
        new_children = []
        for first, group in groups.items():
            if len(group) == 1:
                new_children.append(seq(*group[0]))
            else:
                prefixes = []
                for i in range(min(len(g) for g in group)):
                    if len(set(DumbHashable(g[i]) for g in group)) == 1:
                        prefixes.append(group[0][i])
                    else:
                        break
                suffixes = [g[len(prefixes):] for g in group]
                new_children.append(seq(*prefixes, Choice([seq(*suffix) for suffix in suffixes])))

        new_children = [child.simplify() for child in new_children]

        if eps() in children:
            new_children.append(eps())

        if len(new_children) == 1:
            return new_children[0]

        return Choice(new_children)

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
        first, recursive = self.child.decompose_on_left_recursion(ref)
        child = self.child.simplify()
        first = first.simplify()
        recursive = recursive.simplify()
        if first == child:
            first = self
        else:
            first = seq(first, repeat0(child))
        if recursive == child:
            recursive = self
        else:
            recursive = seq(recursive, repeat0(child))
        return first, recursive

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        first = self.child.replace_left_refs(replacements).simplify()
        child = self.child.simplify()
        if first == child:
            return self
        else:
            return seq(first, repeat0(self.child))

    def simplify(self) -> Node:
        self.child = self.child.simplify()
        return self if self.child != fail() else fail()

    def copy(self) -> Self:
        return Repeat1(self.child.copy())

    def __str__(self) -> str:
        return f'\u001b[35mrepeat1\u001b[0m({str(self.child)})'


@dataclass
class SepRep1(Node):
    child: Node
    separator: Node

    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        first, recursive = self.child.decompose_on_left_recursion(ref)
        child = self.child.simplify()
        first = first.simplify()
        recursive = recursive.simplify()
        if first == child:
            first = self
        else:
            first = seq(first, repeat0(seq(self.separator, child)))
        if recursive == child:
            recursive = self
        else:
            recursive = seq(recursive, repeat0(seq(self.separator, child)))
        return first, recursive

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        first = self.child.replace_left_refs(replacements).simplify()
        child = self.child.simplify()
        if first == child:
            return self
        else:
            return seq(first, repeat0(seq(self.separator, child)))

    def simplify(self) -> Node:
        self.child = self.child.simplify()
        self.separator = self.separator.simplify()
        if self.child == fail():
            return fail()
        if self.separator == fail():
            return self.child
        if self.separator == self.child:
            return repeat1(self.child).simplify()
        if self.child == eps():
            return repeat0(self.separator).simplify()
        if self.separator == eps():
            return repeat1(self.child).simplify()
        return self

    def copy(self) -> Self:
        return SepRep1(self.child.copy(), self.separator.copy())

    def __str__(self) -> str:
        return f'\u001b[35msep_rep1\u001b[0m({str(self.child)}, {str(self.separator)})'


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


@dataclass(frozen=True)
class Term[T](Node):
    value: T

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


@dataclass(frozen=True)
class EpsExternal[T](Node):
    data: T

    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        return self, fail()

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        return self

    def simplify(self) -> Node:
        return self

    def copy(self) -> Self:
        return EpsExternal(self.data)

    def __str__(self) -> str:
        return f'eps(\u001b[36m{repr(self.data)}\u001b[0m)'


@dataclass
class Lookahead(Node):
    child: Node
    positive: bool

    def decompose_on_left_recursion(self, ref: Ref) -> tuple[Node, Node]:
        return self, fail()

    def replace_left_refs(self, replacements: dict[Ref, Node]) -> Node:
        return self

    def simplify(self) -> Node:
        self.child = self.child.simplify()
        return self if self.child != fail() else fail()

    def copy(self) -> Self:
        return Lookahead(self.child.copy(), self.positive)

    def __str__(self) -> str:
        if self.positive:
            return f'\u001b[35mlookahead\u001b[0m({str(self.child)})'
        else:
            return f'\u001b[35mnegative_lookahead\u001b[0m({str(self.child)})'

def seq(*children: Node) -> Seq: return Seq(list(children))
def choice(*children: Node) -> Choice: return Choice(list(children))
def repeat1(child: Node) -> Repeat1: return Repeat1(child)
def sep_rep1(child: Node, separator: Node) -> SepRep1: return SepRep1(child, separator)
def ref(name: str) -> Ref: return Ref(name)
def term(value: str) -> Term: return Term(value)
def eps_external[T](data: T) -> EpsExternal[T]: return EpsExternal(data)
def lookahead(child: Node) -> Lookahead: return Lookahead(child, True)
def negative_lookahead(child: Node) -> Lookahead: return Lookahead(child, False)

def eps() -> Seq: return Seq([])
def fail() -> Choice: return Choice([])
def opt(child: Node) -> Node: return choice(child, eps())
def repeat0(child: Node) -> Node: return opt(repeat1(child))
def sep1(child: Node, sep: Node) -> Node: return SepRep1(child, sep)
def sep0(child: Node, sep: Node) -> Node: return opt(sep1(child, sep))

def prettify_rule(ref: Ref, node: Node) -> str:
    s = StringIO()
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
    elif isinstance(node, Seq) and len(node.children) >= 2:
        s.write(f'{ref} -> \u001b[32mseq\u001b[0m(\n')
        for child in node.children:
            s.write(f'    {child},\n')
        s.write(')\n')
    else:
        s.write(f'{ref} -> {node}\n')
    return s.getvalue().strip()


def prettify_rules(rules: dict[Ref, Node]):
    s = StringIO()
    for ref, node in rules.items():
        s.write(prettify_rule(ref, node) + '\n')
    print(s.getvalue().strip())

if __name__ == '__main__':
    def make_rules(**kwargs):
        return {Ref(name): kwargs[name] for name in kwargs}

    # Test simplifying with common prefixes
    expr = choice(
        seq(term('a'), term('b'), term('c')),
        seq(term('a'), term('b'), term('d')),
        seq(term('b'), term('c'), term('d')),
        seq(term('b'), term('d')),
        term('e'),
        eps(),
    )
    print("Test simplifying with common prefixes:")
    print(f"  Before: {expr}")
    print(f"  After: {expr.simplify()}")
    print()

    # Test resolving left recursion
    rules = make_rules(
        A=choice(seq(ref('A'), term('a')), term('b')),
    )
    print("Test resolving left recursion:")
    print("  Before:")
    prettify_rules(rules)
    rules = resolve_left_recursion(rules)
    print("  After:")
    prettify_rules(rules)
    print()

    # Test nullable rules
    rules = make_rules(
        A=choice(seq(ref('B')), term('b')),
        B=choice(term('a'), eps()),
    )
    nullable_rules = get_nullable_rules(rules)
    print(f"Test nullable rules:")
    print(f"  Nullable rules: {nullable_rules}")
    assert nullable_rules == {Ref('A'), Ref('B')}
    print()