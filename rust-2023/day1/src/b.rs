use utils::ProcessResult;

pub fn process(input: &str) -> ProcessResult {
    let result = input.lines()
        .map(process_line)
        .sum::<i32>();

    return Ok(result.to_string());
}

fn process_line(line: &str) -> i32 {
    let first = (0..line.len())
        .find_map(|i| { get_number(line, i) })
        .expect("first digit");

    let last = (0..line.len())
        .rev()
        .find_map(|i| { get_number(line, i) })
        .expect("last digit");

    format!("{}{}", first, last).parse::<i32>().expect("must be a number")
}

fn get_number(line: &str, idx: usize) -> Option<char> {
    line.chars().nth(idx)
        .filter(char::is_ascii_digit)
        .or_else(|| { find_number(line, idx) })
}

fn find_number(line: &str, idx: usize) -> Option<char> {
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter()
        .enumerate()
        .find(|(_, e)| {
            line[idx..].starts_with(e.chars().as_str())
        })
        .map(|(i, _)| {
            (i + 1).to_string().chars().nth(0)
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process(input).unwrap();
        assert_eq!(result, "281");
    }
}
