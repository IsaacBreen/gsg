--- a/jul1/src/combinators/indent.rs
+++ b/jul1/src/combinators/indent.rs
@@ -127,6 +127,10 @@
         };
         (Parser::IndentCombinatorParser(parser), parse_results)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for IndentCombinatorParser<'_> {

