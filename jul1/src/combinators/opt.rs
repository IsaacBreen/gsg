--- a/jul1/src/combinators/opt.rs
+++ b/jul1/src/combinators/opt.rs
@@ -26,6 +26,18 @@
         }
         (parser, parse_results)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        let (parser, mut parse_results) = self.inner.unambiguous_parse(right_data.clone(), bytes);
+        if !(self.greedy && parse_results.is_unambiguous()) {
+            // TODO: remove the condition below. It's a hack.
+            if parse_results.right_data_vec.is_empty() {  // TODO: remove this line
+                parse_results.right_data_vec.push(right_data);
+            }
+        }
+        (parser, parse_results)
+    }
+
 }
 
 pub fn opt<T: IntoCombinator>(a: T) -> Opt<T::Output> {

