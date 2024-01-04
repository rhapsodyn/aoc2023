type Num = u64;

pub fn ans1(input: &str) -> Num {
    let mut lines = input.split('\n').filter(|l| l.len() > 0);
    let times = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|seg| match seg.chars().nth(0) {
            Some(c) => c.is_ascii_digit(),
            None => false,
        })
        .map(|seg| seg.parse::<Num>().unwrap())
        .collect::<Vec<Num>>();
    let distances = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|seg| match seg.chars().nth(0) {
            Some(c) => c.is_ascii_digit(),
            None => false,
        })
        .map(|seg| seg.parse::<Num>().unwrap())
        .collect::<Vec<Num>>();
    assert!(times.len() == distances.len());
    let mut t2d = vec![];
    for i in 0..times.len() {
        t2d.push((times[i], distances[i]));
    }
    t2d.into_iter()
        .map(|(t, d)| ways(t, d))
        .reduce(|acc, e| acc * e)
        .unwrap()
}

fn ways(t: Num, d: Num) -> Num {
    let mut n = 0;
    for hold in 1..t {
        if hold * (t - hold) > d {
            n += 1;
        }
    }
    n
}

pub fn ans2(input: &str) -> Num {
    let mut lines = input.split('\n').filter(|l| l.len() > 0);
    let time = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|seg| match seg.chars().nth(0) {
            Some(c) => c.is_ascii_digit(),
            None => false,
        })
        .fold(String::new(), |acc, e| acc + e)
        .parse::<Num>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|seg| match seg.chars().nth(0) {
            Some(c) => c.is_ascii_digit(),
            None => false,
        })
        .fold(String::new(), |acc, e| acc + e)
        .parse::<Num>()
        .unwrap();

    dbg!(&time, &distance);

    let mut n = 0;
    for hold in 1..time {
        if hold * (time - hold) > distance {
            n = hold;
            break;
        }
    }

    dbg!(n);

    time - n * 2 + 1
}

#[test]
fn test1() {
    //     assert_eq!(
    //         ans1(
    //             r#"
    // Time: 7 15 30
    // Distance: 9 40 200"#
    //         ),
    //         288
    //     )
}

#[test]
fn run1() {
    // let q = include_str!("../inputs/input6.txt");
    // println!("{}", ans1(&q));
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"
Time: 7 15 30 
Distance: 9 40 200"#
        ),
        71503
    )
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input6.txt");
    println!("{}", ans2(&q));
}
