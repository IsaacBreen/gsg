use crate::parsers::ParserState;
use crate::Readu8::Readu8;
use crate::u8set::u8set;

pub(crate) trait EnumCombinator: ParserState {
    fn init_next(&mut self, position: usize) -> bool;
}

macro_rules! enum_combinator {
    ($EnumCombinatorX:ident, $T:ident, $($TRest:ident),+) => {
        #[derive(Clone)]
        pub enum $EnumCombinatorX<$T: ParserState, $($TRest: ParserState),+> {
            $T($T),
            $($TRest($TRest)),+
        }

        macro_rules! visit {
            ($self:expr, $state:ident => $action:expr) => {
                match $self {
                    $EnumCombinatorX::$T($state) => $action,
                    $($EnumCombinatorX::$TRest($state) => $action,)+
                }
            };
        }

        impl<$T: ParserState, $($TRest: ParserState),+> $EnumCombinatorX<$T, $($TRest),+> {
            pub fn init_next(&mut self, position: usize) -> bool {
                $(
                    if let $EnumCombinatorX::$TRest(state) = self {
                        if state.is_valid() {
                            let new_state = $TRest::new(position);
                            *self = $EnumCombinatorX::$TRest(new_state);
                            return true;
                        }
                    }
                )+
                false
            }
        }

        impl<$T: ParserState, $($TRest: ParserState),+> ParserState for $EnumCombinatorX<$T, $($TRest),+> {
            fn new(position: usize) -> Self { $EnumCombinatorX::$T($T::new(position)) }
            fn parse<F: Readu8>(&mut self, reader: &F) { visit!(self, state => state.parse(reader)); }
            fn valid_next_u8set(&self) -> u8set { visit!(self, state => state.valid_next_u8set()) }
            fn position(&self) -> usize { visit!(self, state => state.position()) }
        }
    };
}

enum_combinator!(EnumCombinator2, S1, S2);
enum_combinator!(EnumCombinator3, S1, S2, S3);
enum_combinator!(EnumCombinator4, S1, S2, S3, S4);
enum_combinator!(EnumCombinator5, S1, S2, S3, S4, S5);
enum_combinator!(EnumCombinator6, S1, S2, S3, S4, S5, S6);
enum_combinator!(EnumCombinator7, S1, S2, S3, S4, S5, S6, S7);
enum_combinator!(EnumCombinator8, S1, S2, S3, S4, S5, S6, S7, S8);
enum_combinator!(EnumCombinator9, S1, S2, S3, S4, S5, S6, S7, S8, S9);
enum_combinator!(EnumCombinator10, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10);
enum_combinator!(EnumCombinator11, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11);
enum_combinator!(EnumCombinator12, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12);
enum_combinator!(EnumCombinator13, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13);
enum_combinator!(EnumCombinator14, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14);
enum_combinator!(EnumCombinator15, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15);
enum_combinator!(EnumCombinator16, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16);

