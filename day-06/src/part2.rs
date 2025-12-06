use nom::{
    character::complete::{line_ending, not_line_ending},
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut grid, ops) = parse_input(input);

    grid = transpose(grid);
    let cols = collapse(&grid);

    let result: i64 = cols.iter().zip(ops.iter())
        .map(|(col, op)| -> i64 {
            match op {
                '*' => col.iter().product(),
                '+' => col.iter().sum(),
                _ => panic!("unknown op: {op}"),
            }
        })
        .sum();

    Ok(result.to_string())
}

fn transpose<T: Copy>(grid: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..grid[0].len())
        .map(|col| grid.iter().map(|row| row[col]).collect())
        .collect()
}

fn collapse(grid: &[Vec<char>]) -> Vec<Vec<i64>> {
    let nums: Vec<Option<i64>> = grid.iter()
        .map(|row| {
            let s: String = row.iter().filter(|&&c| c != ' ').collect();
            if s.is_empty() { None } else { Some(s.parse().unwrap()) }
        })
        .collect();

    nums.split(|n| n.is_none())
        .map(|chunk| chunk.iter().filter_map(|&n| n).collect())
        .collect()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let (_, mut lines) = many1(chars).parse(input).unwrap();
    let ops: Vec<char> = lines.pop().unwrap().into_iter().filter(|&c| c != ' ').collect();
    (lines, ops)
}

fn chars(input: &str) -> IResult<&str, Vec<char>> {
    let (remainder, line) = terminated(not_line_ending, line_ending).parse(input)?;
    Ok((remainder, line.chars().collect()))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   + \n";
        assert_eq!("3263827", process(input)?);
        Ok(())
    }
}
