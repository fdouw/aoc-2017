use std::{cmp::Ordering, fmt};

use itertools::Itertools;
use regex::Regex;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // Removed some spaces to make the pattern matching work
    #[allow(unused_variables)]
    let test_input = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\np=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\np=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\np=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

    let particle_pattern: Regex =
        Regex::new(r"p=<([-0-9,]+)>, v=<([-0-9,]+)>, a=<([-0-9,]+)>").unwrap();

    // Part 1
    // The particle with the smallest acceleration will eventually stay closest to the origin.
    // This does NOT account for particles having the same (absolute) acceleration!
    let mut min_idx = 0;
    let mut min_a = i64::max_value();
    let mut cloud: Vec<_> = input
        .trim()
        .lines()
        .map(|x| Particle::new(x, &particle_pattern))
        .collect();

    for i in 0..cloud.len() {
        let a = cloud[i].accel();
        if a < min_a {
            min_idx = i;
            min_a = a;
        }
    }

    // Part 2
    'evolve: for _step in 0..1_000 {
        // Move the particles
        cloud.iter_mut().for_each(|particle| particle._step());

        // Filter colliding particles
        cloud = cloud.into_iter().sorted().collect();
        let cloud_size = cloud.len();
        let mut next_cloud = Vec::new();
        for (i, p) in cloud.iter().enumerate() {
            if !((i > 0 && p.collides(&cloud[i - 1]))
                || (i + 1 < cloud_size && p.collides(&cloud[i + 1])))
            {
                next_cloud.push(*p);
            }
        }
        cloud = next_cloud;

        // if _step > 950 {
        //     println!("step: {_step}");
        // }

        // if the ordering of particles by velocity and by position is the same, then there is no way for particles to collide.
        // However, particles are accelerating, so we must compare acceleration and position instead.
        // Ties wrt acceleration can be resolved by comparing velocities.
        // We can do this on a per-axis basis.
        // if cloud_size == cloud.len() {
        //     // Don't check everytime (it's expensive)
        //     let mut pos_sorted = cloud.clone();
        //     for axis in [Axis::X, Axis::Y, Axis::Z].iter() {
        //         pos_sorted.sort_by(|a, b| a.cmp_pos_by(b, *axis));
        //         cloud.sort_by(|a, b| a.cmp_acc_by(b, *axis));
        //         for i in 0..cloud.len() {
        //             if !pos_sorted[i].collides(&cloud[i]) {
        //                 continue 'evolve;
        //             }
        //         }
        //         // cloud.sort_by(|a, b| a.cmp_vel_by(b, *axis));
        //         // for i in 0..cloud.len() {
        //         //     if pos_sorted[i] != cloud[i] {
        //         //         continue 'evolve;
        //         //     }
        //         // }
        //     }
        //     println!("Stopping at step {_step}");
        //     break;
        // }

        // Another idea for stopping condition, courtesy of u/fette3lke on Reddit:
        // > I thought part 1 was a hint to that. in every time step I check whether the particle with the largest distance
        // > is also the particle with the largest velocity and acceleration. If True I remove it from the set (and count
        // > it) since it is escaping the rest of the particles. At some point all the particles either collided or escaped.
    }
    let part2 = cloud.len();

    (min_idx.to_string(), part2.to_string())
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}

#[allow(dead_code)]
impl Particle {
    fn new(data: &str, particle_pattern: &Regex) -> Self {
        let caps = particle_pattern.captures(data).unwrap();
        let p: (i64, i64, i64) = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();
        let v: (i64, i64, i64) = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();
        let a: (i64, i64, i64) = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();

        Particle { p, v, a }
    }
    fn accel(&self) -> i64 {
        self.a.0.abs() + self.a.1.abs() + self.a.2.abs()
    }
    fn _step(&mut self) {
        self.v = (
            self.v.0 + self.a.0,
            self.v.1 + self.a.1,
            self.v.2 + self.a.2,
        );
        self.p = (
            self.p.0 + self.v.0,
            self.p.1 + self.v.1,
            self.p.2 + self.v.2,
        );
    }
    fn collides(&self, other: &Particle) -> bool {
        self.p == other.p
    }
    fn cmp_pos_by(&self, other: &Particle, axis: Axis) -> Ordering {
        match axis {
            Axis::X => self.p.0.cmp(&other.p.0),
            Axis::Y => self.p.1.cmp(&other.p.1),
            Axis::Z => self.p.2.cmp(&other.p.2),
        }
    }
    fn cmp_acc_by(&self, other: &Particle, axis: Axis) -> Ordering {
        match axis {
            Axis::X => {
                if self.a.0 == other.a.0 {
                    self.v.0.cmp(&other.v.0)
                } else {
                    self.a.0.cmp(&other.a.0)
                }
            }
            Axis::Y => {
                if self.a.1 == other.a.1 {
                    self.v.1.cmp(&other.v.1)
                } else {
                    self.a.1.cmp(&other.a.1)
                }
            }
            Axis::Z => {
                if self.a.2 == other.a.2 {
                    self.v.2.cmp(&other.v.2)
                } else {
                    self.a.2.cmp(&other.a.2)
                }
            }
        }
    }
    fn cmp_vel_by(&self, other: &Particle, axis: Axis) -> Ordering {
        match axis {
            Axis::X => self.v.0.cmp(&other.v.0),
            Axis::Y => self.v.1.cmp(&other.v.1),
            Axis::Z => self.v.2.cmp(&other.v.2),
        }
    }
}

impl fmt::Debug for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Position")
            .field(&self.p.0)
            .field(&self.p.1)
            .field(&self.p.2)
            .finish()
    }
}
