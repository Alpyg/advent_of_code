#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut l, mut r): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            let mut numbers = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok());
            Some((numbers.next()?, numbers.next()?))
        })
        .unzip();

    l.sort();
    r.sort();

    let result: i32 = l.iter().zip(r.iter()).map(|(a, b)| (a - b).abs()).sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "#;
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
