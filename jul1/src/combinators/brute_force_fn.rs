--- a/jul1/src/combinators/brute_force_fn.rs
+++ b/jul1/src/combinators/brute_force_fn.rs
@@ -112,6 +112,19 @@
             ),
         }
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        let result = (self.run)(right_data.clone(), bytes);
+        let run = self.run.clone();
+        match convert_result(result) {
+            Ok(right_data) => (
+                Parser::FailParser(FailParser),
+                UnambiguousParseResults::new_single(right_data, true)
+            ),
+            Err(_) => (
+                Parser::BruteForceParser(BruteForceParser { run, right_ Some(right_data), bytes: bytes.to_vec() }),
+                UnambiguousParseResults::empty()
+            ),
+        }
+    }
 }
 
 impl ParserTrait for BruteForceParser {

