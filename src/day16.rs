use std::{
    collections::HashSet,
    usize,
};

use crate::common::Matrix;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Dir {
    Ltr,
    Rtl,
    Ttb,
    Btt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i64, i64);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pace {
    point: Point,
    dir: Dir,
}

type Route = HashSet<Pace>;

#[allow(dead_code)]
fn print(r: &Route, w: i64, h: i64) {
    for y in 0..h {
        for x in 0..w {
            if r.iter().find(|r| r.point == Point(x, y)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn ans1(input: &str) -> usize {
    let tiles = Matrix::new(input);
    let mut routes = HashSet::new();
    // start from outside
    let start = Point(-1, 0);
    step(&start, &Dir::Ltr, &tiles, &mut routes);
    //print(&routes, tiles.width, tiles.height);
    count(&routes)
}

fn count(routes: &Route) -> usize {
    let mut point_only = HashSet::new();
    for p in routes.iter() {
        point_only.insert(&p.point);
    }
    point_only.len()
}

fn step(prev: &Point, dir: &Dir, tiles: &Matrix, routes: &mut Route) {
    let curr = match dir {
        Dir::Ltr => Point(prev.0 + 1, prev.1),
        Dir::Rtl => Point(prev.0 - 1, prev.1),
        Dir::Ttb => Point(prev.0, prev.1 + 1),
        Dir::Btt => Point(prev.0, prev.1 - 1),
    };

    let ch = tiles.get_point_char(curr.0, curr.1);
    if ch.is_none() {
        // out of bound
        return;
    }
    let p = Pace {
        point: curr.clone(),
        dir: dir.clone(),
    };
    if routes.contains(&p) {
        // visited before
        return;
    }

    //dbg!(&curr);
    routes.insert(p);
    let mut go = |new_dir: &Dir| {
        step(&curr, new_dir, tiles, routes);
    };

    match ch.unwrap() {
        '.' => {
            // keep going
            go(dir);
        }
        '|' => {
            if dir == &Dir::Ltr || dir == &Dir::Rtl {
                // split
                go(&Dir::Ttb);
                go(&Dir::Btt);
            } else {
                // keep going
                go(dir);
            }
        }
        '-' => {
            if dir == &Dir::Ttb || dir == &Dir::Btt {
                // split
                go(&Dir::Ltr);
                go(&Dir::Rtl);
            } else {
                // keep going
                go(dir);
            }
        }
        '/' => match dir {
            Dir::Ltr => go(&Dir::Btt),
            Dir::Rtl => go(&Dir::Ttb),
            Dir::Ttb => go(&Dir::Rtl),
            Dir::Btt => go(&Dir::Ltr),
        },
        '\\' => match dir {
            Dir::Ltr => go(&Dir::Ttb),
            Dir::Rtl => go(&Dir::Btt),
            Dir::Ttb => go(&Dir::Ltr),
            Dir::Btt => go(&Dir::Rtl),
        },
        _ => unreachable!(),
    }
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
        ),
        46
    );
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input16.txt");
    println!("{}", ans1(&q));
}

pub fn ans2(input: &str) -> usize {
    let tiles = Matrix::new(input);
    let mut route = Route::new();
    let mut max = 0;
    let width = tiles.width;
    let height = tiles.height;
    let mut find_max = |p: &Pace| {
        route.clear();
        step(&p.point, &p.dir, &tiles, &mut route);
        let n = count(&route);
        dbg!(p, n);
        if n > max {
            max = n;
        }
    };
    for x in 0..width {
        let top_start = Pace {
            point: Point(x, -1),
            dir: Dir::Ttb,
        };
        let bottom_start = Pace {
            point: Point(x, height),
            dir: Dir::Btt,
        };
        find_max(&top_start);
        find_max(&bottom_start);
    }
    for y in 0..height {
        let left_start = Pace {
            point: Point(-1, y),
            dir: Dir::Ltr,
        };
        find_max(&left_start);
        let right_start = Pace {
            point: Point(width, y),
            dir: Dir::Rtl,
        };
        find_max(&right_start);
    }
    max
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
        ),
        51
    );
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input16.txt");
    println!("{}", ans2(&q));
}
