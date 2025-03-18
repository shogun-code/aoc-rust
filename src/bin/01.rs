advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let test_input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";

    let (mut lhs, mut rhs) = (Vec::new(), Vec::new());

    for line in test_input.lines() {
        let nums = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect::<Vec<_>>();
        lhs.push(nums[0]);
        rhs.push(nums[1]);
    }

    lhs.sort();
    rhs.sort();

    let result = lhs.iter().zip(rhs.iter()).map(|(a, b)| (a - b).abs()).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let test_input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";

    let (mut lhs, mut rhs) = (Vec::new(), Vec::new());

    for line in test_input.lines() {
        let nums = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect::<Vec<_>>();
        lhs.push(nums[0]);
        rhs.push(nums[1]);
    }

    let mut sum = 0;
    for num_l in &lhs {
        sum += rhs.iter().filter(|&num_r| num_l == num_r).sum::<i32>();
    }

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
