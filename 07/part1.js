'use strict';

function execute(code, input, output) {
    let ip = 0;
    while (true) {
        const instr = code[ip] % 100;
        const argmode1 = Math.trunc(code[ip] / 100) % 10;
        const argmode2 = Math.trunc(code[ip] / 1000) % 10;
        const destmode = Math.trunc(code[ip] / 10000) % 10;

        const arg1 = code[ip + 1];
        const arg2 = code[ip + 2];
        const dest = code[ip + 3];
        //console.log(`ip ${ip} opcode ${instr} arg1 ${arg1} ${argmode1} arg2 ${arg2} ${argmode2} dest ${dest} ${destmode}`);

        let v1 = argmode1 == 1 ? arg1 : code[arg1];
        let v2 = argmode2 == 1 ? arg2 : code[arg2];

        if (instr == 1) {
            code[dest] = v1 + v2;
            ip += 4;
            continue;
        }
        if (instr == 2) {
            code[dest] = v1 * v2;
            ip += 4;
            continue;
        }
        if (instr == 3) {
            code[arg1] = input.shift();
            ip += 2;
            continue;
        }
        if (instr == 4) {
            output.push(v1);
            ip += 2;
            continue;
        }
        if (instr == 5) {
            if (v1 != 0) {
                ip = v2;
            } else {
                ip += 3;
            }
            continue;
        }
        if (instr == 6) {
            if (v1 == 0) {
                ip = v2;
            } else {
                ip += 3;
            }
            continue;
        }
        if (instr == 7) {
            code[dest] = v1 < v2 ? 1 : 0;
            ip += 4;
            continue;
        }
        if (instr == 8) {
            code[dest] = v1 == v2 ? 1 : 0;
            ip += 4;
            continue;
        }
        if (instr == 99) {
            return;
        }
    }
}


let test_code_01 = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
let test_phases_01 = [4,3,2,1,0];

let test_code_02 = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
let test_phases_02 = [0,1,2,3,4];

let test_code_03 = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
let test_phases_03 = [1,0,4,3,2];

let code = [3,8,1001,8,10,8,105,1,0,0,21,46,67,88,101,126,207,288,369,450,99999,3,9,1001,9,5,9,1002,9,5,9,1001,9,5,9,102,3,9,9,101,2,9,9,4,9,99,3,9,102,4,9,9,101,5,9,9,102,5,9,9,101,3,9,9,4,9,99,3,9,1001,9,3,9,102,2,9,9,1001,9,5,9,102,4,9,9,4,9,99,3,9,102,3,9,9,1001,9,4,9,4,9,99,3,9,102,3,9,9,1001,9,3,9,1002,9,2,9,101,4,9,9,102,3,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,99];

let program = test_code_03;
let phases = test_phases_03;

let possible_phases = [0, 1, 2, 3, 4];

function heaps_algo(k, arr, cb) {
    if (k == 1) {
        cb(arr);
    } else {
        heaps_algo(k - 1, arr, cb);

        for (let i = 0; i < k - 1; i++) {
            if (k % 2 == 0) {
                let tmp = arr[i];
                arr[i] = arr[k - 1];
                arr[k - 1] = tmp;
            } else {
                let tmp = arr[0];
                arr[0] = arr[k - 1];
                arr[k - 1] = tmp;
            }
            heaps_algo(k - 1, arr, cb);
        }
    }
}

let max_thrust = 0;

heaps_algo(5, possible_phases, (arr) => {
    let phases = arr.slice(0);
    let output = [0];
    let input = [];
    for (let i = 0; i < 5; i++) {
        input = [phases.shift(), output.shift()];
        execute(code.slice(0), input, output);
    }
    let res = output.shift();
    if (res > max_thrust) {
        console.log(res, arr);
        max_thrust = res;
    }
});

console.log(max_thrust);
