from __future__ import annotations

import abc
import functools
import logging
from collections import defaultdict
from dataclasses import dataclass
from enum import Enum, auto
from io import StringIO
from typing import Self, Iterable, Callable, Any

import itertools
from tqdm import tqdm
import sys

# sys.setrecursionlimit(100)


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


def is_null(node: Node, null_rules: set[Ref]) -> bool:
    match node:
        case Ref(_):
            return node in null_rules
        case Term(_):
            return False
        case EpsExternal(_):
            return True
        case Lookahead(child):
            return True
        case Seq(children):
            return all(is_null(child, null_rules) for child in children)
        case Choice(children):
            return all(is_null(child, null_rules) for child in children)
        case Repeat1(child):
            return is_null(child, null_rules)
        case SepRep1(child, _):
            return is_null(child, null_rules) and is_null(node, null_rules)
        case _:
            raise ValueError(f"Unknown node type: {type(node)}")


def update_dict(original: dict, updates: dict) -> bool:
    common_keys = set(original.keys()) & set(updates.keys())
    updated = False
    for key in common_keys:
        if original[key] != updates[key]:
            updated = True
    original.update(updates)
    return updated


def update_set(original: set, updates: set) -> bool:
    initial_len = len(original)
    original.update(updates)
    return len(original) != initial_len


def add_to_set(s: set, element) -> bool:
    if element not in s:
        s.add(element)
        return True
    return False


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


def get_null_rules(rules: dict[Ref, Node]) -> set[Ref]:
    # Assume all rules are null
    null_rules = set(rules.keys())
    # Keep trying until we can't find anymore
    while True:
        prev_len = len(null_rules)
        for ref, node in rules.items():
            if not is_null(node, null_rules):
                null_rules -= {ref}
        if len(null_rules) == prev_len:
            break
    return null_rules


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
    yield node
    match node:
        case Seq(children) | Choice(children):
            for child in children:
                yield from iter_nodes(child)
        case Repeat1(child):
            yield from iter_nodes(child)
        case SepRep1(child, _):
            yield from iter_nodes(child)
            yield from iter_nodes(node)
        case EpsExternal(_) | Term(_) | Ref(_):
            pass
        case Lookahead(child):
            yield from iter_nodes(child)
        case _:
            raise ValueError(f"Unknown node type: {type(node)}")

def validate_rules(rules: dict[Ref, Node]) -> None:
    for ref, node in rules.items():
        for child in iter_nodes(node):
            assert isinstance(child, Node)
    rule_types = infer_rule_types(rules)
    for ref, rule_type in tqdm(rule_types.items(), desc="Validating rules"):
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

def assert_not_recursive(rules: dict[Ref, Node]) -> None:
    for ref, node in rules.items():
        assert_not_recursive_for_node(node)

def validate(f: Callable[..., Node]) -> Callable[..., Node]:
    # return f
    def wrapper(*args, **kwargs):
        x = f(*args, **kwargs)
        assert_not_recursive_for_node(x)
        return x
    return functools.wraps(f)(wrapper)

@validate
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

@validate
def intersperse_separator_for_node(node: Node, separator: Node, nullable_rules: set[Ref]) -> Node:
    try:
        match node:
            case Seq([]):
                return node
            case Seq(children):
                children = children.copy()
                # Actually, as long as we have a single non-nullable sequent, we're good.
                try:
                    i = [is_nullable(child, nullable_rules) for child in children].index(False)
                except ValueError as e:
                    e.add_note(f"There must be at least one non-nullable sequent in {node}")
                    raise

                # Insert a suffix for all children before i, and a prefix for all children after i.
                for j, child in enumerate(children):
                    if j < i:
                        children[j] = inject_suffix_on_nonnull_for_node(child, separator, nullable_rules)
                    elif j == i:
                        children[j] = intersperse_separator_for_node(child, separator, nullable_rules)
                    else:
                        children[j] = inject_prefix_on_nonnull_for_node(child, separator, nullable_rules)
                return seq(*children)

            case Choice(children):
                return Choice([intersperse_separator_for_node(child, separator, nullable_rules) for child in children])
            case Repeat1(child):
                return sep1(intersperse_separator_for_node(child, separator, nullable_rules), separator)
            case SepRep1(child, sep):
                sep = inject_prefix_on_nonnull_for_node(sep, separator, nullable_rules)
                sep = inject_suffix_on_nonnull_for_node(sep, separator, nullable_rules)
                return sep_rep1(child, sep)
            case Ref(_) | Term(_) | EpsExternal(_):
                return node
            case Lookahead(child, positive):
                return Lookahead(intersperse_separator_for_node(child, separator, nullable_rules), positive)
            case _:
                raise ValueError(f"Unexpected node: {node}")
    except ValueError as e:
        e.add_note(f"Error while interspersing separator in {node}")
        raise


@validate
def inject_prefix_on_nonnull_for_node(node: Node, prefix: Node, nullable_rules: set[Ref]) -> Node:
    try:
        if not is_nullable(node, nullable_rules):
            return seq(prefix, intersperse_separator_for_node(node, prefix, nullable_rules))
        match node:
            case Choice(children):
                return Choice([inject_prefix_on_nonnull_for_node(child, prefix, nullable_rules) for child in children])
            case Seq(children):
                children = [inject_prefix_on_nonnull_for_node(child, prefix, nullable_rules) for child in children]
                return seq(*children)
            case EpsExternal(_):
                return node
            case Lookahead(child, positive):
                return Lookahead(inject_prefix_on_nonnull_for_node(child, prefix, nullable_rules), positive)
            case _:
                raise ValueError(f"Unexpected nullable node: {node}")
    except ValueError as e:
        e.add_note(f"Error while adding prefix in {node}")
        raise


@validate
def inject_suffix_on_nonnull_for_node(node: Node, suffix: Node, nullable_rules: set[Ref]) -> Node:
    try:
        if not is_nullable(node, nullable_rules):
            return seq(intersperse_separator_for_node(node, suffix, nullable_rules), suffix)
        match node:
            case Choice(children):
                return Choice([inject_suffix_on_nonnull_for_node(child, suffix, nullable_rules) for child in children])
            case Seq(children):
                children = [inject_suffix_on_nonnull_for_node(child, suffix, nullable_rules) for child in children]
                return seq(*children)
            case EpsExternal(_):
                return node
            case Lookahead(child, positive):
                return Lookahead(inject_suffix_on_nonnull_for_node(child, suffix, nullable_rules), positive)
            case _:
                raise ValueError(f"Unexpected nullable node: {node}")
    except ValueError as e:
        e.add_note(f"Error while adding suffix in {node}")
        raise

def intersperse_separator(rules: dict[Ref, Node], separator: Node) -> dict[Ref, Node]:
    nullable_rules = get_nullable_rules(rules)
    new_rules = {}
    assert_not_recursive(rules)
    for ref, node in (bar := tqdm(rules.items(), desc="Interspersing separators")):
        assert_not_recursive(new_rules)
        assert_not_recursive_for_node(node)
        bar.set_description(f"Interspersing separators for {ref}")
        try:
            new_rules[ref] = intersperse_separator_for_node(node, separator, nullable_rules).simplify()
        except ValueError as e:
            e.add_note(f"Error while interspersing separator for rule {prettify_rule(ref, node)}")
            raise
    return new_rules


def get_firsts_for_node(node: Node, nullable_rules: set[Ref]) -> set[Ref | Term | EpsExternal]:
    match node:
        case Ref(_):
            return {node}
        case Term(_):
            return {node}
        case EpsExternal(_):
            return {node}
        case Lookahead(child):
            return set()
        case Seq(children):
            result = set()
            for child in children:
                result.update(get_firsts_for_node(child, nullable_rules))
                if not is_nullable(child, nullable_rules):
                    break
            return result
        case Choice(children):
            result = set()
            for child in children:
                result.update(get_firsts_for_node(child, nullable_rules))
            return result
        case Repeat1(child):
            return get_firsts_for_node(child, nullable_rules)
        case SepRep1(child, separator):
            result = get_firsts_for_node(child, nullable_rules)
            if is_nullable(child, nullable_rules):
                result.update(get_firsts_for_node(separator, nullable_rules))
            return result
        case _:
            raise ValueError(f"Unknown node type: {type(node)}")


def get_lasts_for_node(node: Node, nullable_rules: set[Ref]) -> set[Ref | Term | EpsExternal]:
    match node:
        case Ref(_):
            assert isinstance(node, Ref)
            return {node}
        case Term(_):
            assert isinstance(node, Term)
            return {node}
        case EpsExternal(_):
            assert isinstance(node, EpsExternal)
            return {node}
        case Lookahead(child):
            return set()
        case Seq(children):
            result = set()
            for child in reversed(children):
                result.update(get_lasts_for_node(child, nullable_rules))
                if not is_nullable(child, nullable_rules):
                    break
            return result
        case Choice(children):
            result = set()
            for child in children:
                result.update(get_lasts_for_node(child, nullable_rules))
            return result
        case Repeat1(child):
            return get_lasts_for_node(child, nullable_rules)
        case SepRep1(child, separator):
            result = get_lasts_for_node(child, nullable_rules)
            if is_nullable(child, nullable_rules):
                result.update(get_lasts_for_node(separator, nullable_rules))
            return result
        case _:
            raise ValueError(f"Unknown node type: {type(node)}")


def get_firsts2(rules: dict[Ref, Node]) -> dict[Ref, set[Ref | Term | EpsExternal]]:
    firsts: dict[Ref, set[Ref | Term | EpsExternal]] = {}
    nullable_rules = get_nullable_rules(rules)
    for ref, node in tqdm(rules.items(), desc="Computing firsts"):
        firsts[ref] = get_firsts_for_node(node, nullable_rules)
    # Substitute firsts for refs repeatedly until there are no more changes
    while True:
        old_firsts = firsts.copy()
        for ref in firsts:
            for first in list(firsts[ref]):
                if first in firsts:
                    firsts[ref].update(firsts[first])
        if not update_dict(firsts, old_firsts):
            break
    return firsts


def get_lasts(rules: dict[Ref, Node]) -> dict[Ref, set[Ref | Term | EpsExternal]]:
    lasts: dict[Ref, set[Ref | Term | EpsExternal]] = {}
    nullable_rules = get_nullable_rules(rules)
    for ref, node in tqdm(rules.items(), desc="Computing lasts"):
        lasts[ref] = get_lasts_for_node(node, nullable_rules)
    # Substitute lasts for refs repeatedly
    while True:
        old_lasts = lasts.copy()
        for ref in lasts:
            for last in list(lasts[ref]):
                if last in lasts:
                    lasts[ref].update(lasts[last])
        if not update_dict(lasts, old_lasts):
            break
    return lasts


def collect_follows_for_node(node: Node, nullable_rules: set[Ref]) -> dict[Ref | Term | EpsExternal, set[Ref | Term | EpsExternal]]:
    match node:
        case Ref(_):
            return {}
        case Term(_):
            return {}
        case EpsExternal(_):
            return {}
        case Lookahead(child):
            return {}
        case Seq(children):
            result: dict[Ref | Term | EpsExternal, set[Ref | Term | EpsExternal]] = {}
            for i, child in enumerate(children):
                lasts_for_child = get_lasts_for_node(child, nullable_rules)
                rest = Seq(children[i + 1:])
                firsts_for_rest = get_firsts_for_node(rest, nullable_rules)
                for last in lasts_for_child:
                    result.setdefault(last, set()).update(firsts_for_rest)
            return result
        case Choice(children):
            result: dict[Ref | Term | EpsExternal, set[Ref | Term | EpsExternal]] = {}
            for child in children:
                result.update(collect_follows_for_node(child, nullable_rules))
            return result
        case Repeat1(child):
            lasts = get_lasts_for_node(child, nullable_rules)
            firsts = get_firsts_for_node(child, nullable_rules)
            result: dict[Ref | Term | EpsExternal, set[Ref | Term | EpsExternal]] = {}
            for last in lasts:
                result[last] = firsts
            return result
        case SepRep1(child, separator):
            return collect_follows_for_node(seq(child, repeat0(seq(separator, child))), nullable_rules)
        case _:
            raise ValueError(f"Unknown node type: {type(node)}")


def gather_all_leaves(rules: dict[Ref, Node]) -> set[Ref | Term | EpsExternal]:
    result = set()
    for ref, node in rules.items():
        result.add(ref)
        result.update(x for x in iter_nodes(node) if isinstance(x, Ref | Term | EpsExternal))
    return result


def get_follows(rules: dict[Ref, Node]) -> dict[Ref | Term | EpsExternal, set[Ref | Term | EpsExternal]]:
    follow_sets: dict[Ref, set[Ref | Term | EpsExternal]] = {}
    nullable_rules = get_nullable_rules(rules)
    for ref, node in rules.items():
        for leaf, follow_set in collect_follows_for_node(node, nullable_rules).items():
            follow_sets.setdefault(leaf, set()).update(follow_set)
    firsts_for_node = {}
    for ref, node in rules.items():
        firsts_for_node[ref] = get_firsts_for_node(node, nullable_rules)
    for leaf in gather_all_leaves(rules):
        follow_sets.setdefault(leaf, set())
        firsts_for_node.setdefault(leaf, set())
    # Substitute follow sets for refs repeatedly
    while True:
        updated = False
        for ref, follow_set in follow_sets.items():
            queue = list(follow_set)
            while len(queue) > 0:
                node = queue.pop()
                if isinstance(node, Ref) and node in rules:
                    firsts = firsts_for_node[node]
                    new_follows = firsts - follow_set
                    queue.extend(new_follows)
                    updated |= update_set(follow_set, new_follows)
        for ref, node in rules.items():
            # The follow set of a rule should inherit the follow set of its lasts and vice versa
            lasts_for_rule = get_lasts_for_node(node, nullable_rules)
            for last in lasts_for_rule:
                updated |= update_set(follow_sets[last], follow_sets[ref])
                updated |= update_set(follow_sets[ref], follow_sets[last])
        if not updated:
            break
    return follow_sets


def map_left(node: Node, f: Callable[[Node], Node], nullable_rules: set[Ref]) -> Node:
    node = f(node)
    match node:
        case Seq(children):
            children = children.copy()
            for i, child in enumerate(children):
                children[i] = map_left(child, f, nullable_rules)
                if not is_nullable(child, nullable_rules):
                    break
            return Seq(children)
        case Choice(children):
            return Choice([map_left(child, f, nullable_rules) for child in children])
        case Repeat1(child):
            return Repeat1(map_left(child, f, nullable_rules))
        case SepRep1(child, sep):
            return SepRep1(map_left(child, f, nullable_rules), sep)
        case _:
            return node


def map_all_for_node(node: Node, f: Callable[[Node], Node]) -> Node:
    match node:
        case Seq(children):
            node = Seq([map_all_for_node(child, f) for child in children])
        case Choice(children):
            node = Choice([map_all_for_node(child, f) for child in children])
        case Repeat1(child):
            node = Repeat1(map_all_for_node(child, f))
        case SepRep1(child, sep):
            node = SepRep1(map_all_for_node(child, f), sep)
        case _:
            node = node
    node = f(node)
    return node


def map_all(rules: dict[Ref, Node], f: Callable[[Node], Node]) -> dict[Ref, Node]:
    return {ref: map_all_for_node(node, f) for ref, node in rules.items()}


def ensure_lasts_for_node(node: Node, includes: set[Ref | Term | EpsExternal], nullable_rules: set[Ref]) -> Node:
    match node:
        case Seq(children):
            raise NotImplementedError
        case Choice(children):
            return Choice([ensure_lasts_for_node(child, includes, nullable_rules) for child in children])
        case Repeat1(child):
            return seq(repeat0(child), ensure_lasts_for_node(child, includes, nullable_rules))
        case SepRep1(child, sep):
            return SepRep1(ensure_lasts_for_node(child, includes, nullable_rules), sep)
        case Ref(_) | Term(_) | EpsExternal(_):
            return node


def ensure_firsts_for_node(node: Node, includes: set[Ref | Term | EpsExternal], nullable_rules: set[Ref]) -> Node:
    raise NotImplementedError


def forbid_lasts_for_node(node: Node, excludes: set[Ref | Term | EpsExternal], nullable_rules: set[Ref]) -> Node:
    all_leaves = set(leaf for leaf in iter_nodes(node) if isinstance(leaf, Ref | Term | EpsExternal))
    includes = all_leaves - excludes
    return ensure_lasts_for_node(node, includes, nullable_rules)


def forbid_firsts_for_node(node: Node, excludes: set[Ref | Term | EpsExternal], nullable_rules: set[Ref]) -> Node:
    all_leaves = set(leaf for leaf in iter_nodes(node) if isinstance(leaf, Ref | Term | EpsExternal))
    includes = all_leaves - excludes
    return ensure_firsts_for_node(node, includes, nullable_rules)


def forbid_follows_for_node(node: Node, first: Ref | Term | EpsExternal, forbidden_follows: set[Ref | Term | EpsExternal], nullable_rules: set[Ref], null_rules: set[Ref]) -> Node:
    try:
        match node:
            case Seq(children):
                children = children.copy()
                for i, child in enumerate(children):
                    lasts_for_child = get_lasts_for_node(child, nullable_rules)
                    # Decide whether to trigger
                    if first in lasts_for_child:
                        # Put the subsequent `children[i + 1:]` in a new sequence called `rest`, and call the current one (`children[i]`) `child`.
                        #
                        # Replace the rest of this node with `seq(child, rest)`.
                        #
                        # Rewrite `seq(child, rest)` as a `choice(seq(child0, rest0), seq(child1, rest` where:
                        #
                        # - `child0` is a variant of `child` that hits `first` last and `rest0` is a variant of `rest` that doesn't hit anything in `forbidden_follows` first
                        # - `child1` is a variant of `child` that doesn't hit `first` last
                        #
                        # To make it clearer how this works, we can explicitly write out all four cases for `seq(child, rest)`:
                        #
                        # 1. `child` hits `first` last and `rest` hits anything in `forbidden_follows` first
                        # 2. `child` hits `first` last and `rest` hits doesn't hit anything in `forbidden_follows` first
                        # 4. `child` doesn't hit `first` last and `rest` hits anything in `forbidden_follows` first
                        # 3. `child` doesn't hit `first` last and `rest` doesn't hit anything in `forbidden_follows` first
                        #
                        # The case we want to eliminate is case 1 where a `forbidden_follow` follows the `first`. So, we eliminate case 1.
                        #
                        # To show how case 3 and 4 can be merged, let's denote
                        #
                        # - '`child` hits `first` last' by 'A'
                        # - '`child` doesn't hit `first` last' by 'not A'
                        # - '`rest` hits anything in `forbidden_follows` first' by 'B'.
                        # - '`rest` doesn't hit anything in `forbidden_follows` first' by 'not B'.
                        #
                        # Case 3 is of the form 'not A and B', while case 4 is of the form 'not A and not B', so we can merge them into a single case
                        # of the form:
                        #
                        # '(not A and B) or (not A and not B)' which is equivalent to
                        # 'not A and (B or not B)', which is equivalent to
                        # 'not A'
                        #
                        # 'not A' is just '`child` doesn't hit `first` last'. So, putting our merged version of case 3 and 4 together with case 1 gives us:
                        #
                        # - `child0` is a variant of `child` that hits `first` last and `rest0` is a variant of `rest` that doesn't hit anything in `forbidden_follows` first
                        # - `child1` is a variant of `child` that doesn't hit `first` last
                        rest = Seq(children[i + 1:])

                        child0 = forbid_lasts_for_node(child, forbidden_follows, nullable_rules)
                        child1 = ensure_lasts_for_node(child, forbidden_follows, nullable_rules)

                        rest0 = ensure_firsts_for_node(rest, forbidden_follows, nullable_rules)

                        new_node = choice(
                            seq(child0, rest0),
                            seq(child1, rest),
                        )
                        new_node = forbid_follows_for_node(new_node, first, forbidden_follows, nullable_rules, null_rules)
                        children = [children[:i], new_node]
                        return Seq(children)
                return Seq(children)
            case Choice(children):
                return Choice([forbid_follows_for_node(child, first, forbidden_follows, nullable_rules, null_rules) for child in children])
            case Repeat1(child):
                return Repeat1(forbid_follows_for_node(child, first, forbidden_follows, nullable_rules, null_rules))
            case SepRep1(child, sep):
                return SepRep1(forbid_follows_for_node(child, first, forbidden_follows, nullable_rules, null_rules), sep)
            case Ref(_) | Term(_) | EpsExternal(_):
                return node
            case Lookahead(_):
                return node
            case _:
                raise ValueError(f"Unexpected node: {node}")
    except ValueError as e:
        e.add_note(f"Error while forbidding follows for {node} with first {first} and forbidden follows {forbidden_follows}")
        raise


def forbid_follows(rules: dict[Ref, Node], forbidden_follows_table: dict[Ref | Term | EpsExternal, set[Ref | Term | EpsExternal]]) -> dict[Ref, Node]:
    nullable_rules = get_nullable_rules(rules)
    null_rules = get_null_rules(rules)
    firsts = get_firsts2(rules)
    lasts = get_lasts(rules)

    # Expand the forbidden follows table
    while True:
        updated = False
        for first, forbidden_follows in list(forbidden_follows_table.items()):
            for ref, last_set in lasts.items():
                if first in last_set:
                    # if len(last_set) > 1:
                    #     raise ValueError(f"Cannot forbid follows for {ref} with first {first} and forbidden follows {forbidden_follows} because the last set {last_set} for this rule has more than one last (how would we disambiguate?)")
                    # The rule for ref can end with the first for this row in the forbidden follows table.
                    # Any follows that are forbidden ƒor this first must also be forbidden for this ref.
                    updated |= update_set(forbidden_follows_table.setdefault(ref, set()), forbidden_follows)
            for forbidden_follow in list(forbidden_follows):
                if forbidden_follow in firsts:
                    updated |= update_set(forbidden_follows, firsts[forbidden_follow])
        if not updated:
            break

    for ref in tqdm(rules.keys(), desc="Forbidding follows"):
        for first, forbidden_follows in forbidden_follows_table.items():
            rules[ref] = forbid_follows_for_node(rules[ref], first, forbidden_follows, nullable_rules, null_rules)
        rules[ref] = rules[ref].simplify()
    return rules


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
            # if isinstance(child, Choice):
            #     _children.extend(child.children)
            # else:
            #     _children.append(child)
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

# Core combinators
def seq(*children: Node) -> Seq: return Seq(list(children))
def choice(*children: Node) -> Choice: return Choice(list(children))
def repeat1(child: Node) -> Repeat1: return Repeat1(child)
def sep_rep1(child: Node, separator: Node) -> SepRep1: return SepRep1(child, separator)
def ref(name: str) -> Ref: return Ref(name)
def term(value: str) -> Term: return Term(value)
def eps_external[T](data: T) -> EpsExternal[T]: return EpsExternal(data)
def lookahead(child: Node) -> Lookahead: return Lookahead(child, True)
def negative_lookahead(child: Node) -> Lookahead: return Lookahead(child, False)


# Derived combinators
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

    # Test interspersing separators
    rules = make_rules(
        A=seq(term('a'), seq(opt(term('b'))), opt(term('c')), term('d')),
    )
    prettify_rules(rules)
    prettify_rules(intersperse_separator(rules, ref('WS')))

    # Test get_follows
    rules = make_rules(
        A=seq(ref('B'), ref('C')),
        B=choice(term('a'), term('b')),
        C=choice(term('c'), term('d')),
    )
    prettify_rules(rules)
    print("follow sets:")
    print(get_follows(rules))
    print()

    # Test forbid_follows
    rules = make_rules(
        A=seq(ref('A'), opt(ref('B')), ref('C')),
        fstring=repeat1(ref('fstring_middle')),
        fstring_middle=choice(ref('fstring_replacement_field'), ref('FSTRING_MIDDLE')),
    )
    forbidden_follows_table = {
        ref('A'): {ref('B')},
        ref('FSTRING_MIDDLE'): {ref('FSTRING_MIDDLE')},
    }
    # todo: uncomment this when we've completed forbid_follows
    # print("after forbidding follows:")
    # prettify_rules(forbid_follows(rules, forbidden_follows_table))
    # print()

    # Test get_follows more
    rules = make_rules(
        params=repeat1(ref('param')),
        param=ref('NAME'),
    )
    prettify_rules(rules)
    print("follow sets:")
    for r, follow_set in get_follows(rules).items():
        print(f'{r} -> {follow_set}')
    assert ref('NAME') in get_follows(rules)[ref('NAME')]
    print()

    rules = make_rules(
        block=seq(opt(ref('WS')), ref('DEDENT')),
    )
    forbidden_follows_table = {
        ref('WS'): {ref('DEDENT')},
    }
    # todo: uncomment this when we've completed forbid_follows
    # print("after forbidding follows:")
    # prettify_rules(forbid_follows(rules, forbidden_follows_table))
    # print()

    rules = make_rules(
        A=seq(opt(ref('B')), ref('B')),
    )
    for r, follow_set in get_follows(rules).items():
        print(f'{r} -> {follow_set}')
    assert ref('B') in get_follows(rules)[ref('B')]

    rules = make_rules(
        fstring=repeat1(seq(opt(ref('WS')), ref('fstring_middle'))),
        fstring_middle=ref('FSTRING_MIDDLE'),
    )
    prettify_rules(rules)
    print("follow sets:")
    for r, follow_set in get_follows(rules).items():
        print(f'{r} -> {follow_set}')
    assert ref('FSTRING_MIDDLE') in get_follows(rules)[ref('fstring_middle')]

    # Intersperse separators with lookaheads
    rules = make_rules(
        A=seq(term('a'), lookahead(term('b')), term('c')),
    )
    print("before interspersing separators:")
    prettify_rules(rules)
    print("after interspersing separators:")
    prettify_rules(intersperse_separator(rules, ref('WS')))