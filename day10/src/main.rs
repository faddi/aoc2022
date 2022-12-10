use std::collections::VecDeque;

#[derive(Debug)]
enum OpCode {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct InstructionExecution {
    op_code: OpCode,
    execution_cycles: i32,
}

impl InstructionExecution {
    fn is_finished(&self) -> bool {
        match self.op_code {
            OpCode::Noop => self.execution_cycles == 1,
            OpCode::Addx(_) => self.execution_cycles == 2,
        }
    }
}

struct Cpu {
    cycle: i32,
    register: i32,
    current_execution: Option<InstructionExecution>,
    stack: VecDeque<OpCode>,
}

impl Cpu {
    fn new(register: i32, stack: VecDeque<OpCode>) -> Cpu {
        Cpu {
            cycle: 0,
            register: register,
            current_execution: None,
            stack: stack,
        }
    }

    fn get_next_instruction(&mut self) -> Option<InstructionExecution> {
        if self.stack.len() == 0 {
            return None;
        }

        let op_code = self.stack.pop_front().unwrap();
        return Some(InstructionExecution {
            op_code: op_code,
            execution_cycles: 0,
        });
    }

    fn run_one_cycle(&mut self) {
        match self.current_execution {
            Some(ref mut execution) => {
                if execution.is_finished() {
                    match self.current_execution.as_ref().unwrap().op_code {
                        OpCode::Noop => {}
                        OpCode::Addx(x) => {
                            self.register += x;
                        }
                    }
                    self.current_execution = self.get_next_instruction();
                }
            }
            None => {
                self.current_execution = self.get_next_instruction();
            }
        }

        if self.current_execution.is_none() {
            println!("Finished at cycle {}", self.cycle);
            return;
        }

        self.cycle += 1;
        self.current_execution.as_mut().unwrap().execution_cycles += 1;
    }
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let stack: VecDeque<OpCode> = file_contents
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let op_code = *parts.first().unwrap();

            match op_code {
                "noop" => OpCode::Noop,
                "addx" => OpCode::Addx(parts.last().unwrap().parse::<i32>().unwrap()),
                _ => panic!("Unknown op code {}", op_code),
            }
        })
        .collect();

    let mut cpu = Cpu::new(1, stack);

    let mut result = 0;

    let mut part2: Vec<char> = Vec::new();

    while cpu.stack.len() > 0 {
        // part2

        let diff = (cpu.cycle % 40) - cpu.register;

        // cba to fix end of crt line exception
        part2.push(if diff >= 0 && diff <= 2 { '#' } else { '.' });
        println!("{}", part2.last().unwrap());

        cpu.run_one_cycle();
        println!(
            "Cycle: {}, Op: {:?}, Register: {}",
            cpu.cycle, cpu.current_execution, cpu.register
        );

        if cpu.cycle == 20 || (cpu.cycle - 20) % 40 == 0 {
            result += cpu.register * cpu.cycle;
        }
    }

    println!("Result: {}", result);

    // print part 2 40 x n
    for i in 0..part2.len() / 40 {
        println!("{}", part2[i * 40..(i + 1) * 40].iter().collect::<String>());
    }
}
