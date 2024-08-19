--- a/jul1/src/combinators/mod.rs
+++ b/jul1/src/combinators/mod.rs
@@ -18,6 +18,7 @@
 pub use opt::*;
 pub use seqn::*;
 pub use choicen::*;
+pub use unambiguous_parse_results::*;
 
 mod choice;
 mod eat_string;
@@ -45,6 +46,7 @@
 mod fast;
 mod seqn;
 mod choicen;
+mod unambiguous_parse_results;
 
 pub use choice::*;
 pub use eat_string::*;

