use crate::AoCResult;
use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Moon {
            pos: Vec3 { x, y, z },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        }
    }

    fn apply_gravity(&mut self, other: &Moon) {
        self.vel.x += (other.pos.x - self.pos.x).signum();
        self.vel.y += (other.pos.y - self.pos.y).signum();
        self.vel.z += (other.pos.z - self.pos.z).signum();
    }

    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn energy(&self) -> i32 {
        let potential = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        let kinetic = self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();
        potential * kinetic
    }
}

fn simulate(moons: &mut [Moon]) {
    // Apply gravity
    let positions: Vec<Vec3> = moons.iter().map(|m| m.pos).collect();

    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            moons[i].vel.x += (positions[j].x - positions[i].x).signum();
            moons[i].vel.y += (positions[j].y - positions[i].y).signum();
            moons[i].vel.z += (positions[j].z - positions[i].z).signum();
        }
    }

    // Apply velocity
    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}

fn total_energy(moons: &[Moon]) -> i32 {
    moons.iter().map(|m| m.energy()).sum()
}

fn parse_input_simple(path: &str) -> Vec<Moon> {
    let text = fs::read_to_string(path).expect("Failed to read input file");

    text.lines()
        .map(|line| {
            let trimmed = &line[1..line.len() - 1]; // remove < and >
            let coords: Vec<i32> = trimmed
                .split(',')
                .map(|s| {
                    let (_, val) = s.trim().split_once('=').unwrap();
                    val.parse::<i32>().unwrap()
                })
                .collect();
            Moon::new(coords[0], coords[1], coords[2])
        })
        .collect()
}

// fixme: wrong result
pub fn solve_day12a() -> AoCResult<usize> {
    let mut moons = parse_input_simple("data/input_day12a.txt");
    for step in 0..1000 {
        simulate(&mut moons);
    }

    Ok(total_energy(&moons).try_into()?)
}
