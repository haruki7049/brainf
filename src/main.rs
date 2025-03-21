use std::io::Read;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    execute_brainfuck(&input);
}

fn execute_brainfuck(code: &str) {
    let mut memory = vec![0u8; 30000];
    let mut pointer: usize = 0;
    let mut pc: usize = 0;

    let mut loop_stack = Vec::new();

    while pc < code.len() {
        match code.as_bytes()[pc] as char {
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => memory[pointer] = memory[pointer].wrapping_add(1),
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            '.' => print!("{}", memory[pointer] as char),
            ',' => {
                let mut input = [0];
                std::io::stdin().read_exact(&mut input).expect("Failed to read from stdin");
                memory[pointer] = input[0];
            }
            '[' => {
                if memory[pointer] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets > 0 {
                        pc += 1;
                        match code.as_bytes()[pc] as char {
                            '[' => open_brackets += 1,
                            ']' => open_brackets -= 1,
                            _ => {}
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    pc = *loop_stack.last().expect("Loop stack underflow");
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }
        pc += 1;
    }
}
