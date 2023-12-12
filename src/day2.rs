use crate::common::read_until;

pub fn ans(q: &str) -> u64 {
    let lines = q.split('\n');
    let mut total = 0;
    for l in lines {
        let mut begin = read_until(l, 5, |c| c == ':').unwrap();
        let id = &l[5..begin].parse::<u64>().unwrap();
        // dbg!(id);

        begin += 1;
        let mut possible = true;
        while let Some(end) = read_until(l, begin, |c| c == ';') {
            let set = &l[begin..end];
            // dbg!(set);

            let mut begin2 = 1;
            while let Some(end2) = read_until(set, begin2, |c| c == ',') {
                let subset = &set[begin2..end2];
                // dbg!(subset);
                let mut sp = subset.split(' ');
                let dig = sp.next().unwrap().parse::<u64>().unwrap();
                let color = sp.next().unwrap();
                match color {
                    "red" => {
                        if dig > 12 {
                            possible = false;
                            break;
                        }
                    }
                    "green" => {
                        if dig > 13 {
                            possible = false;
                            break;
                        }
                    }
                    "blue" => {
                        if dig > 14 {
                            possible = false;
                            break;
                        }
                    }
                    _ => unreachable!("color err"),
                }

                begin2 = end2 + 2;
            }

            if !possible {
                break;
            }

            begin = end + 1;
        }

        if possible {
            total += id
        }
    }

    total
}

pub fn ans2(q: &str) -> u64 {
    let lines = q.split('\n');
    let mut total = 0;
    for l in lines {
        let mut begin = read_until(l, 5, |c| c == ':').unwrap();
        // let id = &l[5..begin].parse::<u64>().unwrap();
        // dbg!(id);

        begin += 1;
        let (mut r, mut g, mut b) = (1, 1, 1);
        while let Some(end) = read_until(l, begin, |c| c == ';') {
            let set = &l[begin..end];
            // dbg!(set);

            let mut begin2 = 1;
            while let Some(end2) = read_until(set, begin2, |c| c == ',') {
                let subset = &set[begin2..end2];
                // dbg!(subset);
                let mut sp = subset.split(' ');
                let dig = sp.next().unwrap().parse::<u64>().unwrap();
                let color = sp.next().unwrap();
                match color {
                    "red" => {
                        if dig > r {
                            r = dig;
                        }
                    }
                    "green" => {
                        if dig > g {
                            g = dig;
                        }
                    }
                    "blue" => {
                        if dig > b {
                            b = dig;
                        }
                    }
                    _ => unreachable!("color err"),
                }

                begin2 = end2 + 2;
            }

            begin = end + 1;
        }

        // dbg!(r, g, b);
        total += r * g * b;
    }

    total
}

#[test]
fn test() {
    let a = ans(r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#);
    assert_eq!(a, 8)
}

#[test]
fn run() {
    let q = include_str!("../inputs/input2.txt");
    println!("{}", ans(&q));
}

#[test]
fn test2() {
    let a = ans2(
        r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
    );
    assert_eq!(a, 2286)
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input2.txt");
    println!("{}", ans2(&q));
}