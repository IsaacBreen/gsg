--- a/jul1/src/combinators/fail.rs
+++ b/jul1/src/combinators/fail.rs
@@ -15,6 +15,10 @@
     fn parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults) {
         (Parser::FailParser(FailParser), ParseResults::empty_finished())
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        (Parser::FailParser(FailParser), UnambiguousParseResults::empty())
+    }
 }
 
 impl ParserTrait for FailParser {

