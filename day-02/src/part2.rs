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
        for num in start..=end {
            let s = num.to_string();
            let len: i64 = s.len() as i64;
            let factors = factors(len);
            for factor in factors {
                let slice_size = (len / factor) as usize;
                let factor_usize = factor as usize;
                let slices: Vec<i64> = (0..factor_usize)
                    .map(|i| s[i * slice_size..(i + 1) * slice_size].parse().unwrap())
                    .collect();

                if slices.windows(2).all(|w| w[0] == w[1]) {
                    counter += num;
                    break;
                }
            }
        }
    }

    Ok(counter.to_string())
}

fn ranges(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list1(tag(","), separated_pair(complete::i64, tag("-"), complete::i64)).parse(input)
}

fn factors(n: i64) -> Vec<i64> {
    let mut divs = Vec::new();
    let sqrt_n = (n as f64).sqrt() as i64;

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

    divs.sort();
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
