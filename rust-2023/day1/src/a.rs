use utils::ProcessResult;

pub fn process(input: &str) -> ProcessResult {
    let result = input.lines()
        .map(process_line)
        .sum::<i32>();

    return Ok(result.to_string());
}

fn process_line(line: &str) -> i32 {
    let first = line.chars()
        .find(char::is_ascii_digit)
        .expect("first digit");
    let last = line.chars()
        .rev()
        .find(char::is_ascii_digit)
        .expect("last digit");

    format!("{}{}", first, last).parse::<i32>().expect("must be a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(input).unwrap();
        assert_eq!(result, "142");
    }
}
