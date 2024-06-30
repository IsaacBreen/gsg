from dataclasses import dataclass

type u8 = str

@dataclass
class U256:
    def __init__(self):
        self.data = [0] * 8  # 8 32-bit integers

    @classmethod
    def zero(cls):
        return cls()

    def is_set(self, index):
        word, bit = divmod(index, 32)
        return bool(self.data[word] & (1 << bit))

    def set_bit(self, index):
        word, bit = divmod(index, 32)
        self.data[word] |= (1 << bit)

    def clear_bit(self, index):
        word, bit = divmod(index, 32)
        self.data[word] &= ~(1 << bit)

    def copy(self):
        result = U256()
        result.data = self.data.copy()
        return result


@dataclass
class U8Set:
    def __init__(self):
        self.bitset = U256.zero()

    def insert(self, value):
        was_set = self.bitset.is_set(value)
        self.bitset.set_bit(value)
        return not was_set

    def remove(self, value):
        was_set = self.bitset.is_set(value)
        self.bitset.clear_bit(value)
        return was_set

    def update(self, other):
        self.bitset.data = other.bitset.data

    def contains(self, value):
        return self.bitset.is_set(value)

    def len(self):
        return sum(bin(word).count('1') for word in self.bitset.data)

    def is_empty(self):
        return self.len() == 0

    def clear(self):
        self.bitset = U256.zero()

    @classmethod
    def all(cls):
        # All ones
        self = cls()
        for i in range(256):
            self.bitset.set_bit(i)
        return self

    @classmethod
    def none(cls):
        # All zeros
        self = cls()
        for i in range(256):
            self.bitset.clear_bit(i)
        return self

    @classmethod
    def from_chars(cls, chars):
        self = cls()
        for c in chars:
            self.insert(ord(c))
        return self

    def __or__(self, other):
        result = self.copy()
        result.update(other)
        return result

    def __and__(self, other):
        result = self.copy()
        result.clear()
        result.update(other)
        return result

    def copy(self):
        result = U8Set()
        result.bitset = self.bitset.copy()
        return result

    def __iter__(self):
        return U8SetIter(self.bitset)

class U8SetIter:
    def __init__(self, bitset):
        self.bitset = bitset
        self.index = 0

    def __next__(self):
        while self.index < 256:
            if self.bitset.is_set(self.index):
                value = self.index
                self.index += 1
                return value
            self.index += 1
        raise StopIteration()

# Example usage:
if __name__ == "__main__":
    s = U8Set()
    s.insert(5)
    s.insert(10)
    s.insert(15)
    print(list(s))  # Should print [5, 10, 15]
    print(s.contains(10))  # Should print True
    s.remove(10)
    print(s.contains(10))  # Should print False
    print(s.len())  # Should print 2