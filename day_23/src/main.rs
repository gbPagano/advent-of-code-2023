use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    part_one(contents.clone(), 1);
    part_two(contents, 2);
}


fn part_one(contents: String, part: u8) {
    let grid: Vec<_> = contents
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let start_pos = (0, grid[0].iter().position(|c| *c == '.').unwrap());
    let final_pos = (
        grid.len() - 1,
        grid[grid.len() - 1].iter().position(|c| *c == '.').unwrap(),
    );

    let mut main_points = Vec::new();
    main_points.push(start_pos);
    main_points.push(final_pos);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                continue;
            }
            if is_node(&grid, (i, j)) {
                main_points.push((i, j));
            }
        }
    }

    let mut nodes: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();

    for start_pos in main_points.iter() {
        let mut destinations: HashMap<(usize, usize), usize> = HashMap::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut stack: VecDeque<(usize, usize, usize)> = VecDeque::new();
        seen.insert(*start_pos);
        stack.push_back((start_pos.0, start_pos.1, 0));

        while let Some((i, j, steps)) = stack.pop_back() {
            if steps > 0 && main_points.contains(&(i, j)) {
                destinations.insert((i, j), steps);
                continue;
            }

            for (di, dj) in get_delta_dirs(part, grid[i][j]) {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if ni >= 0
                    && ni < grid.len() as isize
                    && nj >= 0
                    && nj < grid[0].len() as isize
                    && grid[ni as usize][nj as usize] != '#'
                    && !seen.contains(&(ni as usize, nj as usize))
                {
                    stack.push_back((ni as usize, nj as usize, steps + 1));
                    seen.insert((ni as usize, nj as usize));
                }
            }
        }

        nodes.insert(*start_pos, destinations);
    }
    let mut seen: HashSet<(usize, usize)> = HashSet::new(); 
    let part_one = depth_search(&mut seen, final_pos, &nodes, start_pos);
    dbg!(part_one);
}

fn part_two(contents: String, part: u8) {
    let grid: Vec<_> = contents
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let start_pos = (0, grid[0].iter().position(|c| *c == '.').unwrap());
    let final_pos = (
        grid.len() - 1,
        grid[grid.len() - 1].iter().position(|c| *c == '.').unwrap(),
    );

    let mut main_points = Vec::new();
    main_points.push(start_pos);
    main_points.push(final_pos);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                continue;
            }
            if is_node(&grid, (i, j)) {
                main_points.push((i, j));
            }
        }
    }

    let mut nodes: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();

    for start_pos in main_points.iter() {
        let mut destinations: HashMap<(usize, usize), usize> = HashMap::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut stack: VecDeque<(usize, usize, usize)> = VecDeque::new();
        seen.insert(*start_pos);
        stack.push_back((start_pos.0, start_pos.1, 0));

        while let Some((i, j, steps)) = stack.pop_back() {
            if steps > 0 && main_points.contains(&(i, j)) {
                destinations.insert((i, j), steps);
                continue;
            }

            for (di, dj) in get_delta_dirs(part, grid[i][j]) {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if ni >= 0
                    && ni < grid.len() as isize
                    && nj >= 0
                    && nj < grid[0].len() as isize
                    && grid[ni as usize][nj as usize] != '#'
                    && !seen.contains(&(ni as usize, nj as usize))
                {
                    stack.push_back((ni as usize, nj as usize, steps + 1));
                    seen.insert((ni as usize, nj as usize));
                }
            }
        }

        nodes.insert(*start_pos, destinations);
    }
    let mut seen: HashSet<(usize, usize)> = HashSet::new(); 
    let part_two = depth_search(&mut seen, final_pos, &nodes, start_pos);
    dbg!(part_two);
}

fn depth_search(
    seen: &mut HashSet<(usize, usize)>,
    end: (usize, usize),
    nodes: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    pos: (usize, usize),
) -> usize {
    if pos == end {
        return 0;
    }
    let mut max = 0;
    seen.insert(pos);
    if let Some(neighbors) = nodes.get(&pos) {
        for (next_pos, distance) in neighbors {
            if !seen.contains(next_pos) {
                max = max.max(depth_search(seen, end, nodes, *next_pos) + distance);
            }
        }
    }   
    seen.remove(&pos);
    max
}

fn get_delta_dirs(part: u8, item: char) -> Vec<(isize, isize)> {
    match (part, item) {
        (1, '^') => vec![(-1, 0)],
        (1, '>') => vec![(0, 1)],
        (1, '<') => vec![(0, -1)],
        (1, 'v') => vec![(1, 0)],
        _ => vec![(0, 1), (1, 0), (-1, 0), (0, -1)],
    }
}

fn is_node(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    let mut total = 0;
    let (i, j) = (pos.0 as isize, pos.1 as isize);
    for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let (ni, nj) = (i + di, j + dj);
        if ni >= 0
            && ni < grid.len() as isize
            && nj >= 0
            && nj < grid[0].len() as isize
            && grid[ni as usize][nj as usize] != '#'
        {
            total += 1;
        }
    }
    total >= 3
}