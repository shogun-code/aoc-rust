advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let mut safe_report = 0;
    for line in test_input.lines() {
        let report = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<u32>>();


        if is_safe(&report) {
            safe_report += 1;
        }
    }

    Some(safe_report)
}

pub fn part_two(input: &str) -> Option<u32> {
    let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let mut safe_report = 0;
    for line in test_input.lines() {
        let report = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<u32>>();


        if is_safe(&report) {
            safe_report += 1;
            continue;
        }

        problem_dampener(&report, &mut safe_report);
    }

    Some(safe_report)
}

fn is_safe(report: &[u32]) -> bool {
    let is_increasing = report
        .windows(2)
        .all(|w| (w[0] <= w[1]) && (1..=3).contains(&(w[1] - w[0])));
    let is_decreasing = report
        .windows(2)
        .all(|w| (w[0] >= w[1]) && (1..=3).contains(&(w[0] - w[1])));

    is_increasing || is_decreasing
}

fn problem_dampener(report: &[u32], safe_report: &mut u32) {
    let mut report = report.to_vec();

    for (i, lvl) in report.clone().iter().enumerate() {
        let removed = report.remove(i);

        if is_safe(&report) {
            *safe_report += 1;
            report.insert(i, removed);
            return;
        }
        report.insert(i, removed);
    }
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
