use std::collections::{HashMap, VecDeque};

use miette::miette;
use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::line_ending,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, devices) = parse_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let paths = find_paths("you", &devices);
    Ok(paths.len().to_string())
}

fn find_paths(start: &str, devices: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    queue.push_back(vec![start.to_string()]);

    while let Some(path) = queue.pop_front() {
        let current_name = path.last().unwrap();

        if current_name == "out" {
            paths.push(path);
            continue;
        }

        if let Some(outputs) = devices.get(current_name) {
            for next in outputs {
                if !path.contains(next) {
                    let mut new_path = path.clone();
                    new_path.push(next.clone());
                    queue.push_back(new_path);
                }
            }
        }
    }

    paths
}

fn parse_input(input: &str) -> IResult<&str, HashMap<String, Vec<String>>> {
    separated_list1(line_ending, parse_device)
        .map(|devices| devices.into_iter().collect())
        .parse(input)
}

fn parse_device(input: &str) -> IResult<&str, (String, Vec<String>)> {
    separated_pair(
        take_until(": "),
        tag(": "),
        separated_list1(
            tag(" "),
            take_while1(|c: char| c.is_alphabetic()).map(|s: &str| s.to_string()),
        )
    ).map(|(name, outputs)| (name.to_string(), outputs)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
