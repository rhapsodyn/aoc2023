pub fn ans(input: &str) -> u64 {
    let lines = input.split('\n');
    let mut total = 0;
    for l in lines {
        let (mut a, mut b) = (None, None);
        for i in 0..l.len() {
            let n = to_num(l, i);
            if n.is_some() {
                if a.is_none() {
                    a = n;
                } else {
                    b = n;
                }
            }
        }
        match (a, b) {
            (Some(i), Some(j)) => total += i * 10 + j,
            (Some(i), None) => total += i * 10 + i,
            _ => {}
        }
    }
    total
}
fn to_num(s: &str, i: usize) -> Option<u64> {
    if i >= 4 {
        match &s[i - 4..i + 1] {
            "three" => return Some(3),
            "seven" => return Some(7),
            "eight" => return Some(8),
            _ => {}
        }
    }

    if i >= 3 {
        match &s[i - 3..i + 1] {
            "four" => return Some(4),
            "five" => return Some(5),
            "nine" => return Some(9),
            _ => {}
        }
    }

    if i >= 2 {
        match &s[i - 2..i + 1] {
            "one" => return Some(1),
            "two" => return Some(2),
            "six" => return Some(6),
            _ => {}
        }
    }

    let c = s.chars().nth(i).unwrap();
    if c.is_ascii_digit() {
        return Some(c.to_string().parse().unwrap());
    }

    None
}

#[test]
fn test() {
    let a = ans(r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#);
    assert_eq!(a, 142);
}

#[test]
fn test2() {
    let a = ans(r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#);
    assert_eq!(a, 281);
}

#[test]
fn run() {
    let q = include_str!("../inputs/input1.txt");
    println!("{}", ans(&q));
}
