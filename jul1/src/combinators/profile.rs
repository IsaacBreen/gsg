--- a/jul1/src/combinators/profile.rs
+++ b/jul1/src/combinators/profile.rs
@@ -117,6 +117,14 @@
             (parser, parse_results)
         })
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        profile!(&self.tag, {
+            let (parser, parse_results) = self.inner.unambiguous_parse(right_data, bytes);
+            let parser = Parser::ProfiledParser(ProfiledParser { inner: Box::new(parser), tag: self.tag.clone() });
+            (parser, parse_results)
+        })
+    }
 }
 
 impl ParserTrait for ProfiledParser<'_> {

