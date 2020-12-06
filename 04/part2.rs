fn is_valid(n: i32) -> bool {
    let mut v = vec![0; 6];

    v[0] = (n / 100000) % 10;
    v[1] = (n / 10000) % 10;
    v[2] = (n / 1000) % 10;
    v[3] = (n / 100) % 10;
    v[4] = (n / 10) % 10;
    v[5] = n % 10;

    for p in 0..5 {
        if v[p] > v[p + 1] {
            return false;
        }
    }

    let mut doubles = 0;
    let mut triple_or_more = 0;
    let mut i = 0;
    while i < v.len() {
        let mut repeats = 0;
        for j in i + 1..v.len() {
            if v[i] == v[j] {
                repeats += 1;
            } else {
                break;
            }
        }

        i += repeats + 1;
        if repeats == 1 {
            doubles += 1;
        } else if repeats > 1 {
            triple_or_more += 1;
        }
    }

    if doubles == 0 {
        return false;
    }

    println!("{} {} {}", n, doubles, triple_or_more);

    true
}

fn main() {
    println!("112233 {}", is_valid(112233));
    println!("123444 {}", is_valid(123444));
    println!("111122 {}", is_valid(111122));

    let mut c = 0;
    for n in 278384..824795 {
        if is_valid(n) {
            c += 1;
        }
    }
    println!("{}", c);
}
