use day3::a::process;

fn main() {
    let input = include_str!("../../input_a.txt");
    let result = process(input).expect("processing part a");
    println!("{}", result);
}
