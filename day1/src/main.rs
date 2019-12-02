fn main() {
    let contents = get_input();
    let mass_values: Vec<&str> = contents.split('\n').collect();

    part1(&mass_values);
    part2(&mass_values);
}

fn part1(mass_values: &[&str]) {
    let mut fuel_sum: i32 = 0;

    for mass in mass_values {
        if !mass.is_empty() {
            let val: i32 = mass.trim().parse().unwrap();
            fuel_sum += get_fuel_for_mass(val);
        }
    }

    println!("Fuel sum #1: {}", fuel_sum);
}

fn part2(mass_values: &[&str]) {
    let mut fuel_sum: i32 = 0;

    for mass in mass_values {
        if !mass.is_empty() {
            let val: i32 = mass.trim().parse().unwrap();
            fuel_sum += get_total_fuel_for_mass(val);
        }
    }

    println!("Fuel sum #2: {}", fuel_sum);
}

fn get_input() -> String {
    use std::fs;

    return fs::read_to_string("src/input.txt").unwrap();
}

fn get_fuel_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_total_fuel_for_mass(mass: i32) -> i32 {
    let mut fuel_amount = get_fuel_for_mass(mass);

    if fuel_amount > 0 {
        fuel_amount += get_total_fuel_for_mass(fuel_amount);
    }

    return if fuel_amount < 0 { 0 } else { fuel_amount };
}
