macro_rules! define_seq {
    ($seq_name:ident, $($types:ident),+) => {
        #[derive(Debug)]
        pub struct $seq_name<$($types),+>
        where
            $($types: CombinatorTrait),+,
        {
            pub(crate) combinators: ( $(Rc<$types>),+ ),
        }

        impl<$($types),+> CombinatorTrait for $seq_name<$($types),+>
        where
            $($types: CombinatorTrait + 'static),+
        {
            fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
                let start_position = right_data.right_data_inner.fields1.position;

                let (first_parser, first_parse_results) = profile!(concat!(stringify!($seq_name), " first child parse"), {
                    self.combinators.0.parse(right_data, &bytes)
                });

                let done = first_parse_results.done();
                if done && first_parse_results.right_data_vec.is_empty() {
                    return (Parser::FailParser(FailParser), first_parse_results);
                }

                let mut parsers = if done {
                    vec![]
                } else {
                    vec![(0, first_parser)]
                };

                let mut final_right_data = VecY::new();
                let mut next_right_data_vec = first_parse_results.right_data_vec;

                fn helper<T: CombinatorTrait>(right_data: RightData, next_combinator: &Rc<T>, bytes: &[u8], start_position: usize, parsers: &mut Vec<(usize, Parser)>) -> VecY<RightData> {
                    let offset = right_data.right_data_inner.fields1.position - start_position;
                    let (parser, parse_results) = profile!("Child Parser", {
                        next_combinator.parse(right_data, &bytes[offset..])
                    });
                    if !parse_results.done() {
                        parsers.push((1, parser));
                    }
                    parse_results.right_data_vec
                }

                let mut next_next_right_data_vec = VecY::new();
                let combinator_tuple = &self.combinators;

                // Iterate over each combinator except the first one
                // We'll use recursion to achieve this

                #[allow(non_snake_case)]
                macro_rules! parse_recursive {
                    // Base case: when there is no more combinator in the tuple to parse
                    (@next $right_data_vec:expr, $final_right_data:expr, $start_position:expr, $bytes:expr, $parsers:expr, ) => {
                        $final_right_data = $right_data_vec;
                    };

                    // Recursive case: parse the next combinator
                    (@next $right_data_vec:expr, $final_right_data:expr, $start_position:expr, $bytes:expr, $parsers:expr, $Cn:ident, $($rest:ident,)* ) => {
                        for right_data in $right_data_vec {
                            next_next_right_data_vec.extend(helper(right_data, &combinator_tuple.$Cn, $bytes, $start_position, &mut $parsers));
                        }
                        let next_right_data_vec = next_next_right_data_vec;
                        let mut next_next_right_data_vec = VecY::new();
                        parse_recursive!(@next next_right_data_vec, $final_right_data, $start_position, $bytes, $parsers, $($rest,)*);
                    };
                }

                parse_recursive!(@next next_right_data_vec, final_right_data, start_position, bytes, parsers, 1, 2, 3,); // You need to put indices corresponding to the combinators


                if parsers.is_empty() {
                    return (Parser::FailParser(FailParser), ParseResults::new(final_right_data, true));
                }

                let parser = Parser::SeqParser(SeqParser {
                    parsers,
                    combinators: Rc::new(vecx![Combinator::Fail(Fail), Combinator::DynRc(self.combinators.1.clone())]), // Change to loop through combinators
                    position: start_position + bytes.len(),
                });

                let parse_results = ParseResults::new(final_right_data, false);

                (parser.into(), parse_results)
            }
        }

        #[macro_export]
        macro_rules! $seq_name {
            ($($combinator:expr),+) => {
                $crate::$seq_name {
                    combinators: ($($combinator),+),
                }
            };
        }
    };
}

// Use the macro to define Seq2, Seq3, etc.
define_seq!(Seq2, A, B);
define_seq!(Seq3, A, B, C);
// You could continue to define more if needed...