use std::collections::{HashMap, HashSet};

use glam::I64Vec2;
use itertools::Itertools;
use miette::miette;
use tracing::info;
use nom::{
    character::complete::{char, i64, line_ending},
    multi::separated_list1,
    IResult, Parser,
};

const DIRECTIONS: [I64Vec2; 4] = [
    I64Vec2::new(0, -1),  // up
    I64Vec2::new(1, 0),   // right
    I64Vec2::new(0, 1),   // down
    I64Vec2::new(-1, 0),  // left
];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, points) = read_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let polygon = polygon(&points);



    Ok("50".to_string())
}

fn polygon(points: &HashSet<I64Vec2>) -> HashSet<Edge> {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut polygon: HashSet<Edge> = HashSet::new();

    for point in points {
        let mut neighbors: Vec<Edge> = Vec::new();

        for direction in DIRECTIONS {
            let mut curr = *point + direction;

            while curr.x >= min_x && curr.x <= max_x && curr.y >= min_y && curr.y <= max_y {
                if points.contains(&curr) {
                    neighbors.push(Edge::new(*point, curr));
                    break;
                }
                curr += direction;
            }
        }

        assert!(neighbors.len() >= 2, "Point {:?} has fewer than 2 neighbors", point);

        neighbors.sort_by_key(|e| e.length);
        polygon.insert(neighbors.remove(0));
        polygon.insert(neighbors.remove(0));
    }
    polygon
}

fn read_input(input: &str) -> IResult<&str, HashSet<I64Vec2>> {
    let (input, points) = separated_list1(line_ending, red_tile).parse(input)?;
    Ok((input, points.into_iter().collect()))
}

fn red_tile(input: &str) -> IResult<&str, I64Vec2> {
    let (input, (x, _, y)) = (i64, char(','), i64).parse(input)?;
    Ok((input, I64Vec2::new(x, y)))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    start: I64Vec2,
    end: I64Vec2,
    length: i64,
}

impl Edge {
    fn new(a: I64Vec2, b: I64Vec2) -> Self {
        let (start, end) = if (a.x, a.y) < (b.x, b.y) { (a, b) } else { (b, a) };
        let d = end - start;
        let length = d.x.abs() + d.y.abs();
        Self { start, end, length }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
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
        assert_eq!("24", process(input)?);
        Ok(())
    }
}
