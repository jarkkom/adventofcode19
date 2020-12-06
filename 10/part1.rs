use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq)]
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

fn build_visibility_set(base: &Asteroid, roids: &Vec<Asteroid>) -> HashMap<i64, i64> {
    roids
        .iter()
        .filter(|&a| base != a)
        .fold(HashMap::new(), |mut map, a| {
            let angle = (-((a.x - base.x) as f64).atan2((a.y - base.y) as f64)
                // (2.0 * std::f64::consts::PI)
                * 65536.0) as i64;
            //println!("{:?} -> {:?} {:?}", base, a, angle);
            map.insert(angle, map.get(&angle).unwrap_or(&0) + 1);
            map
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let asteroids = read_input(input_file.unwrap()).unwrap();

    println!("asteroids = {:?}", asteroids);

    let max = asteroids
        .iter()
        .max_by_key(|a| build_visibility_set(a, &asteroids).len())
        .unwrap();

    println!("max = {:?}", max);

    let visible_max = asteroids
        .iter()
        .find(|&a| a == max)
        .map(|a| build_visibility_set(a, &asteroids))
        .unwrap()
        .len();

    println!("visible_max = {:?}", visible_max);
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

    #[test]
    fn test_build_visibility() {
        let test1 = ".#..#
.....
#####
....#
...##";
        let actual = read_input(test1.as_bytes()).unwrap();

        for a in &actual {
            let vis = build_visibility_set(a, &actual);
            println!("{:?} {:?}", a, vis.len());
        }

        let max = actual
            .iter()
            .max_by_key(|a| build_visibility_set(a, &actual).len())
            .unwrap();
        assert_eq!(max.x, 3);
        assert_eq!(max.y, 4);
    }
}
