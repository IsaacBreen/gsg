from typing import Generic, Optional, TypeVar

from .parse_iteration_result import FrameStack, ParserIterationResult

State = TypeVar('State')


class Combinator(Generic[State]):
    def initial_state(self, signal_id: int, frame_stack: FrameStack) -> State:
        raise NotImplementedError()

    def next_state(self, state: State, c: Optional[str], signal_id: int) -> tuple[State, int, ParserIterationResult]:
        raise NotImplementedError()
