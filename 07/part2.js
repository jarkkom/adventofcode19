'use strict';

function execute(code, ip, input, output) {
    if (ip == undefined) {
        return undefined;
    }

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
            return ip;
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
            return undefined;
        }
    }
}

let test_code_01 = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
let test_phases_01 = [9,8,7,6,5];

let test_code_02 = [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
let test_phases_02 = [9,7,8,5,6];

let code = [3,8,1001,8,10,8,105,1,0,0,21,46,67,88,101,126,207,288,369,450,99999,3,9,1001,9,5,9,1002,9,5,9,1001,9,5,9,102,3,9,9,101,2,9,9,4,9,99,3,9,102,4,9,9,101,5,9,9,102,5,9,9,101,3,9,9,4,9,99,3,9,1001,9,3,9,102,2,9,9,1001,9,5,9,102,4,9,9,4,9,99,3,9,102,3,9,9,1001,9,4,9,4,9,99,3,9,102,3,9,9,1001,9,3,9,1002,9,2,9,101,4,9,9,102,3,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,99];

let possible_phases = [9, 8, 7, 6, 5];

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

function get_thrust(program, arr) {
    let phases = arr.slice(0);
    let output = [0];
    let input = [];

    for (let i = 0 ; i < phases.length; i++) {
        input[i] = [ phases[i] ];
    }

    let ips = [0, 0, 0, 0, 0];
    let code = [program.slice(0), program.slice(0), program.slice(0), program.slice(0), program.slice(0)];
    let i = 0;
    let finished = 0;
    while (true) {
        input[i].push(output.shift());
        console.log(`${i} start at ${ips[i]} in ${input[i]} out ${output}`);
        ips[i] = execute(code[i], ips[i], input[i], output);
        console.log(`${i} finished at ${ips[i]} in ${input[i]} out ${output}`);
        if (ips[i] == undefined) {
            break;
        }
        i = (i + 1) % 5;
    }
    return input.shift();
}

// let t = get_thrust(test_code_02, test_phases_02);
// console.log(t);

let max_thrust = 0;

heaps_algo(5, possible_phases, (arr) => {
    console.log(arr);
    let res = get_thrust(code, arr);
    if (res > max_thrust) {
        console.log(res, arr);
        max_thrust = res;
    }
});

console.log(max_thrust);
