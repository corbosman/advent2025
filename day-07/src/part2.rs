use std::collections::HashMap;
use nom_locate::{position, LocatedSpan};
use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{anychar, line_ending},
    multi::{many0, separated_list1},
    IResult, Parser,
};
use std::mem::swap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tachyon {
    Manifold,
    Splitter,
    Beam
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (map, height)) = read_map(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;
    let splits = fire_beam(&map, height);
    Ok(splits.to_string())
}

pub fn fire_beam(map: &HashMap<IVec2, Tachyon>, mut height: i32) -> i32 {
    let manifold = map.iter().find(|&(_, t)| *t == Tachyon::Manifold).map(|(&pos, _)| pos).unwrap();

    count_timelines(map, height, 0)
}

pub fn count_timelines(map: &HashMap<IVec2, Tachyon>, mut height: i32, mut count: i32) -> i32 {
    todo!()
}

pub fn read_map(input: Span) -> IResult<Span, (HashMap<IVec2, Tachyon>, i32)> {
    let height: i32 = input.lines().count() as i32;
    let (input, rows) = separated_list1(line_ending, many0(splitters)).parse(input)?;
    let hashmap = rows.iter().flatten().flatten().copied().collect::<HashMap<IVec2, Tachyon>>();
    Ok((input, (hashmap, height-1)))
}

fn splitters(input: Span) -> IResult<Span, Option<(IVec2, Tachyon)>> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = anychar.parse(input)?;
    match c {
        '^' => Ok((input, Some((IVec2::new(x, y), Tachyon::Splitter)))),
        'S' => Ok((input, Some((IVec2::new(x, y), Tachyon::Manifold)))),
        _ => Ok((input, None)),
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
