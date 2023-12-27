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
        .collect();

    let result: i32 = elements.iter()
        .filter(|(_, indexes)| {
            indexes.iter()
                .map(|i| input.chars().nth(*i).expect("index must be in range"))
                .any(|c| !c.is_ascii_digit() && c != '.')
        })
        .map(|(e, _)| e.num)
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
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_process_1() {
        let input = "..........
.....*....
.....8....";
        let result = process(input).unwrap();
        assert_eq!(result, "8");
    }

    #[test]
    fn test_process_2() {
        let input = "..........
100.......
100.......
...*......";
        let result = process(input).unwrap();
        assert_eq!(result, "100");
    }

    #[test]
    fn test_compute_box_indexes() {
        /*
        00000
        00000
        00000
        0000X
        */
        let result = compute_box_indexes(19, 5, 4);
        let expected = vec![18, 14, 13];

        dbg!(&result);

        assert_eq!(result.len(), expected.len());
        assert_eq!(result.iter().all(|n| expected.contains(n)), true);
    }

    #[test]
    fn test_compute_box_indexes_1() {
        /*
        00000
        00000
        00000
        00X00
        */
        let result = compute_box_indexes(17, 5, 4);
        let expected = vec![16, 18, 13, 12, 11];

        dbg!(&result);

        assert_eq!(result.len(), expected.len());
        assert_eq!(result.iter().all(|n| expected.contains(n)), true);
    }

    #[test]
    fn test_compute_box_indexes_2() {
        /*
        00000
        00X00
        00000
        00000
        */
        let result = compute_box_indexes(7, 5, 4);
        let expected = vec![6, 8, 1, 2, 3, 11, 12, 13];

        dbg!(&result);

        assert_eq!(result.len(), expected.len());
        assert_eq!(result.iter().all(|n| expected.contains(n)), true);
    }

    #[test]
    fn test_compute_box_indexes_3() {
        /*
        X0000
        00000
        00000
        00000
        */
        let result = compute_box_indexes(0, 5, 4);
        let expected = vec![1, 5, 6];

        dbg!(&result);

        assert_eq!(result.len(), expected.len());
        assert_eq!(result.iter().all(|n| expected.contains(n)), true);
    }

    #[test]
    fn test_compute_box_indexes_4() {
        /*
        00000
        00000
        0000X
        00000
        */
        let result = compute_box_indexes(14, 5, 4);
        let expected = vec![13, 8, 9, 18, 19];

        dbg!(&result);

        assert_eq!(result.len(), expected.len());
        assert_eq!(result.iter().all(|n| expected.contains(n)), true);
    }

    #[test]
    fn test_convert_point() {
        /*
        X0000
        0P000
        00000
        00000
        */
        let result = convert_point(&(1, 1), 5);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_convert_index() {
        /*
        00000
        00000
        00000
        00X00
        */
        let result = convert_index(17, 5);
        assert_eq!(result, (2, 3));
    }

    #[test]
    fn test_convert_index_1() {
        /*
        0000000000
        0000000000
        0000000000
        00000X0000
        */
        let result = convert_index(35, 10);
        assert_eq!(result, (5, 3));
    }
}
