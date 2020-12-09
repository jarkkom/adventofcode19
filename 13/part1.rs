use std::collections::HashMap;

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
        1, 380, 379, 385, 1008, 2399, 648621, 381, 1005, 381, 12, 99, 109, 2400, 1102, 0, 1, 383,
        1102, 1, 0, 382, 20101, 0, 382, 1, 21002, 383, 1, 2, 21102, 1, 37, 0, 1105, 1, 578, 4, 382,
        4, 383, 204, 1, 1001, 382, 1, 382, 1007, 382, 44, 381, 1005, 381, 22, 1001, 383, 1, 383,
        1007, 383, 20, 381, 1005, 381, 18, 1006, 385, 69, 99, 104, -1, 104, 0, 4, 386, 3, 384,
        1007, 384, 0, 381, 1005, 381, 94, 107, 0, 384, 381, 1005, 381, 108, 1105, 1, 161, 107, 1,
        392, 381, 1006, 381, 161, 1102, -1, 1, 384, 1105, 1, 119, 1007, 392, 42, 381, 1006, 381,
        161, 1102, 1, 1, 384, 21002, 392, 1, 1, 21101, 18, 0, 2, 21101, 0, 0, 3, 21102, 1, 138, 0,
        1105, 1, 549, 1, 392, 384, 392, 20102, 1, 392, 1, 21101, 18, 0, 2, 21102, 1, 3, 3, 21102,
        161, 1, 0, 1105, 1, 549, 1101, 0, 0, 384, 20001, 388, 390, 1, 21001, 389, 0, 2, 21102, 180,
        1, 0, 1106, 0, 578, 1206, 1, 213, 1208, 1, 2, 381, 1006, 381, 205, 20001, 388, 390, 1,
        21001, 389, 0, 2, 21101, 0, 205, 0, 1106, 0, 393, 1002, 390, -1, 390, 1102, 1, 1, 384,
        20102, 1, 388, 1, 20001, 389, 391, 2, 21101, 228, 0, 0, 1106, 0, 578, 1206, 1, 261, 1208,
        1, 2, 381, 1006, 381, 253, 20102, 1, 388, 1, 20001, 389, 391, 2, 21101, 253, 0, 0, 1106, 0,
        393, 1002, 391, -1, 391, 1101, 1, 0, 384, 1005, 384, 161, 20001, 388, 390, 1, 20001, 389,
        391, 2, 21101, 0, 279, 0, 1106, 0, 578, 1206, 1, 316, 1208, 1, 2, 381, 1006, 381, 304,
        20001, 388, 390, 1, 20001, 389, 391, 2, 21101, 304, 0, 0, 1106, 0, 393, 1002, 390, -1, 390,
        1002, 391, -1, 391, 1101, 0, 1, 384, 1005, 384, 161, 21001, 388, 0, 1, 20101, 0, 389, 2,
        21101, 0, 0, 3, 21102, 1, 338, 0, 1105, 1, 549, 1, 388, 390, 388, 1, 389, 391, 389, 21002,
        388, 1, 1, 20102, 1, 389, 2, 21101, 4, 0, 3, 21102, 365, 1, 0, 1105, 1, 549, 1007, 389, 19,
        381, 1005, 381, 75, 104, -1, 104, 0, 104, 0, 99, 0, 1, 0, 0, 0, 0, 0, 0, 200, 20, 15, 1, 1,
        22, 109, 3, 22102, 1, -2, 1, 21202, -1, 1, 2, 21101, 0, 0, 3, 21102, 1, 414, 0, 1105, 1,
        549, 21202, -2, 1, 1, 22102, 1, -1, 2, 21101, 429, 0, 0, 1106, 0, 601, 1201, 1, 0, 435, 1,
        386, 0, 386, 104, -1, 104, 0, 4, 386, 1001, 387, -1, 387, 1005, 387, 451, 99, 109, -3,
        2106, 0, 0, 109, 8, 22202, -7, -6, -3, 22201, -3, -5, -3, 21202, -4, 64, -2, 2207, -3, -2,
        381, 1005, 381, 492, 21202, -2, -1, -1, 22201, -3, -1, -3, 2207, -3, -2, 381, 1006, 381,
        481, 21202, -4, 8, -2, 2207, -3, -2, 381, 1005, 381, 518, 21202, -2, -1, -1, 22201, -3, -1,
        -3, 2207, -3, -2, 381, 1006, 381, 507, 2207, -3, -4, 381, 1005, 381, 540, 21202, -4, -1,
        -1, 22201, -3, -1, -3, 2207, -3, -4, 381, 1006, 381, 529, 21202, -3, 1, -7, 109, -8, 2106,
        0, 0, 109, 4, 1202, -2, 44, 566, 201, -3, 566, 566, 101, 639, 566, 566, 1202, -1, 1, 0,
        204, -3, 204, -2, 204, -1, 109, -4, 2105, 1, 0, 109, 3, 1202, -1, 44, 594, 201, -2, 594,
        594, 101, 639, 594, 594, 20101, 0, 0, -2, 109, -3, 2105, 1, 0, 109, 3, 22102, 20, -2, 1,
        22201, 1, -1, 1, 21101, 443, 0, 2, 21102, 1, 397, 3, 21101, 0, 880, 4, 21101, 0, 630, 0,
        1105, 1, 456, 21201, 1, 1519, -2, 109, -3, 2106, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2,
        0, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 2, 0, 2,
        2, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 2, 0, 1, 1, 0, 2, 2, 2, 0, 2, 2, 0, 0, 0, 0, 2, 2, 0, 2,
        0, 0, 2, 2, 0, 2, 0, 2, 2, 0, 2, 0, 2, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 1, 0,
        2, 2, 2, 0, 2, 0, 2, 2, 0, 2, 0, 2, 0, 0, 2, 2, 2, 0, 2, 0, 2, 2, 2, 2, 2, 2, 0, 2, 0, 0,
        2, 0, 0, 2, 0, 2, 2, 0, 0, 0, 0, 1, 1, 0, 0, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 2,
        2, 0, 0, 0, 0, 2, 0, 2, 2, 0, 0, 0, 0, 2, 0, 0, 2, 2, 0, 0, 2, 0, 2, 0, 0, 1, 1, 0, 0, 0,
        2, 0, 0, 2, 0, 2, 0, 0, 2, 0, 2, 0, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 0, 0, 0, 2, 2, 2,
        2, 0, 0, 0, 0, 0, 2, 2, 0, 1, 1, 0, 0, 0, 0, 2, 0, 0, 2, 0, 2, 2, 2, 2, 2, 0, 0, 2, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 2, 0, 2, 0, 0, 2, 0, 0, 0, 0, 1, 1, 0, 0, 0, 2, 2,
        0, 0, 2, 0, 2, 0, 0, 2, 2, 0, 2, 0, 2, 0, 0, 2, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 0, 2, 0, 0,
        0, 0, 2, 2, 0, 0, 0, 1, 1, 0, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 2,
        2, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 2, 2, 0, 0,
        0, 0, 2, 0, 0, 1, 1, 0, 2, 0, 0, 2, 0, 2, 2, 0, 2, 0, 2, 0, 2, 2, 0, 0, 2, 0, 0, 2, 2, 0,
        2, 0, 2, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 2, 2, 0, 1, 1, 0, 0, 2, 0, 0, 0, 0, 2, 0,
        0, 2, 2, 2, 2, 0, 2, 0, 2, 2, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 2, 0, 2, 0, 2, 2,
        2, 2, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 54, 38,
        89, 72, 36, 81, 53, 60, 65, 11, 55, 31, 15, 8, 56, 5, 72, 34, 28, 21, 51, 33, 58, 48, 34,
        47, 76, 7, 68, 45, 59, 16, 94, 91, 89, 90, 54, 6, 7, 6, 40, 15, 52, 33, 53, 13, 68, 12, 91,
        43, 95, 36, 2, 1, 46, 6, 26, 2, 26, 40, 94, 9, 38, 33, 14, 6, 64, 32, 44, 61, 67, 23, 51,
        36, 98, 24, 28, 41, 48, 55, 52, 69, 20, 39, 37, 4, 9, 61, 90, 52, 24, 56, 6, 64, 22, 27,
        80, 39, 70, 29, 95, 21, 91, 98, 42, 19, 15, 31, 83, 55, 13, 38, 46, 22, 3, 42, 88, 79, 4,
        79, 23, 33, 37, 61, 56, 68, 84, 83, 88, 29, 82, 95, 15, 9, 74, 52, 64, 46, 35, 6, 81, 38,
        19, 82, 84, 76, 76, 78, 47, 8, 40, 13, 51, 91, 51, 37, 2, 76, 28, 20, 44, 15, 74, 82, 37,
        77, 46, 50, 53, 40, 26, 51, 10, 94, 24, 68, 59, 18, 50, 78, 7, 77, 74, 54, 90, 83, 51, 39,
        92, 78, 37, 65, 49, 41, 34, 32, 19, 75, 91, 48, 93, 37, 64, 26, 24, 26, 77, 58, 31, 76, 75,
        76, 16, 56, 21, 25, 55, 52, 92, 58, 87, 27, 52, 66, 61, 49, 68, 87, 55, 88, 57, 54, 5, 61,
        50, 70, 27, 5, 84, 43, 30, 33, 85, 69, 72, 95, 76, 36, 56, 60, 61, 51, 61, 64, 68, 81, 42,
        35, 91, 30, 82, 64, 1, 61, 76, 81, 7, 31, 98, 28, 35, 95, 8, 73, 29, 20, 75, 66, 87, 89, 6,
        35, 36, 98, 74, 64, 89, 95, 29, 96, 39, 70, 83, 55, 52, 82, 92, 96, 32, 61, 21, 87, 19, 3,
        93, 76, 97, 5, 67, 69, 43, 68, 72, 57, 56, 51, 77, 19, 22, 5, 84, 91, 88, 98, 91, 11, 52,
        54, 53, 41, 56, 18, 9, 44, 27, 9, 6, 81, 81, 37, 23, 47, 48, 7, 46, 73, 29, 81, 95, 50, 90,
        95, 81, 74, 53, 29, 25, 74, 35, 12, 30, 58, 13, 97, 87, 77, 38, 83, 6, 44, 62, 41, 64, 75,
        32, 93, 31, 66, 63, 97, 65, 58, 80, 49, 28, 62, 7, 36, 26, 72, 96, 5, 65, 81, 30, 60, 5,
        43, 3, 3, 14, 78, 51, 91, 97, 13, 48, 54, 6, 9, 81, 22, 31, 92, 33, 70, 81, 61, 61, 69, 95,
        93, 15, 36, 91, 29, 91, 27, 43, 43, 57, 6, 14, 80, 1, 43, 20, 63, 42, 51, 84, 28, 97, 50,
        50, 65, 70, 48, 79, 62, 97, 94, 93, 96, 42, 57, 40, 46, 50, 69, 47, 41, 64, 17, 2, 40, 28,
        41, 55, 94, 7, 18, 18, 1, 53, 34, 7, 71, 77, 47, 41, 14, 81, 76, 66, 63, 10, 48, 24, 69,
        75, 44, 90, 52, 96, 35, 1, 47, 45, 24, 85, 90, 40, 66, 45, 10, 89, 44, 84, 91, 44, 65, 59,
        93, 19, 57, 66, 79, 51, 22, 13, 67, 50, 6, 56, 40, 18, 36, 48, 71, 87, 65, 36, 37, 77, 37,
        19, 62, 97, 13, 44, 77, 80, 45, 97, 45, 86, 30, 21, 23, 76, 70, 94, 88, 61, 58, 59, 57, 66,
        75, 84, 86, 15, 58, 40, 54, 74, 26, 34, 19, 77, 96, 32, 94, 82, 13, 46, 96, 49, 77, 38, 73,
        92, 42, 98, 80, 10, 89, 49, 66, 10, 50, 97, 83, 89, 6, 76, 78, 9, 27, 60, 76, 49, 24, 45,
        39, 5, 42, 77, 61, 69, 37, 58, 4, 24, 20, 33, 10, 56, 46, 18, 11, 36, 11, 17, 3, 6, 21, 67,
        21, 2, 91, 52, 83, 95, 20, 80, 11, 98, 34, 52, 14, 39, 95, 11, 40, 11, 68, 54, 89, 3, 98,
        61, 49, 30, 71, 30, 2, 25, 10, 38, 93, 46, 64, 46, 40, 9, 1, 96, 21, 98, 88, 66, 83, 44,
        84, 27, 65, 6, 35, 63, 45, 96, 57, 66, 49, 56, 52, 21, 8, 95, 47, 94, 37, 14, 42, 6, 87,
        74, 62, 50, 97, 12, 39, 97, 93, 46, 36, 16, 83, 35, 72, 64, 37, 86, 61, 69, 82, 21, 97, 62,
        54, 68, 71, 60, 37, 67, 81, 10, 57, 45, 74, 71, 13, 82, 1, 85, 2, 13, 74, 14, 46, 52, 44,
        61, 49, 78, 13, 53, 75, 86, 18, 60, 4, 73, 82, 51, 76, 61, 58, 76, 44, 12, 40, 17, 17, 62,
        86, 71, 24, 28, 57, 87, 3, 8, 52, 51, 79, 39, 38, 41, 48, 62, 67, 63, 39, 50, 70, 87, 59,
        35, 57, 75, 24, 2, 89, 34, 77, 6, 20, 66, 40, 74, 54, 41, 34, 27, 68, 63, 46, 69, 23, 93,
        39, 46, 46, 60, 9, 63, 70, 34, 86, 75, 34, 43, 13, 71, 37, 19, 83, 86, 74, 41, 16, 88, 44,
        43, 85, 49, 4, 3, 98, 56, 77, 16, 41, 30, 18, 64, 2, 49, 49, 19, 35, 33, 93, 73, 83, 40,
        19, 72, 12, 83, 33, 31, 54, 58, 72, 90, 40, 79, 51, 64, 85, 44, 84, 67, 3, 15, 57, 648621,
    ];

    let mut zeroes = vec![0; 1000000];
    code.append(&mut zeroes);

    let mut cpu = CPU {
        debug: false,
        mem: code,
        ip: 0,
        bp: 0,
        input: vec![],
        output: Vec::with_capacity(10000),
    };

    let mut pixels: HashMap<(i64, i64), i64> = HashMap::new();

    while cpu.execute() {
        if cpu.output.len() > 2 {
            let x = cpu.output.remove(0);
            let y = cpu.output.remove(0);
            let c = cpu.output.remove(0);

            pixels.insert((x, y), c);
        }
    }

    let blocks = pixels.values().filter(|&c| *c == 2).count();
    println!("blocks {}", blocks);
}