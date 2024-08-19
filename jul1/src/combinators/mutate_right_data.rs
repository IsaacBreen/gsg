--- a/jul1/src/combinators/mutate_right_data.rs
+++ b/jul1/src/combinators/mutate_right_data.rs
@@ -40,6 +40,14 @@
             (Parser::FailParser(FailParser), ParseResults::empty_finished())
         }
     }
+
+    fn unambiguous_parse(&self, mut right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        if (self.run)(&mut right_data) {
+            (Parser::FailParser(FailParser), UnambiguousParseResults::new_single(right_data, true))
+        } else {
+            (Parser::FailParser(FailParser), UnambiguousParseResults::empty())
+        }
+    }
 }
 
 pub fn mutate_right_data(run: impl Fn(&mut RightData) -> bool + 'static) -> MutateRightData {

