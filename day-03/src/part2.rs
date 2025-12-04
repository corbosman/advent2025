use nom::{
    IResult, Parser,
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, mut batteries) = batteries(input).map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    let joltages: Vec<u64> = batteries.iter_mut().map(|row| find_joltage(row)).collect();

    Ok(joltages.iter().sum::<u64>().to_string())
}

fn find_joltage(row: &mut Vec<i8>) -> u64 {
    let mut max_joltage: Vec<i8> = row.drain(0..12).collect();

    while row.len() > 0 {
        let num = row.remove(0);

        for i in 0..12 {
            let mut joltage = max_joltage.clone();
            joltage.remove(i);
            joltage.push(num);
            if array_to_number(&joltage) > array_to_number(&max_joltage) {
                max_joltage = joltage;
                break;
            }
        }
    }

    array_to_number(&max_joltage)
}

fn array_to_number(digits: &[i8]) -> u64 {
    digits.iter().fold(0u64, |acc, &digit| acc * 10 + digit as u64)
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
