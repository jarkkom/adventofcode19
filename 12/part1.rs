#[derive(Debug, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Body {
    p: Vec3,
    v: Vec3,
}

fn calc_velocity_delta(a: &Vec3, b: &Vec3) -> Vec3 {
    let mut v = Vec3{
        x: 0,
        y: 0,
        z: 0,
    };
    if a.x < b.x { v.x = 1};
    if a.x > b.x { v.x = -1};

    if a.y < b.y { v.y = 1};
    if a.y > b.y { v.y = -1};

    if a.z < b.z { v.z = 1};
    if a.z > b.z { v.z = -1};

    v
}

fn calc_velocities(bodies: &Vec<Body>) -> Vec<Body> {
    let mut r = Vec::new();
    for (i1, b1) in bodies.iter().enumerate() {
        let mut nb = Body{
            p: Vec3{
                x: b1.p.x,
                y: b1.p.y,
                z: b1.p.z,
            },
            v: Vec3{
                x: b1.v.x,
                y: b1.v.y,
                z: b1.v.z,
            },
        };
        for (i2, b2) in bodies.iter().enumerate() {
            //println!("{:?}-{:?}", i1, i2);
            if i1 == i2 {
                continue;
            }

            let dx = calc_velocity_delta(&b1.p, &b2.p);
            println!("{:?}-{:?} {:?}", i1, i2, dx);

            nb.v.x += dx.x;
            nb.v.y += dx.y;
            nb.v.z += dx.z;
        }
        r.push(nb);
    }
    r
}

fn apply_velocities(bodies: &Vec<Body>) -> Vec<Body> {
    let mut r = Vec::new();
    for b in bodies.iter() {
        r.push(Body{
            p: apply_velocity(&b.p, &b.v),
            v: Vec3{
                x: b.v.x,
                y: b.v.y,
                z: b.v.z,
            },
        });
    }
    r
}

fn apply_velocity(pos: &Vec3, vel: &Vec3) -> Vec3 {
    Vec3{
        x: pos.x + vel.x,
        y: pos.y + vel.y,
        z: pos.z + vel.z,
    }
}

fn calc_total_energy(b: &Body) -> i64 {
    (b.p.x.abs() + b.p.y.abs() + b.p.z.abs()) * (b.v.x.abs() + b.v.y.abs() + b.v.z.abs())
}

fn main() {
    let mut bodies = vec![
        Body{
            p: Vec3{
                x: -10,
                y: -10,
                z: -13,
            },
            v: Vec3{x: 0, y: 0, z: 0 },
        },
        Body{
            p: Vec3{
                x: 5,
                y: 5,
                z: -9,
            },
            v: Vec3{x: 0, y: 0, z: 0 },
        },
        Body{
            p: Vec3{
                x: 3,
                y: 8,
                z: -16,
            },
            v: Vec3{x: 0, y: 0, z: 0 },
        },
        Body{
            p: Vec3{
                x: 1,
                y: 3,
                z: -3,
            },
            v: Vec3{x: 0, y: 0, z: 0 },
        },
    ];

    for _i in 0..1000 {
        bodies = calc_velocities(&bodies);
        bodies = apply_velocities(&bodies);
    }
    println!("{}", bodies.iter().fold(0, |sum, b|
        sum + calc_total_energy(&b)
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_vel_delta() {
        let ganymede = Vec3{
            x: 3,
            y: 3,
            z: 5,
        };
        let callisto = Vec3{
            x: 5,
            y: 3,
            z: 3,
        };
        let dx = calc_velocity_delta(&ganymede, &callisto);
        println!("dx {:?}", dx);
        assert_eq!(dx.x, 1);
        assert_eq!(dx.y, 0);
        assert_eq!(dx.z, -1);

        let dx2 = calc_velocity_delta(&callisto, &ganymede);
        println!("dx2 {:?}", dx2);
        assert_eq!(dx2.x, -1);
        assert_eq!(dx2.y, 0);
        assert_eq!(dx2.z, 1);

    }

    #[test]
    fn test_read_map() {
        let mut bodies = vec![
            Body{
                p: Vec3{
                    x: -1,
                    y: 0,
                    z: 2,
                },
                v: Vec3{x: 0, y: 0, z: 0 },
            },
            Body{
                p: Vec3{
                    x: 2,
                    y: -10,
                    z: -7,
                },
                v: Vec3{x: 0, y: 0, z: 0 },
            },
            Body{
                p: Vec3{
                    x: 4,
                    y: -8,
                    z: 8,
                },
                v: Vec3{x: 0, y: 0, z: 0 },
            },
            Body{
                p: Vec3{
                    x: 3,
                    y: 5,
                    z: -1,
                },
                v: Vec3{x: 0, y: 0, z: 0 },
            },
        ];

        bodies = calc_velocities(&bodies);
        bodies = apply_velocities(&bodies);

        assert_eq!(bodies[0].p, Vec3{x: 2, y: -1, z: 1});
        assert_eq!(bodies[0].v, Vec3{x: 3, y: -1, z: -1});

        bodies = calc_velocities(&bodies);
        bodies = apply_velocities(&bodies);

        assert_eq!(bodies[1].p, Vec3{x: 1, y: -2, z: 2});
        assert_eq!(bodies[1].v, Vec3{x: -2, y: 5, z: 6});

        bodies = calc_velocities(&bodies);
        bodies = apply_velocities(&bodies);

        assert_eq!(bodies[2].p, Vec3{x: 2, y: 1, z: -5});
        assert_eq!(bodies[2].v, Vec3{x: 1, y: 5, z: -4});

        bodies = calc_velocities(&bodies);
        bodies = apply_velocities(&bodies);

        assert_eq!(bodies[3].p, Vec3{x: 2, y: -9, z: 1});
        assert_eq!(bodies[3].v, Vec3{x: 1, y: -1, z: -1});
    }

    #[test]
    fn test_total_energy() {
        let mut bodies = vec![
            Body{
                p: Vec3{
                    x: 2,
                    y: 1,
                    z: -3,
                },
                v: Vec3{x: -3, y: -2, z: 1 },
            },
            Body{
                p: Vec3{
                    x: 1,
                    y: -8,
                    z: 0,
                },
                v: Vec3{x:-1, y: 1, z: 3 },
            },
            Body{
                p: Vec3{
                    x: 3,
                    y: -6,
                    z: 1,
                },
                v: Vec3{x: 3, y: 2, z: -3 },
            },
            Body{
                p: Vec3{
                    x: 2,
                    y: 0,
                    z: 4,
                },
                v: Vec3{x: 1, y: -1, z: -1 },
            },
        ];

        assert_eq!(calc_total_energy(&bodies[0]), 36);
        assert_eq!(calc_total_energy(&bodies[1]), 45);
        assert_eq!(calc_total_energy(&bodies[2]), 80);
        assert_eq!(calc_total_energy(&bodies[3]), 18);
    }
}
