use std::{cmp::Ordering, collections::HashMap};

type Num = u32;

#[derive(Debug)]
struct Hand {
    label: String,
    bid: Num,
}

pub fn ans1(input: &str) -> Num {
    let mut hands = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            let mut seg = l.split(' ');
            let label = seg.next().unwrap().to_string();
            let bid = seg.next().unwrap().parse::<Num>().unwrap();
            Hand { label, bid }
        })
        .collect::<Vec<_>>();
    hands.sort_by(compare);
    // dbg!(&hands);
    let mut ans = 0;
    for (i, h) in hands.iter().enumerate() {
        ans += h.bid * (i as Num + 1);
    }
    ans
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    HighCard = 0,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

const PRI: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn ch_pri(c: &char) -> usize {
    for (i, p) in PRI.iter().enumerate() {
        if p == c {
            return PRI.len() - i;
        }
    }
    unreachable!()
}

const PRI2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn ch_pri2(c: &char) -> usize {
    for (i, p) in PRI2.iter().enumerate() {
        if p == c {
            return PRI2.len() - i;
        }
    }
    unreachable!()
}

fn kind(label: &str) -> Kind {
    let mut dup = HashMap::<char, u8>::new();
    for ch in label.chars() {
        if dup.contains_key(&ch) {
            *dup.get_mut(&ch).unwrap() += 1;
        } else {
            dup.insert(ch, 1);
        }
    }
    match dup.len() {
        1 => Kind::Five,
        2 => {
            if dup.values().any(|v| v == &4) {
                Kind::Four
            } else {
                Kind::FullHouse
            }
        }
        3 => {
            if dup.values().any(|v| v == &3) {
                Kind::Three
            } else {
                Kind::TwoPair
            }
        }
        4 => Kind::OnePair,
        5 => Kind::HighCard,
        _ => unreachable!(),
    }
}

fn compare(l: &Hand, r: &Hand) -> Ordering {
    let left_label = &l.label;
    let right_label = &r.label;
    assert_eq!(left_label.len(), right_label.len());
    let left_kind = kind(left_label);
    let right_kind = kind(right_label);
    // dbg!(&l, &left_kind);
    if left_kind != right_kind {
        left_kind.cmp(&right_kind)
    } else {
        if left_label == right_label {
            Ordering::Equal
        } else {
            let l_chars: Vec<char> = left_label.chars().collect();
            let r_chars: Vec<char> = right_label.chars().collect();
            for i in 0..left_label.len() {
                let l_ch = l_chars[i];
                let r_ch = r_chars[i];
                if l_ch != r_ch {
                    return ch_pri(&l_ch).cmp(&ch_pri(&r_ch));
                }
            }
            unreachable!()
        }
    }
}

pub fn ans2(input: &str) -> Num {
    let mut hands = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            let mut seg = l.split(' ');
            let label = seg.next().unwrap().to_string();
            let bid = seg.next().unwrap().parse::<Num>().unwrap();
            Hand { label, bid }
        })
        .collect::<Vec<_>>();
    hands.sort_by(compare2);
    // dbg!(&hands);
    let mut ans = 0;
    for (i, h) in hands.iter().enumerate() {
        ans += h.bid * (i as Num + 1);
    }
    ans
}

fn compare2(l: &Hand, r: &Hand) -> Ordering {
    let left_label = &l.label;
    let right_label = &r.label;
    assert_eq!(left_label.len(), right_label.len());
    let left_kind = kind2(left_label);
    let right_kind = kind2(right_label);
    // dbg!(&l, &left_kind);
    if left_kind != right_kind {
        left_kind.cmp(&right_kind)
    } else {
        if left_label == right_label {
            Ordering::Equal
        } else {
            let l_chars: Vec<char> = left_label.chars().collect();
            let r_chars: Vec<char> = right_label.chars().collect();
            for i in 0..left_label.len() {
                let l_ch = l_chars[i];
                let r_ch = r_chars[i];
                if l_ch != r_ch {
                    return ch_pri2(&l_ch).cmp(&ch_pri2(&r_ch));
                }
            }
            unreachable!()
        }
    }
}

#[test]
fn test_cmp2() {
    let h1 = Hand {
        label: "T55J5".to_string(),
        bid: 684,
    };
    let h2 = Hand {
        label: "QQQJA".to_string(),
        bid: 483,
    };

    assert_eq!(compare2(&h2, &h1), Ordering::Greater);
}

fn kind2(label: &str) -> Kind {
    let mut dup = HashMap::<char, u8>::new();
    for ch in label.chars() {
        if dup.contains_key(&ch) {
            *dup.get_mut(&ch).unwrap() += 1;
        } else {
            dup.insert(ch, 1);
        }
    }
    // joker thing:
    let tmp = dup.get(&'J').cloned();
    if let Some(jv) = tmp {
        if jv != 5 {
            // not JJJJJ
            dup.remove(&'J');
            let mut max_kv = ('J', 0);
            for (k, v) in dup.iter() {
                if v > &max_kv.1 {
                    max_kv = (*k, *v);
                }
            }

            *dup.get_mut(&max_kv.0).unwrap() += jv;
        }
    }
    // dbg!(&dup);
    match dup.len() {
        1 => Kind::Five,
        2 => {
            if dup.values().any(|v| v == &4) {
                Kind::Four
            } else {
                Kind::FullHouse
            }
        }
        3 => {
            if dup.values().any(|v| v == &3) {
                Kind::Three
            } else {
                Kind::TwoPair
            }
        }
        4 => Kind::OnePair,
        5 => Kind::HighCard,
        _ => unreachable!(),
    }
}

#[test]
fn test_kind2() {
    assert_eq!(kind2("KTJJT"), Kind::Four);
}

#[test]
fn run1() {
    // let q = include_str!("../inputs/input7.txt");
    // println!("{}", ans1(&q));
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#
        ),
        5905
    )
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input7.txt");
    println!("ans2: {}", ans2(&q));
}
