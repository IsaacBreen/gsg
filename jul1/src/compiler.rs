use std::rc::Rc;
use crate::*;

pub trait Compile {
    fn compile(self) -> Combinator;
}

impl Compile for Combinator {
    fn compile(self) -> Combinator {
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
        let mut flattened_children = Vec::new();
        
        for child in self.children {
            let compiled_child = child.as_ref().clone().compile();
            match compiled_child {
                Combinator::Seq(inner_seq) => flattened_children.extend(inner_seq.children),
                _ => flattened_children.push(Rc::new(compiled_child)),
            }
        }
        
        Combinator::Seq(Seq { children: flattened_children })
    }
}

impl Compile for Choice {
    fn compile(self) -> Combinator {
        let mut merged_u8set = U8Set::none();
        let mut other_children = Vec::new();
        let mut eat_strings = Vec::new();
        
        for child in self.children {
            let compiled_child = child.as_ref().clone().compile();
            match compiled_child {
                Combinator::EatU8(EatU8 { u8set }) => {
                    merged_u8set = merged_u8set.union(&u8set);
                }
                Combinator::EatString(eat_string) => {
                    eat_strings.push(eat_string.string);
                }
                Combinator::Choice(inner_choice) => {
                    other_children.extend(inner_choice.children);
                }
                _ => other_children.push(Rc::new(compiled_child)),
            }
        }
        
        if !merged_u8set.is_empty() {
            other_children.push(Rc::new(Combinator::EatU8(EatU8 { u8set: merged_u8set })));
        }
        
        if eat_strings.len() > 1 {
            other_children.push(Rc::new(Combinator::EatByteStringChoice(EatByteStringChoice::new(eat_strings))));
        } else if let Some(eat_string) = eat_strings.pop() {
            other_children.push(Rc::new(Combinator::EatString(EatString { string: eat_string })));
        }
        
        Combinator::Choice(Choice { children: other_children })
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
        Combinator::Repeat1(Repeat1 { a: Rc::new(compiled_a) })
    }
}

impl Compile for EatByteStringChoice {
    fn compile(self) -> Combinator {
        Combinator::EatByteStringChoice(self)
    }
}
