use std::fs;

fn main() {
    let contents = fs::read_to_string("test_5.txt").unwrap();

    part_one(contents);
}

fn part_one(contents: String) {
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

    let (i, j, k) = lines // grid size
        .iter()
        .map(|line| {
            (
                line[0][0].max(line[1][0]),
                line[0][1].max(line[1][1]),
                line[0][2].max(line[1][2]),
            )
        })
        .reduce(|(max_x, max_y, max_z), (x, y, z)| (max_x.max(x), max_y.max(y), max_z.max(z)))
        .map(|(x, y, z)| (x + 1, y + 1, z))
        .unwrap();
    let grid = vec![vec![vec![0; k]; j]; i]; // vec[x][y][z]

    let bricks: Vec<Brick> = lines
        .iter()
        .enumerate()
        .map(|(idx, vec)| Brick::new(&vec, idx))
        .collect();

    dbg!(&bricks[0]);
    dbg!(&bricks[4]);
}

#[derive(Debug)]
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
        let (zi, zj) = ranges[2];
        let z = (zi - 1, zj - 1);
        Brick { id, x, y, z }
    }
}
