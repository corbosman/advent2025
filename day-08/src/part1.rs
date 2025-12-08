use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
    let (_, lights) = read_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let distances: BinaryHeap<Distance> = lights
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Distance {
            a: *a,
            b: *b,
            distance: a.distance_squared(*b),
        })
        .collect();
    dbg!(&distances.into_sorted_vec());
    todo!()
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
struct Distance {
    a: IVec3,
    b: IVec3,
    distance: i32,
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
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
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
