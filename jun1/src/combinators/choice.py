from typing import list

from ..combinator import Combinator
from ..parse_iteration_result import FrameStack, ParserIterationResult
from ..state import CombinatorState
from ..u8set import U8Set


class ChoiceState(CombinatorState):
    def __init__(self, its: list[list[CombinatorState]]) -> None:
        self.its = its


class Choice(Combinator):
    def __init__(self, combinators: list[Combinator]) -> None:
        self.combinators = combinators

    def initial_state(self, signal_id: int, frame_stack: FrameStack) -> ChoiceState:
        return ChoiceState([
            [combinator.initial_state(signal_id, frame_stack.clone())]
            for combinator in self.combinators
        ])

    def next_state(self, state: ChoiceState, c: Optional[str], signal_id: int) -> tuple[ChoiceState, int, ParserIterationResult]:
        final_result = ParserIterationResult(U8Set.none(), False, FrameStack())
        for combinator, its in zip(self.combinators, state.its):
            new_its = []
            for it in its:
                it, signal_id, result = combinator.next_state(it, c, signal_id)
                if not result.u8set.is_empty():
                    new_its.append(it)
                final_result.merge_assign(result)
            its.clear()
            its.extend(new_its)
        return state, signal_id, final_result
