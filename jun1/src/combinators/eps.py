from ..combinator import Combinator
from ..parse_iteration_result import FrameStack, ParserIterationResult
from ..state import CombinatorState
from ..u8set import U8Set


class EpsState(CombinatorState):
    def __init__(self, frame_stack: FrameStack) -> None:
        self.frame_stack = frame_stack


class Eps(Combinator):
    def initial_state(self, signal_id: int, frame_stack: FrameStack) -> EpsState:
        return EpsState(frame_stack)

    def next_state(self, state: EpsState, c: Optional[str], signal_id: int) -> tuple[EpsState, int, ParserIterationResult]:
        return state, signal_id, ParserIterationResult(U8Set.none(), True, state.frame_stack)
