# src/python/grammar_analysis.py
from __future__ import annotations

import abc
from collections import defaultdict
from dataclasses import dataclass
from typing import Self

from tqdm import tqdm


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

    @abc.abstractmethod
    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        raise NotImplementedError


def resolve_left_recursion_for_rule[T: Node](node: T, ref: Ref, replacements: dict[Ref, Node]) -> Node:
    node.replace_left_refs(replacements)
    first, recursive = node.decompose_on_left_recursion(ref)
    return seq(choice(first), repeat0(choice(recursive))).simplify()


def resolve_left_recursion(rules: dict[Ref, Node]) -> dict[Ref, Node]:
    replacements = {}
    for ref, node in tqdm(rules.items(), desc="Resolving left recursion"):
        replacements[ref] = resolve_left_recursion_for_rule(node, ref, replacements)
    return replacements


def count_refs_in_rules(rules: dict[Ref, Node]) -> dict[Ref, int]:
    ref_counts = defaultdict(int)
    for rule in rules.values():
        rule.count_refs(ref_counts)
    return ref_counts


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
        self.children = [child.replace_left_refs(replacements) for child in self.children]
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

        match children:
            case []:
                return eps()
            case [child]:
                return child
            case _:
                return Seq(children)

    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        for child in self.children:
            child.count_refs(ref_counts)

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

    def __init__(self, value: T):
        self.value = value

    def __hash__(self):
        return hash(type(self.value))

    def __eq__(self, other):
        return self.value == other.value


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

    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        for child in self.children:
            child.count_refs(ref_counts)

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

    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        self.child.count_refs(ref_counts)

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

    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        self.child.count_refs(ref_counts)
        self.separator.count_refs(ref_counts)

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

    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        ref_counts[self] += 1

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

    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        pass

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

    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        pass

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

    def count_refs(self, ref_counts: dict[Ref, int]) -> None:
        self.child.count_refs(ref_counts)

    def copy(self) -> Self:
        return Lookahead(self.child.copy(), self.positive)

    def __str__(self) -> str:
        if self.positive:
            return f'\u001b[35mlookahead\u001b[0m({str(self.child)})'
        else:
            return f'\u001b[35mnegative_lookahead\u001b[0m({str(self.child)})'


def seq(*children: Node) -> Node: return Seq(list(children))
def choice(*children: Node) -> Node: return Choice(list(children))
repeat1 = Repeat1
sep_rep1 = SepRep1
ref = Ref
term = Term
eps_external = EpsExternal
def lookahead(child: Node) -> Node: return Lookahead(child, True)
def negative_lookahead(child: Node) -> Node: return Lookahead(child, False)
def eps() -> Node: return Seq([])
def fail() -> Node: return Choice([])
def opt(child: Node) -> Node: return choice(child, eps())
def repeat0(child: Node) -> Node: return opt(repeat1(child))

def prettify_rules(rules: dict[Ref, Node]):
    for ref, node in rules.items():
        print(f'{ref} -> {node}')


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