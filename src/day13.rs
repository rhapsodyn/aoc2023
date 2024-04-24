pub fn ans1(input: &str) -> usize {
    let sp = input.split('\n');
    let mut block = vec![];
    let mut a = 0;
    for l in sp {
        if l.starts_with(".") || l.starts_with("#") {
            block.push(l.to_owned());
        } else {
            a += score(&block);
            block.clear();
        }
    }

    // last block
    a += score(&block);
    a
}

fn score(block: &Vec<String>) -> usize {
    if block.is_empty() {
        return 0;
    }

    let r = row_by_row(block);
    let a = if r == 0 {
        let c = col_by_col(block);
        assert!(c > 0);
        c
    } else {
        r * 100
    };

    // dbg!(a);
    a
}

fn row_by_row(input: &Vec<String>) -> usize {
    let mut a = 0;
    let len = input.len();
    let half_len = len / 2;
    // x row => x - 1 gap
    for i in 1..input.len() {
        let mut all_refl = true;
        let how_far = if i > half_len { len - i } else { i };
        for f in 1..how_far + 1 {
            let above = &input[i - f];
            let below = &input[i + f - 1];
            if above != below {
                all_refl = false;
                break;
            }
        }

        if all_refl {
            a = i;
            break;
        }
    }

    a
}

fn col_by_col(input: &Vec<String>) -> usize {
    row_by_row(&transpose(input))
}

fn transpose(input: &Vec<String>) -> Vec<String> {
    let len = input[0].len();
    let mut transpose = vec![vec!['_'; input.len()]; len];
    for (y, l) in input.iter().enumerate() {
        assert_eq!(l.len(), len);
        for (x, ch) in l.chars().enumerate() {
            transpose[x][y] = ch;
        }
    }

    transpose
        .into_iter()
        .map(|cs| cs.into_iter().collect())
        .collect()
}

pub fn ans2(input: &str) -> usize {
    let sp = input.split('\n');
    let mut block = vec![];
    let mut a = 0;
    for l in sp {
        if l.starts_with(".") || l.starts_with("#") {
            block.push(l.to_owned());
        } else {
            a += score2(&block);
            block.clear();
        }
    }

    // last block
    a += score2(&block);
    a
}

fn score2(block: &Vec<String>) -> usize {
    if block.is_empty() {
        return 0;
    }

    let (old, in_row) = r_or_c(block);
    let it = FixerIter::new(block);
    for shift in it {
        let a = r_or_c_non_stop(&shift, &(old, in_row));
        if a > 0 {
            return a;
        }
    }

    unreachable!()
}

#[test]
fn test_no_score() {
    let block: Vec<String> = [
        "..##..##...",
        "..#.##.#.##",
        "...#.#.#...",
        "##.#.####..",
        "...###.#.##",
        "######.#.##",
        "######..#.#",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    score2(&block);
}

fn r_or_c(block: &Vec<String>) -> (usize, bool) {
    let r = row_by_row(block);
    if r != 0 {
        (r, true)
    } else {
        let c = col_by_col(block);
        (c, false)
    }
}

fn r_or_c_non_stop(block: &Vec<String>, old: &(usize, bool)) -> usize {
    let rs = row_by_row_non_stop(block);
    let cs = col_by_col_non_stop(block);
    if old.1 {
        // old in row
        if let Some(new) = rs.iter().find(|r| r != &&old.0) {
            // diff row
            return *new * 100;
        }

        if let Some(c) = cs.first() {
            return *c;
        }
    } else {
        // old in col
        if let Some(new) = cs.iter().find(|c| c != &&old.0) {
            // diff col
            return *new;
        }

        if let Some(r) = rs.first() {
            return *r * 100;
        }
    }

    0
}

fn row_by_row_non_stop(input: &Vec<String>) -> Vec<usize> {
    let mut a = vec![];
    let len = input.len();
    let half_len = len / 2;
    // x row => x - 1 gap
    for i in 1..input.len() {
        let mut all_refl = true;
        let how_far = if i > half_len { len - i } else { i };
        for f in 1..how_far + 1 {
            let above = &input[i - f];
            let below = &input[i + f - 1];
            if above != below {
                all_refl = false;
                break;
            }
        }

        if all_refl {
            a.push(i);
        }
    }

    a
}

fn col_by_col_non_stop(input: &Vec<String>) -> Vec<usize> {
    row_by_row_non_stop(&transpose(input))
}

struct FixerIter<'a> {
    block: &'a Vec<String>,
    // y * n + x
    idx: usize,
}

impl<'a> FixerIter<'a> {
    fn new(block: &'a Vec<String>) -> FixerIter<'a> {
        FixerIter { block, idx: 0 }
    }
}

impl<'a> Iterator for FixerIter<'a> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let line_len = self.block[0].len();
        let total = self.block.len() * line_len;
        if self.idx == total {
            return None;
        }

        let mut view = vec![];
        for (y, line) in self.block.iter().enumerate() {
            let mut new_line = String::new();
            for (x, ch) in line.char_indices() {
                let i = y * line_len + x;
                new_line.push(if i == self.idx {
                    if ch == '.' {
                        '#'
                    } else {
                        '.'
                    }
                } else {
                    ch
                });
            }
            view.push(new_line);
        }

        self.idx += 1;
        Some(view)
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

#[test]
fn test1() {
    let a1 = ans1(TEST_INPUT);
    assert_eq!(a1, 405);
}

#[test]
fn run1() {
    println!("ans:{}", ans1(include_str!("../inputs/input13.txt")))
}

#[test]
fn test2() {
    let a2 = ans2(TEST_INPUT);
    assert_eq!(a2, 400);
}

#[test]
fn run2() {
    println!("ans:{}", ans2(include_str!("../inputs/input13.txt")))
}
