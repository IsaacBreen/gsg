from __future__ import annotations

from typing import Optional

from .u8set import U8Set


class Frame:
    def __init__(self, pos: Optional[set[str]] = None) -> None:
        self.pos = pos or set()

    def contains_prefix(self, name_prefix: str) -> bool:
        for name in self.pos:
            if name.startswith(name_prefix):
                return True
        return False

    def excludes_prefix(self, name_prefix: str) -> bool:
        return not self.contains_prefix(name_prefix)

    def contains(self, name: str) -> bool:
        return name in self.pos

    def excludes(self, name: str) -> bool:
        return not self.contains(name)

    def next_u8_given_contains(self, name: bytes) -> tuple[U8Set, bool]:
        u8set = U8Set.none()
        is_complete = False
        for existing_name in self.pos:
            existing_name = existing_name.encode()
            if len(name) <= len(existing_name) and existing_name[:len(name)] == name:
                if len(name) < len(existing_name):
                    u8set.insert(existing_name[len(name)])
                else:
                    is_complete = True
        return u8set, is_complete

    def next_u8_given_excludes(self, name: bytes) -> tuple[U8Set, bool]:
        raise NotImplementedError()

    def push_name(self, name: bytes) -> None:
        name = name.decode()
        assert name not in self.pos
        self.pos.add(name)

    def pop_name(self, name: bytes) -> None:
        name = name.decode()
        assert name in self.pos
        self.pos.remove(name)

    def __or__(self, other: Frame) -> Frame:
        return Frame(self.pos | other.pos)

    def __hash__(self) -> int:
        return hash(frozenset(self.pos))


class FrameStack:
    def __init__(self, frames: Optional[list[Frame]] = None) -> None:
        self.frames = frames or [Frame()]

    def contains_prefix(self, name_prefix: str) -> bool:
        for frame in self.frames:
            if frame.contains_prefix(name_prefix):
                return True
        return False

    def excludes_prefix(self, name_prefix: str) -> bool:
        return not self.contains_prefix(name_prefix)

    def contains(self, name: str) -> bool:
        for frame in self.frames:
            if frame.contains(name):
                return True
        return False

    def excludes(self, name: str) -> bool:
        return not self.contains(name)

    def next_u8_given_contains(self, name: bytes) -> tuple[U8Set, bool]:
        u8set = U8Set.none()
        is_complete = False
        for frame in reversed(self.frames):
            frame_u8set, frame_is_complete = frame.next_u8_given_contains(name)
            u8set |= frame_u8set
            is_complete |= frame_is_complete
        return u8set, is_complete

    def next_u8_given_excludes(self, name: bytes) -> tuple[U8Set, bool]:
        result_set = U8Set.all()
        is_complete = True
        for frame in reversed(self.frames):
            frame_set, frame_complete = frame.next_u8_given_excludes(name)
            result_set &= frame_set
            is_complete &= frame_complete
        return result_set, is_complete

    def push_frame(self, new_frame: Frame) -> None:
        self.frames.append(new_frame)

    def push_empty_frame(self) -> None:
        self.push_frame(Frame())

    def push_name(self, name: bytes) -> None:
        self.frames[-1].push_name(name)

    def pop_name(self, name: bytes) -> None:
        self.frames[-1].pop_name(name)

    def pop(self) -> None:
        self.frames.pop()

    def filter(self, predicate: callable) -> None:
        self.frames = [frame for frame in self.frames if predicate(frame)]

    def filter_contains(self, name: bytes) -> None:
        self.filter(lambda frame: frame.contains(name.decode()))

    def filter_excludes(self, name: bytes) -> None:
        self.filter(lambda frame: not frame.contains(name.decode()))

    def __or__(self, other: FrameStack) -> FrameStack:
        return FrameStack(self.frames + other.frames)


class ParserIterationResult:
    def __init__(self, u8set: U8Set, is_complete: bool, frame_stack: FrameStack) -> None:
        self.u8set = u8set
        self.is_complete = is_complete
        self.frame_stack = frame_stack

    @staticmethod
    def new_empty() -> ParserIterationResult:
        return ParserIterationResult(U8Set.none(), False, FrameStack())

    def u8set(self) -> U8Set:
        return self.u8set

    def merge(self, other: ParserIterationResult) -> ParserIterationResult:
        self.is_complete = self.is_complete or other.is_complete
        self.u8set |= other.u8set
        self.frame_stack = self.frame_stack | other.frame_stack
        return self

    def merge_assign(self, other: ParserIterationResult) -> None:
        self.merge(other)

    def forward(self, other: ParserIterationResult) -> ParserIterationResult:
        self.u8set |= other.u8set
        self.is_complete = other.is_complete
        self.frame_stack = other.frame_stack
        return self

    def forward_assign(self, other: ParserIterationResult) -> None:
        self.forward(other)
