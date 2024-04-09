use std::collections::HashMap;

pub fn ans1(input: &str) -> usize {
    input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(poss_arrg)
        .sum()
}

fn poss_arrg(line: &str) -> usize {
    let mut segs = line.split(' ').map(|s| s.to_owned()).collect::<Vec<_>>();

    let sizes = segs
        .pop()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let fields = segs.pop().unwrap();

    let mut all_possibles = vec![];
    for c in fields.chars() {
        if all_possibles.len() == 0 {
            if c == '?' {
                all_possibles.push(".".to_owned());
                all_possibles.push("#".to_owned());
            } else {
                all_possibles.push(String::from(c));
            }
        } else {
            if c == '?' {
                let mut dups = vec![];
                for s in all_possibles.iter_mut() {
                    let mut dup = s.clone();
                    dup.push('#');
                    dups.push(dup);
                    s.push('.');
                }
                all_possibles.append(&mut dups);
            } else {
                for s in all_possibles.iter_mut() {
                    s.push(c);
                }
            }
        }
    }

    all_possibles
        .iter()
        .filter(|s| {
            s.split('.')
                .filter(|s| s.len() > 0)
                .map(|damaged| damaged.len())
                .collect::<Vec<_>>()
                == sizes
        })
        .count()
}

#[test]
fn run1() {
    println!("{}", ans1(include_str!("../inputs/input12.txt")))
}

pub fn ans2_screwed(input: &str) -> usize {
    input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(poss_arrg2_screwed)
        .sum()
}

fn poss_arrg2_screwed(line: &str) -> usize {
    let mut segs = line.split(' ').map(|s| s.to_owned()).collect::<Vec<_>>();

    let mut sizes = segs
        .pop()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut fields = segs.pop().unwrap();

    sizes = sizes.repeat(5);
    fields = format!("{}?", fields).repeat(5);
    // dbg!(&sizes, &fields);

    // between two damaged, at least one operational
    let sure_count = sizes.iter().sum::<usize>() + sizes.len() - 1;
    let eggs = fields.len() - sure_count;
    let mut brackets = vec![0; sizes.len() + 1];
    brackets[0] = eggs;

    let mut n = 0;
    all_combo(&brackets, 0, &sizes, &fields, &mut n);

    n
}

fn is_possible(operational: &[usize], damaged: &[usize], fields: &str) -> bool {
    let mut guess = String::new();
    for i in 0..damaged.len() {
        guess.push_str(&".".repeat(operational[i]));
        guess.push_str(&"#".repeat(damaged[i]));
        if i < damaged.len() - 1 {
            guess.push('.');
        }
    }
    guess.push_str(&".".repeat(*operational.last().unwrap()));
    println!("guess: {}", guess);
    println!("real:  {}", fields);

    for (i, f) in fields.chars().enumerate() {
        let g = guess.chars().nth(i).unwrap();
        if f == '?' {
            continue;
        }

        if f != g {
            return false;
        }
    }

    true
}

fn all_combo(brackets: &[usize], i: usize, sizes: &[usize], fields: &str, n: &mut usize) {
    if brackets.len() < 2 {
        todo!("too few brackers")
    }

    println!("b:{:?} i:{}", &brackets, i);
    if is_possible(brackets, sizes, fields) {
        *n += 1;
    }
    // } else {
    //     return;
    // }

    if i == brackets.len() - 1 {
        return;
    }

    let mut copy = brackets.to_vec();
    while copy[i] != 0 {
        copy[i] -= 1;
        copy[i + 1] += 1;

        all_combo(&copy, i + 1, sizes, fields, n);
    }
}

type Cache = HashMap<(String, Vec<usize>), usize>;

pub fn ans2(input: &str) -> usize {
    let mut cache = Cache::new();
    let mut ans = 0;
    for l in input.split('\n').filter(|l| l.len() > 0) {
        ans += poss_arrg2(l, &mut cache);
    }

    ans
}

fn poss_arrg2(line: &str, cache: &mut Cache) -> usize {
    let mut segs = line.split(' ').map(|s| s.to_owned()).collect::<Vec<_>>();

    let mut sizes = segs
        .pop()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut fields = segs.pop().unwrap();

    sizes = sizes.repeat(5);
    fields = format!("{}?", fields).repeat(5);

    // let shrink = String::new();
    // for i in 0..fields. {
    //     if i > 0 {
    //
    //     }
    // }
    //
    recur(&fields, &sizes, cache, 0)
}

fn recur(fields: &str, sizes: &[usize], cache: &mut Cache, pretty: usize) -> usize {
    let key = (fields.to_owned(), sizes.to_owned());
    if let Some(v) = cache.get(&key) {
        *v
    } else {
        let n = _recur(fields, sizes, cache, pretty);
        cache.insert(key, n);
        n
    }
}

fn _recur(fields: &str, sizes: &[usize], cache: &mut Cache, pretty: usize) -> usize {
    if fields.len() == 0 {
        if sizes.len() > 0 {
            return 0;
        } else {
            return 1;
        }
    }

    if sizes.len() == 0 {
        if fields.chars().all(|c| c == '.' || c == '?') {
            // println!("!! got one");
            return 1;
        } else {
            return 0;
        }
    }

    let first_ch = fields.chars().nth(0).unwrap();
    let rest = &fields[1..];
    let n = match first_ch {
        '?' => {
            let p1 = format!(".{}", &rest);
            let p2 = format!("#{}", &rest);
            recur(&p1, sizes, cache, pretty) + recur(&p2, sizes, cache, pretty)
        }
        '.' => recur(rest, sizes, cache, pretty + 1),
        '#' => {
            // FIXME: have to be cont !!
            let cont_n = *sizes.first().unwrap();
            if cont_n > fields.len() {
                0
            } else {
                let prefix = if cont_n == fields.len() {
                    &fields
                } else {
                    &fields[0..cont_n]
                };
                if cont_n > rest.len() {
                    // not long enough
                    0
                } else if prefix.chars().any(|c| c == '.') {
                    // not conted
                    0
                } else if fields.chars().nth(cont_n) == Some('#') {
                    // oversized
                    0
                } else {
                    // have to be #.+ or ?.*
                    let shifted = sizes[1..].to_vec();
                    let mut rest2 = fields[cont_n..].to_owned();
                    if rest2.chars().nth(0) == Some('?') {
                        // first of rest have to be a .
                        rest2 = format!(".{}", &rest2[1..]);
                    }
                    // dbg!(&rest);
                    recur(&rest2, &shifted, cache, pretty + cont_n)
                }
            }
        }
        _ => unreachable!(),
    };

    n
}

#[test]
fn test2() {
    let mut c = HashMap::new();
    // assert_eq!(poss_arrg2(".??..??...?##. 1,1,3", &mut c), 4);
    // assert_eq!(poss_arrg2("????.######..#####. 1,6,5", &mut c), 4);
    // assert_eq!(poss_arrg2("???.### 1,1,3", &mut c), 1);
    // assert_eq!(poss_arrg2("?#?#?#?#?#?#?#? 1,3,1,6", &mut c), 1);
    // assert_eq!(poss_arrg2("?###???????? 3,2,1", &mut c), 10);

    assert_eq!(poss_arrg2("???.### 1,1,3", &mut c), 1);
    assert_eq!(poss_arrg2(".??..??...?##. 1,1,3", &mut c), 16384);
    assert_eq!(poss_arrg2("?#?#?#?#?#?#?#? 1,3,1,6", &mut c), 1);
    assert_eq!(poss_arrg2("????.#...#... 4,1,1", &mut c), 16);
    assert_eq!(poss_arrg2("????.######..#####. 1,6,5", &mut c), 2500);
    assert_eq!(poss_arrg2("?###???????? 3,2,1", &mut c), 506250);
}

#[test]
fn run2() {
    println!("{}", ans2(include_str!("../inputs/input12.txt")))
}

#[test]
fn test_all() {
    let empty: Vec<u8> = vec![];
    assert!(empty.iter().all(|i| i == &1));
}

#[test]
fn test_recur() {
    let mut c = HashMap::new();
    let n = recur("?##.", &vec![3], &mut c, 0);
    assert_eq!(n, 1);
}
