fn main() {
    println!("Hello, world!");
}

fn get_input() -> String {
    use std::fs;

    return fs::read_to_string("src/input.txt").unwrap();
}
