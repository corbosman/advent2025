use std::collections::HashSet;

use glam::I64Vec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{char, i64, line_ending},
    multi::separated_list1,
    IResult, Parser,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, points) = read_input(input).map_err(|e| miette!("parse failed {}", e))?;

    // find all the edges of the polygon
    let edges = polygon(&points);

    // find the largest rectangle that fits in the polygon
    // keep a max to prevent trying squares that are too small to beat the current max
    let largest = points
        .iter()
        .tuple_combinations()
        .fold(0i64, |max, (a, b)| {
            let d = *a - *b;
            let area = (d.x.abs() + 1) * (d.y.abs() + 1);
            if area > max && rectangle_fits(*a, *b, &edges) {
                area
            } else {
                max
            }
        });

    Ok(largest.to_string())
}

// we have 2 points that form a rectangle, check if any of the edges of the polygon cross the rectangle
fn rectangle_fits(p1: I64Vec2, p2: I64Vec2, edges: &HashSet<Edge>) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    for edge in edges {
        if polygon_crosses_rectangle(edge, min_x, max_x, min_y, max_y) {
            return false;
        }
    }

    true
}

// check if an edge crosses a rectangle
fn polygon_crosses_rectangle(edge: &Edge, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> bool {
    if edge.start.x == edge.end.x {
        let x = edge.start.x;
        let (y1, y2) = (edge.start.y.min(edge.end.y), edge.start.y.max(edge.end.y));
        x > min_x && x < max_x && y2 > min_y && y1 < max_y
    } else {
        let y = edge.start.y;
        let (x1, x2) = (edge.start.x.min(edge.end.x), edge.start.x.max(edge.end.x));
        y > min_y && y < max_y && x2 > min_x && x1 < max_x
    }
}

// build up a polygon from the points
fn polygon(points: &HashSet<I64Vec2>) -> HashSet<Edge> {
    let mut polygon: HashSet<Edge> = HashSet::new();

    for point in points {
        let mut neighbors: Vec<Edge> = Vec::new();

        // N
        if let Some(closest) = points.iter()
            .filter(|p| p.x == point.x && p.y < point.y)
            .max_by_key(|p| p.y)
        {
            neighbors.push(Edge::new(*point, *closest));
        }
        // S
        if let Some(closest) = points.iter()
            .filter(|p| p.x == point.x && p.y > point.y)
            .min_by_key(|p| p.y)
        {
            neighbors.push(Edge::new(*point, *closest));
        }
        // W
        if let Some(closest) = points.iter()
            .filter(|p| p.y == point.y && p.x < point.x)
            .max_by_key(|p| p.x)
        {
            neighbors.push(Edge::new(*point, *closest));
        }
        // E
        if let Some(closest) = points.iter()
            .filter(|p| p.y == point.y && p.x > point.x)
            .min_by_key(|p| p.x)
        {
            neighbors.push(Edge::new(*point, *closest));
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
