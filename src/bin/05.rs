use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (seed_list, maps) = input.trim().split_once("\n").expect("Splitting error");
    let (_, seeds) = seed_list.trim().split_once(":").expect("Splitting error");
    let seeds = seeds
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Parsing error"))
        .collect::<Vec<usize>>();
    let mut ans = usize::MAX;
    for mut seed in seeds {
        for map in maps.split("\n\n") {
            let (_name, table) = map.trim().split_once(":").expect("Splitting error");
            for line in table.trim().split("\n") {
                if line.is_empty() {
                    break;
                }
                let (d, s, r) = line
                    .trim()
                    .split_whitespace()
                    .map(|a| a.parse::<usize>().expect("Parsing error"))
                    .collect_tuple::<(usize, usize, usize)>()
                    .expect(&format!("Error collecting 3-tuples {}", &line));
                if (s..s + r).contains(&seed) {
                    if s > d {
                        seed -= s - d
                    } else if s == d {
                        seed = seed
                    } else {
                        seed += d - s
                    }
                    break;
                }
            }
        }
        ans = std::cmp::min(ans, seed);
    }
    Some(ans as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
