use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let re = Regex::new(
        r"(?x)
                mul
                \(
                (?<op1>[0-9]{1,3})
                ,
                (?<op2>[0-9]{1,3})
                \)
             "
    ).unwrap();

    let sum = re
        .captures_iter(input)
        .map(|cap| cap["op1"].parse::<u32>().unwrap() * cap["op2"].parse::<u32>().unwrap())
        .sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let re = Regex::new(
        r"(?x)
            (?<do>do\(\))           # match literal `do()`
            |(?<dont>don't\(\))     # match literal `don't()`
            |mul                    # match literal `mul`
            \(
            (?<op1>[0-9]{1,3})      # first operand
            ,
            (?<op2>[0-9]{1,3})      # second operand
            \)
        ",
    )
        .unwrap();

    let mut enabled = true;

    let sum = re
        .captures_iter(input)
        .filter_map(|cap| {
            let r#match = &cap[0];

            match (r#match, enabled) {
                ("do()", _) => {
                    enabled = true;
                    None
                }
                ("don't()", _) => {
                    enabled = false;
                    None
                }
                (_, true) => {
                    let op1 = cap["op1"].parse::<u32>().ok()?;
                    let op2 = cap["op2"].parse::<u32>().ok()?;
                    Some(op1 * op2)
                }
                _ => None,
            }
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
