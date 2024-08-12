use crate::Combinator;

impl Combinator {
    pub fn transpose(&mut self) -> Self {
        // Converts a combinator into a form where the outer expression is a choice between sequences that begin with a terminal.
        //
        // Example transpositions:
        //
        // seq(seq(a, b, c), d, e)) =>
        // choice(seq(a, seq(b, c), seq(d, e))))
        //
        // seq(choice(a, b, c), d, e)) =>
        // choice(seq(a, seq(d, e)), seq(b, seq(d, e)), seq(c, seq(d, e)))

    }
}