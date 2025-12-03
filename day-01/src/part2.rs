#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut dial = 50;
    let mut code = 0;
    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let num: i32 = line[1..].parse().map_err(|e| miette::miette!("Failed to parse number: {}", e))?;

        code += num / 100;
        let remainder = num % 100;

        if remainder != 0 {
            let old_dial = dial;
            dial = match dir {
                'L' => (dial - remainder).rem_euclid(100),
                'R' => (dial + remainder) % 100,
                _ => return Err(miette::miette!("Invalid direction: {}", dir)),
            };

            if ((dir == 'L' && (dial == 0 || dial > old_dial)) || (dir == 'R' && (dial == 0 || dial < old_dial)))
                && old_dial != 0 {
                    code += 1;
                }
        }
    }
    Ok(code.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
