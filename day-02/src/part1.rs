use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, ranges) = ranges(input).map_err(|e| miette::miette!("Parse error: {:?}", e))?;
    let mut counter = 0;

    for range in ranges {
        let (start, end) = range;
        // println!("processing range {} to {}", start, end);
        for num in start..=end {
            let s = num.to_string();
            if s.len() % 2 == 1 {
                continue;
            }
            let mid = s.len() / 2;
            let s1: i64 = s[..mid].parse().unwrap();
            let s2: i64 = s[mid..].parse().unwrap();
            if s1 == s2 {
                // println!("adding {}", num);
                counter += num;
            }
        }
    }

    Ok(counter.to_string())
}

fn ranges(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list1(tag(","), separated_pair(complete::i64, tag("-"), complete::i64)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
