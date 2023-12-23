use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    // part_one(contents, 64);
    // part_two_examples(contents, 64);
    part_two_input(contents, 26501365);
}

fn part_one(contents: String, steps: usize) {
    let mut grid: Vec<_> = contents
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let initial_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|char| *char == 'S').map(|j| (i, j)))
        .unwrap();

    let mut positions_to_eval: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut correct: HashSet<(usize, usize)> = HashSet::new();

    positions_to_eval.push_back((initial_pos.0, initial_pos.1, 0));
    seen.insert(initial_pos);

    while let Some((i, j, cur_step)) = positions_to_eval.pop_front() {
        if steps % 2 == cur_step % 2 {
            correct.insert((i, j));
            grid[i][j] = 'O';
        }
        for next_pos in eval_pos((i, j, cur_step), &mut seen, &grid) {
            if next_pos.2 <= steps {
                positions_to_eval.push_back(next_pos);
            }
        }
    }
    for line in &grid {
        println!("{:?}", line.iter().collect::<String>())
    }
    dbg!(correct.len());
}

fn eval_pos(
    pos: (usize, usize, usize),
    seen: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) -> Vec<(usize, usize, usize)> {
    let (i, j, step) = pos;
    let num_lines = grid.len() as isize;
    let num_columns = grid[0].len() as isize;

    let mut next_positions: Vec<(usize, usize, usize)> = Vec::new();
    for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if ni >= 0
            && ni < num_lines
            && nj >= 0
            && nj < num_columns
            && grid[ni as usize][nj as usize] != '#'
            && !seen.contains(&(ni as usize, nj as usize))
        {
            seen.insert((ni as usize, nj as usize));
            next_positions.push((ni as usize, nj as usize, step + 1));
        }
    }
    next_positions
}

fn part_two_input(contents: String, steps: usize) {
    let mut grid: Vec<_> = contents
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let initial_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|char| *char == 'S').map(|j| (i, j)))
        .unwrap();

    // asumptions
    assert_eq!(grid.len() / 2, grid[0].len() / 2); // it is a square
    assert_eq!(initial_pos, (grid.len() / 2, grid.len() / 2)); // starts on center
    assert!(grid[initial_pos.0].iter().all(|c| *c != '#')); // the middle line is empty
    assert!(grid.iter().all(|line| line[initial_pos.1] != '#')); // the middle column is empty
    assert_eq!(steps % grid.len(), (grid.len() - 1) / 2); // steps end at the end of a grid

    let n_grids = steps / grid.len();

    let mut positions_to_eval: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    positions_to_eval.push_back((initial_pos.0, initial_pos.1, 0));
    seen.insert(initial_pos);

    let mut full_odd = 0;
    let mut edge_odd = 0;
    let mut full_even = 0;
    let mut edge_even = 0;
    while let Some((i, j, cur_step)) = positions_to_eval.pop_front() {
        if cur_step % 2 == 0 {
            full_even += 1;
            if cur_step > 65 {
                edge_even += 1;
            }
        } else {
            full_odd += 1;
            if cur_step > 65 {
                edge_odd += 1;
            }
        }
        for next_pos in eval_pos((i, j, cur_step), &mut seen, &grid) {
            positions_to_eval.push_back(next_pos);
        }
    }
    dbg!(full_odd, full_even, edge_odd, edge_even);

    let res = if n_grids % 2 == 0 {
        let even = full_even * (n_grids).pow(2);
        let odd = full_odd * (n_grids + 1).pow(2);
        let inner_edges = n_grids * edge_even;
        let outer_edges = (n_grids + 1) * edge_odd;
        dbg!(odd, even, outer_edges, inner_edges);
        even + odd + inner_edges - outer_edges
    } else {
        let odd = full_odd * n_grids.pow(2);
        let even = full_even * (n_grids + 1).pow(2);
        let inner_edges = n_grids * edge_odd;
        let outer_edges = (n_grids + 1) * edge_even;

        even + odd + inner_edges - outer_edges
    };

    dbg!(res);
}

fn part_two_examples(contents: String, steps: usize) {
    let mut grid: Vec<_> = contents
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let initial_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|char| *char == 'S').map(|j| (i, j)))
        .unwrap();

    let mut positions_to_eval: VecDeque<(usize, usize, usize, isize, isize)> = VecDeque::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut correct: HashSet<(isize, isize)> = HashSet::new();

    positions_to_eval.push_back((
        initial_pos.0,
        initial_pos.1,
        0,
        initial_pos.0 as isize,
        initial_pos.1 as isize,
    ));
    seen.insert((initial_pos.0 as isize, initial_pos.1 as isize));

    while let Some((i, j, cur_step, ri, rj)) = positions_to_eval.pop_front() {
        if steps % 2 == cur_step % 2 {
            correct.insert((ri, rj));
            grid[i][j] = 'O';
        }

        for next_pos in eval_pos_2((ri, rj, cur_step), &mut seen, &grid) {
            if next_pos.2 <= steps {
                positions_to_eval.push_back(next_pos);
            }
        }
    }
    for line in &grid {
        println!("{:?}", line.iter().collect::<String>())
    }
    dbg!(correct.len());
}

fn eval_pos_2(
    pos: (isize, isize, usize),
    seen: &mut HashSet<(isize, isize)>,
    grid: &Vec<Vec<char>>,
) -> Vec<(usize, usize, usize, isize, isize)> {
    let (ri, rj, step) = pos;
    let num_lines = grid.len() as isize;
    let num_columns = grid[0].len() as isize;

    let mut next_positions: Vec<(usize, usize, usize, isize, isize)> = Vec::new();
    for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let mut ni = ri as isize + di;
        let mut nj = rj as isize + dj;
        let nri = ni;
        let nrj = nj;

        if ni >= num_lines {
            ni = ni % num_lines;
        } else if ni < 0 {
            ni = (((ni % num_lines) + num_lines) as usize % num_lines as usize) as isize;
        }
        if nj >= num_columns {
            nj = nj % num_columns;
        } else if nj < 0 {
            nj = (((nj % num_columns) + num_columns) as usize % num_columns as usize) as isize;
        }

        if grid[ni as usize][nj as usize] != '#' && !seen.contains(&(nri, nrj)) {
            seen.insert((nri, nrj));
            next_positions.push((ri as usize, rj as usize, step + 1, nri, nrj));
        }
    }
    next_positions
}
