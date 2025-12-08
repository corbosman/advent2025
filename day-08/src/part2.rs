use std::collections::HashSet;

use glam::I64Vec3;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{char, line_ending, i64},
    multi::separated_list1,
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, lights) = read_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let pairs: Vec<Pair> = lights
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Pair::new(*a, *b))
        .sorted_by_key(|d| d.distance)
        .collect();

    let distance: i64 = connect_pairs(lights.len(),&pairs);

    Ok(distance.to_string())
}

fn connect_pairs(num_lights: usize, pairs: &[Pair]) -> i64 {
    let mut groups: Vec<HashSet<I64Vec3>> = Vec::new();

    for pair in pairs {
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

        if groups.len() == 1 && groups[0].len() == num_lights {
            return pair.a.x * pair.b.x;
        }
    }
    panic!("should not happen");
}

fn read_input(input: &str) -> IResult<&str, Vec<I64Vec3>> {
    let (input, lights) = separated_list1(line_ending, light).parse(input)?;
    Ok((input, lights))
}

fn light(input: &str) -> IResult<&str, I64Vec3> {
    let (input, (x, _, y, _, z)) = (i64, char(','), i64, char(','), i64).parse(input)?;
    Ok((input, I64Vec3::new(x, y, z)))
}

#[derive(Debug)]
struct Pair {
    a: I64Vec3,
    b: I64Vec3,
    distance: i64,
}

impl Pair {
    fn new(a: I64Vec3, b: I64Vec3) -> Self {
        Self { distance: a.distance_squared(b), a, b }
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
        assert_eq!("25272", process(input)?);
        Ok(())
    }
}
