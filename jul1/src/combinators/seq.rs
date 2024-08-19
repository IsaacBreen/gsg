--- a/jul1/src/combinators/seq.rs
+++ b/jul1/src/combinators/seq.rs
@@ -118,6 +118,112 @@
 
         (parser.into(), parse_results)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        let start_position = right_data.right_data_inner.fields1.position;
+
+        let mut combinator_index = self.start_index;
+
+        // if combinator_index >= self.children.len() {
+        //     return (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true));
+        // }
+
+        let combinator = &self.children[combinator_index];
+        let (parser, parse_results) = profile!("seq first child parse", {
+            combinator.unambiguous_parse(right_data, &bytes)
+        });
+        let done = parse_results.done();
+        if done && parse_results.right_data_vec.is_empty() {
+            // Shortcut
+            return (Parser::FailParser(FailParser), parse_results);
+        }
+        let mut parsers: Vec<(usize, Parser)> = if done {
+            vec![]
+        } else {
+            vec![(combinator_index, parser)]
+        };
+        let mut final_right_ VecY<RightData>;
+        let mut next_right_data_vec: VecY<RightData>;
+        if combinator_index + 1 < self.children.len() {
+            next_right_data_vec = parse_results.right_data_vec;
+            final_right_data = VecY::new();
+        } else {
+            next_right_data_vec = VecY::new();
+            final_right_data = parse_results.right_data_vec;
+        }
+
+        combinator_index += 1;
+
+        let mut helper = |right_ RightData, combinator_index: usize| {
+            let offset = right_data.right_data_inner.fields1.position - start_position;
+            let combinator = &self.children[combinator_index];
+            let (parser, parse_results) = profile!("seq other child parse", {
+                combinator.unambiguous_parse(right_data, &bytes[offset..])
+            });
+            if !parse_results.done() {
+                parsers.push((combinator_index, parser));
+            }
+            if combinator_index + 1 < self.children.len() {
+                parse_results.right_data_vec
+            } else {
+                final_right_data.extend(parse_results.right_data_vec);
+                VecY::new()
+            }
+        };
+
+        while combinator_index < self.children.len() && !next_right_data_vec.is_empty() {
+            if next_right_data_vec.len() == 1 {
+                let right_data = next_right_data_vec.pop().unwrap();
+                next_right_data_vec = helper(right_data, combinator_index);
+            } else {
+                let mut next_next_right_data_vec = VecY::new();
+                for right_data in next_right_data_vec {
+                    next_next_right_data_vec.extend(helper(right_data, combinator_index));
+                }
+                next_right_data_vec = next_next_right_data_vec;
+            }
+            combinator_index += 1;
+        }
+
+        if parsers.is_empty() {
+            return (Parser::FailParser(FailParser), UnambiguousParseResults::new(final_right_data, true));
+        }
+
+        let parser = Parser::SeqParser(SeqParser {
+            parsers,
+            combinators: &self.children,
+            position: start_position + bytes.len(),
+        });
+
+        let parse_results = UnambiguousParseResults::new(final_right_data, false);
+
+        (parser.into(), parse_results)
+    }
+    // fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+    //     let start_position = right_data.right_data_inner.fields1.position;
+    //
+    //     let mut combinator_index = self.start_index;
+    //
+    //     let combinator = &self.children[combinator_index];
+    //     let (parser, parse_results) = profile!("seq first child parse", {
+    //         combinator.unambiguous_parse(right_data, &bytes)
+    //     });
+    //     let done = parse_results.done();
+    //     if done && parse_results.right_data_vec.is_empty() {
+    //         // Shortcut
+    //         return (Parser::FailParser(FailParser), parse_results);
+    //     }
+    //     let mut parsers: Vec<(usize, Parser)> = if done {
+    //         vec![]
+    //     } else {
+    //         vec![(combinator_index, parser)]
+    //     };
+    //
+    //     (Parser::FailParser(FailParser), UnambiguousParseResults::empty())
+    // }
 }
 
 impl ParserTrait for SeqParser<'_> {

