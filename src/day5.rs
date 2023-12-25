use std::ops::Range;

type Num = i64;

#[derive(Debug)]
struct Ranges {
    dest_start: Num,
    src_start: Num,
    len: Num,
}

type Maps = Vec<Vec<Ranges>>;

type Seeds = Vec<Num>;

fn parse(input: &str) -> (Seeds, Maps) {
    let mut lines = input.split('\n').filter(|l| l.len() > 0);

    let first_l = lines.next().unwrap();
    let seeds = parse_nums(&first_l[6..]);

    let mut maps = vec![];
    let mut cat_lines = vec![];
    while let Some(l) = lines.next() {
        match l.chars().nth(0).unwrap() {
            n if n.is_numeric() => {
                // dbg!(l);
                let nums = parse_nums(l);
                // dbg!(&nums);
                let r = Ranges {
                    dest_start: nums[0],
                    src_start: nums[1],
                    len: nums[2],
                };
                cat_lines.push(r);
            }
            s if s.is_ascii_alphabetic() => {
                if cat_lines.len() > 0 {
                    maps.push(cat_lines);
                }

                cat_lines = vec![];
            }
            _ => {}
        }
    }

    maps.push(cat_lines);

    (seeds, maps)
}

fn parse_nums(line: &str) -> Vec<Num> {
    line.split(' ')
        .filter(|seg| seg.len() > 0)
        .map(|seg| seg.parse::<Num>().unwrap())
        .collect()
}

pub fn ans1(input: &str) -> Num {
    let (mut seeds, maps) = parse(input);
    // dbg!(&seeds, &maps);

    for map in maps {
        for s in seeds.iter_mut() {
            *s = conv(*s, &map);
        }
    }

    *seeds.iter().min().unwrap()
}

fn conv(s: Num, map: &[Ranges]) -> Num {
    for r in map {
        if s >= r.src_start && s <= r.src_start + r.len {
            // dbg!(&r);
            return s + (r.dest_start - r.src_start);
        }
    }

    s
}

pub fn ans2(input: &str) -> Num {
    let (seeds, mut maps) = parse(input);
    let ranges = seeds_rangs(seeds);
    maps.reverse();

    // dbg!(&ranges, &maps);

    let mut i = 0;
    loop {
        let init_seed = conv2(i, &maps);
        if ranges.iter().any(|r| r.contains(&init_seed)) {
            break;
        }

        i += 1;
    }

    i
}

fn conv2(i: Num, maps: &[Vec<Ranges>]) -> Num {
    let mut j = i;
    for map in maps {
        for r in map {
            if j >= r.dest_start && j <= r.dest_start + r.len {
                j = j + (r.src_start - r.dest_start);
                break;
            }
        }
        // no match => keep unchange
    }

    j
}

fn seeds_rangs(seeds: Vec<Num>) -> Vec<Range<Num>> {
    let mut i = 0;
    let mut ranges = vec![];
    while i < seeds.len() {
        ranges.push(seeds[i]..(seeds[i] + seeds[i + 1]));
        i += 2;
    }
    ranges.sort_by(|l, r| l.start.cmp(&r.start));
    // dbg!(&ranges);

    ranges
}

#[cfg(test)]
mod tests {
    use crate::day2::ans2;

    const EG: &'static str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test1() {
        // assert_eq!(ans1(EG), 35);
    }

    #[test]
    fn run1() {
        // let q = include_str!("../inputs/input5.txt");
        // println!("ans1: {}", ans1(&q));
    }

    #[test]
    fn test2() {
        assert_eq!(ans2(EG), 46);
    }

    #[test]
    fn run2() {
        let q = include_str!("../inputs/input5.txt");
        println!("ans2: {}", ans2(&q));
    }
}
