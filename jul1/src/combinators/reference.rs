--- a/jul1/src/combinators/reference.rs
+++ b/jul1/src/combinators/reference.rs
@@ -83,6 +83,11 @@
         let combinator = self.get().unwrap();
         combinator.parse(right_data, bytes)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        let combinator = self.get().unwrap();
+        combinator.unambiguous_parse(right_data, bytes)
+    }
 }
 
 impl<T: CombinatorTrait + 'static> CombinatorTrait for StrongRef<T> {
@@ -100,6 +105,13 @@
             .unwrap()
             .parse(right_data, bytes)
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        self.inner
+            .get()
+            .unwrap()
+            .unambiguous_parse(right_data, bytes)
+    }
 }
 
 pub fn strong_ref<T>() -> StrongRef<T> {

