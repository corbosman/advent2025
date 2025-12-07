use std::collections::HashMap;
use nom_locate::{position, LocatedSpan};
use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{anychar, line_ending},
    multi::{many0, separated_list1},
    IResult, Parser,
};
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tachyon {
    Manifold,
    Splitter,
    Beam
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (map, height)) = read_map(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;
    let splits = fire_beam(&map, height);
    Ok(splits.to_string())
}

pub fn fire_beam(map: &HashMap<IVec2, Tachyon>, height: i32) -> i32 {
    let manifold = map.iter().find(|&(_, t)| *t == Tachyon::Manifold).map(|(&pos, _)| pos).unwrap();
    let mut cache: HashMap<IVec2, i32> = HashMap::new();
    count_timelines(map, &mut cache, height, manifold + IVec2::Y)
}

pub fn count_timelines(splitters: &HashMap<IVec2, Tachyon>, cache: &mut HashMap<IVec2, i32>, height: i32, pos: IVec2,
) -> i32 {
    if let Some(&cached) = cache.get(&pos) {
        info!("cached value for {:?}: {}", pos, cached);
        return cached;
    }

    info!("pos: {:?}, height: {}", pos, height);

    if height == 0 {
        return 1;
    }

    let result = if splitters.contains_key(&pos) {
        info!("splitter found at {:?}", pos);
        count_timelines(splitters, cache, height, pos + IVec2::NEG_X)+ count_timelines(splitters, cache, height, pos + IVec2::X)
    } else {
        count_timelines(splitters, cache, height - 1, pos + IVec2::Y)
    };

    info!("caching count for {:?}: {}", pos, result);
    cache.insert(pos, result);
    result
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

    #[test_log::test]
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
