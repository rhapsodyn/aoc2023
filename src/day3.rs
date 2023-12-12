use crate::common::Matrix;

pub fn ans1(input: &str) -> u32 {
    let matrix = Matrix::new(input);
    let mut numbers = vec![];
    let mut exclude_num = 0;
    for y in 0..matrix.height {
        let mut num_begin = -1;
        for x in 0..matrix.width {
            let ch = matrix.get_point_char(x, y).unwrap();
            if ch.is_ascii_digit() {
                if num_begin == -1 {
                    num_begin = x;
                }
            } else {
                if num_begin != -1 {
                    // number end
                    let mut symbol_around = false;
                    for i in num_begin..x {
                        if adjacent_to_symbol(&matrix, i, y) {
                            symbol_around = true;
                            break;
                        }
                    }

                    let n = matrix.get_line(y).unwrap()[num_begin as usize..x as usize]
                        .parse::<u32>()
                        .unwrap();
                    if symbol_around {
                        println!("include: y:{} x:{} {}", y, x, n);
                        numbers.push(n);
                    } else {
                        println!("exclude: y:{} x:{} {}", y, x, n);
                        exclude_num += 1;
                    }

                    num_begin = -1;
                }
            }
        }
        // last number
        if num_begin != -1 {
            let mut symbol_around = false;
            for i in num_begin..matrix.width - 1 {
                if adjacent_to_symbol(&matrix, i, y) {
                    symbol_around = true;
                    break;
                }
            }
            let n = matrix.get_line(y).unwrap()[num_begin as usize..matrix.width as usize]
                .parse::<u32>()
                .unwrap();
            if symbol_around {
                println!("include: y:{} x:last  {}", y, n);
                numbers.push(n);
            } else {
                println!("exclude: y:{} x:last {}", y, n);
                exclude_num += 1;
            }
        }
    }

    dbg!(&numbers.len());
    dbg!(exclude_num);

    numbers.iter().sum()
}

fn adjacent_to_symbol(matrix: &Matrix, x: i64, y: i64) -> bool {
    let arounds = around_8(x, y);
    for (x, y) in arounds {
        match matrix.get_point_char(x, y) {
            Some(c) => {
                if !c.is_ascii_digit() && c != '.' {
                    return true;
                }
            }
            None => {}
        }
    }

    false
}

fn around_8(x: i64, y: i64) -> [(i64, i64); 8] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

pub fn ans2(input: &str) -> u32 {
    let matrix = Matrix::new(input);
    let mut numbers = vec![];
    let mut muls = vec![];
    for y in 0..matrix.height {
        let mut num_begin = -1;
        for x in 0..matrix.width {
            let ch = matrix.get_point_char(x, y).unwrap();
            if ch.is_ascii_digit() {
                if num_begin == -1 {
                    num_begin = x;
                }
            } else {
                if ch == '*' {
                    muls.push((x, y));
                }
                if num_begin != -1 {
                    // number end
                    let n = matrix.get_line(y).unwrap()[num_begin as usize..x as usize]
                        .parse::<u32>()
                        .unwrap();
                    numbers.push(((x, y), n));

                    num_begin = -1;
                }
            }
        }
        if num_begin != -1 {
            // last number
            let n = matrix.get_line(y).unwrap()[num_begin as usize..matrix.width as usize]
                .parse::<u32>()
                .unwrap();
            numbers.push(((matrix.width - 1, y), n));
        }
    }

    dbg!(numbers.len());
    dbg!(&muls);

    let mut total = 0;
    for mul in muls {
        let nums = get_adjacent_nums(&numbers, &mul);
        if nums.len() == 2 {
            total += nums[0] * nums[1];
        }
    }

    total
}

fn get_adjacent_nums(numbers: &Vec<((i64, i64), u32)>, mul: &(i64, i64)) -> Vec<u32> {
    let arounds = around_8(mul.0, mul.1);
    numbers
        .iter()
        .filter(|((x, y), n)| {
            let x_start = x - n.to_string().len() as i64;
            arounds
                .iter()
                .any(|(ax, ay)| ay == y && (ax >= &x_start && ax < x))
        })
        .map(|(_, n)| *n)
        .collect()
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
        ),
        4361
    )
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input3.txt");
    println!("{}", ans1(&q));
}

#[test]
fn test2() {
    assert_eq!(
        ans2(
            r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
        ),
        467835
    )
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input3.txt");
    println!("{}", ans2(&q));
}
