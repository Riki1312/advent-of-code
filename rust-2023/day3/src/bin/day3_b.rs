use day3::b::process;

fn main() {
    let input = include_str!("../../input_b.txt");
    let result = process(input).expect("processing part b");
    println!("{}", result);
}
