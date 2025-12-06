#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (grid, ops) = read_input(input);

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

fn read_input(input: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let grid: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|row| row.iter().map(|n| n.parse().unwrap()).collect())
        .collect();

    let ops: Vec<char> = lines.last().unwrap()
        .iter()
        .map(|s| s.chars().next().unwrap())
        .collect();

    (grid, ops)
}

fn transpose(grid: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    (0..grid[0].len())
        .map(|col| grid.iter().map(|row| row[col]).collect())
        .collect()
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
