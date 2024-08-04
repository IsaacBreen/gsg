use std::rc::Rc;
use std::collections::HashMap;
use crate::*;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum SimpleCombinator {
//     Seq { children: Vec<SimpleCombinator> },
//     Choice { children: Vec<SimpleCombinator>, greedy: bool },
//     Repeat1 { a: SimpleCombinator, greedy: bool },
//     EatU8 { u8set: U8Set },
//     EatString { string: String },
//     EatByteStringChoice { strings: Vec<String> },
//     Eps,
//     Fail,
//     Symbol { symbol: Symbol },
//     MutateRightData { f: Rc<dyn Fn(RightData) -> RightData> },
//     CheckRightData { f: Rc<dyn Fn(RightData) -> bool> },
//     ExcludeStrings { strings: Vec<String> },
//     NegativeLookahead { child: SimpleCombinator },
// }

pub trait Compile {
    fn compile(self) -> Combinator;
}

impl Compile for Combinator {
    fn compile(self) -> Combinator {
        return self;
        match self {
            Combinator::Seq(seq) => seq.compile(),
            Combinator::Choice(choice) => choice.compile(),
            Combinator::EatU8(eat_u8) => eat_u8.compile(),
            Combinator::EatString(eat_string) => eat_string.compile(),
            Combinator::Eps(eps) => eps.compile(),
            Combinator::Fail(fail) => fail.compile(),
            Combinator::Repeat1(repeat1) => repeat1.compile(),
            Combinator::EatByteStringChoice(eat_bytestring_choice) => eat_bytestring_choice.compile(),
            _ => self,
        }
    }
}

impl Compile for Seq {
    fn compile(self) -> Combinator {
        return Combinator::Seq(self);
        // let mut children = Vec::new();
        //
        // for child in self.children.as_ref() {
        //     let compiled_child = child.clone().compile();
        //     match compiled_child {
        //         Combinator::Seq(inner_seq) => children.extend(inner_seq.children.as_ref()),
        //         Combinator::Fail(_) => return Combinator::Fail(Fail),
        //         Combinator::Eps(_) => {},
        //         _ => children.push(&compiled_child),
        //     }
        // }
        //
        // // Optimize repeated patterns
        // let mut i = 0;
        // while i < children.len() - 1 {
        //     if let (Some(a), Some(b)) = (children.get(i), children.get(i + 1)) {
        //         match (a, b) {
        //             //     case (A1, Choice([Repeat1(A2), Seq([])])) if A1 == A2:
        //             //         children[ix] = Repeat1(A1)
        //             //         children.pop(iy)
        //             (a, Combinator::Choice(Choice { children: b_children, greedy })) if b_children.contains(&Rc::new(Combinator::Repeat1(Repeat1 { a: Rc::new(a.clone()), greedy: *greedy }))) && b_children.contains(&Rc::new(Combinator::Eps(Eps))) => {
        //                 children[i] = Rc::new(Combinator::Repeat1(Repeat1 { a: Rc::new(a.clone()), greedy: *greedy }));
        //                 children.remove(i + 1);
        //             },
        //             //     case (Choice([Repeat1(A2), Seq([])]), A1) if A1 == A2:
        //             //         children[iy] = Repeat1(A1)
        //             //         children.pop(ix)
        //             (Combinator::Choice(Choice { children: a_children, greedy }), b) if a_children.contains(&Rc::new(Combinator::Repeat1(Repeat1 { a: Rc::new(b.clone()), greedy: *greedy }))) && a_children.contains(&Rc::new(Combinator::Eps(Eps))) => {
        //                 children[i] = Rc::new(Combinator::Repeat1(Repeat1 { a: Rc::new(b.clone()), greedy: *greedy }));
        //                 children.remove(i + 1);
        //             },
        //             //     case (A1, Repeat1(A2)) if A1 == A2:
        //             //         children[ix] = Repeat1(A1)
        //             //         children.pop(iy)
        //             (a, Combinator::Repeat1(Repeat1 { a: b, greedy })) if *a == **b => {
        //                 children[i] = Rc::new(Combinator::Repeat1(Repeat1 { a: Rc::new(a.clone()), greedy: *greedy }));
        //                 children.remove(i + 1);
        //             },
        //             //     case (Repeat1(A2), A1) if A1 == A2:
        //             //         children[iy] = Repeat1(A1)
        //             //         children.pop(ix)
        //             (Combinator::Repeat1(Repeat1 { a: a, greedy }), b) if **a == *b => {
        //                 children[i] = Rc::new(Combinator::Repeat1(Repeat1 { a: Rc::new(b.clone()), greedy: *greedy }));
        //                 children.remove(i + 1);
        //             },
        //             //     case _:
        //             //         continue
        //             _ => i += 1,
        //         }
        //     } else {
        //         break;
        //     }
        // }
        //
        // match children.len() {
        //     0 => Combinator::Eps(Eps),
        //     1 => Rc::unwrap_or_clone(children.pop().unwrap()),
        //     _ => Combinator::Seq(Seq { children }),
        // }
    }
}

impl Compile for Choice {
    fn compile(self) -> Combinator {
        return Combinator::Choice(self);
        // let mut merged_u8set = U8Set::none();
        // let mut other_children = Vec::new();
        // let mut eat_strings = Vec::new();
        //
        // for child in self.children {
        //     let compiled_child = child.as_ref().clone().compile();
        //     match compiled_child {
        //         Combinator::EatU8(EatU8 { u8set }) => {
        //             merged_u8set = merged_u8set.union(&u8set);
        //         }
        //         Combinator::EatString(eat_string) => {
        //             eat_strings.push(eat_string.string);
        //         }
        //         Combinator::Choice(inner_choice) => {
        //             other_children.extend(inner_choice.children);
        //         }
        //         Combinator::Fail(_) => {},
        //         _ => other_children.push(Rc::new(compiled_child)),
        //     }
        // }
        //
        // if !merged_u8set.is_empty() {
        //     other_children.push(Rc::new(Combinator::EatU8(EatU8 { u8set: merged_u8set })));
        // }
        //
        // if eat_strings.len() > 1 {
        //     other_children.push(Rc::new(Combinator::EatByteStringChoice(EatByteStringChoice::new(eat_strings))));
        // } else if let Some(eat_string) = eat_strings.pop() {
        //     other_children.push(Rc::new(Combinator::EatString(EatString { string: eat_string })));
        // }
        //
        // // Group by common prefixes
        // let mut groups: HashMap<Rc<Combinator>, Vec<Rc<Combinator>>> = HashMap::new();
        // // for child in other_children {
        //     // if let Combinator::Seq(Seq { children }) = child.as_ref() {
        //     //     if let Some(first) = children.first() {
        //     //         groups.entry(Rc::clone(first)).or_default().push(child);
        //     //     } else {
        //     //         groups.entry(Rc::new(Combinator::Eps(Eps))).or_default().push(child);
        //     //     }
        //     // } else {
        //     //     groups.entry(Rc::clone(&child)).or_default().push(child);
        //     // }
        // }
        //
        // // let mut new_children = Vec::new();
        // // for (prefix, group) in groups {
        // //     if group.len() == 1 {
        // //         new_children.push(group[0].clone());
        // //     } else {
        // //         let suffixes: Vec<Rc<Combinator>> = group.into_iter()
        // //             .map(|c| match c.as_ref() {
        // //                 Combinator::Seq(Seq { children }) if children.first() == Some(&prefix) => {
        // //                     Rc::new(Combinator::Seq(Seq { children: children[1..].to_vec() }))
        // //                 },
        // //                 _ => Rc::new(Combinator::Eps(Eps)),
        // //             })
        // //             .collect();
        // //         new_children.push(Rc::new(Combinator::Seq(Seq {
        // //             children: vec![
        // //                 prefix,
        // //                 Rc::new(Combinator::Choice(Choice { children: suffixes , greedy: self.greedy })),
        // //             ],
        // //         })));
        // //     }
        // // }
        //
        // // match new_children.len() {
        // //     0 => Combinator::Fail(Fail),
        // //     1 => Rc::unwrap_or_clone(new_children.pop().unwrap()),
        // //     _ => Combinator::Choice(Choice { children: new_children, greedy: self.greedy }),
        // // }
    }
}

impl Compile for EatU8 {
    fn compile(self) -> Combinator {
        Combinator::EatU8(self)
    }
}

impl Compile for EatString {
    fn compile(self) -> Combinator {
        Combinator::EatString(self)
    }
}

impl Compile for Eps {
    fn compile(self) -> Combinator {
        Combinator::Eps(self)
    }
}

impl Compile for Fail {
    fn compile(self) -> Combinator {
        Combinator::Fail(self)
    }
}

impl Compile for Repeat1 {
    fn compile(self) -> Combinator {
        let compiled_a = self.a.as_ref().clone().compile();
        match compiled_a {
            Combinator::Fail(_) => Combinator::Fail(Fail),
            _ => Combinator::Repeat1(Repeat1 { a: Rc::new(compiled_a), greedy: self.greedy }),
        }
    }
}

impl Compile for EatByteStringChoice {
    fn compile(self) -> Combinator {
        Combinator::EatByteStringChoice(self)
    }
}
