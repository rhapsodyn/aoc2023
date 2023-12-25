use crate::common::read_until;

pub fn ans1(input: &str) -> u32 {
    let cards = parse(input);
    let mut point = 0;
    for (l, r) in cards {
        let mut p = 0;
        for have in r {
            if l.contains(&have) {
                if p == 0 {
                    p = 1;
                } else {
                    p *= 2;
                }
            }
        }
        point += p;
    }
    point
}

pub fn ans2(input: &str) -> u32 {
    let cards = parse(input);
    let mut copies = vec![1; cards.len()];
    for (i, (l, r)) in cards.iter().enumerate() {
        let mut p = 0;
        for have in r {
            if l.contains(&have) {
                p += 1;
            }
        }

        if p == 0 {
            continue;
        }

        for j in 0..p {
            copies[i + j + 1] += copies[i];
        }
    }

    dbg!(&copies);
    copies.iter().sum()
}

fn parse(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    let mut cards = vec![];
    let lines = input.split('\n').filter(|l| l.len() > 0);
    for l in lines {
        let i = read_until(l, 0, |c| c == ':').unwrap();

        let j = read_until(l, i, |c| c == '|').unwrap();
        let first_pat = &l[i + 1..j];
        let winning = first_pat
            .split(' ')
            .filter(|p| p.len() > 0)
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let k = read_until(l, j, |_c| false).unwrap();
        let second_pat = &l[j + 1..k];
        let having = second_pat
            .split(' ')
            .filter(|p| p.len() > 0)
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        cards.push((winning, having));
    }
    // dbg!(&cards);
    cards
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
        ),
        13
    )
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input4.txt");
    println!("{}", ans1(&q));
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
        ),
        30
    )
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input4.txt");
    println!("{}", ans2(&q));
}