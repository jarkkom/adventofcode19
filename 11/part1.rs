use std::collections::HashSet;

struct CPU {
    debug: bool,
    ip: usize,
    bp: i64,
    mem: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
}

impl CPU {
    fn read_value(&mut self, arg: i64, mode: i64) -> i64 {
        match mode {
            0 => self.mem[arg as usize],
            1 => arg,
            2 => self.mem[(self.bp + arg) as usize],
            _ => panic!("invalid read mode"),
        }
    }

    fn write_value(&mut self, arg: i64, mode: i64) -> i64 {
        match mode {
            0 => arg,
            1 => panic!("invalid write mode"),
            2 => self.bp + arg,
            _ => panic!("invalid write mode"),
        }
    }

    pub fn execute(&mut self) -> bool {
        let instr = self.mem[self.ip];

        let opcode = instr % 100;
        let arg_mode_1 = (instr / 100) % 10;
        let arg_mode_2 = (instr / 1000) % 10;
        let dest_mode = (instr / 10000) % 10;

        if self.debug {
            print!(
                "ip {} bp {} instr {} opcode {} m1 {} m2 {} dm {} ",
                self.ip, self.bp, instr, opcode, arg_mode_1, arg_mode_2, dest_mode
            );
        }

        match opcode {
            1 => {
                let arg_1 = self.mem[self.ip + 1];
                let arg_2 = self.mem[self.ip + 2];
                let dest = self.mem[self.ip + 3];
                let value_1 = self.read_value(arg_1, arg_mode_1);
                let value_2 = self.read_value(arg_2, arg_mode_2);
                let value_dest = self.write_value(dest, dest_mode) as usize;

                if self.debug {
                    println!(
                        "add {} {}, {} {}, {} {}",
                        arg_1, value_1, arg_2, value_2, dest, value_dest
                    );
                }

                self.mem[value_dest] = value_1 + value_2;
                self.ip += 4;
            }
            2 => {
                let arg_1 = self.mem[self.ip + 1];
                let arg_2 = self.mem[self.ip + 2];
                let dest = self.mem[self.ip + 3];
                let value_1 = self.read_value(arg_1, arg_mode_1);
                let value_2 = self.read_value(arg_2, arg_mode_2);
                let value_dest = self.write_value(dest, dest_mode) as usize;

                if self.debug {
                    println!(
                        "mul {} {}, {} {}, {} {}",
                        arg_1, value_1, arg_2, value_2, dest, value_dest
                    );
                }

                self.mem[value_dest as usize] = value_1 * value_2;
                self.ip += 4;
            }
            3 => {
                let arg_1 = self.mem[self.ip + 1];
                let value_1 = self.write_value(arg_1, arg_mode_1) as usize;
                let input = self.input.remove(0);

                if self.debug {
                    println!("in {} {} {}", arg_1, value_1, input);
                }

                self.mem[value_1 as usize] = input;
                self.ip += 2;
            }
            4 => {
                let arg_1 = self.mem[self.ip + 1];
                let value_1 = self.read_value(arg_1, arg_mode_1);

                if self.debug {
                    println!("out {} {}", arg_1, value_1);
                }

                self.output.push(value_1);
                self.ip += 2;
            }
            5 => {
                let arg_1 = self.mem[self.ip + 1];
                let arg_2 = self.mem[self.ip + 2];
                let value_1 = self.read_value(arg_1, arg_mode_1);
                let value_2 = self.read_value(arg_2, arg_mode_2);

                if self.debug {
                    println!("jnz {} {}, {} {},", arg_1, value_1, arg_2, value_2);
                }

                if value_1 != 0 {
                    self.ip = value_2 as usize;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                let arg_1 = self.mem[self.ip + 1];
                let arg_2 = self.mem[self.ip + 2];
                let value_1 = self.read_value(arg_1, arg_mode_1);
                let value_2 = self.read_value(arg_2, arg_mode_2);

                if self.debug {
                    println!("jz {} {}, {} {}", arg_1, value_1, arg_2, value_2);
                }

                if value_1 == 0 {
                    self.ip = value_2 as usize;
                } else {
                    self.ip += 3;
                }
            }
            7 => {
                let arg_1 = self.mem[self.ip + 1];
                let arg_2 = self.mem[self.ip + 2];
                let dest = self.mem[self.ip + 3];
                let value_1 = self.read_value(arg_1, arg_mode_1);
                let value_2 = self.read_value(arg_2, arg_mode_2);
                let value_dest = self.write_value(dest, dest_mode) as usize;

                if self.debug {
                    println!(
                        "setle {} {}, {} {}, {} {}",
                        arg_1, value_1, arg_2, value_2, dest, value_dest
                    );
                }

                self.mem[value_dest] = if value_1 < value_2 { 1 } else { 0 };
                self.ip += 4;
            }
            8 => {
                let arg_1 = self.mem[self.ip + 1];
                let arg_2 = self.mem[self.ip + 2];
                let dest = self.mem[self.ip + 3];
                let value_1 = self.read_value(arg_1, arg_mode_1);
                let value_2 = self.read_value(arg_2, arg_mode_2);
                let value_dest = self.write_value(dest, dest_mode) as usize;

                if self.debug {
                    println!(
                        "seteq {} {}, {} {}, {} {}",
                        arg_1, value_1, arg_2, value_2, dest, value_dest
                    );
                }

                self.mem[value_dest] = if value_1 == value_2 { 1 } else { 0 };
                self.ip += 4;
            }
            9 => {
                let arg_1 = self.mem[self.ip + 1];
                let value_1 = self.read_value(arg_1, arg_mode_1);

                if self.debug {
                    println!("addbp {} {}", arg_1, value_1);
                }
                self.bp += value_1;
                self.ip += 2;
            }
            99 => {
                println!();
                return false;
            }
            _ => {
                println!("invalid opcode {}", opcode);
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut code = vec![
        3,8,1005,8,351,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,
        28,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,51,1006,0,85,2,1109,8,10,3,8,1002,8,-1,
        10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,80,1,2,2,10,1,1007,19,10,1,1001,13,10,3,8,1002,8,-1,10,1001,
        10,1,10,4,10,108,1,8,10,4,10,1001,8,0,113,1,2,1,10,1,1109,17,10,1,108,20,10,2,1005,3,10,3,8,102,-1,8,10,
        1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,151,2,5,19,10,1,104,19,10,1,109,3,10,1006,0,78,3,8,102,-1,8,10,
        1001,10,1,10,4,10,1008,8,0,10,4,10,1002,8,1,189,1006,0,3,2,1004,1,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,
        8,1,10,4,10,1001,8,0,218,1,1008,6,10,1,104,8,10,1006,0,13,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,
        10,102,1,8,251,1006,0,17,1006,0,34,1006,0,24,1006,0,4,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,
        102,1,8,285,1006,0,25,2,1103,11,10,1006,0,75,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,316,
        2,1002,6,10,1006,0,30,2,106,11,10,1006,0,21,101,1,9,9,1007,9,1072,10,1005,10,15,99,109,673,104,0,104,1,21101,
        0,937151009684,1,21101,0,368,0,1105,1,472,21102,386979963796,1,1,21102,379,1,0,1106,0,472,3,10,104,0,104,1,3,
        10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,179410325723,0,1,
        21101,426,0,0,1106,0,472,21101,0,179355823195,1,21102,437,1,0,1106,0,472,3,10,104,0,104,0,3,10,104,0,104,0,
        21101,0,825460785920,1,21101,460,0,0,1105,1,472,21102,1,838429614848,1,21102,1,471,0,1105,1,472,99,109,2,
        21202,-1,1,1,21102,40,1,2,21102,1,503,3,21101,493,0,0,1105,1,536,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,
        -1,1001,498,499,514,4,0,1001,498,1,498,108,4,498,10,1006,10,530,1101,0,0,498,109,-2,2106,0,0,0,109,4,2101,0,
        -1,535,1207,-3,0,10,1006,10,553,21101,0,0,-3,21202,-3,1,1,22101,0,-2,2,21101,0,1,3,21101,572,0,0,1105,1,577,
        109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,600,2207,-4,-2,10,1006,10,600,21202,-4,1,-4,1106,0,668,21202,-4,1,
        1,21201,-3,-1,2,21202,-2,2,3,21102,619,1,0,1105,1,577,22102,1,1,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,638,
        21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,660,22101,0,-1,1,21101,660,0,0,106,0,535,21202,-2,-1,-2,
        22201,-4,-2,-4,109,-5,2105,1,0,
    ];

    let mut zeroes = vec![0; 1000000];
    code.append(&mut zeroes);

    let mut cpu = CPU {
        debug: false,
        mem: code,
        ip: 0,
        bp: 0,
        input: vec![0],
        output: Vec::with_capacity(10000),
    };

    //cpu.input.push(1);

    let mut painted: HashSet<(i64, i64)> = HashSet::new();
    let mut white: HashSet<(i64, i64)> = HashSet::new();
    // 0 = up, 1 = right, 2 = down, 3 == left
    let mut direction = 0;
    let mut x = 0;
    let mut y = 0;

    while cpu.execute() {
        if cpu.output.len() > 1 {
            let paint_white = cpu.output.remove(0);
            let turn_right = cpu.output.remove(0);
            painted.insert((x, y));

            if paint_white == 1 {
                white.insert((x, y));
            } else {
                white.remove(&(x, y));
            }

            if turn_right == 1 {
                direction = (direction + 1) & 0x03;
            } else {
                direction = (direction - 1) & 0x03;
            }

            println!("[{:?},{:?}] pos [{},{}] dir {}", 
                paint_white, turn_right,
                x, y, direction);
            //println!("painted {:?}", painted);

            match direction {
                0 => {
                    y += 1;
                }
                1 => {
                    x -= 1;
                }
                2 => {
                    y -= 1;
                }
                3 => {
                    x += 1;
                }
                _ => {}
            }
            if white.contains(&(x, y)) {
                cpu.input.push(1);
            } else {
                cpu.input.push(0);
            }
        }
    }
    println!("{:?}", painted);
    println!("{:?}", painted.len());
}
