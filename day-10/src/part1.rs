use std::collections::{HashSet, VecDeque};

use miette::miette;
use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, line_ending, u64 as nom_u64},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = parse_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let presses: u64 = machines.iter().map(|m| press_buttons(0, m)).sum();
    Ok(presses.to_string())
}

fn press_buttons(start: u64, machine: &Machine) -> u64 {
    let target = machine.target;

    let mut visited: HashSet<u64> = HashSet::new();
    let mut queue: VecDeque<(u64, u64)> = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((state, presses)) = queue.pop_front() {
        for &button in &machine.buttons {
            let new_state = state ^ button;
            if new_state == target {
                return presses + 1;
            }
            if visited.insert(new_state) {
                queue.push_back((new_state, presses + 1));
            }
        }
    }

    panic!("no solution found");
}



// INPUT PARSING

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    let (input, lines) = separated_list1(
        line_ending,
        terminated(
            separated_pair(parse_target, tag(" "), parse_buttons),
            take_while1(|c| c != '\n'),
        ),
    ).parse(input)?;
    let machines = lines.into_iter().map(|(target, buttons)| Machine { target, buttons }).collect();
    Ok((input, machines))
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), parse_button).parse(input)
}

fn parse_button(input: &str) -> IResult<&str, u64> {
    let (input, bits) = delimited(tag("("), separated_list0(char(','), nom_u64), tag(")")).parse(input)?;
    let value = bits.iter().fold(0u64, |acc, &bit| acc | (1 << bit));
    Ok((input, value))
}

fn parse_target(input: &str) -> IResult<&str, u64> {
    let (input, pattern) = delimited(char('['), take_until("]"), char(']')).parse(input)?;
    let value = pattern.chars().enumerate().fold(0u64, |acc, (i, c)| {
        acc | if c == '#' { 1 << i } else { 0 }
    });
    Ok((input, value))
}

#[derive(Debug)]
struct Machine {
    target: u64,
    buttons: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
