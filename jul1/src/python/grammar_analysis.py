from __future__ import annotations

import abc
import logging
from collections import defaultdict
from dataclasses import dataclass
from typing import Self

from tqdm import tqdm

logger = logging.getLogger(__name__)
# TODO: this is a hack to show logging output in PyCharm. There has to be a better way.
logger.info = print

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


def resolve_left_recursion_for_rule[T: Node](node: T, ref: Ref, replacements: dict[Ref, Node]) -> Node:
    node.replace_left_refs(replacements)
    first, recursive = node.decompose_on_left_recursion(ref)
    return seq(choice(first), repeat0(choice(recursive))).simplify()


def resolve_left_recursion(rules: dict[Ref, Node]) -> dict[Ref, Node]:
    """Resolves left recursion in the given grammar rules."""
    # 1. Identify and resolve direct left recursion
    rules = resolve_direct_left_recursion(rules)

    # 2. Identify and resolve indirect left recursion
    rules = resolve_indirect_left_recursion(rules)

    return rules


def resolve_direct_left_recursion(rules: dict[Ref, Node]) -> dict[Ref, Node]:
    """Resolves direct left recursion in the given grammar rules."""
    new_rules = {}
    for ref, node in tqdm(rules.items(), desc="Resolving direct left recursion"):
        if is_directly_left_recursive_for_node(node, ref) and ref not in new_rules:
            logger.info(f"Resolving direct left recursion for rule {ref}")
            new_rules[ref] = resolve_left_recursion_for_rule(node, ref, {})
        else:
            new_rules[ref] = node
    return new_rules


def resolve_indirect_left_recursion(rules: dict[Ref, Node]) -> dict[Ref, Node]:
    """Resolves indirect left recursion in the given grammar rules."""
    # 1. Find cycles of indirect left recursion
    cycles = find_indirect_left_recursive_cycles(rules)

    # 2. Merge cycles into disjoint sets
    disjoint_cycles = merge_cycles(cycles)

    # 3. Resolve left recursion within each disjoint set
    new_rules = rules.copy()
    for cycle in disjoint_cycles:
        logger.info(f"Resolving indirect left recursion for cycle {cycle}")
        new_rules.update(resolve_left_recursion_for_cycle(rules, cycle))

    return new_rules


def resolve_left_recursion_for_cycle(rules: dict[Ref, Node], cycle: set[Ref]) -> dict[Ref, Node]:
    """Resolves left recursion within a cycle of indirectly left-recursive rules."""
    new_rules = {}
    replacements = {}
    # Sort cycle by order of appearance in the grammar
    i_rule = {rule: i for i, rule in enumerate(rules.keys())}
    cycle = sorted(cycle, key=lambda rule: i_rule[rule])
    for ref in cycle:
        # Replace references to rules within the cycle
        node = rules[ref].replace_left_refs(replacements)
        # Resolve left recursion for the current rule
        new_rules[ref] = resolve_left_recursion_for_rule(node, ref, replacements)
        replacements[ref] = new_rules[ref]
    return new_rules


def merge_cycles(cycles: list[list[Ref]]) -> list[set[Ref]]:
    """Merges cycles of indirect left recursion into disjoint sets."""
    disjoint_cycles = []
    for cycle in cycles:
        merged = False
        for i in range(len(disjoint_cycles)):
            if any(ref in disjoint_cycles[i] for ref in cycle):
                disjoint_cycles[i].update(cycle)
                merged = True
                break
        if not merged:
            disjoint_cycles.append(set(cycle))
    return disjoint_cycles


def has_direct_left_recursion(rules: dict[Ref, Node]) -> bool:
    for cycle in find_all_left_recursive_cycles(rules):
        if len(set(cycle)) == 1:
            return True
    return False


def has_indirect_left_recursion(rules: dict[Ref, Node]) -> bool:
    for ref in rules:
        for cycle in find_left_recursive_cycles(rules, ref):
            if len(set(cycle)) > 1:
                return True
    return False


def is_directly_left_recursive_for_node(node: Node, ref: Ref) -> bool:
    cycles = find_left_recursive_cycles({}, node, [ref])
    return len(cycles) > 0


def find_all_left_recursive_cycles(rules: dict[Ref, Node]) -> list[list[Ref]]:
    cycles = []
    for ref in rules:
        for cycle in find_left_recursive_cycles(rules, ref):
            if cycle not in cycles:  # Only add unique cycles with length > 1
                cycles.append(cycle)
    return cycles


def find_indirect_left_recursive_cycles(rules: dict[Ref, Node]) -> list[list[Ref]]:
    for cycle in find_all_left_recursive_cycles(rules):
        if len(set(cycle)) > 1:
            return [cycle]
    return []


def find_left_recursive_cycles(rules: dict[Ref, Node], start_node: Node, path: Optional[list[Ref]] = None) -> list[list[Ref]]:
    if path is None:
        path = []

    def is_nullable(node: Node) -> bool:
        # This is a conservative estimate of whether a node is nullable.
        # It assumes no refs or terminals are nullable.
        if isinstance(node, Seq):
            # Nullable if all children are nullable
            return all(is_nullable(child) for child in node.children)
        elif isinstance(node, Choice):
            # Nullable if any choice is nullable
            return any(is_nullable(child) for child in node.children)
        elif isinstance(node, Repeat1):
            # Repeat1 is not nullable
            return False
        elif isinstance(node, SepRep1):
            # SepRep1 is not nullable
            return False
        elif isinstance(node, Ref):
            # We don't know if a reference is nullable without the rules
            return False  # Conservative estimate
        elif isinstance(node, Lookahead):
            # Lookaheads are always nullable
            return True
        elif isinstance(node, Term):
            # Terminals are not nullable
            return False
        elif isinstance(node, EpsExternal):
            # External epsilon nodes are nullable
            return True
        else:
            raise ValueError(f"Unknown node type: {type(node)}")

    def dfs(node: Node, path: list[Ref]) -> list[list[Ref]]:
        if isinstance(node, Ref):
            if node in path:
                cycle_start = path.index(node)
                return [path[cycle_start:] + [node]]
            elif node in rules:
                return dfs(rules[node], path + [node])
            else:
                return []
        elif isinstance(node, Seq) and node.children:
            cycles = []
            for child in node.children:
                cycles.extend(dfs(child, path))
                if not is_nullable(child):
                    return cycles
            return cycles
        elif isinstance(node, Choice):
            cycles = []
            for child in node.children:
                cycles.extend(dfs(child, path))
            return cycles
        elif isinstance(node, Repeat1):
            return dfs(node.child, path)
        elif isinstance(node, SepRep1):
            cycles = dfs(node.child, path)
            return cycles
        elif isinstance(node, Lookahead):
            return dfs(node.child, path)
        return []

    return dfs(start_node, path)


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


def analyze_rule_usage(rules: dict[Ref, Node]) -> dict[Ref, int]:
    """Analyzes the usage of each rule in the grammar."""
    usage_count = defaultdict(int)

    def count_usage(node: Node):
        if isinstance(node, Ref):
            usage_count[node] += 1
        elif isinstance(node, Seq) or isinstance(node, Choice):
            for child in node.children:
                count_usage(child)
        elif isinstance(node, Repeat1) or isinstance(node, SepRep1) or isinstance(node, Lookahead):
            count_usage(node.child)

    for rule in rules.values():
        count_usage(rule)

    return usage_count


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

    # Test resolving direct left recursion
    rules = make_rules(
        A=choice(seq(ref('A'), term('a')), term('b')),
    )
    print("Test resolving left recursion:")
    print("  Before:")
    prettify_rules(rules)

    print("  Checking for left recursion:")
    print("  Left-recursive cycles:")
    start = Ref('A')
    for cycle in find_left_recursive_cycles(rules, start):
        print(f"    {cycle}")

    rules = resolve_left_recursion(rules)
    print("  After:")
    prettify_rules(rules)
    print()

    # Test resolving indirect left recursion
    rules = make_rules(
        A=choice(seq(ref('B'), term('a')), term('b')),
        B=ref('A'),
    )
    print("Test resolving indirect left recursion:")
    print("  Before:")
    prettify_rules(rules)

    print("  Checking for left recursion:")
    print("  Left-recursive cycles:")
    start = Ref('A')
    for cycle in find_left_recursive_cycles(rules, start):
        print(f"    {cycle}")

    rules = resolve_left_recursion(rules)
    print("  After:")
    prettify_rules(rules)
    print()