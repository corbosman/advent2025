use miette::miette;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, line_ending, u64 as nom_u64},
    multi::{separated_list0, separated_list1},
    sequence::delimited,
    IResult, Parser,
};
use rayon::prelude::*;
use z3::{Optimize, SatResult};
use z3::ast::Int;

// I had to google this puzzle. I did realize it had something to do with linear algebra because the puzzle has a sentence:
//
// "You have to push each button an integer number of times; there's no such thing as "0.5 presses" (nor can you push a button a negative number of times)."
//
// This screamed "math solution" to me as no one would push a button a fraction of a time.
// Basically this is a problem where we have a number of equations with unknown variables.
// I dont do linear algebra much, so I had to look up how to solve this. I found the "gaussian elimination" method.
// I tried to solve this myself using matrices, but I couldnt find a working solution.
// Then I checked for existing rust crates, found some, but neither worked for me.
// Eventually z3 worked
#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = parse_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let total: i64 = machines
        .par_iter()
        .enumerate()
        .map(|(i, machine)| {
            solve_z3(&machine.buttons, &machine.joltage)
                .unwrap_or_else(|| panic!("Machine {i} does not have a solution"))
                .iter()
                .sum::<i64>()
        })
        .sum();

    Ok(total.to_string())
}

fn solve_z3(buttons: &[Vec<usize>], target: &[i64]) -> Option<Vec<i64>> {
    let opt = Optimize::new();

    // this is the target joltage we want to solve for
    let m = target.len();

    // this is the unknown variables we need to solve for (the buttons)
    let n = buttons.len();

    // create the variables for z3 (x1,x2 etc), and add the constraint that they are greater than 0
    let x: Vec<Int> = (0..n)
        .map(|_| {
            let v = Int::fresh_const("x");
            opt.assert(&v.ge(&Int::from_i64(0)));
            v
        })
        .collect();

    // this creates the equations, like x1 + x2 + x3 = 3
    for i in 0..m {
        let mut sum = Int::from_i64(0);
        for j in 0..n {
            if buttons[j].contains(&i) {
                sum = &sum + &x[j];
            }
        }
        opt.assert(&sum.eq(&Int::from_i64(target[i])));
    }

    // tell z3 to minimize the total presses
    let total = x.iter().fold(Int::from_i64(0), |acc, v| &acc + v);
    opt.minimize(&total);

    // solve!
    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            let presses = x
                .iter()
                .map(|v| model.eval(v, true).unwrap().as_i64().unwrap())
                .collect();
            Some(presses)
        }
        _ => None,
    }
}

// INPUT PARSING

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(
        line_ending,
        (parse_target, tag(" "), parse_buttons, tag(" "), parse_joltage)
            .map(|(_, _, buttons, _, joltage)| Machine { buttons, joltage }),
    )
    .parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(tag("{"), separated_list1(char(','), nom_u64), tag("}"))
        .map(|v: Vec<u64>| v.into_iter().map(|x| x as i64).collect())
        .parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(tag(" "), parse_button).parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, indices) =
        delimited(tag("("), separated_list0(char(','), nom_u64), tag(")")).parse(input)?;
    Ok((input, indices.into_iter().map(|i| i as usize).collect()))
}

fn parse_target(input: &str) -> IResult<&str, u64> {
    let (input, pattern) = delimited(char('['), take_until("]"), char(']')).parse(input)?;
    let value = pattern
        .chars()
        .enumerate()
        .fold(0u64, |acc, (i, c)| acc | if c == '#' { 1 << i } else { 0 });
    Ok((input, value))
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
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
        assert_eq!("33", process(input)?);
        Ok(())
    }
}
