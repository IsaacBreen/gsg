use std::rc::Rc;
use crate::Combinator;

impl Combinator {
    pub fn transpose(&mut self) -> Self {
        // Converts a combinator into a form where the outer expression is a choice between sequences that begin with a terminal.
        //
        // Here, a 'terminal' is defined (loosely) as any combinator that is not a sequence or a choice.
        //
        // Example transpositions:
        //
        // seq(seq(a, b, c), d, e)) =>
        // choice(seq(a, seq(b, c), seq(d, e))))
        //
        // seq(choice(a, b, c), d, e)) =>
        // choice(seq(a, seq(d, e)), seq(b, seq(d, e)), seq(c, seq(d, e)))
        match self {
            Combinator::Seq(seq) => {
                let mut result = vec![];
                let mut prefix = vec![];

                for child in seq.children.iter() {
                    match child {
                        Combinator::Seq(inner_seq) => {
                            prefix.extend(inner_seq.children.iter().cloned());
                        }
                        Combinator::Choice(inner_choice) => {
                            let mut new_result = vec![];
                            for inner_child in inner_choice.children.iter() {
                                let mut new_prefix = prefix.clone();
                                new_prefix.push(inner_child.clone());
                                new_result.push(Combinator::Seq(crate::Seq { children: Rc::new(new_prefix.into()) }));
                            }
                            result = new_result;
                            prefix.clear();
                        }
                        _ => {
                            prefix.push(child.clone());
                        }
                    }
                }

                if !prefix.is_empty() {
                    result.push(Combinator::Seq(crate::Seq { children: Rc::new(prefix.into()) }));
                }

                Combinator::Choice(crate::Choice { children: Rc::new(result.into()), greedy: false })
            }
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{choice, eat_char, eps, seq, symbol, Combinator};

    #[test]
    fn test_transpose_simple_seq() {
        let mut combinator = seq!(eat_char('a'), eat_char('b'), eat_char('c'));
        let transposed = combinator.transpose();
        assert_eq!(
            transposed,
            seq!(eat_char('a'), eat_char('b'), eat_char('c'))
        );
    }

    #[test]
    fn test_transpose_nested_seq() {
        let mut combinator = seq!(seq!(eat_char('a'), eat_char('b'), eat_char('c')), eat_char('d'), eat_char('e'));
        let transposed = combinator.transpose();
        assert_eq!(
            transposed,
            choice!(seq!(eat_char('a'), eat_char('b'), eat_char('c'), eat_char('d'), eat_char('e')))
        );
    }

    #[test]
    fn test_transpose_seq_with_choice() {
        let mut combinator = seq!(choice!(eat_char('a'), eat_char('b'), eat_char('c')), eat_char('d'), eat_char('e'));
        let transposed = combinator.transpose();
        assert_eq!(
            transposed,
            choice!(
                seq!(eat_char('a'), eat_char('d'), eat_char('e')),
                seq!(eat_char('b'), eat_char('d'), eat_char('e')),
                seq!(eat_char('c'), eat_char('d'), eat_char('e'))
            )
        );
    }

    #[test]
    fn test_transpose_complex_seq() {
        let mut combinator = seq!(
            eat_char('a'),
            seq!(eat_char('b'), choice!(eat_char('c'), eat_char('d'))),
            eat_char('e'),
            choice!(eat_char('f'), eat_char('g')),
            eat_char('h')
        );
        let transposed = combinator.transpose();
        assert_eq!(
            transposed,
            choice!(
                seq!(eat_char('a'), eat_char('b'), eat_char('c'), eat_char('e'), eat_char('f'), eat_char('h')),
                seq!(eat_char('a'), eat_char('b'), eat_char('c'), eat_char('e'), eat_char('g'), eat_char('h')),
                seq!(eat_char('a'), eat_char('b'), eat_char('d'), eat_char('e'), eat_char('f'), eat_char('h')),
                seq!(eat_char('a'), eat_char('b'), eat_char('d'), eat_char('e'), eat_char('g'), eat_char('h'))
            )
        );
    }

    #[test]
    fn test_transpose_non_seq() {
        let mut combinator: Combinator = eat_char('a').into();
        let transposed = combinator.transpose();
        assert_eq!(transposed, eat_char('a').into());
    }

    #[test]
    fn test_transpose_eps() {
        let mut combinator: Combinator = eps().into();
        let transposed = combinator.transpose();
        assert_eq!(transposed, eps().into());
    }
}
