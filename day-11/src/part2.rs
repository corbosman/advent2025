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

fn dfs<'a>(current: &'a str, mask: u8, devices: &'a HashMap<String, Vec<String>>, cache: &mut HashMap<(&'a str, u8), usize>,
) -> usize {
    // Update mask: bit 0 = seen fft, bit 1 = seen dac
    let mask = mask
        | if current == "fft" { 1 } else { 0 }
        | if current == "dac" { 2 } else { 0 };

    if current == "out" {
        return if mask == 3 { 1 } else { 0 };
    }

    if let Some(&cached) = cache.get(&(current, mask)) {
        return cached;
    }

    let Some(outputs) = devices.get(current) else {
        return 0;
    };

    let count: usize = outputs.iter().map(|next| dfs(next, mask, devices, cache)).sum();

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
