use std::sync::Mutex;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Vec3I32 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Moon {
    position: Vec3I32,
    velocity: Vec3I32,
}

static STEPS: Mutex<i32> = Mutex::new(1000);

pub fn calculate1(input: &str) -> String {
    let mut moons: Vec<Moon> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut z: i32 = 0;
            line[1..line.len() - 1].split(", ").for_each(|sub| {
                let (var, value) = sub.split_at(2);
                let value: i32 = value.parse().unwrap();
                match var {
                    "x=" => { x = value }
                    "y=" => { y = value }
                    "z=" => { z = value }
                    x => { panic!("unknown variable {}", x) }
                }
            });
            Moon {
                position: Vec3I32 { x, y, z },
                velocity: Vec3I32 { x: 0, y: 0, z: 0 },
            }
        }).collect();

    let steps = *STEPS.lock().unwrap();

    for i in 0..steps {
        println!("after {} steps:", i);
        moons.iter().for_each(|it| {
            println!("{:?}", it);
        });

        moons = moons.iter()
            .map(|it| {
                let mut dx = 0;
                let mut dy = 0;
                let mut dz = 0;
                moons.iter().for_each(|another| {
                    if it.position.x < another.position.x {
                        dx += 1;
                    } else if it.position.x > another.position.x {
                        dx -= 1;
                    }
                    if it.position.y < another.position.y {
                        dy += 1;
                    } else if it.position.y > another.position.y {
                        dy -= 1;
                    }
                    if it.position.z < another.position.z {
                        dz += 1;
                    } else if it.position.z > another.position.z {
                        dz -= 1;
                    }
                });
                let velocity = Vec3I32 { x: it.velocity.x + dx, y: it.velocity.y + dy, z: it.velocity.z + dz };
                let position = Vec3I32 { x: it.position.x + velocity.x, y: it.position.y + velocity.y, z: it.position.z + velocity.z };
                Moon { position: position, velocity: velocity }
            })
            .collect();
    }

    let sum: i32 = moons.iter()
        .map(|it| {
            let pot = it.position.x.abs() + it.position.y.abs() + it.position.z.abs();
            let kit = it.velocity.x.abs() + it.velocity.y.abs() + it.velocity.z.abs();
            pot * kit
        }).sum();

    return sum.to_string();
}


pub fn calculate2(input: &str) -> String {
    let mut moons: Vec<Moon> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut z: i32 = 0;
            line[1..line.len() - 1].split(", ").for_each(|sub| {
                let (var, value) = sub.split_at(2);
                let value: i32 = value.parse().unwrap();
                match var {
                    "x=" => { x = value }
                    "y=" => { y = value }
                    "z=" => { z = value }
                    x => { panic!("unknown variable {}", x) }
                }
            });
            Moon {
                position: Vec3I32 { x, y, z },
                velocity: Vec3I32 { x: 0, y: 0, z: 0 },
            }
        }).collect();

    let starting_values = moons.clone();
    let mut counter = 0;
    let mut loop_x = 0;
    let mut loop_y = 0;
    let mut loop_z = 0;

    loop {
        // println!("after {} steps:", i);
        // moons.iter().for_each(|it| {
        //     println!("{:?}", it);
        // });

        moons = moons.iter()
            .map(|it| {
                let mut dx = 0;
                let mut dy = 0;
                let mut dz = 0;
                moons.iter().for_each(|another| {
                    if it.position.x < another.position.x {
                        dx += 1;
                    } else if it.position.x > another.position.x {
                        dx -= 1;
                    }
                    if it.position.y < another.position.y {
                        dy += 1;
                    } else if it.position.y > another.position.y {
                        dy -= 1;
                    }
                    if it.position.z < another.position.z {
                        dz += 1;
                    } else if it.position.z > another.position.z {
                        dz -= 1;
                    }
                });
                let velocity = Vec3I32 { x: it.velocity.x + dx, y: it.velocity.y + dy, z: it.velocity.z + dz };
                let position = Vec3I32 { x: it.position.x + velocity.x, y: it.position.y + velocity.y, z: it.position.z + velocity.z };
                Moon { position: position, velocity: velocity }
            })
            .collect();

        counter += 1;

        if loop_x == 0 && moons.iter().enumerate().all(|(i, it)| starting_values[i].position.x == it.position.x && starting_values[i].velocity.x == it.velocity.x) {
            loop_x = counter;
        }
        if loop_y == 0 && moons.iter().enumerate().all(|(i, it)| starting_values[i].position.y == it.position.y && starting_values[i].velocity.y == it.velocity.y) {
            loop_y = counter;
        }
        if loop_z == 0 && moons.iter().enumerate().all(|(i, it)| starting_values[i].position.z == it.position.z && starting_values[i].velocity.z == it.velocity.z) {
            loop_z = counter;
        }

        if loop_x != 0 && loop_y != 0 && loop_z != 0 {
            break;
        }
    }


    return lcm3(loop_x, loop_y, loop_z).to_string();
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(a, lcm(b, c))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        {
            let mut steps = STEPS.lock().unwrap();
            *steps = 10;
        }

        let result = calculate1("
                <x=-1, y=0, z=2>
                <x=2, y=-10, z=-7>
                <x=4, y=-8, z=8>
                <x=3, y=5, z=-1>
            "
        );
        assert_eq!(result, "179")
    }

    #[test]
    fn test_1_2() {
        {
            let mut steps = STEPS.lock().unwrap();
            *steps = 100;
        }

        let result = calculate1("
                <x=-8, y=-10, z=0>
                <x=5, y=5, z=10>
                <x=2, y=-7, z=3>
                <x=9, y=-8, z=-3>
            "
        );
        assert_eq!(result, "1940")
    }

    #[test]
    fn test_2_1() {
        let result = calculate2("
                <x=-1, y=0, z=2>
                <x=2, y=-10, z=-7>
                <x=4, y=-8, z=8>
                <x=3, y=5, z=-1>
            "
        );
        assert_eq!(result, "2772")
    }

    #[test]
    fn test_2_2() {
        let result = calculate2("
                <x=-8, y=-10, z=0>
                <x=5, y=5, z=10>
                <x=2, y=-7, z=3>
                <x=9, y=-8, z=-3>
            "
        );
        assert_eq!(result, "4686774924")
    }
}