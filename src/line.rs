use std::fmt::Display;

use crate::parser::parse_line;

#[derive(Debug, Clone, Copy)]
pub struct Line<'a> {
    /// The sequence of 'a's and 'd's which represents
    /// the `car` and `cdr` operations.
    /// The parser guarantees that this does not contain any other
    /// characters and is always at least one character long.
    pub(crate) sequence: &'a str,
    /// The identifier being used in this operation
    /// If the line represents `(cadr x)` then `identifier == "x"`.
    identifier: &'a str,
}

impl Display for Line<'_> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let expander = Expander::from_line(*self);

        write!(f, "{expander}")
    }
}

/// A helper struct whose Display impl expands the s-expressions
/// it represents
struct Expander<'a> {
    sequence: &'a str,
    identifier: &'a str,
}

impl<'a> Expander<'a> {
    pub fn from_line(line: Line<'a>) -> Self {
        Self {
            sequence: line.sequence,
            identifier: line.identifier,
        }
    }
}

impl Display for Expander<'_> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self.sequence.len() {
            1 => {
                // Base case: just apply the identifier to the car or cdr operation
                write!(
                    f,
                    "(c{}r {})",
                    self.sequence, self.identifier
                )
            }
            _ => {
                // split_at will never panic here since the sequence is surely always ASCII and therefore
                // will always be a UTF-8 code boundary. The length is also checked due to the pattern match.
                let (first, rest) = self.sequence.split_at(1);

                let expand_rest = Self {
                    sequence: rest,
                    identifier: self.identifier,
                };

                // Recursively expand the rest of the operations.
                write!(f, "(c{first}r {expand_rest})")
            }
        }
    }
}

impl<'a> Line<'a> {
    pub fn parse(input: &'a str) -> Result<Self, String> {
        let stringify_error = |error| match error {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error(error)
            | nom::Err::Failure(error) => {
                nom::error::convert_error(input, error)
            }
        };

        parse_line(input)
            .map_err(stringify_error)
            .map(|(_, line)| line)
    }

    pub fn new(sequence: &'a str, identifier: &'a str) -> Self {
        Self {
            sequence,
            identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn parses_and_expands_correctly() {
        assert_eq!(
            Line::parse("(car x)").unwrap().to_string(),
            "(car x)"
        );

        assert_eq!(
            Line::parse("(cdr y)").unwrap().to_string(),
            "(cdr y)"
        );

        assert_eq!(
            Line::parse("(cadr y)").unwrap().to_string(),
            "(car (cdr y))"
        );

        assert_eq!(
            Line::parse("(cdadr z)").unwrap().to_string(),
            "(cdr (car (cdr z)))"
        );

        assert_eq!(
            Line::parse("(cdaaaaddr z2)").unwrap().to_string(),
            "(cdr (car (car (car (car (cdr (cdr z2)))))))"
        );

        // Fails to parse: 'b' is not accepted in this context
        assert!(Line::parse("(cdadbr z)").is_err());

        // Fails to parse: no operand found
        assert!(Line::parse("(cdr)").is_err());

        // Fails to parse: unknown operation
        assert!(Line::parse("(cdd x)").is_err());

        // Fails to parse: identifier starts with a digit
        assert!(Line::parse("(cdr 2x)").is_err());

        // Fails to parse: missing brackets
        assert!(Line::parse("(cdr x").is_err());
        assert!(Line::parse("cdr x)").is_err());
        assert!(Line::parse("cdr x").is_err());
    }
}
