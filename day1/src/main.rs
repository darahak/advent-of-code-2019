fn get_input() -> String {
    use std::env;
    use std::fs;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing filename!");
    }

    let filename = &args[1];

    return fs::read_to_string(filename).unwrap();
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
