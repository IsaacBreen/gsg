from typing import Optional

from .combinator import Combinator
from .parse_iteration_result import FrameStack, ParserIterationResult


class ActiveCombinator:
    def __init__(self, combinator: Combinator, names: Optional[list[str]] = None) -> None:
        self.combinator = combinator
        self.signal_id = 0
        if names is not None:
            self.frame_stack = FrameStack()
            for name in names:
                self.frame_stack.push_name(name.encode())
            self.state = combinator.initial_state(self.signal_id, self.frame_stack)
        else:
            self.state = combinator.initial_state(self.signal_id, FrameStack())

    def send(self, c: Optional[str]) -> ParserIterationResult:
        self.state, self.signal_id, result = self.combinator.next_state(self.state, c, self.signal_id)
        return result
