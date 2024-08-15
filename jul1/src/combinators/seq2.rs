#[macro_export]
macro_rules! define_seq {
    ($name:ident, $($child:ident),+) => {
        #[derive(Debug)]
        pub struct $name<$($child),+>
        where
            $($child: CombinatorTrait),+
        {
            $(pub(crate) $child: Rc<$child>),+
        }

        impl<$($child),+> CombinatorTrait for $name<$($child),+>
        where
            $($child: CombinatorTrait + 'static),+
        {
            fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
                let start_position = right_data.right_data_inner.fields1.position;

                let mut parsers = Vec::new();
                let mut final_right_data = VecY::new();
                let mut next_right_data_vec = vecx![right_data];

                $(
                    let mut next_next_right_data_vec = VecY::new();
                    for right_data in next_right_data_vec {
                        let offset = right_data.right_data_inner.fields1.position - start_position;
                        let (parser, parse_results) = profile!(concat!("seq ", stringify!($name), " child parse"), {
                            self.$child.parse(right_data, &bytes[offset..])
                        });
                        if !parse_results.done() {
                            parsers.push((parsers.len(), parser));
                        }
                        next_next_right_data_vec.extend(parse_results.right_data_vec);
                    }
                    next_right_data_vec = next_next_right_data_vec;

                    if next_right_data_vec.is_empty() {
                        return (Parser::FailParser(FailParser), ParseResults::new(VecY::new(), true));
                    }
                )+

                final_right_data = next_right_data_vec;

                if parsers.is_empty() {
                    return (Parser::FailParser(FailParser), ParseResults::new(final_right_data, true));
                }

                let parser = Parser::SeqParser(SeqParser {
                    parsers,
                    combinators: Rc::new(vecx![$(Combinator::DynRc(self.$child.clone())),+]),
                    position: start_position + bytes.len(),
                });

                let parse_results = ParseResults::new(final_right_data, false);

                (parser.into(), parse_results)
            }
        }

        #[macro_export]
        macro_rules! $name {
            ($($child_expr:expr),+) => {
                $crate::$name {
                    $($child: Rc::new($child_expr)),+
                }
            };
        }
    };
}

define_seq!(Seq2, A, B);
define_seq!(Seq3, A, B, C);
define_seq!(Seq4, A, B, C, D);
// ... and so on