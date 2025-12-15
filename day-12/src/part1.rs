use miette::miette;
use nalgebra::SMatrix;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1, u32 as nom_u32},
    combinator::value,
    multi::{count, many1, separated_list0, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};


// This was a weird day.
// I included nalgreba, expecting to do matrix operations (fast for flips/rotations)
// Decided to do bitwise mapping, assuming I could quickly test overlaps etc.
// Loaded in the puzzle data.
// Since this is typical binpacking, I decided to filter out regions that could never fit because the presents were too big
// I printed this as a dbg! output so I could see the numbers and see if I was parsing ok
// Then I noticed the first few regions that DID fit, all fit A LOT. There was tons of free space.
// So I checked them all visually, didnt even compute it. It looked like they all fit very well.
// Surely..no? I decided to just try the current count in the puzzle solve, assuming it would be wrong and I had to start for real.
// But it was correct. Ok then.

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (presents, regions)) = parse_input(input).map_err(|e| miette!("parse failed {}", e))?;

    // this was my debug loop
    for region in &regions {
        if !region.fits(&presents) {
            continue;
        }
        let area = region.area();
        let total_size = region.total_present_size(&presents);
        println!(
            "{}x{} presents={:?} | area={} total_size={}",
            region.width, region.height, region.presents, area, total_size
        );
    }

    let valid_regions: Vec<_> = regions
        .into_iter()
        .filter(|r| r.fits(&presents))
        .collect();

    Ok(valid_regions.len().to_string())
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Present>, Vec<Region>)> {
    let (input, shapes) = many1(terminated(
        parse_shape,
        tag("\n\n")
    )).parse(input)?;
    let (input, regions) = separated_list1(line_ending, parse_region).parse(input)?;
    Ok((input, (shapes, regions)))
}

fn parse_shape(input: &str) -> IResult<&str, Present> {
    let (input, _) = terminated(digit1, tag(":\n")).parse(input)?;
    let (input, rows) = separated_list1(line_ending, parse_row).parse(input)?;
    let data: Vec<u8> = rows.into_iter().flatten().collect();
    Ok((input, Present::from_row_slice(&data)))
}

fn parse_row(input: &str) -> IResult<&str, [u8; 3]> {
    let (input, cells) = count(parse_cell, 3).parse(input)?;
    Ok((input, [cells[0], cells[1], cells[2]]))
}

fn parse_cell(input: &str) -> IResult<&str, u8> {
    alt((
        value(1, tag("#")),
        value(0, tag(".")),
    )).parse(input)
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    // Parse "AxB: count0 count1 count2 ..."
    let (input, (width, height)) = separated_pair(nom_u32, tag("x"), nom_u32).parse(input)?;
    let (input, _) = tag(": ").parse(input)?;
    let (input, presents) = separated_list0(space1, nom_u32).parse(input)?;
    Ok((input, Region {
        width,
        height,
        presents,
    }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Present {
    grid: SMatrix<u8, 3, 3>,
    size: u8,
}

impl Present {
    fn from_row_slice(data: &[u8]) -> Self {
        let grid = SMatrix::from_row_slice(data);
        let size = data.iter().sum();
        Self { grid, size }
    }

    fn print(&self) {
        for r in 0..3 {
            for c in 0..3 {
                print!("{}", if self.grid[(r, c)] == 1 { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    width: u32,
    height: u32,
    presents: Vec<u32>
}

impl Region {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn total_present_size(&self, all_presents: &[Present]) -> u32 {
        self.presents.iter()
            .enumerate()
            .map(|(idx, &count)| count * all_presents[idx].size as u32)
            .sum()
    }

    fn fits(&self, all_presents: &[Present]) -> bool {
        self.total_present_size(all_presents) <= self.area()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
