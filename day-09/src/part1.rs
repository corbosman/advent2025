use glam::I64Vec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{char, i64, line_ending},
    multi::separated_list1,
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, points) = read_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let largest_area = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let d = a - b;
            (d.x.abs() + 1) * (d.y.abs() + 1)
        })
        .max()
        .unwrap();

    Ok(largest_area.to_string())
}

fn read_input(input: &str) -> IResult<&str, Vec<I64Vec2>> {
    separated_list1(line_ending, red_tile).parse(input)
}

fn red_tile(input: &str) -> IResult<&str, I64Vec2> {
    let (input, (x, _, y)) = (i64, char(','), i64).parse(input)?;
    Ok((input, I64Vec2::new(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
        assert_eq!("50", process(input)?);
        Ok(())
    }
}
