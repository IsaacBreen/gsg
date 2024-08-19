--- a/jul1/src/combinators/eat_string.rs
+++ b/jul1/src/combinators/eat_string.rs
@@ -37,6 +37,10 @@
         let parse_results = parser.parse(bytes);
         (Parser::EatStringParser(parser), parse_results)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for EatStringParser<'_> {

