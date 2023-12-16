use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use itertools::Itertools;


fn main() {    
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // part_one(contents);
    part_two(contents);
}

fn part_one(contents: String) {
    let mut universe = contents
        .split('\n')
        .into_iter()
        .map(|item| item.chars().map(|c| if c == '.' {0} else {-1}).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();

    
    let mut galaxies_positions = numerate_galaxies(&mut universe);
    
    duplicate_lines(&universe, &mut galaxies_positions, 1);
    duplicate_columns(&universe, &mut galaxies_positions, 1);

    let mut sum = 0;
    for combination in galaxies_positions.iter().combinations(2) {
        let [galaxy_a, galaxy_b] = combination.as_slice() else { unreachable!() };
        let distance = (galaxy_a.1.0 - galaxy_b.1.0).abs() + (galaxy_a.1.1 - galaxy_b.1.1).abs();
        // println!("{} -> {} : {}", galaxy_a.0, galaxy_b.0, distance);
        sum += distance;
    }
    println!("{sum}");
}

fn part_two(contents: String) {
    let mut universe = contents
        .split('\n')
        .into_iter()
        .map(|item| item.chars().map(|c| if c == '.' {0} else {-1}).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();

    
    let mut galaxies_positions = numerate_galaxies(&mut universe);
    
    duplicate_lines(&universe, &mut galaxies_positions, 999999);
    duplicate_columns(&universe, &mut galaxies_positions, 999999);

    let mut sum = 0;
    for combination in galaxies_positions.iter().combinations(2) {
        let [galaxy_a, galaxy_b] = combination.as_slice() else { unreachable!() };
        let distance = (galaxy_a.1.0 - galaxy_b.1.0).abs() + (galaxy_a.1.1 - galaxy_b.1.1).abs();
        // println!("{} -> {} : {}", galaxy_a.0, galaxy_b.0, distance);
        sum += distance;
    }
    println!("{sum}");
}

fn numerate_galaxies(universe: &mut Vec<Vec<i64>>) -> HashMap<i64, (i64, i64)> {
    let mut galaxies_positions: HashMap<i64, (i64, i64)> = HashMap::new();
    let mut counter = 1;
    for (i, line) in universe.iter_mut().enumerate() {
        for (j, item) in line.iter_mut().enumerate() {
            if item == &-1 {
                *item = counter;
                galaxies_positions.insert(counter, (i as i64, j as i64));
                counter += 1;
            }
        }
    }
    galaxies_positions
}


fn duplicate_lines(
    universe: &Vec<Vec<i64>>, 
    galaxies_positions: &mut HashMap<i64, (i64, i64)>,
    value: i64,
) {
    let idxs = universe
        .into_iter()
        .enumerate()
        .filter_map(|(idx, line)| {
            if line.into_iter().all(|c| *c == 0) {
                Some(idx)
            } else {
                None
            }
        })
        .rev()
        .collect::<Vec<usize>>();
    
    for idx in idxs {
        for galaxy in galaxies_positions
            .iter_mut()
            .filter(|(_, (i, _))| *i > idx as i64)
        {
            galaxy.1.0 += value;
        }
    }
}

fn duplicate_columns(
    universe: &Vec<Vec<i64>>, 
    galaxies_positions: &mut HashMap<i64, (i64, i64)>,
    value: i64,
) {
    let num_columns = universe[0].len();

    let idxs = (0..num_columns)
        .filter_map(|col_idx| {
            if universe.into_iter().all(|line| line[col_idx] == 0) {
                Some(col_idx)
            } else {
                None
            }
        })
        .rev()
        .collect::<Vec<usize>>();
    
    for idx in idxs {
        for galaxy in galaxies_positions
            .iter_mut()
            .filter(|(_, (_, j))| *j > idx as i64)
        {
            galaxy.1.1 += value;
        }
    }


    // for idx in idxs {
    //     for line in universe.iter_mut() {
    //         line.insert(idx, 0);
    //     }
    // }
}