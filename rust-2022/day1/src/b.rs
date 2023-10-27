pub fn run() {
    let mut vec = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| x.lines().map(parse_str).sum::<u32>())
        .collect::<Vec<u32>>();
    vec.sort_unstable();

    let result = vec.into_iter().rev().take(3).sum::<u32>();
    println!("{}", result);
}

fn parse_str(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}
