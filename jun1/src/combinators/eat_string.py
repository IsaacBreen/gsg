from ..combinator import Combinator
from ..parse_iteration_result import FrameStack, ParserIterationResult
from ..state import CombinatorState
from ..u8set import U8Set


class EatStringState(CombinatorState):
    def __init__(self, index: int, frame_stack: FrameStack) -> None:
        self.index = index
        self.frame_stack = frame_stack


class EatString(Combinator):
    def __init__(self, string: str) -> None:
        self.string = string

    def initial_state(self, signal_id: int, frame_stack: FrameStack) -> EatStringState:
        return EatStringState(0, frame_stack)

    def next_state(self, state: EatStringState, c: Optional[str], signal_id: int) -> tuple[EatStringState, int, ParserIterationResult]:
        if state.index > len(self.string):
            return state, signal_id, ParserIterationResult(U8Set.none(), False, state.frame_stack)
        if state.index == len(self.string):
            return state, signal_id, ParserIterationResult(U8Set.none(), True, state.frame_stack)
        u8set = U8Set.from_chars(self.string[state.index])
        state.index += 1
        return state, signal_id, ParserIterationResult(u8set, False, state.frame_stack)
