use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

use crate::common::Matrix;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
struct Point(i64, i64);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::Left => write!(f, "L"),
            Dir::Right => write!(f, "R"),
            Dir::Up => write!(f, "U"),
            Dir::Down => write!(f, "D"),
        }
    }
}

#[derive(Clone, Debug)]
struct Vector {
    point: Point,
    dir: Option<Dir>,
}

type Memo = Vec<Vec<u32>>;

pub fn ans1(input: &str) -> u32 {
    let blocks = Matrix::new(input);
    dijk1(&blocks)
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
struct Move {
    next_dir: Dir,
    point: Point,
    straight: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct MoveLoss(Move, u32);

impl Ord for MoveLoss {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min heap
        //println!("cmp");
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for MoveLoss {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijk1(tiles: &Matrix) -> u32 {
    let mut visited = HashSet::<Move>::new();
    let mut pq = BinaryHeap::<MoveLoss>::new();
    pq.push(MoveLoss(
        Move {
            next_dir: Dir::Right,
            point: Point(0, 0),
            straight: 0,
        },
        0,
    ));
    pq.push(MoveLoss(
        Move {
            next_dir: Dir::Down,
            point: Point(0, 0),
            straight: 0,
        },
        0,
    ));
    //let mut visited2 = HashSet::<String>::new();

    while let Some(mc) = pq.pop() {
        if visited.contains(&mc.0) {
            continue;
        }
        visited.insert(mc.0.clone());

        let curr_point = &mc.0.point;
        let curr_loss = mc.1;
        //let curr_cost = mc.1 + score(tiles, &curr_point);
        if curr_point.0 == tiles.width - 1 && curr_point.1 == tiles.height - 1 {
            //panic!("cost: {}, {}", mc.1, visited.len());
            //panic!("cost: {}, {}", mc.1, visited2.len());
            return mc.1;
        }

        let mut next_dirs = vec![];
        match &mc.0.next_dir {
            Dir::Left | Dir::Right => {
                next_dirs.push((Dir::Up, true));
                next_dirs.push((Dir::Down, true));
            }
            Dir::Up | Dir::Down => {
                next_dirs.push((Dir::Left, true));
                next_dirs.push((Dir::Right, true));
            }
        }
        if mc.0.straight < 3 {
            // keep going
            next_dirs.push((mc.0.next_dir.clone(), false));
        }

        for (dir, reset) in next_dirs {
            let next_point = match dir {
                Dir::Left => Point(curr_point.0 - 1, curr_point.1),
                Dir::Right => Point(curr_point.0 + 1, curr_point.1),
                Dir::Up => Point(curr_point.0, curr_point.1 - 1),
                Dir::Down => Point(curr_point.0, curr_point.1 + 1),
            };
            if tiles.get_point(next_point.0, next_point.1).is_some() {
                let next_loss = score(tiles, &next_point);
                let new_mc = MoveLoss(
                    Move {
                        next_dir: dir,
                        point: next_point,
                        straight: if reset { 1 } else { mc.0.straight + 1 },
                    },
                    curr_loss + next_loss,
                );

                pq.push(new_mc);
            }
        }
    }

    unreachable!()
}

#[allow(dead_code)]
fn pp(blocks: &Matrix, path: &[Vector]) {
    for y in 0..blocks.height {
        for x in 0..blocks.width {
            if let Some(v) = path.iter().find(|v| v.point.0 == x && v.point.1 == y) {
                match &v.dir {
                    Some(d) => match d {
                        Dir::Left => print!("<"),
                        Dir::Right => print!(">"),
                        Dir::Up => print!("^"),
                        Dir::Down => print!("v"),
                    },
                    None => print!("#"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn pp2(m: &Memo) {
    for ys in m.iter() {
        for xs in ys {
            print!("{:04},", xs);
        }
        println!()
    }
    println!()
}

fn score(blocks: &Matrix, p: &Point) -> u32 {
    blocks.get_point(p.0, p.1).unwrap().parse::<u32>().unwrap()
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#
        ),
        102
    );
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input17.txt");
    println!("{}", ans1(&q));
}

pub fn ans2(input: &str) -> u32 {
    dijk2(&Matrix::new(input))
}

fn dijk2(tiles: &Matrix) -> u32 {
    let mut visited = HashSet::<Move>::new();
    let mut pq = BinaryHeap::<MoveLoss>::new();
    pq.push(MoveLoss(
        Move {
            next_dir: Dir::Right,
            point: Point(0, 0),
            straight: 0,
        },
        0,
    ));
    pq.push(MoveLoss(
        Move {
            next_dir: Dir::Down,
            point: Point(0, 0),
            straight: 0,
        },
        0,
    ));
    //let mut visited2 = HashSet::<String>::new();

    while let Some(mc) = pq.pop() {
        if visited.contains(&mc.0) {
            continue;
        }
        visited.insert(mc.0.clone());

        let curr_point = &mc.0.point;
        let curr_loss = mc.1;
        //let curr_cost = mc.1 + score(tiles, &curr_point);
        if curr_point.0 == tiles.width - 1 && curr_point.1 == tiles.height - 1 {
            //panic!("cost: {}, {}", mc.1, visited.len());
            //panic!("cost: {}, {}", mc.1, visited2.len());
            return mc.1;
        }

        let mut next_dirs = vec![];
        if mc.0.straight < 4 {
            // min
            next_dirs.push((mc.0.next_dir.clone(), false));
        } else if mc.0.straight < 10 {
            // 3 ways to go
            next_dirs.push((mc.0.next_dir.clone(), false));
            match &mc.0.next_dir {
                Dir::Left | Dir::Right => {
                    next_dirs.push((Dir::Up, true));
                    next_dirs.push((Dir::Down, true));
                }
                Dir::Up | Dir::Down => {
                    next_dirs.push((Dir::Left, true));
                    next_dirs.push((Dir::Right, true));
                }
            }
        } else {
            // max
            match &mc.0.next_dir {
                Dir::Left | Dir::Right => {
                    next_dirs.push((Dir::Up, true));
                    next_dirs.push((Dir::Down, true));
                }
                Dir::Up | Dir::Down => {
                    next_dirs.push((Dir::Left, true));
                    next_dirs.push((Dir::Right, true));
                }
            }
        }

        for (dir, reset) in next_dirs {
            let next_point = match dir {
                Dir::Left => Point(curr_point.0 - 1, curr_point.1),
                Dir::Right => Point(curr_point.0 + 1, curr_point.1),
                Dir::Up => Point(curr_point.0, curr_point.1 - 1),
                Dir::Down => Point(curr_point.0, curr_point.1 + 1),
            };
            if tiles.get_point(next_point.0, next_point.1).is_some() {
                let next_loss = score(tiles, &next_point);
                let new_mc = MoveLoss(
                    Move {
                        next_dir: dir,
                        point: next_point,
                        straight: if reset { 1 } else { mc.0.straight + 1 },
                    },
                    curr_loss + next_loss,
                );

                pq.push(new_mc);
            }
        }
    }

    unreachable!()
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#
        ),
        94
    );
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input17.txt");
    println!("{}", ans2(&q));
}
