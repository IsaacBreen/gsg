// flick between seq.rs and choice.rs to see how similar they are.
use crate::combinators::r#enum::EnumCombinator;
use crate::parsers::ParserState;
use crate::Readu8::Readu8;
use crate::u8set::u8set;

#[derive(Clone)]
pub struct SeqState<EC: EnumCombinator> {
    // initial_position: usize,
    state: EC,
}

impl<EC: EnumCombinator> ParserState for SeqState<EC> {
    fn new(position: usize) -> Self {
        SeqState {
            // initial_position: position,
            state: EC::new(position),
        }
    }

    fn parse<F: Readu8>(&mut self, reader: &F) {
        while {
            self.state.parse(reader);
            self.state.is_valid() && self.state.init_next(self.state.position())
        } {}
    }

    fn valid_next_u8set(&self) -> u8set {
        self.state.valid_next_u8set()
    }

    fn position(&self) -> usize {
        self.state.position()
    }
}

macro_rules! seq {
    // Up to 16
    ($T1:ident, $T2:ident) => { SeqState<EnumCombinator2<$T1, $T2>> };
    ($T1:ident, $T2:ident, $T3:ident) => { SeqState<EnumCombinator3<$T1, $T2, $T3>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident) => { SeqState<EnumCombinator4<$T1, $T2, $T3, $T4>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident) => { SeqState<EnumCombinator5<$T1, $T2, $T3, $T4, $T5>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident) => { SeqState<EnumCombinator6<$T1, $T2, $T3, $T4, $T5, $T6>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident) => { SeqState<EnumCombinator7<$T1, $T2, $T3, $T4, $T5, $T6, $T7>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident) => { SeqState<EnumCombinator8<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident, $T9:ident) => { SeqState<EnumCombinator9<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8, $T9>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident, $T9:ident, $T10:ident) => { SeqState<EnumCombinator10<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8, $T9, $T10>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident, $T9:ident, $T10:ident, $T11:ident) => { SeqState<EnumCombinator11<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8, $T9, $T10, $T11>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident, $T9:ident, $T10:ident, $T11:ident, $T12:ident) => { SeqState<EnumCombinator12<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8, $T9, $T10, $T11, $T12>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident, $T9:ident, $T10:ident, $T11:ident, $T12:ident, $T13:ident) => { SeqState<EnumCombinator13<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8, $T9, $T10, $T11, $T12, $T13>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident, $T9:ident, $T10:ident, $T11:ident, $T12:ident, $T13:ident, $T14:ident) => { SeqState<EnumCombinator14<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8, $T9, $T10, $T11, $T12, $T13, $T14>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident, $T9:ident, $T10:ident, $T11:ident, $T12:ident, $T13:ident, $T14:ident, $T15:ident) => { SeqState<EnumCombinator15<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8, $T9, $T10, $T11, $T12, $T13, $T14, $T15>> };
    ($T1:ident, $T2:ident, $T3:ident, $T4:ident, $T5:ident, $T6:ident, $T7:ident, $T8:ident, $T9:ident, $T10:ident, $T11:ident, $T12:ident, $T13:ident, $T14:ident, $T15:ident, $T16:ident) => { SeqState<EnumCombinator16<$T1, $T2, $T3, $T4, $T5, $T6, $T7, $T8, $T9, $T10, $T11, $T12, $T13, $T14, $T15, $T16>> };
}