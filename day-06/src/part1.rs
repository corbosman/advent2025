use nom::{
    character::complete::{anychar, i64 as parse_i64, multispace1, newline, space0, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (grid, ops) = parse_input(input);

    let result: i64 = transpose(grid).iter().zip(ops.iter())
        .map(|(row, op)| -> i64 {
            match op {
                '*' => row.iter().product(),
                '+' => row.iter().sum(),
                _ => panic!("unknown op: {op}"),
            }
        })
        .sum();

    Ok(result.to_string())
}

fn transpose(grid: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    (0..grid[0].len())
        .map(|col| grid.iter().map(|row| row[col]).collect())
        .collect()
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let (_, result) = (many1(numbers), operators).parse(input).unwrap();
    result
}

fn numbers(input: &str) -> IResult<&str, Vec<i64>> {
    terminated(preceded(space0, separated_list1(space1, parse_i64)), newline).parse(input)
}

fn operators(input: &str) -> IResult<&str, Vec<char>> {
    separated_list1(multispace1, anychar).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
