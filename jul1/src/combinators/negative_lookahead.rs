--- a/jul1/src/combinators/negative_lookahead.rs
+++ b/jul1/src/combinators/negative_lookahead.rs
@@ -43,6 +43,10 @@
             start_position,
         }), parse_results)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for ExcludeBytestringsParser<'_> {

