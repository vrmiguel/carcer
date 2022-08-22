use nom::{
    bytes::complete::take_while1,
    character::complete::{char, digit1, multispace0},
    combinator::{cut, not, recognize},
    error::{context, VerboseError},
    sequence::{delimited, pair, preceded},
    Parser,
};

use crate::line::Line;

/// The result of a parsing operation with some added error
/// context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;

pub(crate) fn parse_line(input: &str) -> IResult<Line> {
    fn parse_inner_line(input: &str) -> IResult<Line> {
        let (rest, sequence) = parse_car_cdr(input)?;
        let (rest, identifier) =
            preceded(multispace0, parse_identifier)(rest)?;

        let line = Line {
            sequence,
            identifier,
        };

        Ok((rest, line))
    }

    parse_parenthesis_enclosed(parse_inner_line)(input)
}

fn parse_car_cdr(input: &str) -> IResult<&str> {
    let is_part_of_sequence = |ch| matches!(ch, 'a' | 'd');
    let sequence_parser = context(
        "car cdr sequence",
        take_while1(is_part_of_sequence),
    );
    delimited(
        char('c'),
        preceded(multispace0, sequence_parser),
        context(
            "car or cdr sequence",
            cut(preceded(multispace0, char('r'))),
        ),
    )(input)
}

fn parse_identifier(input: &str) -> IResult<&str> {
    let acceptable_chars = |ch: char| {
        ch.is_ascii_alphanumeric()
            || matches!(ch, '-' | '_' | '?')
    };

    let (rest, identifier) = recognize(pair(
        // Ensure that the identifier doesn't start with a
        // digit
        context("must not start with a digit", not(digit1)),
        take_while1(acceptable_chars),
    ))(input)?;

    // For the sake of simplicity, let's not let identifiers
    // be a valid car/cdr sequence
    not(parse_car_cdr)(input)?;

    Ok((rest, identifier))
}

fn parse_parenthesis_enclosed<'a, T, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<T>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    delimited(
        char('('),
        preceded(multispace0, inner),
        context(
            "closing parenthesis",
            cut(preceded(multispace0, char(')'))),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::parse_car_cdr;

    #[test]
    fn parses_car_cdr_sequences() {
        assert_eq!(parse_car_cdr("car"), Ok(("", "a")));
        assert_eq!(parse_car_cdr("cdr"), Ok(("", "d")));
        assert_eq!(parse_car_cdr("cadr"), Ok(("", "ad")));
        assert_eq!(parse_car_cdr("caddr"), Ok(("", "add")));
        assert_eq!(parse_car_cdr("caaaar "), Ok((" ", "aaaa")));

        // Wrong: not finished by 'r'
        assert!(parse_car_cdr("ca").is_err());
        // Wrong: not preceded by 'c'
        assert!(parse_car_cdr("ar").is_err());
        // Wrong: 's' is invalid in this context
        assert!(parse_car_cdr("casr").is_err());
        // Wrong: missing at least one of either `a` or `d`
        assert!(parse_car_cdr("cr").is_err());
    }
}
