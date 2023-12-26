use regex::Regex;
use utils::ProcessResult;

pub fn process(input: &str) -> ProcessResult {
    let result = input.lines()
        .map(parse_line)
        .map(compute_game_set)
        .map(|set| { set.red * set.green * set.blue })
        .sum::<i32>();

    return Ok(result.to_string());
}

struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

struct GameSet {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_line(line: &str) -> Game {
    let split: Vec<&str> = line.split(":").collect();

    Game {
        id: split[0].replace("Game ", "").parse().expect("id must be a number"),
        sets: split[1].split(";")
            .map(|set| {
                GameSet {
                    red: extract_number(set, "red"),
                    green: extract_number(set, "green"),
                    blue: extract_number(set, "blue"),
                }
            }).collect(),
    }
}

fn extract_number(text: &str, str: &str) -> i32 {
    let pattern = Regex::new(format!(r"(\d+)\s+{str}").as_str()).unwrap();

    pattern.captures(text)
        .and_then(|captures| { captures.get(1) })
        .and_then(|result| { result.as_str().parse::<i32>().ok() })
        .unwrap_or(0)
}

fn compute_game_set(game: Game) -> GameSet {
    GameSet {
        red: game.sets.iter().map(|set| { set.red }).max().unwrap_or(0),
        green: game.sets.iter().map(|set| { set.green }).max().unwrap_or(0),
        blue: game.sets.iter().map(|set| { set.blue }).max().unwrap_or(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(input).unwrap();
        assert_eq!(result, "2286");
    }
}
