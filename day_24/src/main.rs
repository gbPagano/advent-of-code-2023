use itertools::Itertools;
use nalgebra::{DMatrix, DVector, Matrix3, Matrix6, RowVector3, RowVector6, Vector6};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    // part_one(contents.clone(), (7, 27));
    // part_one(contents, (200000000000000, 400000000000000));
    part_two(contents);
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
        .map(|(pos, vel)| HailStone::new(pos, vel))
        .collect::<Vec<_>>();

    let part_one = hails
        .iter()
        .tuple_combinations()
        .filter(|(hail_a, hail_b)| hail_a.will_cross_in_test_area(hail_b, area))
        .count();

    dbg!(part_one);
    println!();
}

fn part_two(contents: String) {
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
        .map(|(pos, vel)| HailStone::new(pos, vel))
        .collect::<Vec<_>>();

    let a = &hails[0];
    let b = &hails[1];
    let c = &hails[2];

    let hails_matrix = Matrix3::from_rows(&[
        RowVector3::from_row_slice(&a.get_vel_vec()),
        RowVector3::from_row_slice(&b.get_vel_vec()),
        RowVector3::from_row_slice(&c.get_vel_vec()),
    ]);
    let det = hails_matrix.determinant();
    assert!(det != 0.); // linearly independent

    let coefficients = Matrix6::from_rows(&[
        RowVector6::new(
            (b.vy - a.vy) as f64,
            (a.vx - b.vx) as f64,
            0.,
            (a.y - b.y) as f64,
            (b.x - a.x) as f64,
            0.,
        ),
        RowVector6::new(
            (c.vy - a.vy) as f64,
            (a.vx - c.vx) as f64,
            0.,
            (a.y - c.y) as f64,
            (c.x - a.x) as f64,
            0.,
        ),
        RowVector6::new(
            (b.vz - a.vz) as f64,
            0.,
            (a.vx - b.vx) as f64,
            (a.z - b.z) as f64,
            0.,
            (b.x - a.x) as f64,
        ),
        RowVector6::new(
            (c.vz - a.vz) as f64,
            0.,
            (a.vx - c.vx) as f64,
            (a.z - c.z) as f64,
            0.,
            (c.x - a.x) as f64,
        ),
        RowVector6::new(
            0.,
            (b.vz - a.vz) as f64,
            (a.vy - b.vy) as f64,
            0.,
            (a.z - b.z) as f64,
            (b.y - a.y) as f64,
        ),
        RowVector6::new(
            0.,
            (c.vz - a.vz) as f64,
            (a.vy - c.vy) as f64,
            0.,
            (a.z - c.z) as f64,
            (c.y - a.y) as f64,
        ),
    ]);

    let constants = Vector6::new(
        ((a.y * a.vx - b.y * b.vx) - (a.x * a.vy - b.x * b.vy)) as f64,
        ((a.y * a.vx - c.y * c.vx) - (a.x * a.vy - c.x * c.vy)) as f64,
        ((a.z * a.vx - b.z * b.vx) - (a.x * a.vz - b.x * b.vz)) as f64,
        ((a.z * a.vx - c.z * c.vx) - (a.x * a.vz - c.x * c.vz)) as f64,
        ((a.z * a.vy - b.z * b.vy) - (a.y * a.vz - b.y * b.vz)) as f64,
        ((a.z * a.vy - c.z * c.vy) - (a.y * a.vz - c.y * c.vz)) as f64,
    );

    if let Some(solution) = coefficients.lu().solve(&constants) {
        dbg!(solution.data.as_slice());
        let part_two = solution.data.0[0].iter().take(3).sum::<f64>();
        dbg!(part_two);
    } else {
        println!("failed!");
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HailStone {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

impl HailStone {
    fn new(pos: Vec<isize>, vel: Vec<isize>) -> HailStone {
        let x = pos[0];
        let y = pos[1];
        let z = pos[2];
        let vx = vel[0];
        let vy = vel[1];
        let vz = vel[2];

        HailStone {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }

    fn intersect_trajectory(&self, other: &HailStone) -> (f64, f64) {
        let m1 = self.vy as f64 / self.vx as f64;
        let b1 = self.vy as f64 / self.vx as f64 * -self.x as f64 + self.y as f64;
        let m2 = other.vy as f64 / other.vx as f64;
        let b2 = other.vy as f64 / other.vx as f64 * -other.x as f64 + other.y as f64;

        let x = (b1 - b2) / (m2 - m1);
        let y = m1 * x + b1;
        (x, y)
    }

    fn will_cross_in_test_area(&self, other: &HailStone, area: (usize, usize)) -> bool {
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

    fn get_vel_vec(&self) -> [f64; 3] {
        [self.vx as f64, self.vy as f64, self.vz as f64]
    }
}
