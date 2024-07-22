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

