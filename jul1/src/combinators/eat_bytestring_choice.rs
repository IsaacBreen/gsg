--- a/jul1/src/combinators/eat_bytestring_choice.rs
+++ b/jul1/src/combinators/eat_bytestring_choice.rs
@@ -38,6 +38,10 @@
         let parse_results = parser.parse(bytes);
         (Parser::EatByteStringChoiceParser(parser), parse_results)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for EatByteStringChoiceParser<'_> {

