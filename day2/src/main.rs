// Valid opcodes:
// 1: add
// 2: multiply
// 99: end of program
fn main() {
    let contents = get_input();
    let mut codes: Vec<usize> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    run_opcodes(codes.as_mut_slice());

    println!("Value at position 0: {}", codes[0]);
}

fn run_opcodes(opcodes: &mut [usize]) {
    opcodes[1] = 12;
    opcodes[2] = 2;

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
                return;
            }
            _ => println!("Unexpected opcode {} at position {}", opcodes[i], i),
        }
    }
}

fn get_input() -> String {
    use std::fs;

    return fs::read_to_string("src/input.txt").unwrap();
}
