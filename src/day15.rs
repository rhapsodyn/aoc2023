pub fn ans1(input: &str) -> u32 {
    let sp = input.trim().split(',');
    let mut sum = 0;
    for h in sp {
        let a = hash(h);
        sum += a;
    }
    sum
}

fn hash(input: &str) -> u32 {
    let mut curr_val = 0;
    for ch in input.chars() {
        let code = (ch as u8) as u32;
        curr_val += code;
        curr_val *= 17;
        curr_val %= 256;
    }
    curr_val
}

#[test]
fn test1() {
    assert_eq!(
        ans1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        1320
    )
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input15.txt");
    println!("{}", ans1(&q));
}

#[derive(Debug)]
struct Slot {
    hash_key: String,
    focal: u32,
}

type Dict = Vec<Vec<Slot>>;

pub fn ans2(input: &str) -> usize {
    let mut map: Dict = vec![];
    for _ in 0..256 {
        map.push(vec![]);
    }
    let sp = input.trim().split(',');
    for op in sp {
        if op.ends_with('-') {
            remove(&mut map, &op[0..op.len() - 1]);
        } else {
            let mut kv = op.split("=");
            insert(&mut map, kv.next().unwrap(), kv.next().unwrap());
        }
    }
    //dbg!(&map);

    let mut sum = 0;
    for (i, ss) in map.iter().enumerate() {
        for (j, s) in ss.iter().enumerate() {
            sum += (i + 1) * (j + 1) * s.focal as usize;
        }
    }

    sum
}

fn remove(map: &mut Dict, key: &str) {
    //dbg!("remove", key);
    let h = hash(key) as usize;
    let slots = &mut map[h];
    let mut found = None;
    for (i, e) in slots.iter().enumerate() {
        if e.hash_key == key {
            found = Some(i);
        }
    }

    if let Some(i) = found {
        slots.remove(i);
    }
}

fn insert(map: &mut Dict, key: &str, value: &str) {
    //dbg!("insert", key, value);
    let h = hash(key) as usize;
    let focal = value.parse().unwrap();
    let slots = &mut map[h];
    let mut found = false;
    for e in slots.iter_mut() {
        if e.hash_key == key {
            e.focal = focal;
            found = true;
            break;
        }
    }

    if !found {
        slots.push(Slot {
            hash_key: key.to_owned(),
            focal,
        })
    }
}

#[test]
fn test2() {
    assert_eq!(
        ans2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        145
    )
}

#[test]
fn run2() {
    let q = include_str!("../inputs/input15.txt");
    println!("{}", ans2(&q));
}
