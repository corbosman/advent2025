use std::collections::HashSet;

use glam::IVec3;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{char, line_ending, i32},
    multi::separated_list1,
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    process_with_max(input, 1000)
}

pub fn process_with_max(input: &str, max: i32) -> miette::Result<String> {
    let (_, lights) = read_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let pairs: Vec<Pair> = lights
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Pair::new(*a, *b))
        .sorted_by_key(|d| d.distance)
        .collect();

    let strings: Vec<_> = connect_pairs(&pairs, max)
        .into_iter()
        .sorted_by_key(|s| std::cmp::Reverse(s.len()))
        .collect();

    Ok((strings[0].len()*strings[1].len()*strings[2].len()).to_string())
}

fn connect_pairs(pairs: &[Pair], max: i32) -> Vec<HashSet<IVec3>> {
    let mut groups: Vec<HashSet<IVec3>> = Vec::new();

    for pair in pairs.iter().take(max as usize) {

        let a_idx = groups.iter().position(|g| g.contains(&pair.a));
        let b_idx = groups.iter().position(|g| g.contains(&pair.b));

        match (a_idx, b_idx) {

            (Some(i), Some(j)) if i == j => {}

            (Some(i), Some(j)) => {
                let b_group = groups.remove(j.max(i));
                let a_group = groups.remove(i.min(j));
                let mut merged = a_group;
                merged.extend(b_group);
                groups.push(merged);
            }

            (Some(i), None) => {
                groups[i].insert(pair.b);
            }

            (None, Some(j)) => {
                groups[j].insert(pair.a);
            }

            (None, None) => {
                groups.push(HashSet::from([pair.a, pair.b]));
            }
        }
    }

    groups
}

fn read_input(input: &str) -> IResult<&str, Vec<IVec3>> {
    let (input, lights) = separated_list1(line_ending, light).parse(input)?;
    Ok((input, lights))
}

fn light(input: &str) -> IResult<&str, IVec3> {
    let (input, (x, _, y, _, z)) = (i32, char(','), i32, char(','), i32).parse(input)?;
    Ok((input, IVec3::new(x, y, z)))
}

#[derive(Debug)]
struct Pair {
    a: IVec3,
    b: IVec3,
    distance: i64,
}

impl Pair {
    fn new(a: IVec3, b: IVec3) -> Self {
        let d = a - b;
        let distance = (d.x as i64).pow(2) + (d.y as i64).pow(2) + (d.z as i64).pow(2);
        Self { distance, a, b }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        assert_eq!("40", process_with_max(input, 10)?);
        Ok(())
    }
}
