--- a/jul1/src/combinators/cache.rs
+++ b/jul1/src/combinators/cache.rs
@@ -140,6 +140,10 @@
     fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
         f(&self.inner);
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for CacheContextParser<'_> {
@@ -220,6 +224,10 @@
     fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
         f(&self.inner);
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        todo!()
+    }
 }
 
 impl ParserTrait for CachedParser {

