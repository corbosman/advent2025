use std::collections::HashMap;

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
    let mut cache: HashMap<(&str, u8), usize> = HashMap::new();
    let count = dfs("svr", 0, &devices, &mut cache);
    Ok(count.to_string())
}

// We use a DFS, keep tracking using a bitmask as this copies really fast with rust
fn dfs<'a>(current: &'a str, mask: u8, devices: &'a HashMap<String, Vec<String>>, cache: &mut HashMap<(&'a str, u8), usize>,
) -> usize {

    let mask = mask | match current {
        "fft" => 0b01,
        "dac" => 0b10,
        _ => 0,
    };

    // we reached the end, make sure we saw both by checking if both bits are set
    if current == "out" {
        return match mask {
            0b11 => 1,
            _ => 0,
        };
    }

    // we have a cache hit, return the cached value
    if let Some(&cached) = cache.get(&(current, mask)) {
        return cached;
    }

    // get the outputs for the current device
    let Some(outputs) = devices.get(current) else {
        return 0;
    };

    // recurse through the outputs
    let count: usize = outputs.iter().map(|next| dfs(next, mask, devices, cache)).sum();

    // cache the result
    cache.insert((current, mask), count);

    count
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
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
