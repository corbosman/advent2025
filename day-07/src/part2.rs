use std::collections::{HashMap, HashSet};
use nom_locate::{position, LocatedSpan};
use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{anychar, line_ending},
    multi::{many0, separated_list1},
    IResult, Parser,
};

pub fn process(input: &str) -> miette::Result<String> {
    let (_, (splitters, manifold, height)) = read_map(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;
    let mut cache = HashMap::new();
    let result = count_timelines(&splitters, &mut cache, height, manifold + IVec2::Y);
    Ok(result.to_string())
}

pub fn count_timelines(splitters: &HashSet<IVec2>, cache: &mut HashMap<IVec2, i64>, height: i64, pos: IVec2) -> i64 {
    if let Some(&cached) = cache.get(&pos) {
        return cached;
    }

    if height == 0 {
        return 1;
    }

    let result = if splitters.contains(&pos) {
        count_timelines(splitters, cache, height - 1, pos + IVec2::new(-1, 1)) + count_timelines(splitters, cache, height - 1, pos + IVec2::new(1, 1))
    } else {
        count_timelines(splitters, cache, height - 1, pos + IVec2::Y)
    };

    cache.insert(pos, result);
    result
}

pub fn read_map(input: Span) -> IResult<Span, (HashSet<IVec2>, IVec2, i64)> {
    let height = input.lines().count() as i64;
    let (input, rows) = separated_list1(line_ending, many0(parse_cell)).parse(input)?;
    let mut splitters = HashSet::new();
    let mut manifold = IVec2::ZERO;
    for (pos, is_splitter) in rows.into_iter().flatten().flatten() {
        if is_splitter {
            splitters.insert(pos);
        } else {
            manifold = pos;
        }
    }
    Ok((input, (splitters, manifold, height - 1)))
}

fn parse_cell(input: Span) -> IResult<Span, Option<(IVec2, bool)>> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = anychar.parse(input)?;
    match c {
        '^' => Ok((input, Some((IVec2::new(x, y), true)))),
        'S' => Ok((input, Some((IVec2::new(x, y), false)))),
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
