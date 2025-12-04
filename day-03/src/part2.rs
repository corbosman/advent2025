use nom::{
    IResult, Parser,
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
};
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, batteries) = batteries(input).map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    let sum: u32 = batteries
        .iter()
        .map(|row| {
            let mut row_pairs: Vec<u32> = row
                .iter()
                .combinations(2)
                .map(|pair| *pair[0] as u32 * 10 + *pair[1] as u32)
                .collect();
            row_pairs.sort_by(|a, b| b.cmp(a));
            row_pairs[0]
        })
        .sum();

    Ok(sum.to_string())
}

fn batteries(input: &str) -> IResult<&str, Vec<Vec<i8>>> {
    separated_list1(
        line_ending,
        many1(satisfy(|c| c.is_ascii_digit()).map(|c| c.to_digit(10).unwrap() as i8))
    ).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}
