--- a/jul1/src/combinator.rs
+++ b/jul1/src/combinator.rs
@@ -129,6 +129,7 @@
     }
     fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {}
     fn parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults);
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults);
     fn compile(mut self) -> Self where Self: Sized {
         self.compile_inner();
         self
@@ -175,6 +176,10 @@
     fn parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, ParseResults) {
         (**self).parse(right_data, bytes)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        (**self).unambiguous_parse(right_data, bytes)
+    }
 
     fn compile_inner(&self) {
         (**self).compile_inner();

