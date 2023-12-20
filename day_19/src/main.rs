use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
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
    let data = contents
        .split("\n\n")
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (data_1, data_2) = data.split_at(1);
    let regex = Regex::new(r"([a-zA-Z0-9]+)\{([^}]+)\}").unwrap();

    let workflows: HashMap<&str, Vec<Rule>> = data_1[0]
        .iter()
        .map(|line| {
            regex
                .captures(line)
                .map(|caps| {
                    (
                        caps.get(1).map(|m| m.as_str()).unwrap(),
                        caps.get(2)
                            .map(|m| m.as_str())
                            .unwrap()
                            .split(",")
                            .map(|rule| Rule::new(rule))
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap()
        })
        .collect();

    let regex = Regex::new(r"\{(.*?)\}").unwrap();
    let parts: Vec<_> = data_2[0]
        .iter()
        .map(|line| {
            regex
                .captures(line)
                .map(|caps| caps.get(1).map(|m| m.as_str()).unwrap())
                .unwrap()
        })
        .map(|part| Part::new(part))
        .collect();

    let initial_key = "in";
    let accepted_parts: usize = parts
        .iter()
        .filter(|part| part.evaluate(initial_key, &workflows))
        .map(|part| part.sum())
        .sum();

    println!("res: {:?}", accepted_parts);
}

fn part_two(contents: String) {
    let data = contents
        .split("\n\n")
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let regex = Regex::new(r"([a-zA-Z0-9]+)\{([^}]+)\}").unwrap();

    let workflows: HashMap<&str, Vec<Rule>> = data[0]
        .iter()
        .map(|line| {
            regex
                .captures(line)
                .map(|caps| {
                    (
                        caps.get(1).map(|m| m.as_str()).unwrap(),
                        caps.get(2)
                            .map(|m| m.as_str())
                            .unwrap()
                            .split(",")
                            .map(|rule| Rule::new(rule))
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap()
        })
        .collect();

    let mut accepted: Vec<PartRange> = Vec::new();
    let mut parts: VecDeque<PartRange> = VecDeque::new();
    parts.push_back(PartRange::new([(1, 4000); 4]));

    while let Some(mut part) = parts.pop_front() {
        let (part_res, rejected) = part.evaluate("in", &workflows);
        if let Some(success) = part_res {
            accepted.push(success);
        }
        parts.extend(rejected);
    }
    let res: usize = accepted.iter().map(|part| part.get_quantity()).sum();

    println!("res: {res}");
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}
impl PartRange {
    fn new(values: [(usize, usize); 4]) -> PartRange {
        PartRange {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }

    fn evaluate(
        &mut self,
        initial_key: &str,
        workflows: &HashMap<&str, Vec<Rule>>,
    ) -> (Option<PartRange>, Vec<PartRange>) {
        let mut key = initial_key;
        let mut rejected: Vec<PartRange> = Vec::new();
        loop {
            match key {
                "A" => return (Some(*self), rejected),
                "R" => return (None, rejected),
                _ => (),
            }
            for rule in workflows.get(key).unwrap() {
                if let Some(first) = rule.first {
                    match first {
                        'x' => match rule.eval_range(self.x) {
                            (Some(_), None) => {
                                key = rule.destination;
                                break;
                            }
                            (Some(acepted), Some(rejec)) => {
                                self.x = acepted;
                                rejected.push(PartRange { x: rejec, ..*self });

                                key = rule.destination;
                                break;
                            }
                            _ => (),
                        },
                        'm' => match rule.eval_range(self.m) {
                            (Some(_), None) => {
                                key = rule.destination;
                                break;
                            }
                            (Some(acepted), Some(rejec)) => {
                                self.m = acepted;
                                rejected.push(PartRange { m: rejec, ..*self });

                                key = rule.destination;
                                break;
                            }
                            _ => (),
                        },
                        'a' => match rule.eval_range(self.a) {
                            (Some(_), None) => {
                                key = rule.destination;
                                break;
                            }
                            (Some(acepted), Some(rejec)) => {
                                self.a = acepted;
                                rejected.push(PartRange { a: rejec, ..*self });

                                key = rule.destination;
                                break;
                            }
                            _ => (),
                        },
                        's' => match rule.eval_range(self.s) {
                            (Some(_), None) => {
                                key = rule.destination;
                                break;
                            }
                            (Some(acepted), Some(rejec)) => {
                                self.s = acepted;
                                rejected.push(PartRange { s: rejec, ..*self });

                                key = rule.destination;
                                break;
                            }
                            _ => (),
                        },
                        _ => unreachable!(),
                    }
                }
                key = rule.destination;
            }
        }
    }

    fn get_quantity(&self) -> usize {
        let qx = self.x.1 - self.x.0 + 1;
        let qm = self.m.1 - self.m.0 + 1;
        let qa = self.a.1 - self.a.0 + 1;
        let qs = self.s.1 - self.s.0 + 1;
        qx * qm * qa * qs
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    fn new(part: &str) -> Part {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        for item in part
            .split(",")
            .map(|item| item.split("=").collect::<Vec<_>>())
        {
            match item[0] {
                "x" => x += item[1].parse::<usize>().unwrap(),
                "m" => m += item[1].parse::<usize>().unwrap(),
                "a" => a += item[1].parse::<usize>().unwrap(),
                "s" => s += item[1].parse::<usize>().unwrap(),
                _ => unreachable!(),
            }
        }
        Part { x, m, a, s }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn evaluate(&self, initial_key: &str, workflows: &HashMap<&str, Vec<Rule>>) -> bool {
        let mut key = initial_key;
        loop {
            match key {
                "A" => return true,
                "R" => return false,
                _ => (),
            }
            for rule in workflows.get(key).unwrap() {
                if let Some(first) = rule.first {
                    match first {
                        'x' => {
                            if rule.eval(self.x) {
                                key = rule.destination;
                                break;
                            }
                        }
                        'm' => {
                            if rule.eval(self.m) {
                                key = rule.destination;
                                break;
                            }
                        }
                        'a' => {
                            if rule.eval(self.a) {
                                key = rule.destination;
                                break;
                            }
                        }
                        's' => {
                            if rule.eval(self.s) {
                                key = rule.destination;
                                break;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                key = rule.destination;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    GreaterThan,
    LessThan,
}
impl Operation {
    fn from_ch(ch: char) -> Operation {
        match ch {
            '>' => Operation::GreaterThan,
            '<' => Operation::LessThan,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule<'a> {
    first: Option<char>,
    op: Option<Operation>,
    last: Option<usize>,
    destination: &'a str,
}
impl Rule<'_> {
    fn new(rule: &str) -> Rule {
        match rule.split(":").collect::<Vec<_>>().as_slice() {
            [name_op_last, destination] => Rule {
                first: name_op_last.chars().nth(0),
                op: Some(Operation::from_ch(name_op_last.chars().nth(1).unwrap())),
                last: Some(name_op_last[2..].parse::<usize>().unwrap()),
                destination,
            },
            [destination] => Rule {
                first: None,
                op: None,
                last: None,
                destination,
            },
            _ => unreachable!(),
        }
    }

    fn eval(&self, val: usize) -> bool {
        match self.op.unwrap() {
            Operation::GreaterThan => val > self.last.unwrap(),
            Operation::LessThan => val < self.last.unwrap(),
        }
    }

    fn eval_range(&self, val: (usize, usize)) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
        let (min, max) = val;
        let comp = self.last.unwrap();
        // -> accepted, rejected
        match self.op.unwrap() {
            Operation::GreaterThan => {
                if min > comp {
                    (Some((min, max)), None)
                } else if max > comp {
                    (Some((comp + 1, max)), Some((min, comp)))
                } else {
                    (None, None)
                }
            }
            Operation::LessThan => {
                if max < comp {
                    (Some((min, max)), None)
                } else if min < comp {
                    (Some((min, comp - 1)), Some((comp, max)))
                } else {
                    (None, None)
                }
            }
        }
    }
}
