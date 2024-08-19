--- a/jul1/src/combinators/seqn.rs
+++ b/jul1/src/combinators/seqn.rs
@@ -111,6 +111,108 @@
                 // (Parser::FailParser(FailParser), parse_results)
             }
         }
+
+            fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+                let start_position = right_data.right_data_inner.fields1.position;
+
+                let first_combinator = &self.$first;
+                let (first_parser, first_parse_results) = profile!(stringify!($seq_name, " first child parse"), {
+                    first_combinator.unambiguous_parse(right_data, &bytes)
+                });
+
+                let mut all_done = first_parse_results.done();
+                if all_done && first_parse_results.right_data_vec.is_empty() {
+                    // Shortcut
+                    return (Parser::FailParser(FailParser), first_parse_results);
+                }
+                let first_parser_vec = if all_done { vec![] } else { vec![first_parser] };
+
+                let mut next_right_data_vec = first_parse_results.right_data_vec;
+
+                fn helper<'a, T: CombinatorTrait>(right_ RightData, next_combinator: &'a T, bytes: &[u8], start_position: usize) -> (Parser<'a>, UnambiguousParseResults) {
+                    let offset = right_data.right_data_inner.fields1.position - start_position;
+                    profile!(stringify!($seq_name, " child parse"), {
+                        next_combinator.unambiguous_parse(right_data, &bytes[offset..])
+                    })
+                }
+
+                let mut seqn_parser = $seq_parser_name {
+                    combinator: self,
+                    $first: first_parser_vec,
+                    $($rest: vec![],)+
+                    position: start_position + bytes.len(),
+                };
+
+                // Macro to process each child combinator
+                $(
+                    if next_right_data_vec.is_empty() {
+                        // let mut parser = $seq_parser_name {
+                        //     combinator: self,
+                        //     $first: first_parser_vec,
+                        //     $($rest: vec![],)+
+                        //     position: start_position + bytes.len(),
+                        // };
+                        // todo: hack
+                        return (Parser::DynParser(Box::new(seqn_parser)), UnambiguousParseResults::empty());
+                        // return (Parser::FailParser(FailParser), ParseResults::empty(all_done));
+                    }
+
+                    let mut next_next_right_data_vec = VecY::new();
+                    for right_data in next_right_data_vec {
+                        let (parser, parse_results) = helper(right_data, &self.$rest, &bytes, start_position);
+                        if !parse_results.done() {
+                            all_done = false;
+                            seqn_parser.$rest.push(parser);
+                        }
+                        next_next_right_data_vec.extend(parse_results.right_data_vec);
+                    }