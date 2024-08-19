--- a/jul1/src/combinators/fast.rs
+++ b/jul1/src/combinators/fast.rs
@@ -47,6 +47,10 @@
             (Parser::FastParserWrapper(FastParserWrapper { regex_state, right_ Some(right_data) }), ParseResults::new(right_data_vec, done))
         }
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for FastParserWrapper<'_> {

