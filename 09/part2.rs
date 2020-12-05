struct CPU {
    ip: usize,
    bp: i64,
    mem: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
}

impl CPU {
    fn read_value(&mut self, arg: i64, mode: i64) -> i64 {
        return match mode {
            0 => self.mem[arg as usize],
            1 => arg,
            2 => self.mem[(self.bp + arg) as usize],
            _ => panic!("invalid read mode"),
        };
    }

    fn write_value(&mut self, arg: i64, mode: i64) -> i64 {
        return match mode {
            0 => arg,
            1 => panic!("invalid write mode"),
            2 => self.bp + arg,
            _ => panic!("invalid write mode"),
        };
    }

    pub fn execute(&mut self) -> bool {
        let instr = self.mem[self.ip];

        let opcode = instr % 100;
        let arg_mode_1 = (instr / 100) % 10;
        let arg_mode_2 = (instr / 1000) % 10;
        let dest_mode = (instr / 10000) % 10;

        print!(
            "ip {} bp {} instr {} opcode {} m1 {} m2 {} dm {} ",
            self.ip, self.bp, instr, opcode, arg_mode_1, arg_mode_2, dest_mode
        );

        match opcode {
            1 => {
                let arg_1 = self.mem[self.ip + 1];
                let arg_2 = self.mem[self.ip + 2];
                let dest = self.mem[self.ip + 3];
                let value_1 = self.read_value(arg_1, arg_mode_1);
                let value_2 = self.read_value(arg_2, arg_mode_2);
                let value_dest = self.write_value(dest, dest_mode) as usize;

                println!(
                    "add {} {}, {} {}, {} {}",
                    arg_1, value_1, arg_2, value_2, dest, value_dest
                );

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

                println!(
                    "mul {} {}, {} {}, {} {}",
                    arg_1, value_1, arg_2, value_2, dest, value_dest
                );

                self.mem[value_dest as usize] = value_1 * value_2;
                self.ip += 4;
            }
            3 => {
                let arg_1 = self.mem[self.ip + 1];
                let value_1 = self.write_value(arg_1, arg_mode_1) as usize;
                let input = self.input.remove(0);

                println!("in {} {} {}", arg_1, value_1, input);

                self.mem[value_1 as usize] = input;
                self.ip += 2;
            }
            4 => {
                let arg_1 = self.mem[self.ip + 1];
                let value_1 = self.read_value(arg_1, arg_mode_1);

                println!("out {} {}", arg_1, value_1);

                self.output.push(value_1);
                self.ip += 2;
            }
            5 => {
                let arg_1 = self.mem[self.ip + 1];
                let arg_2 = self.mem[self.ip + 2];
                let value_1 = self.read_value(arg_1, arg_mode_1);
                let value_2 = self.read_value(arg_2, arg_mode_2);

                println!("jnz {} {}, {} {},", arg_1, value_1, arg_2, value_2);

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

                println!("jz {} {}, {} {}", arg_1, value_1, arg_2, value_2);

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

                println!(
                    "setle {} {}, {} {}, {} {}",
                    arg_1, value_1, arg_2, value_2, dest, value_dest
                );

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

                println!(
                    "seteq {} {}, {} {}, {} {}",
                    arg_1, value_1, arg_2, value_2, dest, value_dest
                );

                self.mem[value_dest] = if value_1 == value_2 { 1 } else { 0 };
                self.ip += 4;
            }
            9 => {
                let arg_1 = self.mem[self.ip + 1];
                let value_1 = self.read_value(arg_1, arg_mode_1);

                println!("addbp {} {}", arg_1, value_1);

                self.bp = self.bp + value_1;
                self.ip += 2;
            }
            99 => {
                println!("");
                return false;
            }
            _ => {
                println!("invalid opcode {}", opcode);
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let mut code01: Vec<i64> = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut code02: Vec<i64> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut code03: Vec<i64> = vec![104, 1125899906842624, 99];

    let mut code = vec![
        1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 0, 3, 1000, 109,
        988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000,
        2, 63, 1005, 63, 902, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99,
        4, 17, 104, 0, 99, 0, 0, 1102, 32, 1, 1019, 1101, 0, 500, 1023, 1101, 0, 636, 1025, 1102,
        36, 1, 1010, 1101, 0, 29, 1013, 1102, 864, 1, 1029, 1102, 21, 1, 1000, 1102, 1, 507, 1022,
        1102, 1, 28, 1011, 1102, 38, 1, 1008, 1101, 0, 35, 1004, 1101, 25, 0, 1018, 1102, 24, 1,
        1005, 1102, 30, 1, 1009, 1102, 1, 869, 1028, 1101, 0, 37, 1007, 1102, 1, 23, 1017, 1102, 1,
        20, 1015, 1102, 1, 22, 1003, 1101, 0, 39, 1001, 1102, 1, 31, 1012, 1101, 701, 0, 1026,
        1101, 0, 641, 1024, 1101, 0, 34, 1016, 1102, 1, 0, 1020, 1102, 698, 1, 1027, 1102, 33, 1,
        1002, 1102, 26, 1, 1006, 1101, 0, 1, 1021, 1101, 0, 27, 1014, 109, 12, 21101, 40, 0, 0,
        1008, 1012, 40, 63, 1005, 63, 203, 4, 187, 1105, 1, 207, 1001, 64, 1, 64, 1002, 64, 2, 64,
        109, -11, 1207, 7, 37, 63, 1005, 63, 223, 1105, 1, 229, 4, 213, 1001, 64, 1, 64, 1002, 64,
        2, 64, 109, 14, 1206, 5, 247, 4, 235, 1001, 64, 1, 64, 1105, 1, 247, 1002, 64, 2, 64, 109,
        -2, 1207, -4, 31, 63, 1005, 63, 269, 4, 253, 1001, 64, 1, 64, 1105, 1, 269, 1002, 64, 2,
        64, 109, -6, 1208, -5, 35, 63, 1005, 63, 289, 1001, 64, 1, 64, 1106, 0, 291, 4, 275, 1002,
        64, 2, 64, 109, 9, 21108, 41, 39, -1, 1005, 1015, 311, 1001, 64, 1, 64, 1105, 1, 313, 4,
        297, 1002, 64, 2, 64, 109, -5, 2101, 0, -9, 63, 1008, 63, 33, 63, 1005, 63, 339, 4, 319,
        1001, 64, 1, 64, 1106, 0, 339, 1002, 64, 2, 64, 1205, 10, 351, 4, 343, 1106, 0, 355, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, -18, 2108, 35, 9, 63, 1005, 63, 375, 1001, 64, 1, 64,
        1105, 1, 377, 4, 361, 1002, 64, 2, 64, 109, 18, 1205, 9, 389, 1105, 1, 395, 4, 383, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, 7, 21107, 42, 41, -8, 1005, 1010, 415, 1001, 64, 1, 64,
        1106, 0, 417, 4, 401, 1002, 64, 2, 64, 109, -12, 2102, 1, 0, 63, 1008, 63, 29, 63, 1005,
        63, 437, 1106, 0, 443, 4, 423, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 3, 1208, 0, 30, 63,
        1005, 63, 461, 4, 449, 1105, 1, 465, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 5, 1202, -5, 1,
        63, 1008, 63, 31, 63, 1005, 63, 489, 1001, 64, 1, 64, 1106, 0, 491, 4, 471, 1002, 64, 2,
        64, 109, 15, 2105, 1, -6, 1001, 64, 1, 64, 1106, 0, 509, 4, 497, 1002, 64, 2, 64, 109, -10,
        1206, 2, 525, 1001, 64, 1, 64, 1106, 0, 527, 4, 515, 1002, 64, 2, 64, 109, -18, 1202, 0, 1,
        63, 1008, 63, 39, 63, 1005, 63, 553, 4, 533, 1001, 64, 1, 64, 1106, 0, 553, 1002, 64, 2,
        64, 109, 1, 2107, 21, 1, 63, 1005, 63, 571, 4, 559, 1105, 1, 575, 1001, 64, 1, 64, 1002,
        64, 2, 64, 109, 7, 2102, 1, -8, 63, 1008, 63, 39, 63, 1005, 63, 601, 4, 581, 1001, 64, 1,
        64, 1105, 1, 601, 1002, 64, 2, 64, 109, 2, 1201, -7, 0, 63, 1008, 63, 35, 63, 1005, 63,
        623, 4, 607, 1106, 0, 627, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 20, 2105, 1, -7, 4, 633,
        1106, 0, 645, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -16, 21107, 43, 44, -4, 1005, 1011,
        663, 4, 651, 1105, 1, 667, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -11, 2107, 36, 0, 63,
        1005, 63, 687, 1001, 64, 1, 64, 1106, 0, 689, 4, 673, 1002, 64, 2, 64, 109, 19, 2106, 0, 4,
        1106, 0, 707, 4, 695, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -14, 21108, 44, 44, 6, 1005,
        1015, 725, 4, 713, 1105, 1, 729, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 1, 1201, -6, 0, 63,
        1008, 63, 36, 63, 1005, 63, 749, 1106, 0, 755, 4, 735, 1001, 64, 1, 64, 1002, 64, 2, 64,
        109, -1, 21101, 45, 0, 10, 1008, 1019, 42, 63, 1005, 63, 775, 1105, 1, 781, 4, 761, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, 16, 21102, 46, 1, -7, 1008, 1018, 44, 63, 1005, 63, 801,
        1105, 1, 807, 4, 787, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -3, 21102, 47, 1, -4, 1008,
        1018, 47, 63, 1005, 63, 833, 4, 813, 1001, 64, 1, 64, 1105, 1, 833, 1002, 64, 2, 64, 109,
        -14, 2108, 38, 0, 63, 1005, 63, 851, 4, 839, 1105, 1, 855, 1001, 64, 1, 64, 1002, 64, 2,
        64, 109, 17, 2106, 0, 3, 4, 861, 1106, 0, 873, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -31,
        2101, 0, 10, 63, 1008, 63, 36, 63, 1005, 63, 897, 1001, 64, 1, 64, 1106, 0, 899, 4, 879, 4,
        64, 99, 21101, 0, 27, 1, 21101, 0, 913, 0, 1106, 0, 920, 21201, 1, 53612, 1, 204, 1, 99,
        109, 3, 1207, -2, 3, 63, 1005, 63, 962, 21201, -2, -1, 1, 21102, 940, 1, 0, 1106, 0, 920,
        21202, 1, 1, -1, 21201, -2, -3, 1, 21101, 955, 0, 0, 1106, 0, 920, 22201, 1, -1, -2, 1105,
        1, 966, 21201, -2, 0, -2, 109, -3, 2106, 0, 0,
    ];

    for _ in 1..1000000 {
        code.push(0);
    }

    let mut cpu = CPU {
        mem: code,
        ip: 0,
        bp: 0,
        input: vec![2],
        output: Vec::with_capacity(10000),
    };

    //cpu.input.push(1);

    while cpu.execute() {}

    println!("{:?}", cpu.output);
}
