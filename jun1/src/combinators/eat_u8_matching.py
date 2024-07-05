from ..combinator import Combinator
from ..parse_iteration_result import FrameStack, ParserIterationResult
from ..state import CombinatorState
from ..u8set import U8Set


class EatU8MatchingState(CombinatorState):
    def __init__(self, state: int, frame_stack: FrameStack) -> None:
        self.state = state
        self.frame_stack = frame_stack


class EatU8Matching(Combinator):
    def __init__(self, u8set: U8Set) -> None:
        self.u8set = u8set

    def initial_state(self, signal_id: int, frame_stack: FrameStack) -> EatU8MatchingState:
        return EatU8MatchingState(0, frame_stack)

    def next_state(self, state: EatU8MatchingState, c: Optional[str], signal_id: int) -> tuple[EatU8MatchingState, int, ParserIterationResult]:
        if state.state == 0:
            state.state = 1
            return state, signal_id, ParserIterationResult(self.u8set, False, state.frame_stack)
        if state.state == 1:
            state.state = 2
            is_complete = c is not None and self.u8set.contains(ord(c))
            return state, signal_id, ParserIterationResult(U8Set.none(), is_complete, state.frame_stack)
        raise Exception("EatU8Matching: state out of bounds")
