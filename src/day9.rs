pub fn ans1(input: &str) -> i32 {
    let number_lines = input.split('\n').filter(|l| l.len() > 0).map(|l| {
        l.split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut sum = 0;
    for nl in number_lines {
        sum += extrapolate(nl);
    }
    sum
}

fn extrapolate(numbers: Vec<i32>) -> i32 {
    let size = numbers.len();
    let mut matrix = vec![];
    matrix.push(numbers);
    while matrix.last().unwrap().last().unwrap() != &0 {
        let mut next_line = vec![0; size];
        let prev_line = matrix.last().unwrap();
        for i in matrix.len()..size {
            next_line[i] = prev_line[i] - prev_line[i - 1];
        }
        matrix.push(next_line);
    }
    // dbg!(&matrix);
    matrix.iter().map(|l| l.last().unwrap()).sum()
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input9.txt");
    println!("{}", ans1(&q));
}

pub fn ans2(input: &str) -> i32 {
    let number_lines = input.split('\n').filter(|l| l.len() > 0).map(|l| {
        l.split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut sum = 0;
    for nl in number_lines {
        sum += extrapolate2(nl);
    }
    sum
}

fn extrapolate2(numbers: Vec<i32>) -> i32 {
    let size = numbers.len();
    let mut matrix = vec![];
    matrix.push(numbers);
    while matrix.last().unwrap().last().unwrap() != &0 {
        let mut next_line = vec![0; size];
        let prev_line = matrix.last().unwrap();
        for i in matrix.len()..size {
            next_line[i] = prev_line[i] - prev_line[i - 1];
        }
        matrix.push(next_line);
    }
    // dbg!(&matrix);
    matrix.reverse();
    let mut total = 0;
    let m_len = matrix.len();
    for (i, l) in matrix.iter().enumerate() {
        let first_num = l[m_len - i - 1];
        // dbg!(&first_num);
        total = first_num - total;
    }

    total
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
        ),
        2
    )
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input9.txt");
    println!("{}", ans2(&q));
}
