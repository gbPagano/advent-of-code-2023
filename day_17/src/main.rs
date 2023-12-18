use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // part_one(contents);
    part_two(contents);
}

fn part_one(contents: String) {
    let grid = contents
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<_>>();

    let mut heap_queue = BinaryHeap::new();
    let mut seen: HashSet<(isize, isize, isize, isize, isize)> = HashSet::new();
    let start: (isize, isize, isize, isize, isize, isize) = (0, 0, 0, 0, 0, 0);
    heap_queue.push(Reverse(start));

    while let Some(Reverse(item)) = heap_queue.pop() {
        let (heap, i, j, di, dj, steps) = item;

        if i + 1 == grid.len() as isize && j + 1 == grid[0].len() as isize {
            println!("res: {heap}");
            break;
        }

        if seen.contains(&(i, j, di, dj, steps)) {
            continue;
        }
        seen.insert((i, j, di, dj, steps));

        if steps < 3
            && i + di >= 0
            && i + di < grid.len() as isize
            && j + dj >= 0
            && j + dj < grid[0].len() as isize
            && (di, dj) != (0, 0)
        {
            let next = (
                heap + grid[(i + di) as usize][(j + dj) as usize],
                i + di,
                j + dj,
                di,
                dj,
                steps + 1,
            );
            heap_queue.push(Reverse(next));
        }

        for (ndi, ndj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            if i + ndi >= 0
                && i + ndi < grid.len() as isize
                && j + ndj >= 0
                && j + ndj < grid[0].len() as isize
                && (ndi, ndj) != (di, dj)
                && (ndi, ndj) != (-di, -dj)
            {
                let next = (
                    heap + grid[(i + ndi) as usize][(j + ndj) as usize],
                    i + ndi,
                    j + ndj,
                    ndi,
                    ndj,
                    1,
                );
                heap_queue.push(Reverse(next));
            }
        }
    }
}

fn part_two(contents: String) {
    let grid = contents
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<_>>();

    let mut heap_queue = BinaryHeap::new();
    let mut seen: HashSet<(isize, isize, isize, isize, isize)> = HashSet::new();
    // (heap, i, j, di, dj, steps)
    let start: (isize, isize, isize, isize, isize, isize) = (0, 0, 0, 0, 0, 4);
    heap_queue.push(Reverse(start));

    while let Some(Reverse(item)) = heap_queue.pop() {
        let (heap, i, j, di, dj, steps) = item;
        // print!("{:?}", grid[i as usize][j as usize]);

        if i + 1 == grid.len() as isize && j + 1 == grid[0].len() as isize && steps >= 4{
            println!("res: {heap}");
            break;
        }

        if seen.contains(&(i, j, di, dj, steps)) {
            continue;
        }
        seen.insert((i, j, di, dj, steps));

        if steps < 10
            && i + di >= 0
            && i + di < grid.len() as isize
            && j + dj >= 0
            && j + dj < grid[0].len() as isize
            && (di, dj) != (0, 0)
        {
            let next = (
                heap + grid[(i + di) as usize][(j + dj) as usize],
                i + di,
                j + dj,
                di,
                dj,
                steps + 1,
            );
            heap_queue.push(Reverse(next));
        }

        for (ndi, ndj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            if i + ndi >= 0
                && i + ndi < grid.len() as isize
                && j + ndj >= 0
                && j + ndj < grid[0].len() as isize
                && (ndi, ndj) != (di, dj)
                && (ndi, ndj) != (-di, -dj)
                && steps >= 4
            {
                let next = (
                    heap + grid[(i + ndi) as usize][(j + ndj) as usize],
                    i + ndi,
                    j + ndj,
                    ndi,
                    ndj,
                    1,
                );
                heap_queue.push(Reverse(next));
            }
        }
    }
}