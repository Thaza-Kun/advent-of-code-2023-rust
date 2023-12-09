use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0u32;
    for cards in input.trim().split("\n") {
        let (_card_id, card) = cards.split_once(":").expect("Delimiter not found");
        let (winning, owned) = card.trim().split_once("|").expect("Delimiter not found");
        let win_numbers = winning
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Invalid number"))
            .collect::<Vec<u32>>();
        let own_numbers = owned
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Invalid number"))
            .filter(|s| win_numbers.contains(s))
            .collect::<Vec<u32>>();
        if own_numbers.len() > 0 {
            total += 2u32.pow(own_numbers.len().saturating_sub(1) as u32);
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut instances = HashMap::<u32, u32>::new();
    for (ident, cards) in input.trim().split("\n").enumerate() {
        let (_card_id, card) = cards.split_once(":").expect("Delimiter not found");
        let copies = match instances.get(&((ident + 1) as u32)) {
            Some(i) => i.to_owned(),
            None => {
                instances.insert((ident + 1) as u32, 1);
                1
            }
        };
        let (winning, owned) = card.trim().split_once("|").expect("Delimiter not found");
        let win_numbers = winning
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Invalid number"))
            .collect::<Vec<u32>>();
        let own_numbers = owned
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Invalid number"))
            .filter(|s| win_numbers.contains(s))
            .collect::<Vec<u32>>();
        if own_numbers.len() > 0 {
            for next_cards in ident + 2..=ident + own_numbers.len() + 1 {
                let curr = instances.get(&(next_cards as u32)).unwrap_or(&1u32);
                instances.insert(next_cards as u32, *curr + (1 * copies));
            }
        }
    }
    Some(instances.iter().fold(0, |acc, i| acc + i.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
