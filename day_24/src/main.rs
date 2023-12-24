use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    // part_one(contents, (7, 27));
    part_one(contents, (200000000000000, 400000000000000));
}

fn part_one(contents: String, area: (usize, usize)) {
    let hails = contents
        .split("\n")
        .map(|line| {
            let [pos, vel] = line.split(" @ ").collect::<Vec<_>>()[..2] else {
                unreachable!()
            };
            let pos = pos
                .split(", ")
                .map(|c| c.trim().parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let vel = vel
                .split(", ")
                .map(|c| c.trim().parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            (pos, vel)
        })
        .map(|(pos, vel)| Hail::new(pos, vel))
        .collect::<Vec<_>>();

    let part_one = hails
        .iter()
        .tuple_combinations()
        .filter(|(hail_a, hail_b)| hail_a.will_cross_in_test_area(hail_b, area))
        .count();

    dbg!(part_one);
}

#[derive(Debug, PartialEq, Eq)]
struct Hail {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Hail {
    fn new(pos: Vec<isize>, vel: Vec<isize>) -> Hail {
        let x = pos[0];
        let y = pos[1];
        let vx = vel[0];
        let vy = vel[1];

        Hail { x, y, vx, vy }
    }

    fn intersect_trajectory(&self, other: &Hail) -> (f64, f64) {
        let m1 = self.vy as f64 / self.vx as f64;
        let b1 = self.vy as f64 / self.vx as f64 * -self.x as f64 + self.y as f64;
        let m2 = other.vy as f64 / other.vx as f64;
        let b2 = other.vy as f64 / other.vx as f64 * -other.x as f64 + other.y as f64;

        let x = (b1 - b2) / (m2 - m1);
        let y = m1 * x + b1;
        (x, y)
    }

    fn will_cross_in_test_area(&self, other: &Hail, area: (usize, usize)) -> bool {
        let (x, y) = self.intersect_trajectory(other);
        let (low_limit, high_limit) = (area.0 as f64, area.1 as f64);

        x >= low_limit 
            && x <= high_limit
            && y >= low_limit 
            && y <= high_limit
            && (self.x + self.vx > self.x) == (x > self.x as f64)
            && (other.x + other.vx > other.x) == (x > other.x as f64)
            && (self.y + self.vy > self.y) == (y > self.y as f64)
            && (other.y + other.vy > other.y) == (y > other.y as f64)
    }
}
