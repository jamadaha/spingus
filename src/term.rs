use nom::{branch::alt, character::complete::multispace1, multi::separated_list1, IResult};

use crate::shared::{named, spaced};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Term {
    pub name: String,
    pub parameters: Vec<String>,
}

fn parse_with_parameters(input: &str) -> IResult<&str, Term> {
    let (remainder, name) = spaced(named)(input)?;
    let (remainder, parameters) = separated_list1(multispace1, named)(remainder)?;
    Ok((
        remainder,
        Term {
            name: name.to_string(),
            parameters: parameters.iter().map(|p| p.to_string()).collect(),
        },
    ))
}

fn parse_without_parameters(input: &str) -> IResult<&str, Term> {
    let (remainder, name) = spaced(named)(input)?;
    Ok((
        remainder,
        Term {
            name: name.to_string(),
            parameters: vec![],
        },
    ))
}

pub(super) fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((parse_with_parameters, parse_without_parameters))(input)
}

#[test]
fn test() {
    assert_eq!(
        Ok((
            "",
            Term {
                name: "name".to_string(),
                parameters: vec![],
            }
        )),
        parse_term("name")
    );
    assert_eq!(
        Ok((
            "",
            Term {
                name: "name".to_string(),
                parameters: vec!["?a".to_string()],
            }
        )),
        parse_term("name ?a")
    );
    assert_eq!(
        Ok((
            "",
            Term {
                name: "name".to_string(),
                parameters: vec!["?a".to_string(), "?b".to_string()],
            }
        )),
        parse_term("name ?a ?b")
    );
    assert_eq!(
        Ok((
            "",
            Term {
                name: "name".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()],
            }
        )),
        parse_term("name a b")
    );
}
