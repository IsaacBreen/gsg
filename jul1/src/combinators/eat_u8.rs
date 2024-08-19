--- a/jul1/src/combinators/eat_u8.rs
+++ b/jul1/src/combinators/eat_u8.rs
@@ -27,6 +27,10 @@
         let parse_results = parser.parse(bytes);
         (Parser::EatU8Parser(parser), parse_results)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for EatU8Parser {

