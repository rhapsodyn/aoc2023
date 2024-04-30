type Map = Vec<Vec<char>>;

const DEBUG: bool = true;

pub fn ans1(input: &str) -> usize {
    let mut input_map = vec![];
    for l in input.split('\n') {
        if l.len() > 0 {
            input_map.push(l.chars().collect::<Vec<char>>());
        }
    }
    pretty(&input_map);

    let rolled = roll_north(&input_map);
    pretty(&rolled);

    score(&rolled)
}

fn score(m: &Map) -> usize {
    let mut a = 0;
    for (i, l) in m.iter().enumerate() {
        let o_count = l.iter().filter(|c| c == &&'O').count();
        a += (m.len() - i) * o_count;
    }
    a
}

fn pretty(m: &Map) {
    if DEBUG {
        println!();
        for l in m {
            for c in l {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn roll_north(map: &Map) -> Map {
    let mut rolled = map.clone();
    let width = map[0].len();
    let height = map.len();

    for x in 0..width {
        // col by col, left to right
        for y in 1..height {
            if rolled[y][x] == 'O' {
                // only rock matters
                // row by row, top to bottom
                let mut next = y;
                // == 0 => top row
                while next > 0 {
                    next -= 1;
                    match rolled[next][x] {
                        '.' => {
                            // go ahead
                            continue;
                        }
                        '#' | 'O' => {
                            // meet barrier
                            // go back
                            next += 1;
                            break;
                        }
                        _ => unreachable!(),
                    }
                }

                if next != y {
                    // swap
                    rolled[next][x] = 'O';
                    rolled[y][x] = '.';
                }
            }
        }
    }

    rolled
}

pub fn ans2(input: &str) -> usize {
    let mut map = vec![];
    for l in input.split('\n') {
        if l.len() > 0 {
            map.push(l.chars().collect::<Vec<char>>());
        }
    }

    let mut history = vec![];
    let n = 1000000000;
    let mut x = 0;
    let mut y = 0;
    for i in 0..n {
        roll2(&mut map);
        let s = score(&map);
        // pretty(&map);
        // println!("{}:{}", i, s);
        history.push(s);
        if i > 5 {
            if let Some(n) = loop_array(&history) {
                y = n;
                x = history.len() - 2 * n;
                break;
            }
        }
    }

    let i = (n - 1 - x) % y;
    dbg!(x, y, i, &history);
    history[x + i]
}

fn loop_array(arr: &Vec<usize>) -> Option<usize> {
    let len = arr.len();
    for loop_n in 5..len {
        let mut ok = true;
        for i in 1..loop_n + 1 {
            if arr[len - i] != arr[len - i - loop_n] {
                ok = false;
                break;
            }
        }

        if ok {
            return Some(loop_n);
        }
    }

    None
}

///
/// roll north, then west, then south, then east
///
fn roll2(map: &mut Map) {
    let width = map[0].len();
    let height = map.len();
    // north
    for x in 0..width {
        // col by col, left to right
        for y in 1..height {
            if map[y][x] == 'O' {
                // only rock matters
                // row by row, top to bottom
                let mut next = y;
                // == 0 => top row
                while next > 0 {
                    next -= 1;
                    match map[next][x] {
                        '.' => {
                            // go ahead
                            continue;
                        }
                        '#' | 'O' => {
                            // meet barrier
                            // go back
                            next += 1;
                            break;
                        }
                        _ => unreachable!(),
                    }
                }

                if next != y {
                    // swap
                    map[next][x] = 'O';
                    map[y][x] = '.';
                }
            }
        }
    }
    // west
    for y in 0..height {
        // row by row, up to bottom
        for x in 1..width {
            // col by col, left to right
            if map[y][x] == 'O' {
                let mut next_x = x;
                while next_x > 0 {
                    // roll to left
                    next_x -= 1;
                    match map[y][next_x] {
                        '.' => {
                            // go ahead
                            continue;
                        }
                        '#' | 'O' => {
                            // meet barrier
                            // go back
                            next_x += 1;
                            break;
                        }
                        _ => unreachable!(),
                    }
                }

                if next_x != x {
                    // swap
                    map[y][next_x] = 'O';
                    map[y][x] = '.';
                }
            }
        }
    }
    // south
    for x in 0..width {
        // col by col, left to right
        for y in (0..height - 1).rev() {
            // row by row, bottom to top
            if map[y][x] == 'O' {
                let mut next_y = y;
                while next_y < height - 1 {
                    // roll to bottom
                    next_y += 1;
                    match map[next_y][x] {
                        '.' => {
                            // go ahead
                            continue;
                        }
                        '#' | 'O' => {
                            // meet barrier
                            // go back
                            next_y -= 1;
                            break;
                        }
                        _ => unreachable!(),
                    }
                }

                if next_y != y {
                    // swap
                    map[next_y][x] = 'O';
                    map[y][x] = '.';
                }
            }
        }
    }
    // east
    for y in 0..height {
        // row by row, top to bottom
        for x in (0..width - 1).rev() {
            // col by col, right to left
            if map[y][x] == 'O' {
                let mut next_x = x;
                while next_x < width - 1 {
                    // roll to right
                    next_x += 1;
                    match map[y][next_x] {
                        '.' => {
                            // go ahead
                            continue;
                        }
                        '#' | 'O' => {
                            // meet barrier
                            // go back
                            next_x -= 1;
                            break;
                        }
                        _ => unreachable!(),
                    }
                }

                if next_x != x {
                    // swap
                    map[y][next_x] = 'O';
                    map[y][x] = '.';
                }
            }
        }
    }
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#
        ),
        136
    );
}

#[test]
fn test2() {
    let a = ans2(
        r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#,
    );
    assert_eq!(a, 64);
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input14.txt");
    println!("{}", ans1(&q));
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input14.txt");
    println!("{}", ans2(&q));
}
