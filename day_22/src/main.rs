use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let (supports, is_supported) = part_one(contents);
    part_two(supports, is_supported);
}

fn part_one(contents: String) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    let lines: Vec<_> = contents
        .split("\n")
        .map(|line| {
            line.split("~")
                .map(|block| {
                    block
                        .split(",")
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut bricks: Vec<Brick> = lines
        .iter()
        .enumerate()
        .map(|(idx, vec)| Brick::new(&vec, idx))
        .collect();
    bricks.sort(); // sorting by index z
    
    for i in 0..bricks.len() {    
        let (previous_bricks, brick_and_future) = bricks.split_at_mut(i);
        let brick = &mut brick_and_future[0];
        
        let mut new_start_z = 1;
        for previous_brick in previous_bricks {
            if brick.overlaps_xy(&previous_brick) {
                new_start_z = new_start_z.max(previous_brick.z.1 + 1);
            }
        }
        let new_end_z = brick.z.1 - (brick.z.0 - new_start_z);
        
        brick.z = (new_start_z, new_end_z);
    }
    
    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut is_supported: HashMap<usize, HashSet<usize>> = HashMap::new();

    for brick in &bricks {
        is_supported.insert(brick.id, HashSet::new());
        supports.insert(brick.id, HashSet::new());
    }
    for i in 0..bricks.len() {
        let (previous_bricks, brick_and_future) = bricks.split_at_mut(i);
        let brick = &mut brick_and_future[0];

        for previous_brick in previous_bricks {
            if brick.overlaps_xy(&previous_brick) && brick.z.0 == previous_brick.z.1 + 1 {
                is_supported
                    .entry(brick.id)
                    .and_modify(|set| {
                        set.insert(previous_brick.id);
                    });
                supports
                    .entry(previous_brick.id)
                    .and_modify(|set| {
                        set.insert(brick.id);
                    });
            }
        }
    }

    let part_one = supports
        .iter()
        .filter(|(_, v)| 
            v.iter().all(|k| is_supported.get(k).unwrap().len() > 1)
        )
        .count();

    dbg!(part_one);

    (supports, is_supported)
}

fn part_two(supports: HashMap<usize, HashSet<usize>>, is_supported: HashMap<usize, HashSet<usize>>) {
    println!();

    let mut part_two = 0;
    for k in 0..supports.len() {
        let mut desintegrated: HashSet<usize> = HashSet::from_iter([k]);
        let mut queue: VecDeque<&usize> = VecDeque::from_iter(supports.get(&k).unwrap());
        while let Some(j) = queue.pop_front() {
            if desintegrated.contains(j) {
                continue;
            }
            if is_supported.get(j).unwrap().iter().all(|v| desintegrated.contains(v)) {
                part_two += 1;
                desintegrated.insert(*j);
                queue.extend(supports.get(j).unwrap());
            }
        }
    }
    dbg!(part_two);
}

#[derive(Debug, PartialEq, Eq)]
struct Brick {
    id: usize,
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

impl Brick {
    fn new(vec: &Vec<Vec<usize>>, id: usize) -> Brick {
        let ranges = vec[0]
            .iter()
            .zip(&vec[1])
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<_>>();
        let x = ranges[0];
        let y = ranges[1];
        let z = ranges[2];
        Brick { id, x, y, z }
    }

    fn overlaps_xy(&self, other: &Brick) -> bool {
        self.x.0.max(other.x.0) <= self.x.1.min(other.x.1) && self.y.0.max(other.y.0) <= self.y.1.min(other.y.1)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.z.partial_cmp(&other.z)
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}