from typing import list

from ..combinator import Combinator
from ..parse_iteration_result import FrameStack, ParserIterationResult
from ..state import CombinatorState


class SeqState(CombinatorState):
    def __init__(self, its: list[list[CombinatorState]]) -> None:
        self.its = its


class Seq(Combinator):
    def __init__(self, combinators: list[Combinator]) -> None:
        self.combinators = combinators

    def initial_state(self, signal_id: int, frame_stack: FrameStack) -> SeqState:
        return SeqState([
            [combinator.initial_state(signal_id, frame_stack.clone())]
            if i == 0 else []
            for i, combinator in enumerate(self.combinators)
        ])

    def next_state(self, state: SeqState, c: Optional[str], signal_id: int) -> tuple[SeqState, int, ParserIterationResult]:
        a_result = ParserIterationResult.new_empty()
        for i, (combinator, its) in enumerate(zip(self.combinators, state.its)):
            for j in range(len(its)):
                state.its[i][j], signal_id, result = combinator.next_state(state.its[i][j], c, signal_id)
                if i == 0:
                    a_result.merge_assign(result)
                else:
                    if a_result.is_complete:
                        b_it = combinator.initial_state(signal_id, a_result.frame_stack.clone())
                        b_it, signal_id, b_result = combinator.next_state(b_it, None, signal_id)
                        state.its[i].append(b_it)
                        a_result.forward_assign(b_result)
                    a_result.merge_assign(result)
        return state, signal_id, a_result
