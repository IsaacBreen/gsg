from __future__ import annotations

import abc
from collections import defaultdict
from dataclasses import dataclass
from enum import Enum, auto
from io import StringIO
from typing import Self, Iterable


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
        case Seq(children):
            return all(is_nullable(child, nullable_rules) for child in children)
        case Choice(children):
            return any(is_nullable(child, nullable_rules) for child in children)
        case Repeat1(child):
            return is_nullable(child, nullable_rules)
        case _:
            raise ValueError(f"Unknown node type: {type(node)}")


def get_nullable_rules(rules: dict[Ref, Node]) -> set[Ref]:
    # Assume all rules are nullable
    nullable_rules = set(rules.keys())
    # Keep trying until we can't find anymore
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
    else:
        raise ValueError(f"Unknown node type: {type(node)}")


def first_refs(node: Node, nullable_rules: set[Ref]) -> set[Ref]:
    return {ref for ref in get_firsts(node, nullable_rules) if isinstance(ref, Ref)}


def is_left_recursive(node: Node, rules: dict[Ref, Node], seen: set[Ref] = None) -> bool:
    if seen is None:
        seen = set()
    firsts = first_refs(node, seen)
    for ref in firsts:
        if ref in seen:
            return True
        if ref not in rules:
            # If it's all capitalized, assume it's a token
            if ref.name[0].isupper():
                return True
            raise ValueError(f"Unknown rule: {ref}")
        if is_left_recursive(rules[ref], rules, seen | {ref}):
            return True
    return False


def infer_rule_types(rules: dict[Ref, Node]) -> dict[Ref, RuleType]:
    nullable_rules = get_nullable_rules(rules)
    rule_types = {}
    for ref, node in rules.items():
        if ref in nullable_rules:
            assert not is_left_recursive(node, rules)
            rule_types[ref] = RuleType.NULLABLE
        elif is_left_recursive(node, rules):
            rule_types[ref] = RuleType.LEFT_RECURSIVE
        else:
            rule_types[ref] = RuleType.NORMAL
    return rule_types


def iter_nodes(node: Node) -> Iterable[Node]:
    match node:
        case Seq(children) | Choice(children):
            yield node
            for child in children:
                yield from iter_nodes(child)
        case Repeat1(child):
            yield node
            yield from iter_nodes(child)
        case _:
            yield node

def validate_rules(rules: dict[Ref, Node]) -> None:
    for ref, node in rules.items():
        for child in iter_nodes(node):
            assert isinstance(child, Node)
    rule_types = infer_rule_types(rules)
    for ref, rule_type in rule_types.items():
        firsts = first_refs(rules[ref], get_nullable_rules(rules))
        def get_rule_type(ref: Ref, rule_types: dict[Ref, RuleType]) -> RuleType:
            # If it's all capitalized, assume it's a token
            if ref not in rules:
                if ref.name[0].isupper():
                    return RuleType.NORMAL
            return rule_types[ref]
        if rule_type == RuleType.LEFT_RECURSIVE:
            if any(get_rule_type(first_ref, rule_types) == RuleType.NULLABLE for first_ref in firsts):
                raise ValueError(f"Firsts for left-recursive rule must not be nullable. Found {firsts} for {rules[ref]}")
        elif rule_type == RuleType.NULLABLE:
            if any(get_rule_type(first_ref, rule_types) == RuleType.LEFT_RECURSIVE for first_ref in firsts):
                raise ValueError(f"Firsts for nullable rule must not be left-recursive. Found {firsts} for {rules[ref]}")
        elif rule_type == RuleType.NORMAL:
            if any(get_rule_type(first_ref, rule_types) == RuleType.LEFT_RECURSIVE for first_ref in firsts):
                raise ValueError(f"Firsts for non-nullable rule must not be left-recursive. Found {firsts} for {rules[ref]}")
        else:
            raise ValueError(f"Unknown rule type: {rule_type}")


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
        if len(self.children) == 0:
            return self, self
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
        if len(children) == 1:
            return children[0]

        # Merge any choices of sequences with a shared prefix.
        # It's hard to find the optimal merge strategy, but we do our best by employing a greedy strategy.
        # Convert each child into a list of sequents. If the child is not a sequence, wrap it in a singleton list.
        # Group each child sequence by its first element.
        # For each group, merge on their longest common prefix.
        # Recursively simplify the new children.
        # Ignore the order of children.
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


# Core combinators
def seq(*children: Node) -> Seq: return Seq(list(children))
def choice(*children: Node) -> Choice: return Choice(list(children))
def repeat1(child: Node) -> Repeat1: return Repeat1(child)
def ref(name: str) -> Ref: return Ref(name)
def term(value: str) -> Term: return Term(value)
def eps_external[T](data: T) -> EpsExternal[T]: return EpsExternal(data)


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

    # Test nullable rules
    rules = make_rules(
        A=choice(seq(ref('B')), term('b')),
        B=choice(term('a'), eps()),
    )
    nullable_rules = get_nullable_rules(rules)
    print(f"Nullable rules: {nullable_rules}")
    assert nullable_rules == {Ref('A'), Ref('B')}

    # Test simplifying with common prefixes
    expr = choice(
        seq(term('a'), term('b'), term('c')),
        seq(term('a'), term('b'), term('d')),
        seq(term('b'), term('c'), term('d')),
        seq(term('b'), term('d')),
        term('e'),
        eps(),
    )
    print(expr)
    print(expr.simplify())

    rules = make_rules(
        A=seq(opt(ref('B')), term('a')),
    )
    prettify_rules(rules)
    prettify_rules(resolve_left_recursion(rules))
