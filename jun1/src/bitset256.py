from typing import Optional


class BitSet256:
    def __init__(self, x: Optional[int] = None, y: Optional[int] = None) -> None:
        self.x = x or 0
        self.y = y or 0

    def is_set(self, index: int) -> bool:
        if index < 128:
            return self.x & (1 << index) != 0
        return self.y & (1 << (index - 128)) != 0

    def set_bit(self, index: int) -> None:
        if index < 128:
            self.x |= 1 << index
        else:
            self.y |= 1 << (index - 128)

    def clear_bit(self, index: int) -> None:
        if index < 128:
            self.x &= ~(1 << index)
        else:
            self.y &= ~(1 << (index - 128))

    def update(self, other: 'BitSet256') -> None:
        self.x |= other.x
        self.y |= other.y

    def __len__(self) -> int:
        return self.x.bit_count() + self.y.bit_count()

    def is_empty(self) -> bool:
        return self.x == 0 and self.y == 0

    def clear(self) -> None:
        self.x = 0
        self.y = 0

    @staticmethod
    def all() -> 'BitSet256':
        return BitSet256(x=(1 << 128) - 1, y=(1 << 128) - 1)

    @staticmethod
    def none() -> 'BitSet256':
        return BitSet256()

    def __or__(self, other: 'BitSet256') -> 'BitSet256':
        return BitSet256(self.x | other.x, self.y | other.y)

    def __and__(self, other: 'BitSet256') -> 'BitSet256':
        return BitSet256(self.x & other.x, self.y & other.y)

    def __invert__(self) -> 'BitSet256':
        return BitSet256(~self.x, ~self.y)

    def __ior__(self, other: 'BitSet256') -> 'BitSet256':
        self.x |= other.x
        self.y |= other.y
        return self

    def __iand__(self, other: 'BitSet256') -> 'BitSet256':
        self.x &= other.x
        self.y &= other.y
        return self
