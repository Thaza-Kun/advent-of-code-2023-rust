advent_of_code::solution!(3);

use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Symbol<'a> {
    start: u32,
    end: u32,
    window: &'a Window<'a>,
    item: char,
    adjacents: Vec<u32>,
}
impl<'a> Symbol<'a> {
    fn from_windows(value: &'a Vec<Window<'a>>, pattern: &regex::Regex) -> Vec<Self> {
        let mut vector = Vec::<Symbol>::new();
        for window in value {
            for ch in pattern.find_iter(window.center) {
                vector.push(Symbol {
                    start: ch.start() as u32,
                    end: ch.end() as u32,
                    item: ch.as_str().parse::<char>().expect("Parsing error"),
                    window,
                    adjacents: Vec::<u32>::new(),
                })
            }
        }
        vector
    }

    fn collect_adjacents(mut self, pattern: &regex::Regex) -> Self {
        let scope = (self.start as usize).saturating_sub(1)
            ..=(std::cmp::min(self.end as usize, self.window.center.len() - 1));
        if let Some(s) = self.window.top {
            for c in pattern.find_iter(s) {
                if scope.contains(&c.start()) || scope.contains(&(c.end() - 1)) {
                    self.adjacents
                        .push(c.as_str().parse::<u32>().expect("Parsing error"))
                }
            }
        }
        for c in pattern.find_iter(&self.window.center) {
            if scope.contains(&c.start()) || scope.contains(&(c.end() - 1)) {
                self.adjacents
                    .push(c.as_str().parse::<u32>().expect("Parsing error"))
            }
        }
        if let Some(s) = self.window.bottom {
            for c in pattern.find_iter(s) {
                if scope.contains(&c.start()) || scope.contains(&(c.end() - 1)) {
                    self.adjacents
                        .push(c.as_str().parse::<u32>().expect("Parsing error"))
                }
            }
        }
        self.clone()
    }
}

#[derive(Debug)]
struct Window<'a> {
    top: Option<&'a str>,
    center: &'a str,
    bottom: Option<&'a str>,
}

impl<'a> Window<'a> {
    fn build_from_input(input: &'a str) -> Vec<Window<'a>> {
        let windows = input
            .trim()
            .split("\n")
            .tuple_windows::<(&'a str, &'a str, &'a str)>();
        let mut vector = Vec::<Window<'a>>::new();
        let (first, second, _) = windows.clone().next().unwrap();
        let (_, second_last, last) = windows.clone().last().unwrap();
        vector.push(Window {
            top: None,
            center: first,
            bottom: Some(second),
        });
        for item in windows {
            vector.push(Window {
                top: Some(item.0),
                center: item.1,
                bottom: Some(item.2),
            })
        }
        vector.push(Window {
            top: Some(second_last),
            center: last,
            bottom: None,
        });
        vector
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let windows = Window::build_from_input(input);
    let non_dot_pattern = regex::Regex::new(r#"[^.0-9]"#).expect("Invalid regex");
    let symbols = Symbol::from_windows(&windows, &non_dot_pattern);
    let number_pattern = regex::Regex::new(r#"[0-9]+"#).expect("Invalid regex");
    let symbols_with_adjacent = symbols
        .into_iter()
        .map(|s| s.clone().collect_adjacents(&number_pattern))
        .filter(|s| s.adjacents.len() > 0)
        .collect::<Vec<Symbol>>();
    Some(symbols_with_adjacent.into_iter().fold(0, |mut total, s| {
        total += s.adjacents.into_iter().sum::<u32>();
        total
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let windows = Window::build_from_input(input);
    let starchar = regex::Regex::new(r#"[*]"#).expect("Invalid regex");
    let characters = Symbol::from_windows(&windows, &starchar);
    let numbers = regex::Regex::new(r#"[0-9]+"#).expect("Invalid regex");
    let asterisk_with_adjacent_number = characters
        .into_iter()
        .map(|s| s.clone().collect_adjacents(&numbers))
        .filter(|s| s.adjacents.len() == 2)
        .collect::<Vec<Symbol>>();
    Some(
        asterisk_with_adjacent_number
            .into_iter()
            .fold(0, |mut total, s| {
                total += s.adjacents.into_iter().product::<u32>();
                total
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
