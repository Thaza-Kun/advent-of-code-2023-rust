advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let digits = regex::Regex::new(r#"\d"#).unwrap();
    let mut total: u32 = 0;
    for line in input.split("\n") {
        let captures = digits
            .find_iter(line)
            .fold(String::from(""), |curr, next| curr + next.as_str());
        let first_and_last = captures.chars().next().unwrap_or_default().to_string()
            + captures
                .chars()
                .last()
                .unwrap_or_default()
                .to_string()
                .as_str();
        total += dbg!(first_and_last.parse::<u32>().unwrap_or(0));
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits =
        regex::Regex::new(r#"\d|(oneight|twone|threeight|fiveight|nineight|tenine|eightwo|eighthree|sevenine|eighten)|(one|two|three|four|five|six|seven|eight|nine|ten)"#)
            .unwrap();
    let digits_regex_set = regex::RegexSet::new([
        r#"\d"#,
        r#"(oneight|twone|threeight|fiveight|nineight|tenine|eightwo|eighthree|sevenine|eighten)|(one|two|three|four|five|six|seven|eight|nine|ten)"#,
    ])
    .unwrap();
    let mut total = 0;
    let convert_to_number = |input: &str, first: bool| -> String {
        let matches = digits_regex_set.matches(input);
        if matches.matched(0) {
            String::from(input)
        } else if matches.matched(1) {
            match (input, first) {
                ("one", _) => String::from("1"),
                ("two", _) => String::from("2"),
                ("three", _) => String::from("3"),
                ("four", _) => String::from("4"),
                ("five", _) => String::from("5"),
                ("six", _) => String::from("6"),
                ("seven", _) => String::from("7"),
                ("eight", _) => String::from("8"),
                ("nine", _) => String::from("9"),
                ("ten", _) => String::from("10"),
                // Handle mixed cases (take last)
                ("oneight", false) => String::from("8"),
                ("twone", false) => String::from("1"),
                ("eightwo", false) => String::from("2"),
                ("eighthree", false) => String::from("3"),
                ("threeight", false) => String::from("8"),
                ("fiveight", false) => String::from("8"),
                ("nineight", false) => String::from("8"),
                ("sevenine", false) => String::from("9"),
                ("tenine", false) => String::from("9"),
                ("eighten", false) => String::from("10"),
                // Handle mixed cases (take first)
                ("oneight", true) => String::from("1"),
                ("twone", true) => String::from("2"),
                ("eightwo", true) => String::from("8"),
                ("eighthree", true) => String::from("8"),
                ("threeight", true) => String::from("3"),
                ("fiveight", true) => String::from("5"),
                ("nineight", true) => String::from("9"),
                ("sevenine", true) => String::from("7"),
                ("tenine", true) => String::from("10"),
                ("eighten", true) => String::from("8"),
                (any, _) => todo!("{}", any),
            }
        } else {
            unimplemented!()
        }
    };
    for line in input.split("\n") {
        let captures = digits
            .find_iter(line)
            .into_iter()
            .fold(vec![], |mut v, next| {
                v.push(next.as_str());
                v
            });
        if let Some(i) = captures.first() {
            let first = convert_to_number(*i, true)
                .chars()
                .next()
                .unwrap_or_default()
                .to_string();
            let last = convert_to_number(dbg!(*captures.last().unwrap()), false)
                .chars()
                .last()
                .unwrap_or_default()
                .to_string();
            let first_and_last = first + last.as_str();
            assert_eq!(first_and_last.len(), 2);
            total += dbg!(first_and_last.parse::<u32>().unwrap_or(0));
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
