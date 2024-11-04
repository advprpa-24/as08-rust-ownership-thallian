use crate::term::*;
use std::fmt;

/// Pretty print a term.
///
/// Do not care about when to leave parentheses out.
pub fn pretty_print(term: &Term) -> String {
    match term {
        Term::Var(var) => var.to_string(),
        Term::Abs(var, abs_term) => format!("(λ{var}. {})", pretty_print(abs_term)),
        Term::App(m, n) => format!("({} {})", pretty_print(m), pretty_print(n)),
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_print_1() {
        // (λx. x y) z
        let input = app(abs("x", app(var("x"), var("y"))), var("z"));
        let expected = "((λx. (x y)) z)";
        assert_eq!(expected, format!("{input}"));
    }

    #[test]
    fn test_pretty_print_2() {
        let input = abs(
            "n",
            abs(
                "f",
                abs("x", app(var("f"), app(app(var("n"), var("f")), var("x")))),
            ),
        );
        let expected = "(λn. (λf. (λx. (f ((n f) x)))))";
        assert_eq!(expected, format!("{input}"));
    }
}
