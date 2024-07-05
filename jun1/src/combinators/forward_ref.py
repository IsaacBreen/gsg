from typing import Optional, cast

from ..combinator import Combinator
from ..parse_iteration_result import FrameStack, ParserIterationResult
from ..state import CombinatorState


class ForwardRefState(CombinatorState):
    def __init__(self, inner_state: Optional[CombinatorState] = None) -> None:
        self.inner_state = inner_state


class ForwardRef(Combinator):
    def __init__(self, combinator: Optional[Combinator] = None) -> None:
        self.combinator = combinator

    def set(self, combinator: Combinator) -> None:
        self.combinator = combinator

    def initial_state(self, signal_id: int, frame_stack: FrameStack) -> ForwardRefState:
        if self.combinator is None:
            raise Exception("ForwardRef not set")
        return ForwardRefState(self.combinator.initial_state(signal_id, frame_stack))

    def next_state(self, state: ForwardRefState, c: Optional[str], signal_id: int) -> tuple[ForwardRefState, int, ParserIterationResult]:
        if state.inner_state is None or self.combinator is None:
            raise Exception("Forward reference not set before use")
        state.inner_state, signal_id, result = self.combinator.next_state(state.inner_state, c, signal_id)
        return state, signal_id, result
