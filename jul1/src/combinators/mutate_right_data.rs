// use std::any::Any;
// use std::rc::Rc;
// use crate::*;
//
// #[derive(PartialEq)]
// pub struct MutateRightData<F: Fn(&mut RightData) -> bool> {
//     pub run: Rc<F>,
// }
//
// impl<F: Fn(&mut RightData) -> bool + 'static> CombinatorTrait for MutateRightData<F> {
//     fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
//         if (self.run)(&mut right_data) {
//             (MutateRightData { run: self.run.clone() }, ParseResults {
//                 right_data_vec: vec![right_data],
//                 up_data_vec: vec![],
//                 done: true
//             })
//         } else {
//             (MutateRightData { run: self.run.clone() }, ParseResults {
//                 right_data_vec: vec![],
//                 up_data_vec: vec![],
//                 done: true,
//             })
//         }
//     }
// }
//
// impl<F: Fn(&mut RightData) -> bool + 'static> ParserTrait for MutateRightData<F> {
//     fn step(&mut self, c: u8) -> ParseResults {
//         panic!("MutateRightData parser already consumed")
//     }
//
//     fn collect_stats(&self, stats: &mut Stats) {
//         todo!()
//     }
//
//     fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
//         todo!()
//     }
//
//     fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
//         todo!()
//     }
// }
//
// pub fn mutate_right_data<F: Fn(&mut RightData) -> bool>(run: F) -> MutateRightData<F> {
//     MutateRightData { run: Rc::new(run) }
// }
