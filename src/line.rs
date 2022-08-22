use std::fmt::Display;

use crate::parser::parse_line;

#[derive(Debug, Clone, Copy)]
pub struct Line<'a> {
    pub(crate) sequence: &'a str,
    pub(crate) identifier: &'a str,
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
            0 => Ok(()),
            1 => {
                write!(
                    f,
                    "(c{}r {})",
                    self.sequence, self.identifier
                )
            }
            _ => {
                let (first, rest) = self.sequence.split_at(1);
                let expand_rest = Self {
                    sequence: rest,
                    identifier: self.identifier,
                };
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
}

#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn expands_correctly() {
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
            Line::parse("(cdaaaaddr z)").unwrap().to_string(),
            "(cdr (car (car (car (car (cdr (cdr z)))))))"
        );
    }
}
