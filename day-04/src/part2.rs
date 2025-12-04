use nom_locate::{position, LocatedSpan};
use glam::IVec2;
use miette::miette;
use std::collections::{HashMap};
use nom::{
    character::complete::{anychar, line_ending},
    multi::{many0, separated_list1},
    IResult, Parser,
};

const DIRECTIONS: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, mut map) = read_map(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;

    let height: i32 = input.lines().count() as i32;
    let width: i32 = input.lines().next().map(|l| l.len() as i32).unwrap_or(0);

    let mut result = 1;
    let mut total_rolls = 0;

    while result > 0 {
        let mut deleted_rolls: Vec<IVec2> = Vec::new();

        result = map.iter().fold(0, |mut acc, (pos, _)| {

            let mut neighbors = 0;

            for direction in DIRECTIONS {
                let new_pos = pos + direction;
                if new_pos.x < 0 || new_pos.x >= width || new_pos.y < 0 || new_pos.y >= height {
                    continue;
                }
                if map.contains_key(&new_pos) {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
                acc += 1;
                deleted_rolls.push(*pos);
            }

            acc
        });

        total_rolls += result;

        for pos in &deleted_rolls {
            map.remove(pos);
        }
    }

    Ok(total_rolls.to_string())
}

pub fn read_map(input: Span) -> IResult<Span, HashMap<IVec2, char>> {
    let (input, rows) = separated_list1(line_ending, many0(rolls)).parse(input)?;
    let hashmap = rows.iter().flatten().flatten().copied().collect::<HashMap<IVec2, char>>();
    Ok((input, hashmap))
}

fn rolls(input: Span) -> IResult<Span, Option<(IVec2, char)>> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = anychar.parse(input)?;
    if c == '@' {
        Ok((input, Some((IVec2::new(x, y), c))))
    } else {
        Ok((input, None))
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
