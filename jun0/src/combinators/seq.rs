// use crate::combinators::r#enum::{EnumCombinator};
// use crate::parsers::ParserState;
// use crate::Readu8::Readu8;
// use crate::u8set::u8set;
//
// #[derive(Clone)]
// pub struct SequenceState<S1: ParserState, S2: ParserState> {
//     state: EnumCombinator<S1, S2>,
//     next: Option<S2>,
// }
//
// impl<S1: ParserState, S2: ParserState> ParserState for SequenceState<S1, S2> {
//     fn new(position: usize) -> Self {
//         SequenceState {
//             state: EnumCombinator::new(position),
//             next: Some(S2::new(position)),
//         }
//     }
//
//     fn parse<F: Readu8>(&mut self, reader: &F) {
//         while {
//             self.state.parse(reader);
//             self.state.is_valid() && self.state.init_next(self.state.position())
//         } {}
//     }
//
//     fn valid_next_u8set(&self) -> u8set {
//         self.state.valid_next_u8set()
//     }
//
//     fn position(&self) -> usize {
//         self.state.position()
//     }
// }