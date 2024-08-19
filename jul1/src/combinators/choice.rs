--- a/jul1/src/combinators/choice.rs
+++ b/jul1/src/combinators/choice.rs
@@ -49,6 +49,26 @@
             f(child);
         }
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        let mut parsers = Vec::new();
+        let mut combined_results = UnambiguousParseResults::empty();
+
+        for child in self.children.iter() {
+            let (parser, parse_results) = child.unambiguous_parse(right_data.clone(), bytes);
+            if !parse_results.done() {
+                parsers.push(parser);
+            }
+            let discard_rest = self.greedy && parse_results.is_unambiguous();
+            combined_results.merge_assign(parse_results);
+            if discard_rest {
+                break;
+            }
+        }
+
+        (Parser::ChoiceParser(ChoiceParser { parsers, greedy: self.greedy }), combined_results)
+    }
 }
 
 impl ParserTrait for ChoiceParser<'_> {

