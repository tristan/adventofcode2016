#[derive(Clone, Copy)]
enum Register {
    A = 0,
    B,
    C,
    D
}

impl From<&str> for Register {
    fn from(input: &str) -> Self {
        match input {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => panic!("invalid register: {input}")
        }
    }
}

#[derive(Clone, Copy)]
enum Value {
    Register(Register),
    Number(i64),
}

impl From<&str> for Value {
    fn from(input: &str) -> Self {
        if &input[..1] == "-" {
            if let Ok(num) = input[1..].parse::<i64>() {
                Value::Number(-num)
            } else {
                panic!("invalid value");
            }
        } else if let Ok(num) = input.parse() {
            Value::Number(num)
        } else {
            Value::Register(input.into())
        }
    }
}

#[derive(Clone, Copy)]
enum Op {
    Cpy(Value, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Value, i64),
}

fn main() {
    let instructions = include_str!("../day_12_input.txt").trim().split('\n')
        .map(|line| {
            match &line[..3] {
                "cpy" => {
                    let (v1, v2) = line[4..].split_once(' ').unwrap();
                    Op::Cpy(v1.into(), v2.into())
                }
                "inc" => {
                    Op::Inc(line[4..].into())
                }
                "dec" => {
                    Op::Dec(line[4..].into())
                }
                "jnz" => {
                    let (v1, v2) = line[4..].split_once(' ').unwrap();
                    Op::Jnz(v1.into(), v2.parse().unwrap())
                }
                _ => unreachable!()
            }
        })
        .collect::<Vec<_>>();

    for part in [1, 2] {
        let mut offset = 0;
        let mut registers = [0, 0, part - 1, 0];
        while offset < instructions.len() {
            match instructions[offset] {
                Op::Cpy(from, to) => {
                    registers[to as usize] = match from {
                        Value::Register(reg) => registers[reg as usize],
                        Value::Number(value) => value,
                    };
                    offset += 1;
                }
                Op::Inc(reg) => {
                    registers[reg as usize] += 1;
                    offset += 1;
                }
                Op::Dec(reg) => {
                    registers[reg as usize] -= 1;
                    offset += 1;
                }
                Op::Jnz(test, dist) => {
                    let value = match test {
                        Value::Register(reg) => registers[reg as usize],
                        Value::Number(value) => value,
                    };
                    if value != 0 {
                        offset = (offset as i64 + dist) as usize;
                    } else {
                        offset += 1;
                    }
                }
            }
        }

        println!("part{}: {}", part, registers[0]);
    }
}
