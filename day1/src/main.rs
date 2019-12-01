fn get_input() -> String {
    use std::fs;

    return fs::read_to_string("src/input.txt").unwrap();
}

fn main() {
    let contents = get_input();
    let mass_values: Vec<&str> = contents.split('\n').collect();

    let mut fuel_sum: i32 = 0;

    for mass in mass_values {
        if !mass.is_empty() {
            let val: i32 = mass.parse().unwrap();
            fuel_sum += val / 3 - 2;
        }
    }

    println!("Fuel sum: {}", fuel_sum);
}
