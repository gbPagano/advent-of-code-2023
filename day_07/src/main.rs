use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // part_one(contents);
    part_two(contents);
}


fn classify_hand(hand: &Vec<i8>) -> i8 {
    let mut counts = std::collections::HashMap::new();
    let mut has_joker = false;
    let mut jokes = 0;
    for &card in hand {
        if card == 1 {
            has_joker = true;
            jokes += 1;
        } else {
            *counts.entry(card).or_insert(0) += 1;
        }
    }

    let num_distinct_labels = counts.len();
    let max_ocorrencias = *counts.values().max().unwrap_or(&0);

    match (num_distinct_labels, max_ocorrencias, has_joker, jokes) {
        (1, 5, false, _) => 7,
        (2, 4, false, _) => 6,
        (2, 3, false, _) => 5,
        (3, 3, false, _) => 4,
        (3, 2, false, _) => 3,
        (4, 2, false, _) => 2,
        (5, 1, false, _) => 1,
        (1, 4, true, 1) => 7,  // Jxxxx
        (1, 3, true, 2) => 7,   // JJxxx
        (2, 3, true, 1) => 6,   // Jyxxx
        (1, 2, true, 3) => 7, // JJJxx
        (2, 2, true, 2) => 6,     // JJxyy
        (2, 2, true, 1) => 5,     // Jxxyy
        (3, 2, true, 1) => 4,     // Jzxyy
        (1, 1, true, 4) => 7,     // JJJJy
        (2, 1, true, 3) => 6,     // JJJyz
        (3, 1, true, 2) => 4,    // JJxyz
        (4, 1, true, 1) => 2,    // Jzyxw
        (0, 0, true, _) => 7,    // JJJJJ
        _ => unreachable!(),
    }
}

fn part_one(contents: String) {
    let lines: Vec<&str> = contents.split('\n').into_iter().collect();

    println!("{:?}", lines);

    let mut hands: Vec<Vec<i8>> = Vec::new(); 
    let mut bids: HashMap<String, i32> = HashMap::new(); 
    let mut hand_ranks: Vec<i8> = Vec::new(); 
    for line in lines {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let [hand_str, bid] = line.as_slice() else { unreachable!() };

        let mut hand: Vec<i8> = Vec::new();
        for card in hand_str.chars() {
            match card {
                'A' => hand.push(14),
                'K' => hand.push(13),
                'Q' => hand.push(12),
                'J' => hand.push(11),
                'T' => hand.push(10),
                n => hand.push(n.to_digit(10).unwrap() as i8),
            }
        }
        let key: String = hand.iter().map(|&x| x.to_string()).collect();
        bids.insert(key, bid.parse::<i32>().unwrap());
        hands.push(hand);
    }
    for hand in &hands {
        hand_ranks.push(classify_hand(hand));
    }

    
    let mut combined: Vec<_> = hand_ranks.into_iter().zip(hands).collect();
    combined.sort_by(|(a_bid, a_hand), (b_bid, b_hand)| {
        let comparison = a_bid.cmp(&b_bid);
        if comparison == std::cmp::Ordering::Equal {
            a_hand.cmp(&b_hand)
        } else {
            comparison
        }
    });
    let ordened: Vec<_> = combined.into_iter().map(|(_, outro_valor)| outro_valor).collect();
    
    let mut result = 0;
    for (idx, hand) in ordened.iter().enumerate() {
        let key: String = hand.iter().map(|&x| x.to_string()).collect();

        let value = bids.get(&key).unwrap();
        // println!("{:?} {}", value, idx+1);

        result += value * (idx + 1) as i32;
    }

    println!("{:?}", result);
}

fn part_two(contents: String) {
    let lines: Vec<&str> = contents.split('\n').into_iter().collect();

    // println!("{:?}", lines);

    let mut hands: Vec<Vec<i8>> = Vec::new(); 
    let mut bids: HashMap<String, i32> = HashMap::new(); 
    let mut hand_ranks: Vec<i8> = Vec::new(); 
    for line in lines {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let [hand_str, bid] = line.as_slice() else { unreachable!() };

        let mut hand: Vec<i8> = Vec::new();
        for card in hand_str.chars() {
            match card {
                'A' => hand.push(13),
                'K' => hand.push(12),
                'Q' => hand.push(11),
                'T' => hand.push(10),
                'J' => hand.push(1),
                n => hand.push(n.to_digit(10).unwrap() as i8),
            }
        }
        let key: String = hand.iter().map(|&x| x.to_string()).collect();
        bids.insert(key, bid.parse::<i32>().unwrap());
        hands.push(hand);
    }
    for hand in &hands {
        hand_ranks.push(classify_hand(hand));
    }

    
    let mut combined: Vec<_> = hand_ranks.into_iter().zip(hands).collect();
    combined.sort_by(|(a_bid, a_hand), (b_bid, b_hand)| {
        let comparison = a_bid.cmp(&b_bid);
        if comparison == std::cmp::Ordering::Equal {
            a_hand.cmp(&b_hand)
        } else {
            comparison
        }
    });
    let ordened: Vec<_> = combined.into_iter().map(|(_, outro_valor)| outro_valor).collect();
    
    let mut result = 0;
    for (idx, hand) in ordened.iter().enumerate() {
        let key: String = hand.iter().map(|&x| x.to_string()).collect();

        let value = bids.get(&key).unwrap();
        // println!("{:?} {}", value, idx+1);

        result += value * (idx + 1) as i32;
    }

    println!("{:?}", result);
}