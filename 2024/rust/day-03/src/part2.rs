use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

#[tracing::instrument(skip(input))]
fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pairs) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pairs.0, pairs.1)))
}

#[tracing::instrument(skip(input))]
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

#[tracing::instrument(skip(input))]
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let (_, result) = instructions
        .iter()
        .fold((true, 0), |(process, acc), ins| match ins {
            Instruction::Mul(a, b) => {
                if process {
                    (process, acc + a * b)
                } else {
                    (process, acc)
                }
            }
            Instruction::Do => (true, acc),
            Instruction::Dont => (false, acc),
        });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
