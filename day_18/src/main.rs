use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("test_62.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // part_one(contents);
    part_two(contents);
}

fn part_one(contents: String) {
    
    let mut instructions = contents
        .split('\n')
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|vec| (Direction::from_str(vec[0]).unwrap(), vec[1].parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let first = instructions[0].0;
    let last = instructions[instructions.len() - 1].0;
    instructions.push(instructions[0]);

    let mut terrain: Vec<Vec<char>> = vec![vec![compare_dirs(last, first)]];

    let mut i = 0;
    let mut j = 0;
    for (dir, val, chars) in transform_instructions(instructions) {
        match dir {
            Direction::Right => {
                let k = j + val;
                let mut char_idx = 0;
                while j < k {
                    if j + 1 >= terrain[0].len() {
                        add_column(&mut terrain, true);
                    }
                    j += 1;
                    terrain[i][j] = chars[char_idx];

                    char_idx += 1;
                }
            },
            Direction::Down => {
                let k = i + val;
                let mut char_idx = 0;
                while i < k {
                    if i + 1 >= terrain.len() {
                        add_line(&mut terrain, true);
                    }
                    i += 1;
                    terrain[i][j] = chars[char_idx];

                    char_idx += 1;
                }
            },
            Direction::Left => {
                let mut char_idx = 0;
                if val > j {
                    for _ in 0..val-j {
                        add_column(&mut terrain, false);
                    }
                    j += val - j;
                }
                for _ in 0..val {
                    j -= 1;
                    terrain[i][j] = chars[char_idx];

                    char_idx += 1;
                }

            },
            Direction::Up => {
                let mut char_idx = 0;
                if val > i {
                    for _ in 0..val-i {
                        add_line(&mut terrain, false);
                    }
                    i += val - i;
                } 
                for _ in 0..val {
                    i -= 1;
                    terrain[i][j] = chars[char_idx];

                    char_idx += 1;
                }

            },
        }
    }
    
    fill_terrain(&mut terrain);
    let res: usize = terrain.iter().flat_map(|c| c).filter(|c| *c != &'.').count();
    
    for line in &terrain {
        println!("{:?}", line.iter().collect::<String>());
    }

    println!("res: {:?}", res);
}

fn part_two(contents: String) {
    
    let mut instructions = contents
        .split('\n')
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|vec| vec[2].trim_matches(|char| ['(', ')', '#'].contains(&char)))
        .map(|item| (
            Direction::from_ch(item.chars().nth(5).unwrap()).unwrap(), 
            usize::from_str_radix(&item[..5], 16).unwrap()
        ))
        .collect::<Vec<_>>();


    let first = instructions[0].0;
    let last = instructions[instructions.len() - 1].0;
    instructions.push(instructions[0]);

    let mut terrain: Vec<Vec<char>> = vec![vec![compare_dirs(last, first)]];

    let mut i = 0;
    let mut j = 0;
    for (dir, val, chars) in transform_instructions(instructions) {
        match dir {
            Direction::Right => {
                let k = j + val;
                let mut char_idx = 0;
                while j < k {
                    if j + 1 >= terrain[0].len() {
                        add_column(&mut terrain, true);
                    }
                    j += 1;
                    terrain[i][j] = chars[char_idx];
                    
                    char_idx += 1;
                }
            },
            Direction::Down => {
                let k = i + val;
                let mut char_idx = 0;
                while i < k {
                    if i + 1 >= terrain.len() {
                        add_line(&mut terrain, true);
                    }
                    i += 1;
                    terrain[i][j] = chars[char_idx];
                    
                    char_idx += 1;
                }
            },
            Direction::Left => {
                let mut char_idx = 0;
                if val > j {
                    for _ in 0..val-j {
                        add_column(&mut terrain, false);
                    }
                    j += val - j;
                }
                for _ in 0..val {
                    j -= 1;
                    terrain[i][j] = chars[char_idx];

                    char_idx += 1;
                }

            },
            Direction::Up => {
                let mut char_idx = 0;
                if val > i {
                    for _ in 0..val-i {
                        add_line(&mut terrain, false);
                    }
                    i += val - i;
                } 
                for _ in 0..val {
                    i -= 1;
                    terrain[i][j] = chars[char_idx];

                    char_idx += 1;
                }

            },
        }
    }
    
    // fill_terrain(&mut terrain);
    let res: usize = terrain.iter().flat_map(|c| c).filter(|c| *c != &'.').count();
    
    for line in &terrain {
        println!("{:?}", line.iter().collect::<String>());
    }

    println!("res: {:?}", res);
}

fn transform_instructions(instructions: Vec<(Direction, usize)>) -> Vec<(Direction, usize, Vec<char>)> {
    let mut processed: Vec<(Direction, usize, Vec<char>)> = Vec::new();
    for inst in instructions.windows(2) {
        let [current, next] = inst else {unreachable!()};

        let (dir, val) = current;
        let (ndir, _) = next;

        let mut chars: Vec<char> = Vec::new();
        for _ in 1..*val {
            chars.push(dir.to_char());
        }
        // get last
        chars.push(compare_dirs(*dir, *ndir));

        processed.push((*dir, *val, chars));
    }
    processed
}



fn compare_dirs(current: Direction, next: Direction) -> char {
    match (current, next) {
        (Direction::Up, Direction::Right) => 'F',
        (Direction::Up, Direction::Left) => '7',
        (Direction::Up, _) => '|',
        (Direction::Down, Direction::Right) => 'L',
        (Direction::Down, Direction::Left) => 'J',
        (Direction::Down, _) => '|',
        (Direction::Right, Direction::Up) => 'J',
        (Direction::Right, Direction::Down) => '7',
        (Direction::Right, _) => '-',
        (Direction::Left, Direction::Up) => 'L',
        (Direction::Left, Direction::Down) => 'F',
        (Direction::Left, _) => '-',
    }  
}


fn fill_terrain(terrain: &mut Vec<Vec<char>>) {
    for i in 0..terrain.len() {
        for j in 0..terrain[0].len() {
            if terrain[i][j] == '.' && it_is_inside(terrain, (i, j)){
                terrain[i][j] = '#';
            }
        }  
    }  
}

fn it_is_inside(grid: &Vec<Vec<char>>, current_pos: (usize, usize)) -> bool {
    let (i, j) = current_pos;
    let mut counts = 0;
    for k in j+1..grid[i].len() {
        let char = grid[i][k];
        if ['|', 'J', 'L'].contains(&char) {
            counts += 1;
        }

    }
    counts % 2 == 1
}

fn add_line(terrain: &mut Vec<Vec<char>>, end: bool) {
    if end {
        terrain.push(vec!['.'; terrain[0].len()]);
    } else {
        terrain.insert(0, vec!['.'; terrain[0].len()]);
    }
}

fn add_column(terrain: &mut Vec<Vec<char>>, end: bool) {
    if end {
        terrain.iter_mut().for_each(|line| line.push('.'));
    } else {
        terrain.iter_mut().for_each(|line| line.insert(0, '.'))
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn from_str(ch: &str) -> Option<Direction> {
        match ch {
           "D" => Some(Direction::Down),
           "U" => Some(Direction::Up),
           "L" => Some(Direction::Left),
           "R" => Some(Direction::Right),
            _ => None,
        }
    }

    fn from_ch(ch: char) -> Option<Direction> {
        match ch {
           '0' => Some(Direction::Right),
           '1' => Some(Direction::Down),
           '2' => Some(Direction::Left),
           '3' => Some(Direction::Up),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::Down | Direction::Up => '|',
            Direction::Left | Direction::Right => '-',
        }
    }
}