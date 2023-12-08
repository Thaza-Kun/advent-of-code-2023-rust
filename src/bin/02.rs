use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0u32;
    let mut test_value = HashMap::new();
    test_value.insert("red", 12);
    test_value.insert("green", 13);
    test_value.insert("blue", 14);
    dbg!(&test_value);
    for (game_id, games) in input.split("\n").enumerate() {
        let (_name, cube_sets) = games.split_once(":").unwrap_or(("", ""));
        let mut possible = true;
        for cubes in cube_sets.split(";") {
            dbg!(cubes);
            for cube in cubes.split(",") {
                let (val, colour) = cube.trim().split_once(" ").unwrap_or(("1000", "red"));
                if test_value.get(colour).expect("Invalid key")
                    < &val.parse::<u32>().ok().expect("Invalid number")
                {
                    possible = false;
                }
            }
        }
        if possible == true {
            total += game_id as u32 + 1
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0u32;
    let mut test_value = HashMap::new();
    for (game_id, games) in input.split("\n").enumerate() {
        let (_name, cube_sets) = games.split_once(":").unwrap_or(("", ""));
        test_value.insert("red", 0);
        test_value.insert("green", 0);
        test_value.insert("blue", 0);
        for cubes in cube_sets.split(";") {
            for cube in cubes.split(",") {
                let (val, colour) = cube.trim().split_once(" ").unwrap_or(("0", "red"));
                let val = val.parse::<u32>().ok().expect("Invalid number");
                if test_value.get(colour).expect("Invalid key") < &val {
                    test_value.insert(colour, val);
                }
            }
        }
        let power = test_value.get("red")? * test_value.get("green")? * test_value.get("blue")?;
        total += power;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
