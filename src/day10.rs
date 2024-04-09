use std::collections::HashMap;

use crate::common::Matrix;

type Pos = (i64, i64);

pub fn ans1(input: &str) -> usize {
    let mat = Matrix::new(input);
    let start = get_start(&mat);
    let mut path = vec![];
    // CHEAT: always go down
    let mut curr = (start.0, start.1 + 1);
    let mut prev = start.clone();
    // dbg!(&prev, &curr);

    while curr != start {
        path.push(curr.clone());
        let tmp = curr.clone();
        curr = next(&mat, &prev, &curr);
        prev = tmp;

        // dbg!(&prev, &curr);
    }

    (path.len() + 1) / 2
}

fn next(m: &Matrix, prev: &Pos, curr: &Pos) -> Pos {
    // let p_ch = m.get_point_char(prev.0, prev.1).unwrap();
    let c_ch = m.get_point_char(curr.0, curr.1).unwrap();
    // let p_dir = ch2dir(&p_ch);
    // let c_dir = ch2dir(&c_ch);
    //
    // let next_dir = if p_dir.contains(&c_dir[0]) {
    //     &c_dir[1]
    // } else {
    //     &c_dir[0]
    // };
    // dbg!(&p_ch, &c_ch, &next_dir);
    //
    // match next_dir {
    //     Direction::Up => (curr.0, curr.1 - 1),
    //     Direction::Down => (curr.0, curr.1 + 1),
    //     Direction::Left => (curr.0 - 1, curr.1),
    //     Direction::Right => (curr.0 + 1, curr.1),
    // }

    // dbg!(&prev);
    // dbg!(&curr);
    let next_dir = match c_ch {
        '|' => {
            if prev.1 < curr.1 {
                // from top
                Dir::Down
            } else {
                Dir::Up
            }
        }
        '-' => {
            if prev.0 < curr.0 {
                // from left
                Dir::Right
            } else {
                Dir::Left
            }
        }
        'L' => {
            if prev.1 < curr.1 {
                // from top
                Dir::Right
            } else {
                Dir::Up
            }
        }
        'J' => {
            if prev.1 < curr.1 {
                // from top
                Dir::Left
            } else {
                Dir::Up
            }
        }
        '7' => {
            if prev.1 > curr.1 {
                // from bottom
                Dir::Left
            } else {
                Dir::Down
            }
        }
        'F' => {
            if prev.1 > curr.1 {
                // from bottom
                Dir::Right
            } else {
                Dir::Down
            }
        }
        _ => unreachable!(),
    };
    // dbg!(&p_ch, &c_ch, &next_dir);
    match next_dir {
        Dir::Up => (curr.0, curr.1 - 1),
        Dir::Down => (curr.0, curr.1 + 1),
        Dir::Left => (curr.0 - 1, curr.1),
        Dir::Right => (curr.0 + 1, curr.1),
        Dir::None => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    None,
}

// fn ch2dir(ch: &char) -> [Dir; 2] {
//     match ch {
//         '|' => [Dir::Up, Dir::Down],
//         '-' => [Dir::Left, Dir::Right],
//         'L' => [Dir::Up, Dir::Right],
//         'J' => [Dir::Up, Dir::Left],
//         '7' => [Dir::Left, Dir::Down],
//         'F' => [Dir::Right, Dir::Down],
//         // CHEAT
//         'S' => [Dir::Down, Dir::Right],
//         _ => unreachable!("ch: {}", ch),
//     }
// }

fn get_start(m: &Matrix) -> Pos {
    for y in 0..m.height {
        for x in 0..m.width {
            if m.get_point_char(x, y).unwrap() == 'S' {
                return (x, y);
            }
        }
    }

    unreachable!()
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#
        ),
        8
    )
}

#[test]
fn run1() {
    println!("{}", ans1(include_str!("../inputs/input10.txt")))
}

pub fn ans2(input: &str) -> usize {
    let mat = Matrix::new(input);
    let start = Vertex {
        pos: get_start(&mat),
        front_face: [Dir::None, Dir::None],
        // CHEAT: should be calc, but emmmmmmm
        ch: 'F',
        toward: Dir::None,
    };
    // CHEAT: always go down
    let mut curr = Vertex {
        pos: (start.pos.0, start.pos.1 + 1),
        front_face: [Dir::None, Dir::None],
        ch: mat.get_point_char(start.pos.0, start.pos.1 + 1).unwrap(),
        toward: Dir::Down,
    };
    let start_pos = start.pos.clone();
    let mut loop_path = vec![start];

    while curr.pos != start_pos {
        // println!(
        //     "curr: {:?} {:?}",
        //     &curr.pos,
        //     mat.get_point_char(curr.pos.0, curr.pos.1)
        // );
        let next = next2(&mat, loop_path.last().unwrap(), &mut curr);
        loop_path.push(curr);
        curr = next;
    }

    // println!("{:?}", &loop_path);
    let mut loop_map = HashMap::new();
    for v in loop_path.into_iter() {
        loop_map.insert(v.pos, v);
    }

    let mut inner_count = 0;
    for y in 0..mat.height {
        let mut inner = false;
        let mut starting_ch = ' ';
        for x in 0..mat.width {
            let p = (x, y);
            println!("p: {:?}", &p);
            if let Some(e) = loop_map.get(&p) {
                println!("pipe: {}", e.ch);
                // '|' | 'F--J' | 'L--7' => switch
                // 'F--7' | 'L--J' => no affect
                match e.ch {
                    '|' => inner = !inner,
                    'F' | 'L' => starting_ch = e.ch,
                    'J' => {
                        if starting_ch == 'F' {
                            inner = !inner;
                        }
                    }
                    '7' => {
                        if starting_ch == 'L' {
                            inner = !inner;
                        }
                    }
                    _ => {}
                }
            } else {
                if inner {
                    println!("found: {:?}", &p);
                    inner_count += 1;
                }
            }
            println!("inner: {}", inner);
        }
    }

    inner_count

    // let mut inner_count = 0;
    // let max = (mat.width - 1, mat.height - 1);
    // for y in 0..mat.height {
    //     for x in 0..mat.width {
    //         let p = (x, y);
    //         // dbg!(&p);
    //         // println!("p: {:?}", &p);
    //         if loop_map.contains_key(&p) {
    //             // dbg!("is edge");
    //             continue;
    //         } else {
    //             if ray_test(&loop_map, p, max) {
    //                 // if winding_number(&loop_map, p, max) {
    //                 println!("inner: {:?}", p);
    //                 inner_count += 1;
    //             } else {
    //                 println!("ouuter: {:?}", p);
    //             }
    //         }
    //
    //         // if reach_border_or_backface(&loop_map, p, max) {
    //         //     println!("reach_border_or_backface: {:?}", p);
    //         //     outter_count += 1;
    //         // } else {
    //         //     println!("inner: {:?}", p);
    //         // }
    //     }
    // }
    //
    // inner_count

    // dbg!(total, loop_map.len(), outter_count);
    // total as usize - loop_map.len() - outter_count
    // loop_map.len() / 2
}

#[allow(dead_code)]
fn winding_number(edges: &HashMap<(i64, i64), Vertex>, p: (i64, i64), max: (i64, i64)) -> bool {
    let mut number = 0;
    // a horizontal ray
    for x in (p.0 + 1)..max.0 {
        if let Some(node) = edges.get(&(x, p.1)) {
            if p.1 == 1 {
                dbg!(&node);
            }
            if node.toward == Dir::Up {
                number += 1
            } else if node.toward == Dir::Down {
                number -= 1
            }
        }
    }

    number > 0
}

#[allow(dead_code)]
fn ray_test(edges: &HashMap<(i64, i64), Vertex>, p: (i64, i64), max: (i64, i64)) -> bool {
    let mut hit = 0;
    // a horizontal ray
    for x in (p.0 + 1)..max.0 {
        if let Some(node) = edges.get(&(x, p.1)) {
            // if node.ch == '|' {
            if ['|', 'L', 'J'].contains(&node.ch) {
                hit += 1;
            }
        }
    }

    if p == (3, 2) {
        dbg!(&hit);
    }

    (hit % 2) == 1
}

#[allow(dead_code)]
fn reach_border_or_backface(
    loop_map: &HashMap<(i64, i64), [Dir; 2]>,
    p: (i64, i64),
    max: (i64, i64),
) -> bool {
    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
        let mut cursor = p.clone();
        loop {
            cursor = match dir {
                Dir::Up => (cursor.0, cursor.1 - 1),
                Dir::Down => (cursor.0, cursor.1 + 1),
                Dir::Left => (cursor.0 - 1, cursor.1),
                Dir::Right => (cursor.0 + 1, cursor.1),
                Dir::None => unreachable!(),
            };

            if cursor.0 < 0 || cursor.0 > max.0 || cursor.1 < 0 || cursor.1 > max.1 {
                // outof border
                return true;
            }

            if let Some(brick) = loop_map.get(&cursor) {
                if p == (3, 2) {
                    dbg!(cursor, brick);
                }
                if brick.contains(&dir) {
                    // hit backface
                    return true;
                } else {
                    // change direction
                    break;
                }
            }
        }
    }

    false
}

#[derive(Debug, Clone)]
struct Vertex {
    pos: Pos,
    front_face: [Dir; 2],
    ch: char,
    toward: Dir,
}

fn next2(m: &Matrix, prev: &Vertex, curr: &mut Vertex) -> Vertex {
    let c_ch = m.get_point_char(curr.pos.0, curr.pos.1).unwrap();
    // dbg!(&prev.pos);
    // dbg!(&curr.pos);
    // let mut toward = Dir::None;
    let next_dir = match c_ch {
        '|' => {
            if prev.pos.1 < curr.pos.1 {
                // from top
                curr.front_face = [Dir::Right, Dir::Right];
                curr.toward = Dir::Down;
                // toward = Dir::Down;
                Dir::Down
            } else {
                curr.front_face = [Dir::Left, Dir::Left];
                curr.toward = Dir::Up;
                // toward = Dir::Up;
                Dir::Up
            }
        }
        '-' => {
            if prev.pos.0 < curr.pos.0 {
                // from left
                curr.front_face = [Dir::Up, Dir::Up];
                Dir::Right
            } else {
                curr.front_face = [Dir::Down, Dir::Down];
                Dir::Left
            }
        }
        'L' => {
            if prev.pos.1 < curr.pos.1 {
                // from top
                curr.front_face = [Dir::Right, Dir::Up];
                curr.toward = Dir::Down;
                Dir::Right
            } else {
                curr.front_face = [Dir::Down, Dir::Left];
                curr.toward = Dir::Up;
                Dir::Up
            }
        }
        'J' => {
            if prev.pos.1 < curr.pos.1 {
                // from top
                curr.front_face = [Dir::Right, Dir::Down];
                curr.toward = Dir::Down;
                Dir::Left
            } else {
                curr.front_face = [Dir::Up, Dir::Left];
                curr.toward = Dir::Up;
                Dir::Up
            }
        }
        '7' => {
            if prev.pos.1 > curr.pos.1 {
                // from bottom
                curr.front_face = [Dir::Left, Dir::Down];
                curr.toward = Dir::Up;
                Dir::Left
            } else {
                curr.front_face = [Dir::Up, Dir::Right];
                curr.toward = Dir::Down;
                Dir::Down
            }
        }
        'F' => {
            if prev.pos.1 > curr.pos.1 {
                // from bottom
                curr.front_face = [Dir::Left, Dir::Up];
                curr.toward = Dir::Up;
                Dir::Right
            } else {
                curr.front_face = [Dir::Down, Dir::Right];
                curr.toward = Dir::Down;
                Dir::Down
            }
        }
        _ => unreachable!(),
    };
    // dbg!(&curr);

    // dbg!(&p_ch, &c_ch, &next_dir);
    let pos = match next_dir {
        Dir::Up => (curr.pos.0, curr.pos.1 - 1),
        Dir::Down => (curr.pos.0, curr.pos.1 + 1),
        Dir::Left => (curr.pos.0 - 1, curr.pos.1),
        Dir::Right => (curr.pos.0 + 1, curr.pos.1),
        Dir::None => unreachable!(),
    };

    Vertex {
        pos,
        front_face: [Dir::None, Dir::None],
        ch: m.get_point_char(pos.0, pos.1).unwrap(),
        toward: Dir::None,
    }
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
"#
        ),
        4
    );

    assert_eq!(
        ans2(
            r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
        "#
        ),
        8
    );
}

#[test]
fn run2() {
    println!("{}", ans2(include_str!("../inputs/input10.txt")))
}
