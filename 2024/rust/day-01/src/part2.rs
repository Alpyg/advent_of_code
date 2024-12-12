#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (l, r): (Vec<usize>, Vec<usize>) = input
        .lines()
        .filter_map(|line| {
            let mut numbers = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok());
            Some((numbers.next()?, numbers.next()?))
        })
        .unzip();

    let result: usize = l
        .iter()
        .map(|n| n * r.iter().filter(|r| &n == r).count())
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
