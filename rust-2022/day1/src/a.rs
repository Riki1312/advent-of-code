pub fn run() {
    let result = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| x.lines().map(parse_str).sum::<u32>())
        .max()
        .unwrap();

    println!("{}", result);
}

fn parse_str(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}
