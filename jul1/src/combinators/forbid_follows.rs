--- a/jul1/src/combinators/forbid_follows.rs
+++ b/jul1/src/combinators/forbid_follows.rs
@@ -28,6 +28,11 @@
         Rc::make_mut(&mut right_data.right_data_inner).fields1.forbidden_consecutive_matches.prev_match_ids = self.match_ids;
         (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        Rc::make_mut(&mut right_data.right_data_inner).fields1.forbidden_consecutive_matches.prev_match_ids = self.match_ids;
+        (combinator::Parser::FailParser(FailParser), UnambiguousParseResults::new_single(right_data, true))
+    }
 }
 
 impl CombinatorTrait for ForbidFollowsClear {
@@ -38,6 +43,11 @@
         Rc::make_mut(&mut right_data.right_data_inner).fields1.forbidden_consecutive_matches.prev_match_ids = 0;
         (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        Rc::make_mut(&mut right_data.right_data_inner).fields1.forbidden_consecutive_matches.prev_match_ids = 0;
+        (combinator::Parser::FailParser(FailParser), UnambiguousParseResults::new_single(right_data, true))
+    }
 }
 
 impl CombinatorTrait for ForbidFollowsCheckNot {
@@ -52,6 +62,14 @@
             (combinator::Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
         }
     }
+
+    fn unambiguous_parse(&self, right_ RightData, bytes: &[u8]) -> (Parser, UnambiguousParseResults) {
+        if right_data.right_data_inner.fields1.forbidden_consecutive_matches.prev_match_ids & self.match_ids != 0 {
+            (combinator::Parser::FailParser(FailParser), UnambiguousParseResults::empty())
+        } else {
+            Rc::make_mut(&mut right_data.right_data_inner).fields1.forbidden_consecutive_matches.prev_match_ids = 0;
+            (combinator::Parser::FailParser(FailParser), UnambiguousParseResults::new_single(right_data, true))
+        }
+    }
 }
 
 pub fn forbid_follows(match_ids: &[usize]) -> ForbidFollows { // Using a bitset

