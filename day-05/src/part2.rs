use std::ops::RangeInclusive;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    multi::{separated_list1},
    sequence::separated_pair,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, ranges) = parse_input(input).map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    let sum = implode_ranges(ranges).iter().fold(0u64, |acc, range| {
        acc + (range.end() - range.start() + 1)
    });

    Ok(sum.to_string())
}

fn implode_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut imploded_ranges = Vec::new();
    imploded_ranges.push(ranges[0].clone());

    for range in ranges[1..].iter() {
        let previous = imploded_ranges.last().unwrap();

        match () {
            _ if range.start() >= previous.start() && range.end() <= previous.end() => {
                continue;
            }
            _ if range.start() <= previous.end() => {
                let last = imploded_ranges.pop().unwrap();
                imploded_ranges.push(merge_ranges(&last, range));
            }
            _ => {
                imploded_ranges.push(range.clone());
            }
        }
    }

    imploded_ranges
}

fn merge_ranges(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    *a.start().min(b.start())..=*a.end().max(b.end())
}

fn parse_input(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let (input, ranges) = separated_list1(line_ending, parse_range).parse(input)?;
    Ok((input, (ranges)))
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
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
