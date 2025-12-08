use std::cmp::Ordering;
use std::collections::BinaryHeap;

use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{char, line_ending, u32},
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
            a: a.clone(),
            b: b.clone(),
            distance: a.distance(b),
        })
        .collect();
    dbg!(&distances.into_sorted_vec().len());
    todo!()
}

fn read_input(input: &str) -> IResult<&str, Vec<Light>> {
    let (input, lights) = separated_list1(line_ending, light).parse(input)?;
    Ok((input, lights))
}

fn light(input: &str) -> IResult<&str, Light> {
    let (input, (x, _, y, _, z)) = (u32, char(','), u32, char(','), u32).parse(input)?;
    Ok((input, Light { x, y, z }))
}

#[derive(Debug, Clone)]
struct Light {
    x: u32,
    y: u32,
    z: u32,
}

impl Light {
    fn distance(&self, light: &Light) -> f64 {
        let dx = self.x as f64 - light.x as f64;
        let dy = self.y as f64 - light.y as f64;
        let dz = self.z as f64 - light.z as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug)]
struct Distance {
    a: Light,
    b: Light,
    distance: f64,
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
        self.distance.partial_cmp(&other.distance).unwrap_or(Ordering::Equal)
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
