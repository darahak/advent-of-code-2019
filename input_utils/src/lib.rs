pub mod aoc {
  pub fn get_input(path: &str) -> String {
    use std::fs;

    return fs::read_to_string(path).unwrap();
  }

  pub fn get_lines(contents: &String) -> Vec<&str> {
    return contents.trim().lines().collect();
  }
}
