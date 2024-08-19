--- a/jul1/src/combinators/eps.rs
+++ b/jul1/src/combinators/eps.rs
@@ -15,6 +15,10 @@
     fn parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults) {
         (Parser::EpsParser(EpsParser), ParseResults::new_single(right_data, true))
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        (Parser::EpsParser(EpsParser), UnambiguousParseResults::new_single(right_data, true))
+    }
 }
 
 impl ParserTrait for EpsParser {

