use std::ops::RangeInclusive;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    multi::{many0, separated_list1},
    sequence::separated_pair,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (ranges, ingredients)) = parse_input(input).map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    let fresh = ingredients.iter().fold(0, |acc, &ingredient| {
        if ranges.iter().any(|range| range.contains(&ingredient)) {
            acc + 1
        } else {
            acc
        }
    });

    Ok(fresh.to_string())
}

fn parse_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (input, ranges) = separated_list1(line_ending, parse_range).parse(input)?;
    let (input, _) = many0(line_ending).parse(input)?;
    let (input, ingredients) = separated_list1(line_ending, u64).parse(input)?;
    Ok((input, (ranges, ingredients)))
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(u64, tag("-"), u64)
        .map(|(first, last)| first..=last)
        .parse(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
