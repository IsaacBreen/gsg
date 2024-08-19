--- a/jul1/src/combinators/continuation.rs
+++ b/jul1/src/combinators/continuation.rs
@@ -106,6 +106,10 @@
         // }
         todo!()
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for ContinuationParser {

