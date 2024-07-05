from typing import list

from ..combinator import Combinator
from ..parse_iteration_result import FrameStack, ParserIterationResult
from ..state import CombinatorState


class Repeat1State(CombinatorState):
    def __init__(self, a_its: list[CombinatorState]) -> None:
        self.a_its = a_its


class Repeat1(Combinator):
    def __init__(self, combinator: Combinator) -> None:
        self.combinator = combinator

    def initial_state(self, signal_id: int, frame_stack: FrameStack) -> Repeat1State:
        return Repeat1State([
            self.combinator.initial_state(signal_id, frame_stack)
        ])

    def next_state(self, state: Repeat1State, c: Optional[str], signal_id: int) -> tuple[Repeat1State, int, ParserIterationResult]:
        a_result = ParserIterationResult.new_empty()
        for i in range(len(state.a_its)):
            state.a_its[i], signal_id, result = self.combinator.next_state(state.a_its[i], c, signal_id)
            a_result.merge_assign(result)
        if a_result.is_complete:
            b_it = self.combinator.initial_state(signal_id, a_result.frame_stack.clone())
            b_it, signal_id, b_result = self.combinator.next_state(b_it, None, signal_id)
            state.a_its.append(b_it)
            a_result.forward_assign(b_result)
        a_result.merge_assign(ParserIterationResult(a_result.u8set, a_result.is_complete, a_result.frame_stack))
        return state, signal_id, a_result
