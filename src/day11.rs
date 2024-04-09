use crate::common::Matrix;

pub fn ans1(input: &str) -> u64 {
    let mut m = Matrix::new(input);
    expand(&mut m);
    let mut galaxies = vec![];
    for y in 0..m.height {
        for x in 0..m.width {
            if m.get_point_char(x, y).unwrap() == '#' {
                galaxies.push((x, y));
            }
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in 0..i {
            let from = galaxies[i];
            let to = galaxies[j];
            let dis = from.0.abs_diff(to.0) + from.1.abs_diff(to.1);
            sum += dis;
        }
    }
    sum
}

fn expand(m: &mut Matrix) {
    let mut y = 0;
    let x_count = m.data[0].len();
    while y < m.data.len() {
        if m.data[y].contains('#') {
            y += 1;
        } else {
            m.data.insert(y, ".".repeat(x_count));
            y += 2;
        }
    }
    let mut x = 0;
    while x < m.data[0].len() {
        if m.data.iter().any(|l| l.chars().nth(x).unwrap() == '#') {
            x += 1;
        } else {
            for l in m.data.iter_mut() {
                l.insert(x, '.');
            }
            x += 2;
        }
    }
    m.height = m.data.len() as i64;
    m.width = m.data[0].len() as i64;
}

#[test]
fn run1() {
    println!("{}", ans1(include_str!("../inputs/input11.txt")))
}

pub fn ans2(input: &str, factor: i64) -> u64 {
    let mut m = Matrix::new(input);
    let (xs, ys) = expand2(&mut m);

    let mut galaxies = vec![];
    for y in 0..m.height {
        for x in 0..m.width {
            if m.get_point_char(x, y).unwrap() == '#' {
                galaxies.push((x, y));
            }
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in 0..i {
            let from = galaxies[i];
            let to = galaxies[j];
            let dis = dis(from.0, to.0, &xs, factor) + dis(from.1, to.1, &ys, factor);
            sum += dis;
        }
    }
    sum
}

fn dis(n1: i64, n2: i64, ns: &[i64], f: i64) -> u64 {
    let expand_count = ns
        .iter()
        .filter(|n| (n > &&n1 && n < &&n2) || (n > &&n2 && n < &&n1))
        .count() as u64;

    n1.abs_diff(n2) - expand_count + (expand_count * f as u64)
}

fn expand2(m: &Matrix) -> (Vec<i64>, Vec<i64>) {
    let mut ys = vec![];
    for y in 0..m.data.len() {
        if !m.data[y].contains('#') {
            ys.push(y as i64);
        }
    }
    let mut xs = vec![];
    for x in 0..m.data[0].len() {
        if !m.data.iter().any(|l| l.chars().nth(x).unwrap() == '#') {
            xs.push(x as i64)
        }
    }

    (xs, ys)
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#,
            10
        ),
        1030
    );

    assert_eq!(
        ans2(
            r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#,
            100
        ),
        8410
    );
}

#[test]
fn run2() {
    println!("{}", ans2(include_str!("../inputs/input11.txt"), 1000000));
}
