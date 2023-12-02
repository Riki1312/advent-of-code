use utils::ProcessResult;

pub fn process(input: &str) -> ProcessResult {
    Ok("Hello form day1-b!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "";
        let result = process(input).unwrap();
        assert_eq!(result, "Hello form day1-b!");
    }
}
