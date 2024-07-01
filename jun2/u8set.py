from __future__ import annotations

from dataclasses import dataclass
from typing import Iterator, Callable


@dataclass
class BitSet:
    x: int

    def is_set(self, index: int) -> bool:
        return bool(self.x & (1 << index))

    def set_bit(self, index: int) -> None:
        self.x |= (1 << index)

    def clear_bit(self, index: int) -> None:
        self.x &= ~(1 << index)

    def copy(self) -> 'BitSet':
        return BitSet(self.x)

@dataclass
class U8Set:
    bitset: BitSet

    def insert(self, value: int) -> bool:
        if not 0 <= value < 256:
            raise ValueError("Value must be between 0 and 255")
        if self.contains(value):
            return False
        self.bitset.set_bit(value)
        return True

    def remove(self, value: int) -> bool:
        if not 0 <= value < 256:
            raise ValueError("Value must be between 0 and 255")
        if not self.contains(value):
            return False
        self.bitset.clear_bit(value)
        return True

    def update(self, other: 'U8Set') -> None:
        self.bitset.x |= other.bitset.x

    def contains(self, value: int | str) -> bool:
        if isinstance(value, str):
            value = ord(value)
        if not 0 <= value < 256:
            raise ValueError("Value must be between 0 and 255")
        return self.bitset.is_set(value)

    def len(self) -> int:
        return bin(self.bitset.x).count('1')

    def is_empty(self) -> bool:
        return self.bitset.x == 0

    def clear(self) -> None:
        self.bitset.x = 0

    @classmethod
    def all(cls) -> 'U8Set':
        return cls(BitSet((1 << 256) - 1))

    @classmethod
    def none(cls) -> 'U8Set':
        return cls(BitSet(0))

    @classmethod
    def from_chars(cls, chars: str) -> 'U8Set':
        result = cls.none()
        for char in chars:
            result.insert(ord(char))
        return result

    @classmethod
    def from_match_fn(cls, fn: Callable[[int], bool]) -> 'U8Set':
        result = cls.none()
        for i in range(256):
            if fn(i):
                result.insert(i)
        return result

    def __or__(self, other: 'U8Set') -> 'U8Set':
        return U8Set(BitSet(self.bitset.x | other.bitset.x))

    def __and__(self, other: 'U8Set') -> 'U8Set':
        return U8Set(BitSet(self.bitset.x & other.bitset.x))

    def copy(self) -> 'U8Set':
        return U8Set(self.bitset.copy())

    def __iter__(self) -> Iterator[int]:
        for i in range(256):
            if self.bitset.is_set(i):
                yield i

    def __repr__(self) -> str:
        # return f"U8Set({set(self)})"
        return f"U8Set({set(chr(i) for i in self)})"

    def __str__(self) -> str:
        return repr(self)

    def __contains__(self, item):
        return self.contains(item)