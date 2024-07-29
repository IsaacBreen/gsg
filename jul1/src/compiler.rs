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
            _ => panic!("Unsupported combinator type for compilation"),
        }
    }
}

impl Compile for Seq {
    fn compile(self) -> Combinator {
        let compiled_children: Vec<Combinator> = self.children.into_iter()
            .map(|child| child.compile())
            .collect();
        
        // Optimization: Flatten nested Seq combinators
        let flattened_children: Vec<Combinator> = compiled_children.into_iter()
            .flat_map(|child| {
                if let Combinator::Seq(seq) = child {
                    seq.children.into_iter()
                } else {
                    vec![child].into_iter()
                }
            })
            .collect();
        
        Combinator::Seq(Seq { children: flattened_children })
    }
}

impl Compile for Choice {
    fn compile(self) -> Combinator {
        let compiled_children: Vec<Combinator> = self.children.into_iter()
            .map(|child| child.compile())
            .collect();
        
        // Optimization: Merge EatU8 combinators
        let mut merged_u8set = U8Set::none();
        let mut other_children = Vec::new();
        
        for child in compiled_children {
            if let Combinator::EatU8(EatU8 { u8set }) = child {
                merged_u8set = merged_u8set.union(&u8set);
            } else {
                other_children.push(child);
            }
        }
        
        if !merged_u8set.is_empty() {
            other_children.push(Combinator::EatU8(EatU8 { u8set: merged_u8set }));
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
        let compiled_a = self.a.compile();
        Combinator::Repeat1(Repeat1 { a: Rc::new(compiled_a) })
    }
}

impl Compile for EatByteStringChoice {
    fn compile(self) -> Combinator {
        Combinator::EatByteStringChoice(self)
    }
}
