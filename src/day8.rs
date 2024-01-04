use std::{collections::HashMap, usize};
type Network = HashMap<String, (String, String)>;

pub fn ans1(input: &str) -> usize {
    let (instructions, network) = parse(input);
    // dbg!(&instructions, &network);
    let mut n = 0;
    let lrs = instructions.chars().collect::<Vec<_>>();
    let mut node = "AAA";
    loop {
        let ins = lrs[n % lrs.len()];
        let next_node = match ins {
            'L' => &network.get(node).unwrap().0,
            'R' => &network.get(node).unwrap().1,
            _ => unreachable!(),
        };
        n += 1;
        if next_node == "ZZZ" {
            return n;
        } else {
            node = next_node.as_str();
        }
    }
}

fn parse(input: &str) -> (String, Network) {
    let mut network = HashMap::new();
    let mut lines = input.split('\n').filter(|l| l.len() > 0);
    let instructions = lines.next().unwrap().to_string();
    for l in lines {
        let from = l[0..3].to_string();
        let to = (l[7..10].to_owned(), l[12..15].to_owned());
        network.insert(from, to);
    }
    (instructions, network)
}

#[test]
fn test() {
    assert_eq!(
        ans1(
            r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
        ),
        6
    )
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input8.txt");
    println!("{}", ans1(&q));
}

pub fn ans2(input: &str) -> usize {
    let (instructions, network) = parse(input);
    // dbg!(&instructions, &network);
    let init_nodes = network
        .keys()
        .filter(|n| n.ends_with("A"))
        .collect::<Vec<_>>();
    // let init_nodes = ["AAA"];
    dbg!(&init_nodes);
    let mut loops = vec![];
    for n in init_nodes {
        let l = inspect(n, &network, &instructions);
        loops.push(l);
    }
    dbg!(&loops);

    lcm_n(loops)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcm_n(n: Vec<usize>) -> usize {
    n.into_iter().reduce(lcm).unwrap()
}

fn inspect(node: &str, network: &Network, instructions: &str) -> usize {
    let mut n = 1;
    let mut cur = node.to_owned();
    let lrs = instructions.chars().collect::<Vec<_>>();
    let len = instructions.len();

    loop {
        let next = network.get(&cur).unwrap();
        cur = match lrs[(n - 1) % len] {
            'L' => next.0.clone(),
            'R' => next.1.clone(),
            _ => unreachable!(),
        };

        if cur.ends_with('Z') {
            return n;
        }

        n += 1;
    }
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#
        ),
        6
    );
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input8.txt");
    println!("{}", ans2(&q));
}
