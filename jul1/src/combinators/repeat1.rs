--- a/jul1/src/combinators/repeat1.rs
+++ b/jul1/src/combinators/repeat1.rs
@@ -95,6 +95,64 @@
             ParseResults::new(right_data_vec, done)
         )
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        let start_position = right_data.right_data_inner.fields1.position;
+        let (parser, parse_results) = self.a.unambiguous_parse(right_data, bytes);
+        if parse_results.done() && parse_results.right_data_vec.is_empty() {
+            // Shortcut
+            return (parser, parse_results);
+        } else if parse_results.right_data_vec.is_empty() {
+            return (Parser::Repeat1Parser(Repeat1Parser {
+                a: &self.a,
+                a_parsers: vec![parser],
+                position: start_position + bytes.len(),
+                greedy: self.greedy
+            }), UnambiguousParseResults::new(vecy![], false));
+        }
+        let mut parsers = if parse_results.done() {
+            vec![]
+        } else {
+            vec![parser]
+        };
+        let mut all_prev_succeeded_decisively = parse_results.is_unambiguous();
+        let mut right_data_vec = parse_results.right_data_vec;
+
+        let mut next_right_data = right_data_vec.clone();
+        while next_right_data.len() > 0 {
+            for new_right_data in std::mem::take(&mut next_right_data) {
+                let offset = new_right_data.right_data_inner.fields1.position - start_position;
+                let (parser, parse_results) = self.a.unambiguous_parse(new_right_data, &bytes[offset..]);
+                if !parse_results.done() {
+                    parsers.push(parser);
+                }
+                all_prev_succeeded_decisively &= parse_results.is_unambiguous();
+                if self.greedy && all_prev_succeeded_decisively {
+                    right_data_vec.clear();
+                    parsers.clear();
+                }
+                // if !(self.greedy && parse_results.succeeds_decisively()) && parse_results.right_data_vec.len() > 0 && right_data_vec.len() > 0 {
+                //     println!("parse_results: {:?}", parse_results);
+                // }
+                next_right_data.extend(parse_results.right_data_vec);
+            }
+            if !right_data_vec.is_empty() && !next_right_data.is_empty() {
+                let end_pos = start_position + bytes.len();
+                let pos1 = right_data_vec[0].right_data_inner.fields1.position;
+                let pos2 = next_right_data[0].right_data_inner.fields1.position;
+                if end_pos < pos1 + 1000 || end_pos < pos2 + 1000 {
+                    right_data_vec.clear();
+                }
+            }
+            right_data_vec.extend(next_right_data.clone());
+        }
+
+        let done = parsers.is_empty();
+
+        (
+            Parser::Repeat1Parser(Repeat1Parser {
+                a: &self.a,
+                a_parsers: parsers,
+                position: start_position + bytes.len(),
+                greedy: self.greedy
+            }),
+            UnambiguousParseResults::new(right_data_vec, done)
+        )
+    }
 }
 
 impl ParserTrait for Repeat1Parser<'_> {

