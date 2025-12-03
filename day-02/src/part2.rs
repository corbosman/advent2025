use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
};
use rayon::prelude::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, ranges) = ranges(input).map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    let counter: u64 = ranges
        .par_iter()
        .flat_map(|(start, end)| *start..=*end)
        .map(|num| {
            let s = num.to_string();
            let len: u64 = s.len() as u64;
            for factor in factors(len) {
                let slice_size = (len / factor) as usize;
                let factor_usize = factor as usize;
                let slices: Vec<u64> = (0..factor_usize)
                    .map(|i| s[i * slice_size..(i + 1) * slice_size].parse().unwrap())
                    .collect();

                if slices.windows(2).all(|w| w[0] == w[1]) {
                    return num;
                }
            }
            0
        })
        .sum();

    Ok(counter.to_string())
}

fn ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(tag(","), separated_pair(complete::u64, tag("-"), complete::u64)).parse(input)
}

fn factors(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 2..=sqrt_n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
    }

    if n > 1 {
        divs.push(n);
    }

    divs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }
}
