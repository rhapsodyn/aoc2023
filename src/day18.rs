type Num = i64;

#[derive(Debug)]
struct Inst {
    dir: Dir,
    len: Num,
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Point(Num, Num);

#[derive(Debug)]
enum VH {
    Vertical,
    Horizontal,
}

/// non-directional
#[derive(Debug)]
struct Line {
    vh: VH,
    start: Point,
    end: Point,
}

impl Line {
    fn x_proj(&self) -> Num {
        match self.vh {
            VH::Vertical => 1,
            VH::Horizontal => self.start.0.abs_diff(self.end.0) as Num + 1,
        }
    }
}

fn shoelace(vertices: &[Point]) -> Num {
    let mut sum = 0;
    for i in 0..vertices.len() - 1 {
        let p1 = &vertices[i];
        let p2 = &vertices[i + 1];
        sum += p1.0 * p2.1 - p1.1 * p2.0;
    }

    sum
}

pub fn ans1(input: &str) -> Num {
    let mut insts = vec![];
    for l in input.lines() {
        let mut seg = l.split(" ");
        let dir = match seg.next().unwrap() {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => unreachable!(),
        };
        let len = seg.next().unwrap().parse().unwrap();
        insts.push(Inst { dir, len });
    }
    //dbg!(&insts);

    in_my_shoes(insts)
}

fn in_my_shoes(insts: Vec<Inst>) -> i64 {
    // first == last, intentionally
    let mut vertices = vec![Point(0, 0)];
    let mut perimeter = 0;
    for inst in insts {
        let last = vertices.last().unwrap();
        let curr = match inst.dir {
            Dir::Left => Point(last.0 - inst.len, last.1),
            Dir::Right => Point(last.0 + inst.len, last.1),
            Dir::Up => Point(last.0, last.1 - inst.len),
            Dir::Down => Point(last.0, last.1 + inst.len),
        };
        vertices.push(curr);
        perimeter += inst.len;
    }
    //dbg!(&vertices);
    //dbg!(perimeter);

    // 1 for starter
    (shoelace(&vertices) + perimeter) / 2 + 1
}

#[allow(dead_code)]
fn _ans1(input: &str) -> Num {
    let mut insts = vec![];
    for l in input.lines() {
        let mut seg = l.split(" ");
        let dir = match seg.next().unwrap() {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => unreachable!(),
        };
        let len = seg.next().unwrap().parse().unwrap();
        insts.push(Inst { dir, len });
    }
    //dbg!(&insts);

    let mut cursor = Point(0, 0);
    let mut lines = vec![];
    for i in insts {
        //dbg!(&i, &cursor);
        let (s, e, vh) = match i.dir {
            Dir::Left => (
                Point(cursor.0 - 1, cursor.1),
                Point(cursor.0 - i.len, cursor.1),
                VH::Horizontal,
            ),
            Dir::Right => (
                Point(cursor.0 + 1, cursor.1),
                Point(cursor.0 + i.len, cursor.1),
                VH::Horizontal,
            ),
            Dir::Up => (
                Point(cursor.0, cursor.1 - 1),
                Point(cursor.0, cursor.1 - i.len),
                VH::Vertical,
            ),
            Dir::Down => (
                Point(cursor.0, cursor.1 + 1),
                Point(cursor.0, cursor.1 + i.len),
                VH::Vertical,
            ),
        };
        cursor = e.clone();
        // easy to cmp
        let (start, end) = if s.0 < e.0 || s.1 < e.1 {
            (s, e)
        } else {
            (e, s)
        };
        lines.push(Line { start, end, vh });
    }
    //dbg!(&lines);

    let mut max_y = 0;
    let mut min_y = 0;
    let mut x1 = 0;
    let mut x2 = 0;
    for l in lines.iter() {
        let y_bigger = l.end.1.max(l.start.1);
        let y_smaller = l.end.1.min(l.start.1);
        if y_bigger > max_y {
            max_y = y_bigger
        }
        if y_smaller < min_y {
            min_y = y_smaller
        }
        let x_b = l.end.0.max(l.start.0);
        let x_s = l.end.0.min(l.start.0);
        if x_b > x1 {
            x1 = x_b;
        }
        if x_s < x2 {
            x2 = x_b;
        }
    }
    dbg!(x1, x2);

    let mut sum = 0;
    for y in min_y..max_y + 1 {
        //dbg!(y);
        let mut intersections = vec![];
        let mut x_sum = 0;
        for l in lines.iter() {
            match l.vh {
                VH::Vertical => {
                    let y1 = l.start.1.min(l.end.1);
                    let y2 = l.start.1.max(l.end.1);
                    if y >= y1 && y <= y2 {
                        intersections.push(l);
                    }
                }
                VH::Horizontal => {
                    if l.start.1 == y {
                        intersections.push(l);
                    }
                }
            }
        }
        // from left to right
        intersections.sort_by(|l, r| l.start.0.cmp(&r.start.0));
        //println!("intersections = {:?}", &intersections);
        let mut interior = true;
        for i in 0..intersections.len() - 1 {
            let left = intersections[i];
            let right = intersections[i + 1];
            x_sum += left.x_proj();
            let x_gap = right.start.0 - left.end.0 - 1;
            //dbg!(x_gap);
            if interior {
                x_sum += x_gap;
            }
            if x_gap > 0 {
                interior = !interior;
            }
        }
        x_sum += intersections.last().unwrap().x_proj();
        println!("y:{} sum:{}", y, x_sum);
        sum += x_sum;
    }

    sum
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#
        ),
        62
    );
}

//#[test]
//fn test1_1() {
//    assert_eq!(
//        ans1(
//            r#"R 2 ()
//D 2 ()
//R 2 ()
//U 2 ()
//R 2 ()
//D 4 ()
//L 5 ()
//U 3 ()"#
//        ),
//        28
//    );
//}

#[test]
fn run1() {
    let q = include_str!("../inputs/input18.txt");
    println!("{}", ans1(&q));
}

pub fn ans2(input: &str) -> Num {
    let mut insts = vec![];
    for l in input.lines() {
        let seg = l.split(" ");
        let mut rbg = seg.last().unwrap();
        rbg = &rbg[2..rbg.len() - 1];
        //dbg!(rbg);

        let len = i64::from_str_radix(&rbg[..rbg.len() - 1], 16).unwrap();
        let dir = match rbg.chars().last().unwrap() {
            '0' => Dir::Right,
            '1' => Dir::Down,
            '2' => Dir::Left,
            '3' => Dir::Up,
            _ => unreachable!(),
        };
        //dbg!(len, &dir);

        insts.push(Inst { dir, len });
    }

    in_my_shoes(insts)
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#
        ),
        952408144115
    );
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input18.txt");
    println!("{}", ans2(&q));
}
