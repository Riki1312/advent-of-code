use std::collections::HashSet;
use regex::Regex;
use utils::ProcessResult;

#[derive(Debug)]
struct Element {
    num: i32,
    start: usize,
    size: usize,
}

pub fn process(raw: &str) -> ProcessResult {
    let size_y = raw.lines().count();
    let size_x = raw.lines().last().unwrap().chars().count();
    let input = raw.replace("\n", "");

    let elements: Vec<(Element, HashSet<usize>)> = Regex::new(r"\d+").unwrap()
        .find_iter(input.as_str())
        .map(|m| Element {
            num: m.as_str().parse().expect("string must be a number"),
            start: m.start(),
            size: m.as_str().len(),
        })
        .map(|e| {
            let mut indexes = HashSet::new();
            for i in e.start..=(e.start + e.size - 1) {
                indexes.extend(compute_box_indexes(i, size_x, size_y));
            }

            (e, indexes)
        })
        .map(|(e, indexes)| {
            (e, indexes.into_iter()
                .filter(|i| input.chars().nth(*i).expect("index must be in range") == '*')
                .collect::<HashSet<usize>>())
        })
        .collect();

    let gears: HashSet<usize> = elements.iter()
        .flat_map(|(_, indexes)| indexes.clone())
        .collect();

    let gears_nums: HashSet<(usize, Vec<i32>)> = gears.into_iter()
        .map(|i| {
            (i, elements.iter()
                .filter(|(_, indexes)| indexes.contains(&i))
                .map(|(e, _)| e.num)
                .collect::<Vec<i32>>())
        })
        .filter(|(_, v)| v.len() >= 2)
        .collect();

    let result: i32 = gears_nums.into_iter()
        .map(|(_, v)| v.iter().product::<i32>())
        .sum();

    return Ok(result.to_string());
}

fn compute_box_indexes(i: usize, size_x: usize, size_y: usize) -> HashSet<usize> {
    let (x, y) = convert_index(i, size_x);
    let points: Vec<(i32, i32)> = vec![
        (x - 1, y),
        (x + 1, y),
        (x, y + 1),
        (x, y - 1),
        (x - 1, y + 1),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
    ];

    HashSet::from_iter(
        points.iter()
            .filter(|p| p.0 >= 0 && p.1 >= 0)
            .filter(|p| p.0 < size_x as i32 && p.1 < size_y as i32)
            .map(|p| convert_point(p, size_x))
    )
}

fn convert_index(i: usize, size_x: usize) -> (i32, i32) {
    let x = i % size_x;
    let y = i / size_x;
    (x as i32, y as i32)
}

fn convert_point(point: &(i32, i32), size_x: usize) -> usize {
    (point.1 * size_x as i32 + point.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(input).unwrap();
        assert_eq!(result, "467835");
    }
}
