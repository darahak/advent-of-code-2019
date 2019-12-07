use input_utils::aoc;

// Valid opcodes:
// 1: add
// 2: multiply
// 99: end of program
fn main() {
    let contents = aoc::get_input("src/input.txt");
    let codes: Vec<usize> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    part1(codes.clone().as_mut_slice());
    part2(&codes);
}

fn part1(opcodes: &mut [usize]) {
    opcodes[1] = 12;
    opcodes[2] = 2;

    let result = run_opcodes(opcodes);

    println!("Result for part 1: {}", result);
}

fn part2(opcodes: &Vec<usize>) {
    let expected: usize = 19690720;
    let (noun, verb) = find_inputs_for_result(expected, &opcodes);

    if noun == 0 && verb == 0 {
        println!("Failed to find inputs for result {}", expected);
    } else {
        println!("Inputs for result {}: {} and {}", expected, noun, verb);
    }
}

fn run_opcodes(opcodes: &mut [usize]) -> usize {
    let mut i = 0;

    while i < opcodes.len() {
        let a = opcodes[i + 1];
        let b = opcodes[i + 2];
        let res = opcodes[i + 3];

        match opcodes[i] {
            1 => {
                opcodes[res] = opcodes[a] + opcodes[b];

                i += 4;
            }
            2 => {
                opcodes[res] = opcodes[a] * opcodes[b];

                i += 4;
            }
            99 => {
                // End of program.
                break;
            }
            _ => println!("Unexpected opcode {} at position {}", opcodes[i], i),
        }
    }

    return opcodes[0];
}

fn find_inputs_for_result(expected: usize, opcodes: &Vec<usize>) -> (usize, usize) {
    let max: usize = 100;
    let mut noun: usize = 0;
    let mut verb: usize = 0;

    for i in 0..max {
        for j in 0..max {
            let mut clone = opcodes.clone();
            let program = clone.as_mut_slice();

            program[1] = i;
            program[2] = j;

            let result = run_opcodes(program);

            if result == expected {
                noun = i;
                verb = j;

                break;
            }
        }
    }

    return (noun, verb);
}
