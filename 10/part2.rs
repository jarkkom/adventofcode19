use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Asteroid {
    pub x: i64,
    pub y: i64,
}

fn open_input(filename: &str) -> io::Result<File> {
    let path = Path::new(filename);
    File::open(path)
}

fn read_input(reader: impl Read) -> Result<Vec<Asteroid>, String> {
    let reader = BufReader::new(reader);

    let mut asteroids: Vec<Asteroid> = Vec::new();

    for (y, line_iter) in reader.lines().enumerate() {
        match line_iter {
            Ok(line) => {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' {
                        asteroids.push(Asteroid {
                            x: x as i64,
                            y: y as i64,
                        });
                    }
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(asteroids)
}

fn build_visibility_set(base: &Asteroid, roids: &[Asteroid]) -> HashMap<i64, Vec<Asteroid>> {
    roids
        .iter()
        .filter(|&a| base != a)
        .fold(HashMap::new(), |mut map, a| {
            let angle = (-(((a.x - base.x) as f64).atan2((a.y - base.y) as f64)
                + std::f64::consts::PI)
                / (2.0 * std::f64::consts::PI)
                * 65536.0) as i64;
            //println!("{:?} -> {:?} {:?}", base, a, angle);
            if !map.contains_key(&angle) {
                let v = vec![];
                map.insert(angle, v);
            }
            map.get_mut(&angle).unwrap().push(*a);
            map
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let asteroids = read_input(input_file.unwrap()).unwrap();

    let base = asteroids
        .iter()
        //.find(|&a| a.x == 8 && a.y == 3)
        .find(|&a| a.x == 26 && a.y == 28)
        .unwrap();

    println!("base {:?}", base);

    let mut vis = build_visibility_set(base, &asteroids);

    let mut removed = 1;
    while removed < 201 {
        let mut angles: Vec<i64> = vis.keys().copied().collect();
        angles.sort_unstable();

        for angle in angles {
            let roids = vis.get_mut(&angle).unwrap();
            roids.sort_unstable_by_key(|&a| {
                (a.x - base.x) * (a.x - base.x) + (a.y - base.y) * (a.y - base.y)
            });
            //println!("remove {} angle {:?} {:?}", removed, angle, roids[0]);
            let r = roids.remove(0);
            if removed == 200 {
                println!("{}", r.x * 100 + r.y);
            }
            removed += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_map() {
        let test1 = ".#..#
.....
#####
....#
...##";
        let actual = read_input(test1.as_bytes()).unwrap();

        assert_eq!(actual[0], Asteroid { x: 1, y: 0 });
        assert_eq!(actual[1], Asteroid { x: 4, y: 0 });
        assert_eq!(actual[9], Asteroid { x: 4, y: 4 });
    }
}
