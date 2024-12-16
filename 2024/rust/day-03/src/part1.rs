use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

enum Instruction {
    Mul(i32, i32),
}

#[tracing::instrument(skip(input))]
fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pairs) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pairs.0, pairs.1)))
}

#[tracing::instrument(skip(input))]
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let result: i32 = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
