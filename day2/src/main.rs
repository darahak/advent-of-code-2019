fn main() {
    let contents = get_input();
}

fn get_input() -> String {
    use std::fs;

    return fs::read_to_string("src/input.txt").unwrap();
}
