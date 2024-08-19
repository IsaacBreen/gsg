--- a/jul1/src/combinators/lookahead.rs
+++ b/jul1/src/combinators/lookahead.rs
@@ -54,6 +54,10 @@
             (Parser::FailParser(FailParser), ParseResults::empty_finished())
         }
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 pub fn lookahead(combinator: impl CombinatorTrait + 'static) -> impl CombinatorTrait {

