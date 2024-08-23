// use std::fmt::Display;
//
// type ParseResult = Result<bool, String>;
//
// struct ParserWrapper<'a, T: ParserTrait>(&'a T);
//
// pub trait CombinatorTrait {
//     type Parser: ParserTrait;
//     fn init_parser<'a>(&'a self) -> ParserWrapper<'a, Self::Parser>;
// }
// pub trait ParserTrait {
//     fn parse(&mut self, c: char) -> ParseResult;
// }
//
// struct Eat {
//     c: char,
// }
// struct EatParser {
//     c: char,
// }
// impl CombinatorTrait for Eat {
//     type Parser = EatParser;
//     fn init_parser<'a>(&'a self) -> ParserWrapper<'a, Self::Parser> {
//         EatParser { c: self.c }
//     }
// }
// impl ParserTrait for EatParser {
//     fn parse(&mut self, c: char) -> ParseResult {
//         if c == self.c {
//             Ok(true)
//         } else {
//             Err(format!("Expected {}, got {}", self.c, c))
//         }
//     }
// }
//
// struct Seq<L: CombinatorTrait, R: CombinatorTrait> {
//     left: L,
//     right: R,
// }
// enum SeqParser<'a, L: CombinatorTrait, R: CombinatorTrait> where Self: 'a {
//     Left {
//         left: L::Parser,
//         right: &'a R,
//     },
//     Right {
//         right: R::Parser,
//     },
//     Done,
// }
// impl<L: CombinatorTrait, R: CombinatorTrait> ParserTrait for SeqParser<'_, L, R> {
//     fn parse(&mut self, c: char) -> ParseResult {
//         match self {
//             SeqParser::Left { left, right } => {
//                 let mut result = left.parse(c);
//                 if let Ok(true) = result {
//                     result = Ok(false);
//                     *self = SeqParser::Right {
//                         right: right.init_parser(),
//                     };
//                 } else {
//                     *self = SeqParser::Done;
//                 }
//                 result
//             }
//             SeqParser::Right { right } => {
//                 let result = right.parse(c);
//                 *self = SeqParser::Done;
//                 result
//             }
//             SeqParser::Done => {
//                 Err("Sequence already exhausted".to_string())
//             }
//         }
//     }
// }
// impl<'b, L: CombinatorTrait, R: CombinatorTrait> CombinatorTrait for Seq<L, R> {
//     type Parser = SeqParser<'b, L, R>;
//     fn init_parser<'a>(&'a self) -> ParserWrapper<'a, Self::Parser> {
//         SeqParser::Left {
//             left: self.left.init_parser(),
//             right: &self.right,
//         }
//     }
// }
//
// pub struct DynCombinator<T> {
//     inner: T,
// }
// pub struct DynParser<'a> {
//     inner: Box<dyn ParserTrait + 'a>,
// }
// impl<'b, T: CombinatorTrait> CombinatorTrait for DynCombinator<T> where Self: 'b {
//     type Parser = Box<dyn ParserTrait + 'b>;
//     fn init_parser<'a>(&'a self) -> ParserWrapper<'a, Self::Parser> {
//         let inner = self.inner.init_parser();
//         Box::new(inner)
//     }
// }
// impl ParserTrait for Box<dyn ParserTrait + '_> {
//     fn parse(&mut self, c: char) -> ParseResult {
//         (**self).parse(c)
//     }
// }
//
// // Helper functions
// fn eat(c: char) -> Eat {
//     Eat { c }
// }
// fn seq(left: impl CombinatorTrait, right: impl CombinatorTrait) -> impl CombinatorTrait {
//     Seq { left, right }
// }
// fn make_dyn(inner: impl CombinatorTrait) -> Box<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>>> {
//     Box::new(DynCombinator { inner })
// }
//
// #[test]
// fn test() {
//     let eat_a = eat('a');
//     let eat_b = eat('b');
//     let eat_ab = seq(eat_a, eat_b);
//     let dyn_eat_ab = make_dyn(eat_ab);
//
//     let mut parser = dyn_eat_ab.init_parser();
//     assert_eq!(parser.parse('a'), Ok(false));
//     assert_eq!(parser.parse('b'), Ok(true));
//
//     let mut parser = dyn_eat_ab.init_parser();
//     assert_eq!(parser.parse('a'), Ok(false));
//     assert_eq!(parser.parse('c'), Err("Expected b, got c".to_string()));
// }