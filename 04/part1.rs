fn is_valid(n: i32) -> bool {
    let a = (n / 100000) % 10;
    let b = (n / 10000) % 10;
    let c = (n / 1000) % 10;
    let d = (n / 100) % 10;
    let e = (n / 10) % 10;
    let f = n % 10;

    if a > b || b > c || c > d || d > e || e > f {
        return false;
    }

    if a != b && b != c && c != d && d != e && e != f {
        return false;
    }

    return true;
}

fn main() {
    // println!("111111 {}", is_valid(111111));
    // println!("223450 {}", is_valid(223450));
    // println!("123789 {}", is_valid(123789));

    let mut c = 0;
    for n in 278384..824795 {
        if is_valid(n) {
            c += 1;
        }
    }
    println!("{}", c);
}
