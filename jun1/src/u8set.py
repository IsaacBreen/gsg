from __future__ import annotations

from typing import Iterator

from .bitset256 import BitSet256


class U8Set:
    def __init__(self, bitset: Optional[BitSet256] = None) -> None:
        self.bitset = bitset or BitSet256()

    @staticmethod
    def from_char(char: str) -> U8Set:
        return U8Set.from_chars(char)

    @staticmethod
    def from_range(start: int, end: int) -> U8Set:
        u8set = U8Set()
        for i in range(start, end + 1):
            u8set.insert(i)
        return u8set

    def insert(self, value: int) -> bool:
        if self.contains(value):
            return False
        self.bitset.set_bit(value)
        return True

    def remove(self, value: int) -> bool:
        if not self.contains(value):
            return False
        self.bitset.clear_bit(value)
        return True

    def update(self, other: U8Set) -> None:
        self.bitset.update(other.bitset)

    def contains(self, value: int) -> bool:
        return self.bitset.is_set(value)

    def __len__(self) -> int:
        return len(self.bitset)

    def is_empty(self) -> bool:
        return self.bitset.is_empty()

    def clear(self) -> None:
        self.bitset.clear()

    @staticmethod
    def all() -> U8Set:
        return U8Set(BitSet256.all())

    @staticmethod
    def none() -> U8Set:
        return U8Set()

    @staticmethod
    def from_chars(chars: str) -> U8Set:
        result = U8Set()
        for char in chars:
            result.insert(ord(char))
        return result

    def __iter__(self) -> Iterator[int]:
        for i in range(256):
            if self.contains(i):
                yield i

    def __or__(self, other: U8Set) -> U8Set:
        return U8Set(self.bitset | other.bitset)

    def __and__(self, other: U8Set) -> U8Set:
        return U8Set(self.bitset & other.bitset)

    def __ior__(self, other: U8Set) -> U8Set:
        self.bitset |= other.bitset
        return self

    def __iand__(self, other: U8Set) -> U8Set:
        self.bitset &= other.bitset
        return self

    def __repr__(self) -> str:
        return f"U8Set({', '.join(chr(i) for i in self)})"
